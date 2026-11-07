use gtk::prelude::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use gtk::{
    glib, Application, ApplicationWindow, Box, Button, EventControllerMotion, Label,
    Orientation, Revealer, RevealerTransitionType, SearchEntry, Separator, Stack,
    StackTransitionType, ToggleButton,
};
use protea_core::{DeviceClass, DeviceProfile, HardwareTier, Permission, ProteaIdentity, ProteaMode, ProteaState, StateStore};

/// A spawned application tracked in the taskbar. Protea has no compositor-level
/// window management yet (see docs/IMPLEMENTATION-STATUS.md), so a taskbar entry
/// can report whether its app is still running and let the user end it, but it
/// cannot focus/restore the app's actual window - that needs real Wayland
/// shell-protocol integration with Weston, which does not exist yet.
struct RunningApp {
    child: std::process::Child,
    button: Button,
}

type RunningApps = Rc<RefCell<Vec<RunningApp>>>;

const APP_ID: &str = "org.protea.os.desktop";

fn main() -> glib::ExitCode {
    // protea-desktop doubles as the launcher for the handful of bundled
    // apps (see discover_applications/the .desktop files in the overlay):
    // each one is just `protea-desktop --app <name>`, a second, independent
    // process. GApplication's single-instance enforcement needs a D-Bus
    // session bus to detect the existing instance, and this image has none
    // (no machine-id - see S99protea's "Unable to acquire session bus"
    // warning), so every spawned process runs its own real, local Activate
    // regardless of sharing the same application_id. That is relied upon
    // here: read our own "--app" flag from the real argv before GApplication
    // gets anywhere near it, then hand GApplication an argv containing only
    // the program name so it never tries to interpret "--app" itself as an
    // (unregistered) GOption and fail the process before activation.
    let args: Vec<String> = std::env::args().collect();
    let app_mode = args
        .iter()
        .position(|a| a == "--app")
        .and_then(|i| args.get(i + 1))
        .cloned();
    let argv0 = args.first().cloned().unwrap_or_else(|| "protea-desktop".to_string());

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        match app_mode.as_deref() {
            Some("settings") => {
                open_settings_window(app);
                return;
            }
            Some("notes") => {
                open_notes_window(app);
                return;
            }
            Some("calculator") => {
                open_calculator_window(app);
                return;
            }
            Some("mmino") => {
                open_unavailable_app_notice(app, "Mmino", "music player");
                return;
            }
            _ => {}
        }

        let store = StateStore::new(state_path());
        let state = load_or_initialize(&store);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Protea OS")
            .default_width(1100)
            .default_height(700)
            .build();
        // GTK draws its own client-side title bar (with minimize/maximize/
        // close icon buttons) unless told not to. This image has no icon
        // theme installed, so those icon lookups were failing - producing
        // "GtkImage ... reported baselines of minimum -2147483648" warnings
        // that then fed garbage sizing into the render pipeline and crashed
        // it. Protea is a fullscreen single-app shell with no need for a
        // window title bar at all, so just turn decorations off outright.
        window.set_decorated(false);
        // This image has no Xcursor theme installed (pulling one in via
        // Buildroot means enabling the whole X.org subsystem just for cursor
        // data files on a pure-Wayland system - not worth it). Without an
        // explicit cursor, GDK's Wayland backend tries to look up the
        // themed default cursor itself, gets back a garbage/uninitialised
        // size, and crashes trying to allocate the shared-memory buffer for
        // it ("Truncating shared memory file failed: Invalid argument").
        // A texture-based cursor needs no theme lookup at all.
        window.set_cursor(Some(&build_pointer_cursor()));

        let stack = Stack::new();
        stack.set_hexpand(true);
        stack.set_vexpand(true);
        stack.set_transition_type(StackTransitionType::Crossfade);
        stack.set_transition_duration(500);

        let (splash, splash_pulse_timer) = build_splash_page();
        stack.add_named(&splash, Some("splash"));

        let (lock_page, sign_in_button) = build_lock_page(&state);
        stack.add_named(&lock_page, Some("lock"));

        let shell_page = build_shell_page(app, &state);
        stack.add_named(&shell_page, Some("shell"));

        stack.set_visible_child_name("splash");

        {
            let stack = stack.clone();
            // RefCell so the closure (Fn, called at most once since it
            // returns Break) can still move the SourceId out to cancel it -
            // leaving the splash page's pulse timer running forever after
            // we switch away from it was a real, previously-unnoticed bug:
            // it kept calling progress.pulse() on an off-screen widget for
            // the rest of the process's life, landing squarely in the
            // window where every crash this session has consistently
            // happened (~4-5s after present, a fixed g_object_ref count,
            // identical regardless of app content) - a background timer
            // repeatedly invalidating a hidden widget is a much better fit
            // for "content-independent, timing-dependent" than anything
            // content-related tried before.
            let splash_pulse_timer = std::cell::RefCell::new(Some(splash_pulse_timer));
            glib::timeout_add_local(std::time::Duration::from_millis(2200), move || {
                stack.set_visible_child_name("lock");
                eprintln!("PROTEA_DESKTOP_LOCK_READY");
                if let Some(timer) = splash_pulse_timer.borrow_mut().take() {
                    timer.remove();
                }
                glib::ControlFlow::Break
            });
        }

        {
            let stack = stack.clone();
            sign_in_button.connect_clicked(move |_| {
                stack.set_visible_child_name("shell");
                eprintln!("PROTEA_DESKTOP_SIGNED_IN");
            });
        }

        if std::env::var("PROTEA_QEMU_TEST")
            .map(|value| value == "1")
            .unwrap_or(false)
        {
            eprintln!("PROTEA_DESKTOP_TEST_MODE");
            let sign_in_button = sign_in_button.clone();
            glib::timeout_add_seconds_local(3, move || {
                eprintln!("PROTEA_DESKTOP_TEST_SIGN_IN_ACTIVATING");
                sign_in_button.emit_clicked();
                glib::ControlFlow::Break
            });

            let stack = stack.clone();
            glib::timeout_add_seconds_local(2, move || {
                if stack.visible_child_name().as_deref() == Some("shell") {
                    eprintln!("PROTEA_DESKTOP_HEARTBEAT");
                }
                glib::ControlFlow::Continue
            });
        }

        window.set_child(Some(&stack));
        apply_css();
        window.present();
        eprintln!("PROTEA_DESKTOP_WINDOW_PRESENTED");

        // Requesting fullscreen before the compositor has done its first
        // configure/commit round-trip on this surface can hand GDK's
        // Wayland backend an invalid (effectively garbage) buffer size for
        // its very first frame, which crashes outright trying to create
        // the backing Cairo surface. Let the window complete one normal
        // frame first, then ask to go fullscreen.
        {
            let window = window.clone();
            glib::timeout_add_local(std::time::Duration::from_millis(400), move || {
                window.fullscreen();
                glib::ControlFlow::Break
            });
        }
    });

    app.run_with_args(&[argv0])
}

/// A small solid-circle pointer cursor built from raw pixel data rather than
/// a themed Xcursor lookup (see the comment at window.set_cursor's call
/// site for why that lookup crashes on this image).
fn build_pointer_cursor() -> gtk::gdk::Cursor {
    const SIZE: usize = 20;
    let mut pixels = vec![0u8; SIZE * SIZE * 4];
    let center = SIZE as f64 / 2.0;
    let radius = SIZE as f64 / 2.6;

    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as f64 - center;
            let dy = y as f64 - center;
            let dist = (dx * dx + dy * dy).sqrt();
            let idx = (y * SIZE + x) * 4;
            if dist <= radius {
                pixels[idx] = 255;
                pixels[idx + 1] = 255;
                pixels[idx + 2] = 255;
                pixels[idx + 3] = 255;
            } else if dist <= radius + 1.6 {
                pixels[idx] = 15;
                pixels[idx + 1] = 15;
                pixels[idx + 2] = 15;
                pixels[idx + 3] = 255;
            }
        }
    }

    let bytes = glib::Bytes::from_owned(pixels);
    let texture = gtk::gdk::MemoryTexture::new(
        SIZE as i32,
        SIZE as i32,
        gtk::gdk::MemoryFormat::R8g8b8a8,
        &bytes,
        SIZE * 4,
    );
    gtk::gdk::Cursor::from_texture(&texture, (SIZE / 2) as i32, (SIZE / 2) as i32, None)
}

fn build_splash_page() -> (Box, glib::SourceId) {
    let page = Box::new(Orientation::Vertical, 0);
    page.add_css_class("protea-splash");
    page.set_hexpand(true);
    page.set_vexpand(true);

    let centered = Box::new(Orientation::Vertical, 16);
    centered.set_halign(gtk::Align::Center);
    centered.set_valign(gtk::Align::Center);
    centered.set_hexpand(true);
    centered.set_vexpand(true);

    let logo = Label::new(Some("PROTEA"));
    logo.add_css_class("protea-logo");
    let tagline = Label::new(Some("One identity. One system. Adaptive everywhere."));
    tagline.add_css_class("dim-label");

    // A pulsing ProgressBar rather than a GtkSpinner: Spinner's internal
    // animation implementation was the source of a real crash on this
    // image (repeated "g_object_ref: assertion G_IS_OBJECT (object) failed"
    // criticals immediately after the window presented, then a fatal Gdk
    // shared-memory error). ProgressBar is a plain CSS-drawn rectangle with
    // no equivalent internal machinery.
    let progress = gtk::ProgressBar::new();
    progress.set_show_text(false);
    progress.set_width_request(220);
    progress.set_halign(gtk::Align::Center);
    let pulse_timer = {
        let progress = progress.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
            progress.pulse();
            glib::ControlFlow::Continue
        })
    };

    let status = Label::new(Some("Starting Protea..."));
    status.add_css_class("dim-label");

    centered.append(&logo);
    centered.append(&tagline);
    centered.append(&progress);
    centered.append(&status);
    page.append(&centered);
    (page, pulse_timer)
}

fn build_lock_page(state: &ProteaState) -> (Box, Button) {
    let page = Box::new(Orientation::Vertical, 0);
    page.add_css_class("protea-lockscreen");
    page.set_hexpand(true);
    page.set_vexpand(true);

    let centered = Box::new(Orientation::Vertical, 14);
    centered.set_halign(gtk::Align::Center);
    centered.set_valign(gtk::Align::Center);
    centered.set_hexpand(true);
    centered.set_vexpand(true);

    let display_name = state
        .identity
        .as_ref()
        .map(|i| i.display_name.clone())
        .unwrap_or_else(|| "Protea User".to_string());
    let initial = display_name.chars().next().unwrap_or('P').to_uppercase().to_string();

    let avatar = Label::new(Some(&initial));
    avatar.add_css_class("protea-avatar");
    avatar.set_size_request(96, 96);
    avatar.set_halign(gtk::Align::Center);

    let name_label = Label::new(Some(&display_name));
    name_label.add_css_class("protea-lock-name");

    let subtext = Label::new(Some("Local account  •  no password required"));
    subtext.add_css_class("dim-label");

    let sign_in_button = Button::with_label("Sign In");
    sign_in_button.add_css_class("suggested-action");
    sign_in_button.set_halign(gtk::Align::Center);
    sign_in_button.set_margin_top(10);

    centered.append(&avatar);
    centered.append(&name_label);
    centered.append(&subtext);
    centered.append(&sign_in_button);
    page.append(&centered);

    (page, sign_in_button)
}

fn build_desktop_page(app_stack: &Stack, state: &ProteaState) -> (Box, Button) {
    let content = Box::new(Orientation::Vertical, 18);
    content.set_vexpand(true);
    content.set_margin_top(32);
    content.set_margin_bottom(32);
    content.set_margin_start(32);
    content.set_margin_end(32);

    let title = Label::new(Some("PROTEA"));
    title.add_css_class("title-1");
    let subtitle = Label::new(Some("One identity. One system. Adaptive everywhere."));
    subtitle.add_css_class("dim-label");

    let welcome = Label::new(Some(
        "Hi, welcome to Protea. This is a new operating system — this is Protea OS.",
    ));
    welcome.add_css_class("protea-welcome");
    welcome.set_halign(gtk::Align::Start);
    welcome.set_wrap(true);

    let status = Label::new(Some(&format!(
        "Device: {:?}  •  Tier: {:?}  •  RAM: {} MB  •  Storage: {} MB  •  GPU: {}  •  Battery: {}  •  Mode: {:?}",
        state.device.class, state.device.tier, state.device.memory_mb, state.device.storage_mb,
        if state.device.has_gpu { "yes" } else { "no" },
        if state.device.has_battery { "yes" } else { "no" }, state.mode
    )));

    let start = Button::with_label("Start");
    start.add_css_class("suggested-action");

    let gaming = ToggleButton::with_label("Gaming");
    gaming.set_active(state.mode == ProteaMode::Gaming);
    let office = ToggleButton::with_label("Office");
    office.set_active(state.mode == ProteaMode::Office);
    gaming.set_group(Some(&office));

    let modes = Box::new(Orientation::Horizontal, 8);
    modes.append(&gaming);
    modes.append(&office);

    let identity = Label::new(Some(
        &state.identity
            .as_ref()
            .map(|i| format!("Signed in locally as {}", i.display_name))
            .unwrap_or_else(|| "No local identity".to_string()),
    ));

    let settings_count = Label::new(Some(
        &format!("Settings stored locally: {}", state.settings.len()),
    ));

    let separator = Separator::new(Orientation::Horizontal);

    content.append(&title);
    content.append(&subtitle);
    content.append(&welcome);
    content.append(&separator);
    content.append(&status);
    content.append(&identity);
    content.append(&settings_count);
    content.append(&modes);
    content.append(&start);

    let system = Box::new(Orientation::Horizontal, 8);
    let settings_button = Button::with_label("Settings");
    let reboot_button = Button::with_label("Restart");
    let shutdown_button = Button::with_label("Shut down");
    system.append(&settings_button);
    system.append(&reboot_button);
    system.append(&shutdown_button);
    content.append(&system);

    let settings_app_stack = app_stack.clone();
    settings_button.connect_clicked(move |_| {
        settings_app_stack.set_visible_child_name("settings");
    });

    reboot_button.connect_clicked(move |_| {
        request_power_action("reboot");
    });

    shutdown_button.connect_clicked(move |_| {
        request_power_action("poweroff");
    });

    let store_gaming = StateStore::new(state_path());
    let status_gaming = status.clone();
    gaming.connect_toggled(move |button| {
        if button.is_active() {
            let mut current = load_or_initialize(&store_gaming);
            current.set_mode(ProteaMode::Gaming);
            let _ = store_gaming.save(&current);
            apply_mode_policy(current.mode);
            status_gaming.set_text(&format!(
                "Device: {:?}  •  Tier: {:?}  •  Mode: Gaming",
                current.device.class, current.device.tier
            ));
        }
    });

    let store_office = StateStore::new(state_path());
    let status_office = status.clone();
    office.connect_toggled(move |button| {
        if button.is_active() {
            let mut current = load_or_initialize(&store_office);
            current.set_mode(ProteaMode::Office);
            let _ = store_office.save(&current);
            apply_mode_policy(current.mode);
            status_office.set_text(&format!(
                "Device: {:?}  •  Tier: {:?}  •  Mode: Office",
                current.device.class, current.device.tier
            ));
        }
    });

    apply_mode_policy(state.mode);

    (content, start)
}

/// The persistent, Windows-like shell shown after sign-in: an inner Stack
/// holding the desktop plus each bundled app as its own page, wrapped with
/// one taskbar that is never torn down when switching between them. This
/// replaces the old per-app `protea-desktop --app <name>` separate-process
/// windows as the primary way to use the bundled apps, trading per-app
/// process isolation for instant switching - Weston 14.0.1 has no
/// wlr-foreign-toplevel-management support, so a cross-process taskbar
/// could not focus/restore another client's window anyway (see
/// docs/IMPLEMENTATION-STATUS.md). The `--app` entry points still exist
/// standalone for debugging (see open_settings_window and friends).
fn build_shell_page(app: &Application, state: &ProteaState) -> Box {
    let page = Box::new(Orientation::Vertical, 0);
    page.set_hexpand(true);
    page.set_vexpand(true);

    let app_stack = Stack::new();
    app_stack.set_hexpand(true);
    app_stack.set_vexpand(true);
    app_stack.set_transition_type(StackTransitionType::Crossfade);
    app_stack.set_transition_duration(250);

    let (desktop_page, desktop_start_button) = build_desktop_page(&app_stack, state);
    app_stack.add_named(&desktop_page, Some("desktop"));

    {
        let back_stack = app_stack.clone();
        let settings_page = build_settings_page("← Back", move || {
            back_stack.set_visible_child_name("desktop");
        });
        app_stack.add_named(&settings_page, Some("settings"));
    }
    {
        let back_stack = app_stack.clone();
        let notes_page = build_notes_page("← Back", move || {
            back_stack.set_visible_child_name("desktop");
        });
        app_stack.add_named(&notes_page, Some("notes"));
    }
    {
        let back_stack = app_stack.clone();
        let calculator_page = build_calculator_page("← Back", move || {
            back_stack.set_visible_child_name("desktop");
        });
        app_stack.add_named(&calculator_page, Some("calculator"));
    }

    app_stack.set_visible_child_name("desktop");

    // Bottom taskbar, Windows-style: wrapped in a Revealer so it can
    // auto-hide, placed after `app_stack` (which is vexpand) so it stays
    // flush against the bottom edge of the screen.
    let taskbar = Box::new(Orientation::Horizontal, 10);
    taskbar.add_css_class("protea-taskbar");
    taskbar.set_margin_start(12);
    taskbar.set_margin_end(12);
    taskbar.set_margin_top(8);
    taskbar.set_margin_bottom(8);

    let start_menu = Button::with_label("Protea");
    taskbar.append(&start_menu);

    // Pinned quick-switch buttons: the actual "taskbar can switch between
    // them" behaviour - instant, no process spawn, no window-manager
    // protocol needed since it's all one Stack in one process.
    let pinned = Box::new(Orientation::Horizontal, 4);
    let pinned_apps: [(&str, &str); 4] = [
        ("Desktop", "desktop"),
        ("Settings", "settings"),
        ("Notes", "notes"),
        ("Calculator", "calculator"),
    ];
    for (label, target) in pinned_apps {
        let button = Button::with_label(label);
        let app_stack_for_pin = app_stack.clone();
        button.connect_clicked(move |_| {
            app_stack_for_pin.set_visible_child_name(target);
        });
        pinned.append(&button);
    }
    taskbar.append(&pinned);

    let running_box = Box::new(Orientation::Horizontal, 6);
    running_box.set_hexpand(true);
    taskbar.append(&running_box);
    let running_apps: RunningApps = Rc::new(RefCell::new(Vec::new()));

    {
        let running_apps = running_apps.clone();
        glib::timeout_add_seconds_local(2, move || {
            let mut apps = running_apps.borrow_mut();
            apps.retain_mut(|app| match app.child.try_wait() {
                Ok(Some(_status)) => {
                    app.button.unparent();
                    false
                }
                _ => true,
            });
            glib::ControlFlow::Continue
        });
    }

    let tray = Box::new(Orientation::Horizontal, 10);
    let tray_mode = Label::new(Some(&format!("{:?}", state.mode)));
    tray.append(&tray_mode);
    if state.device.has_battery {
        tray.append(&Label::new(Some("Battery")));
    }
    let clock = Label::new(Some(&current_datetime_string()));
    tray.append(&clock);
    taskbar.append(&tray);

    {
        let clock = clock.clone();
        glib::timeout_add_seconds_local(1, move || {
            clock.set_text(&current_datetime_string());
            glib::ControlFlow::Continue
        });
    }

    let revealer = Revealer::new();
    revealer.set_transition_type(RevealerTransitionType::SlideUp);
    revealer.set_transition_duration(200);
    revealer.set_reveal_child(true);
    revealer.set_child(Some(&taskbar));
    revealer.set_valign(gtk::Align::End);

    page.append(&app_stack);
    page.append(&revealer);

    install_taskbar_auto_hide(&page, &revealer);

    let start_menu_app = app.clone();
    let start_menu_running = running_apps.clone();
    let start_menu_running_box = running_box.clone();
    let start_menu_app_stack = app_stack.clone();
    start_menu.connect_clicked(move |_| {
        open_app_launcher(
            &start_menu_app,
            start_menu_running.clone(),
            &start_menu_running_box,
            &start_menu_app_stack,
        );
    });

    let start_app = app.clone();
    let start_running = running_apps.clone();
    let start_running_box = running_box.clone();
    let start_app_stack = app_stack.clone();
    desktop_start_button.connect_clicked(move |_| {
        open_app_launcher(&start_app, start_running.clone(), &start_running_box, &start_app_stack);
    });

    page
}

/// Reveals the bottom taskbar while the pointer is near the bottom edge of
/// the shell, and hides it again a short moment after the pointer moves
/// away - the same "auto-hide" behaviour as the Windows taskbar.
fn install_taskbar_auto_hide(page: &Box, revealer: &Revealer) {
    const HOT_ZONE_PX: f64 = 56.0;
    const HIDE_DELAY_MS: u64 = 900;

    let hide_timer: Rc<RefCell<Option<glib::SourceId>>> = Rc::new(RefCell::new(None));
    let motion = EventControllerMotion::new();

    {
        let revealer = revealer.clone();
        let hide_timer = hide_timer.clone();
        let page_for_height = page.clone();
        motion.connect_motion(move |_controller, _x, y| {
            let height = page_for_height.height() as f64;
            let near_bottom = height <= 0.0 || y >= height - HOT_ZONE_PX;

            if let Some(source) = hide_timer.borrow_mut().take() {
                source.remove();
            }

            if near_bottom {
                revealer.set_reveal_child(true);
            } else {
                let revealer = revealer.clone();
                let hide_timer_for_tick = hide_timer.clone();
                let source = glib::timeout_add_local(
                    std::time::Duration::from_millis(HIDE_DELAY_MS),
                    move || {
                        revealer.set_reveal_child(false);
                        *hide_timer_for_tick.borrow_mut() = None;
                        glib::ControlFlow::Break
                    },
                );
                *hide_timer.borrow_mut() = Some(source);
            }
        });
    }

    {
        let revealer = revealer.clone();
        let hide_timer = hide_timer.clone();
        motion.connect_leave(move |_controller| {
            if let Some(source) = hide_timer.borrow_mut().take() {
                source.remove();
            }
            let revealer = revealer.clone();
            let hide_timer_for_tick = hide_timer.clone();
            let source = glib::timeout_add_local(
                std::time::Duration::from_millis(HIDE_DELAY_MS),
                move || {
                    revealer.set_reveal_child(false);
                    *hide_timer_for_tick.borrow_mut() = None;
                    glib::ControlFlow::Break
                },
            );
            *hide_timer.borrow_mut() = Some(source);
        });
    }

    page.add_controller(motion);

    // Hide the taskbar automatically a few seconds after boot so the
    // "hidden by default, reveals on hover" behaviour is visible without
    // the user having to move the mouse away first.
    let revealer_for_initial_hide = revealer.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(3000), move || {
        revealer_for_initial_hide.set_reveal_child(false);
        glib::ControlFlow::Break
    });
}

/// Real wall-clock date/time, shelled out to `date` (BusyBox on this image)
/// rather than hand-rolled calendar math. This image has no timezone
/// database configured, so the system clock is UTC - that is still the
/// correct, live, host-synced time (QEMU/the real hardware clock already
/// supplies it), just labelled honestly rather than implied to be a local
/// zone we have not actually configured.
fn current_datetime_string() -> String {
    std::process::Command::new("date")
        .arg("+%A, %d %B %Y  —  %H:%M:%S")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|text| format!("{} UTC", text.trim()))
        .unwrap_or_else(|| "Time unavailable".to_string())
}

fn state_path() -> std::path::PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        return std::path::PathBuf::from(home).join(".local/share/protea/state");
    }
    std::path::PathBuf::from(".protea/state")
}

fn load_or_initialize(store: &StateStore) -> ProteaState {
    if let Ok(state) = store.load() {
        return state;
    }

    let memory_mb = detect_memory_mb();
    let mut state = ProteaState::new(DeviceProfile::new(
        DeviceClass::Pc,
        HardwareTier::from_memory_mb(memory_mb),
        memory_mb,
        detect_storage_mb(),
        detect_gpu(),
        detect_battery(),
    ));
    state.set_identity(ProteaIdentity::new("local-0001", "Protea User"));
    state.settings.set("theme", "coral");
    state.permissions.grant(Permission::DeviceInformation);
    state.permissions.grant(Permission::Network);
    let _ = store.save(&state);
    state
}

fn detect_memory_mb() -> u64 {
    std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|text| {
            text.lines()
                .find(|line| line.starts_with("MemTotal:"))
                .and_then(|line| line.split_whitespace().nth(1))
                .and_then(|value| value.parse::<u64>().ok())
                .map(|kb| kb / 1024)
        })
        .unwrap_or(0)
}

const WALLPAPER_DIR: &str = "/usr/share/protea/wallpapers";

/// Bundled wallpapers (licensed South African heritage photography plus
/// Protea branding art - see assets/wallpapers/CREDITS.md), sorted for a
/// stable, predictable order wherever they're listed (here and in the
/// Settings wallpaper picker). Empty when the directory is absent, e.g.
/// running the shell outside the built Protea image during development.
fn list_wallpapers() -> Vec<std::path::PathBuf> {
    let mut entries: Vec<_> = std::fs::read_dir(WALLPAPER_DIR)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("jpg") | Some("jpeg") | Some("png")
            )
        })
        .collect();
    entries.sort();
    entries
}

/// Picks the wallpaper to show: the user's saved choice from Settings if
/// one is set (an empty "wallpaper" setting means "no preference"), else
/// falls back to the current time as a cheap, dependency-free source of
/// variety. Returns None when there are no bundled wallpapers at all.
fn select_wallpaper() -> Option<std::path::PathBuf> {
    // Confirmed by direct test: the crash still happens identically with
    // this returning None unconditionally (see git history for the
    // temporary diagnostic that proved it) - and separately, the CI boot
    // test never clicks "Sign In" so it never even reaches the desktop
    // page this CSS applies to. Wallpaper loading was never the cause;
    // restored to real behaviour.
    let entries = list_wallpapers();
    if entries.is_empty() {
        return None;
    }

    if let Ok(state) = StateStore::new(state_path()).load() {
        if let Some(chosen) = state.settings.get("wallpaper") {
            if !chosen.is_empty() {
                if let Some(path) = entries
                    .iter()
                    .find(|path| path.file_name().and_then(|name| name.to_str()) == Some(chosen))
                {
                    return Some(path.clone());
                }
            }
        }
    }

    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let index = (nanos as usize) % entries.len();
    Some(entries[index].clone())
}

fn apply_css() {
    let provider = gtk::CssProvider::new();
    let background = match select_wallpaper() {
        Some(path) => format!(
            // "contain" rather than "cover": one bundled wallpaper
            // (richtersveld.jpg) has an extreme ~7:1 banner aspect ratio.
            // "cover" would need to scale that up enormously to fill a
            // normal screen height, which is a strong candidate for the
            // "too big" Cairo surface crash that started appearing the
            // moment wallpapers and JPEG decoding both actually started
            // working for the first time. "contain" never scales up
            // anywhere near as aggressively regardless of source aspect
            // ratio, at the cost of occasional letterboxing.
            "window {{ background-image: url(\"{}\"); background-size: contain; background-position: center; background-repeat: no-repeat; }}",
            path.display()
        ),
        None => "window { background: #17120f; }".to_string(),
    };
    provider.load_from_data(&format!(
        "{background}
        .title-1 {{ font-weight: 800; font-size: 34px; }}
        .protea-logo {{ font-weight: 900; font-size: 56px; letter-spacing: 4px; color: #f4e9c1; }}
        .protea-splash {{ background-color: #17120f; }}
        .protea-lockscreen {{ background-color: rgba(0,0,0,0.45); }}
        .protea-lock-name {{ font-weight: 700; font-size: 22px; }}
        .protea-avatar {{ background-color: #c0453a; border-radius: 48px; color: white; font-weight: 700; font-size: 34px; }}
        .protea-welcome {{ font-size: 15px; font-style: italic; color: #f4e9c1; }}
        .protea-taskbar {{ background-color: rgba(23,18,15,0.78); }}
        "
    ));
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}


fn detect_storage_mb() -> u64 {
    std::process::Command::new("df")
        .args(["-Pm", "/"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .and_then(|text| text.lines().nth(1).map(str::to_string))
        .and_then(|line| line.split_whitespace().nth(1).and_then(|v| v.parse::<u64>().ok()))
        .unwrap_or(0)
}

fn detect_gpu() -> bool {
    std::fs::read_dir("/sys/class/drm")
        .ok()
        .map(|entries| entries.flatten().any(|entry| {
            entry.file_name().to_string_lossy().starts_with("card")
        }))
        .unwrap_or(false)
}

fn detect_battery() -> bool {
    std::fs::read_dir("/sys/class/power_supply")
        .ok()
        .map(|entries| entries.flatten().any(|entry| {
            let name = entry.file_name().to_string_lossy().to_ascii_uppercase();
            name.starts_with("BAT")
        }))
        .unwrap_or(false)
}

/// eth0 is the only interface Protea brings up (see build/pc-overlay's
/// S20network: `ip link set eth0 up` + `udhcpc -i eth0`) - there is no
/// Wi-Fi stack and no static-IP configuration to show, so this is a
/// status readout, not a settings surface. `ip` is already relied on by
/// S20network, so it is known to be present on this image.
fn detect_network_status() -> String {
    let operstate = std::fs::read_to_string("/sys/class/net/eth0/operstate")
        .map(|value| value.trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    if operstate != "up" {
        return "eth0: not connected".to_string();
    }

    let output_text = std::process::Command::new("ip")
        .args(["-4", "-o", "addr", "show", "eth0"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok());

    let address = output_text.as_deref().and_then(|text| {
        let mut words = text.split_whitespace();
        words.find(|&word| word == "inet")?;
        words.next()
    }).map(|cidr| cidr.split('/').next().unwrap_or(cidr).to_string());

    match address {
        Some(ip) => format!("eth0: {ip} (DHCP)"),
        None => "eth0: connected, no address yet".to_string(),
    }
}


/// Where a launcher entry's Exec command should go. Protea's own bundled
/// apps are all `protea-desktop --app <name>`; recognising that pattern
/// lets the launcher switch the shell's app_stack in place instead of
/// spawning a second process - the one piece of real window management
/// Protea has without compositor-level foreign-toplevel support (see
/// docs/IMPLEMENTATION-STATUS.md). Anything else falls back to a real
/// spawned process, same as before.
enum LauncherTarget {
    InternalPage(&'static str),
    Unavailable(&'static str, &'static str),
}

fn classify_internal_app(command: &[String]) -> Option<LauncherTarget> {
    let pos = command.iter().position(|arg| arg == "--app")?;
    match command.get(pos + 1)?.as_str() {
        "settings" => Some(LauncherTarget::InternalPage("settings")),
        "notes" => Some(LauncherTarget::InternalPage("notes")),
        "calculator" => Some(LauncherTarget::InternalPage("calculator")),
        "mmino" => Some(LauncherTarget::Unavailable("Mmino", "music player")),
        _ => None,
    }
}

fn open_app_launcher(
    app: &Application,
    running_apps: RunningApps,
    running_box: &Box,
    app_stack: &Stack,
) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Protea Start Menu")
        .default_width(520)
        .default_height(620)
        .build();

    let root = Box::new(Orientation::Vertical, 12);
    root.set_margin_top(20);
    root.set_margin_bottom(20);
    root.set_margin_start(20);
    root.set_margin_end(20);

    let title = Label::new(Some("Protea"));
    title.add_css_class("title-2");
    root.append(&title);

    let search = SearchEntry::new();
    search.set_placeholder_text(Some("Search applications"));
    root.append(&search);

    let list = gtk::ListBox::new();
    list.set_selection_mode(gtk::SelectionMode::None);

    let applications = discover_applications();
    if applications.is_empty() {
        list.append(&Label::new(Some("No desktop applications were found.")));
    } else {
        for (name, command) in applications {
            let button = Button::with_label(&name);
            button.set_halign(gtk::Align::Fill);
            button.set_hexpand(true);
            let launch_command = command.clone();
            let launch_name = name.clone();
            let running_apps = running_apps.clone();
            let running_box = running_box.clone();
            let window_for_launch = window.clone();
            let app_for_launch = app.clone();
            let app_stack_for_launch = app_stack.clone();
            button.connect_clicked(move |_| {
                if let Some(target) = classify_internal_app(&launch_command) {
                    window_for_launch.close();
                    match target {
                        LauncherTarget::InternalPage(page_name) => {
                            app_stack_for_launch.set_visible_child_name(page_name);
                        }
                        LauncherTarget::Unavailable(name, kind) => {
                            open_unavailable_app_notice(&app_for_launch, name, kind);
                        }
                    }
                    return;
                }

                match launch_application(&launch_command) {
                    Ok(child) => {
                        spawn_tracked(child, &launch_name, running_apps.clone(), &running_box);
                        window_for_launch.close();
                    }
                    Err(error) => {
                        eprintln!("Protea: could not launch {launch_name}: {error}");
                    }
                }
            });

            let row = gtk::ListBoxRow::new();
            row.set_child(Some(&button));
            row.set_widget_name(&name.to_ascii_lowercase());
            list.append(&row);
        }
    }

    let list_for_filter = list.clone();
    search.connect_search_changed(move |entry| {
        let query = entry.text().to_string().to_ascii_lowercase();
        list_for_filter.set_filter_func(move |row| {
            query.is_empty() || row.widget_name().to_ascii_lowercase().contains(&query)
        });
    });

    root.append(&list);
    window.set_child(Some(&root));
    window.present();
    search.grab_focus();
}

/// Adds a taskbar entry for a just-launched app. Protea cannot focus or
/// restore the app's real window yet (no compositor shell-protocol
/// integration), so the entry's one honest action is ending the process.
fn spawn_tracked(child: std::process::Child, name: &str, running_apps: RunningApps, running_box: &Box) {
    let kill_button = Button::with_label(name);
    kill_button.set_tooltip_text(Some("Running - click to end this application"));
    running_box.append(&kill_button);
    running_apps.borrow_mut().push(RunningApp { child, button: kill_button.clone() });

    let running_apps_for_kill = running_apps.clone();
    let name_for_kill = name.to_string();
    kill_button.connect_clicked(move |button| {
        let mut apps = running_apps_for_kill.borrow_mut();
        if let Some(pos) = apps.iter().position(|running| &running.button == button) {
            let mut running = apps.remove(pos);
            if let Err(error) = running.child.kill() {
                eprintln!("Protea: could not stop {name_for_kill}: {error}");
            }
            running.button.unparent();
        }
    });
}

fn discover_applications() -> Vec<(String, Vec<String>)> {
    let mut apps = BTreeMap::<String, Vec<String>>::new();
    let mut directories = vec![std::path::PathBuf::from("/usr/share/applications")];

    if let Ok(home) = std::env::var("HOME") {
        directories.push(std::path::PathBuf::from(home).join(".local/share/applications"));
    }

    for directory in directories {
        let Ok(entries) = std::fs::read_dir(directory) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|v| v.to_str()) != Some("desktop") {
                continue;
            }

            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let Some((name, exec, hidden, no_display, try_exec, terminal)) =
                parse_desktop_entry(&text)
            else {
                continue;
            };

            if hidden || no_display || terminal || !desktop_entry_supported(&try_exec) {
                continue;
            }

            let Some(command) = parse_exec_command(&exec) else {
                continue;
            };

            apps.entry(name).or_insert(command);
        }
    }

    apps.into_iter().collect()
}

fn parse_desktop_entry(
    text: &str,
) -> Option<(String, String, bool, bool, Option<String>, bool)> {
    let mut in_entry = false;
    let mut name = None;
    let mut exec = None;
    let mut hidden = false;
    let mut no_display = false;
    let mut try_exec = None;
    let mut terminal = false;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line == "[Desktop Entry]" {
            in_entry = true;
            continue;
        }
        if !in_entry {
            continue;
        }
        if line.starts_with('[') {
            break;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let value = value.trim();

        match key {
            "Type" if value != "Application" => return None,
            "Name" if name.is_none() => name = Some(value.to_string()),
            "Exec" if exec.is_none() => exec = Some(value.to_string()),
            "Hidden" => hidden = value.eq_ignore_ascii_case("true"),
            "NoDisplay" => no_display = value.eq_ignore_ascii_case("true"),
            "TryExec" if !value.is_empty() => try_exec = Some(value.to_string()),
            "Terminal" => terminal = value.eq_ignore_ascii_case("true"),
            _ => {}
        }
    }

    Some((name?, exec?, hidden, no_display, try_exec, terminal))
}

fn desktop_entry_supported(try_exec: &Option<String>) -> bool {
    let Some(program) = try_exec else {
        return true;
    };

    if program.contains('/') {
        return std::path::Path::new(program).is_file();
    }

    std::env::var_os("PATH")
        .map(|path| std::env::split_paths(&path).any(|dir| dir.join(program).is_file()))
        .unwrap_or(false)
}

fn parse_exec_command(exec: &str) -> Option<Vec<String>> {
    let tokens = shell_like_tokens(exec)?;
    let mut command = Vec::with_capacity(tokens.len());

    for token in tokens {
        if token.starts_with('%') {
            continue;
        }

        let cleaned = token.replace("%u", "").replace("%U", "").replace("%f", "").replace("%F", "");
        if !cleaned.is_empty() {
            command.push(cleaned);
        }
    }

    if command.is_empty() {
        None
    } else {
        Some(command)
    }
}

fn shell_like_tokens(input: &str) -> Option<Vec<String>> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut quote: Option<char> = None;

    while let Some(ch) = chars.next() {
        if let Some(q) = quote {
            if ch == q {
                quote = None;
            } else if q == '"' && ch == '\\' {
                let next = chars.next()?;
                current.push(next);
            } else {
                current.push(ch);
            }
            continue;
        }

        match ch {
            '"' | '\'' => quote = Some(ch),
            '\\' => {
                let next = chars.next()?;
                current.push(next);
            }
            c if c.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            c => current.push(c),
        }
    }

    if quote.is_some() {
        return None;
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    Some(tokens)
}


fn launch_application(command: &[String]) -> std::io::Result<std::process::Child> {
    let Some(program) = command.first() else {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "empty command"));
    };

    let mut process = std::process::Command::new(program);
    if command.len() > 1 {
        process.args(&command[1..]);
    }
    process.spawn()
}


/// Settings' real content, shared by the in-shell "settings" stack page and
/// the standalone `--app settings` debug window - `back_label`/`on_back`
/// are the only difference between the two (switch the shell's app_stack
/// back vs. close a real window).
fn build_settings_page(back_label: &str, on_back: impl Fn() + 'static) -> Box {
    let store = StateStore::new(state_path());
    let state = load_or_initialize(&store);

    let root = Box::new(Orientation::Vertical, 14);
    root.set_margin_top(24);
    root.set_margin_bottom(24);
    root.set_margin_start(24);
    root.set_margin_end(24);

    let title = Label::new(Some("Protea Settings"));
    title.add_css_class("title-2");
    root.append(&title);

    root.append(&Label::new(Some(&format!(
        "Device: {:?}  •  Tier: {:?}  •  RAM: {} MB  •  Storage: {} MB  •  GPU: {}  •  Battery: {}  •  Mode: {:?}",
        state.device.class, state.device.tier, state.device.memory_mb, state.device.storage_mb,
        if state.device.has_gpu { "yes" } else { "no" },
        if state.device.has_battery { "yes" } else { "no" }, state.mode
    ))));

    root.append(&Label::new(Some(&detect_network_status())));
    root.append(&Separator::new(Orientation::Horizontal));

    let name_row = Box::new(Orientation::Horizontal, 10);
    let name_label = Label::new(Some("Display name"));
    name_label.set_width_chars(12);
    let name_entry = gtk::Entry::new();
    name_entry.set_hexpand(true);
    name_entry.set_text(
        state.identity.as_ref().map(|identity| identity.display_name.as_str()).unwrap_or("Protea User"),
    );
    name_row.append(&name_label);
    name_row.append(&name_entry);
    root.append(&name_row);

    let theme_row = Box::new(Orientation::Horizontal, 10);
    let theme_label = Label::new(Some("Theme"));
    theme_label.set_width_chars(12);
    let theme_entry = gtk::Entry::new();
    theme_entry.set_hexpand(true);
    theme_entry.set_text(state.settings.get("theme").unwrap_or("coral"));
    theme_row.append(&theme_label);
    theme_row.append(&theme_entry);
    root.append(&theme_row);

    // "Random" is always index 0, so an unset/empty "wallpaper" setting
    // (see select_wallpaper) maps straight back to it.
    let mut wallpaper_options: Vec<String> = vec!["Random".to_string()];
    wallpaper_options.extend(
        list_wallpapers()
            .iter()
            .filter_map(|path| path.file_name().and_then(|name| name.to_str()).map(String::from)),
    );
    let option_refs: Vec<&str> = wallpaper_options.iter().map(String::as_str).collect();

    let wallpaper_row = Box::new(Orientation::Horizontal, 10);
    let wallpaper_label = Label::new(Some("Wallpaper"));
    wallpaper_label.set_width_chars(12);
    let wallpaper_dropdown = gtk::DropDown::from_strings(&option_refs);
    wallpaper_dropdown.set_hexpand(true);
    let current_wallpaper = state.settings.get("wallpaper").unwrap_or("");
    let initial_index = if current_wallpaper.is_empty() {
        0
    } else {
        wallpaper_options.iter().position(|name| name == current_wallpaper).unwrap_or(0) as u32
    };
    wallpaper_dropdown.set_selected(initial_index);
    wallpaper_row.append(&wallpaper_label);
    wallpaper_row.append(&wallpaper_dropdown);
    root.append(&wallpaper_row);

    let info = Label::new(Some(&format!(
        "{} local settings. Changes are written to the local Protea state file.",
        state.settings.len()
    )));
    info.add_css_class("dim-label");
    root.append(&info);

    let actions = Box::new(Orientation::Horizontal, 10);
    let save = Button::with_label("Save");
    save.add_css_class("suggested-action");
    let back = Button::with_label(back_label);
    actions.append(&save);
    actions.append(&back);
    root.append(&actions);

    let save_store = StateStore::new(state_path());
    let save_name_entry = name_entry.clone();
    let save_theme_entry = theme_entry.clone();
    let save_info = info.clone();
    save.connect_clicked(move |_| {
        let mut current = load_or_initialize(&save_store);

        let display_name = save_name_entry.text().trim().to_string();
        if display_name.is_empty() {
            save_info.set_text("Display name cannot be empty.");
            return;
        }
        let theme = save_theme_entry.text().trim().to_string();
        if theme.is_empty() {
            save_info.set_text("Theme cannot be empty.");
            return;
        }

        let local_id = current.identity.as_ref()
            .map(|identity| identity.local_id.clone())
            .unwrap_or_else(|| "local-0001".to_string());
        current.set_identity(ProteaIdentity::new(local_id, display_name));
        current.settings.set("theme", theme);

        let selected_wallpaper = wallpaper_options
            .get(wallpaper_dropdown.selected() as usize)
            .cloned()
            .unwrap_or_else(|| "Random".to_string());
        current.settings.set(
            "wallpaper",
            if selected_wallpaper == "Random" { String::new() } else { selected_wallpaper },
        );

        match save_store.save(&current) {
            Ok(()) => {
                save_info.set_text("Settings saved locally.");
                apply_css();
            }
            Err(error) => save_info.set_text(&format!("Could not save settings: {error:?}")),
        }
    });

    back.connect_clicked(move |_| on_back());

    root
}

/// Standalone `protea-desktop --app settings` debug entry point: wraps the
/// shared page in a real window instead of switching the shell's app_stack.
fn open_settings_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Protea Settings")
        .default_width(600)
        .default_height(520)
        .build();

    let window_close = window.clone();
    let page = build_settings_page("Close", move || window_close.close());
    window.set_child(Some(&page));
    window.present();
}

fn notes_path() -> std::path::PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        return std::path::PathBuf::from(home).join(".local/share/protea/notes.txt");
    }
    std::path::PathBuf::from(".protea/notes.txt")
}

/// Notes' real content, shared by the in-shell "notes" stack page and the
/// standalone `--app notes` debug window.
fn build_notes_page(back_label: &str, on_back: impl Fn() + 'static) -> Box {
    let path = notes_path();

    let root = Box::new(Orientation::Vertical, 10);
    root.set_margin_top(16);
    root.set_margin_bottom(16);
    root.set_margin_start(16);
    root.set_margin_end(16);

    let title = Label::new(Some("Ngwala Notes"));
    title.add_css_class("title-2");
    root.append(&title);

    let scroller = gtk::ScrolledWindow::new();
    scroller.set_vexpand(true);
    let text_view = gtk::TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::WordChar);
    let existing = std::fs::read_to_string(&path).unwrap_or_default();
    text_view.buffer().set_text(&existing);
    scroller.set_child(Some(&text_view));
    root.append(&scroller);

    let status = Label::new(Some("Notes are saved locally to your Protea profile."));
    status.add_css_class("dim-label");
    root.append(&status);

    let actions = Box::new(Orientation::Horizontal, 8);
    let save = Button::with_label("Save");
    save.add_css_class("suggested-action");
    let back = Button::with_label(back_label);
    actions.append(&save);
    actions.append(&back);
    root.append(&actions);

    let save_path = path.clone();
    let save_buffer = text_view.buffer();
    let save_status = status.clone();
    save.connect_clicked(move |_| {
        let (start, end) = (save_buffer.start_iter(), save_buffer.end_iter());
        let text = save_buffer.text(&start, &end, false);
        if let Some(parent) = save_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        match std::fs::write(&save_path, text.as_str()) {
            Ok(()) => save_status.set_text("Saved."),
            Err(error) => save_status.set_text(&format!("Could not save: {error}")),
        }
    });

    back.connect_clicked(move |_| on_back());

    root
}

/// Standalone `protea-desktop --app notes` debug entry point.
fn open_notes_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Ngwala — Notes")
        .default_width(480)
        .default_height(560)
        .build();

    let window_close = window.clone();
    let page = build_notes_page("Close", move || window_close.close());
    window.set_child(Some(&page));
    window.present();
}

type CalculatorState = Rc<RefCell<(f64, Option<char>, String)>>;

/// Calculator's real content, shared by the in-shell "calculator" stack
/// page and the standalone `--app calculator` debug window.
fn build_calculator_page(back_label: &str, on_back: impl Fn() + 'static) -> Box {
    let root = Box::new(Orientation::Vertical, 10);
    root.set_margin_top(16);
    root.set_margin_bottom(16);
    root.set_margin_start(16);
    root.set_margin_end(16);

    let header = Box::new(Orientation::Horizontal, 8);
    let back = Button::with_label(back_label);
    header.append(&back);
    root.append(&header);

    let display = Label::new(Some("0"));
    display.set_halign(gtk::Align::End);
    display.add_css_class("title-2");
    root.append(&display);

    let state: CalculatorState = Rc::new(RefCell::new((0.0, None, String::new())));

    let grid = gtk::Grid::new();
    grid.set_row_spacing(6);
    grid.set_column_spacing(6);
    grid.set_vexpand(true);

    let buttons: [(&str, i32, i32); 16] = [
        ("7", 0, 0), ("8", 1, 0), ("9", 2, 0), ("/", 3, 0),
        ("4", 0, 1), ("5", 1, 1), ("6", 2, 1), ("*", 3, 1),
        ("1", 0, 2), ("2", 1, 2), ("3", 2, 2), ("-", 3, 2),
        ("0", 0, 3), ("C", 1, 3), ("=", 2, 3), ("+", 3, 3),
    ];

    for (label, col, row) in buttons {
        let button = Button::with_label(label);
        button.set_hexpand(true);
        button.set_vexpand(true);
        let state = state.clone();
        let display = display.clone();
        let input = label.to_string();
        button.connect_clicked(move |_| {
            calculator_handle_input(&state, &display, &input);
        });
        grid.attach(&button, col, row, 1, 1);
    }

    root.append(&grid);
    back.connect_clicked(move |_| on_back());
    root
}

/// Standalone `protea-desktop --app calculator` debug entry point.
fn open_calculator_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Bala — Calculator")
        .default_width(300)
        .default_height(420)
        .resizable(false)
        .build();

    let window_close = window.clone();
    let page = build_calculator_page("Close", move || window_close.close());
    window.set_child(Some(&page));
    window.present();
}

fn calculator_handle_input(state: &CalculatorState, display: &Label, input: &str) {
    let mut s = state.borrow_mut();
    match input {
        "C" => {
            s.0 = 0.0;
            s.1 = None;
            s.2.clear();
            display.set_text("0");
        }
        "+" | "-" | "*" | "/" => {
            let entry: f64 = s.2.parse().unwrap_or(0.0);
            apply_pending(&mut s, entry);
            s.1 = input.chars().next();
            s.2.clear();
            display.set_text(&format_number(s.0));
        }
        "=" => {
            let entry: f64 = s.2.parse().unwrap_or(0.0);
            apply_pending(&mut s, entry);
            s.1 = None;
            s.2.clear();
            display.set_text(&format_number(s.0));
        }
        digit => {
            s.2.push_str(digit);
            display.set_text(&s.2);
        }
    }
}

fn apply_pending(s: &mut (f64, Option<char>, String), entry: f64) {
    match s.1 {
        Some('+') => s.0 += entry,
        Some('-') => s.0 -= entry,
        Some('*') => s.0 *= entry,
        Some('/') => {
            if entry != 0.0 {
                s.0 /= entry;
            }
        }
        None => s.0 = entry,
        _ => {}
    }
}

fn format_number(n: f64) -> String {
    if (n - n.trunc()).abs() < f64::EPSILON {
        format!("{}", n as i64)
    } else {
        format!("{:.4}", n)
    }
}

fn open_unavailable_app_notice(app: &Application, name: &str, kind: &str) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(name)
        .default_width(380)
        .default_height(160)
        .resizable(false)
        .build();

    let root = Box::new(Orientation::Vertical, 14);
    root.set_margin_top(24);
    root.set_margin_bottom(24);
    root.set_margin_start(24);
    root.set_margin_end(24);

    root.append(&Label::new(Some(&format!(
        "{name} ({kind}) has not been built yet in this version of Protea OS."
    ))));

    let close = Button::with_label("Close");
    close.set_halign(gtk::Align::End);
    let window_close = window.clone();
    close.connect_clicked(move |_| window_close.close());
    root.append(&close);

    window.set_child(Some(&root));
    window.present();
}

fn request_power_action(action: &str) {
    let command = if action == "reboot" { "reboot" } else { "poweroff" }.to_string();
    let action_name = if action == "reboot" { "restart" } else { "shut down" }.to_string();

    let dialog = gtk::MessageDialog::builder()
        .text(format!("Confirm {action_name}?"))
        .secondary_text("Any unsaved application data may be lost.")
        .buttons(gtk::ButtonsType::Cancel)
        .build();
    dialog.add_button(&action_name, gtk::ResponseType::Accept);

    dialog.connect_response(move |dialog, response| {
        dialog.close();
        if response != gtk::ResponseType::Accept {
            return;
        }

        let status = std::process::Command::new("loginctl")
            .arg(&command)
            .status();

        if !matches!(status, Ok(status) if status.success()) {
            match std::process::Command::new(&command).status() {
                Ok(status) if status.success() => {}
                Ok(status) => eprintln!("Protea: {} exited with {}", command, status),
                Err(error) => eprintln!("Protea: could not execute {}: {}", command, error),
            }
        }
    });

    dialog.present();
}

fn apply_mode_policy(mode: ProteaMode) {
    let profile = match mode {
        ProteaMode::Gaming => "performance",
        ProteaMode::Office => "balanced",
    };

    let result = std::process::Command::new("powerprofilesctl")
        .args(["set", profile])
        .status();

    if result.is_err() {
        eprintln!("Protea: powerprofilesctl unavailable; keeping existing system power profile");
    }
}

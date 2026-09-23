use gtk::prelude::*;
use std::collections::BTreeMap;
use gtk::{glib, Application, ApplicationWindow, Box, Button, Label, Orientation, Separator, ToggleButton};
use protea_core::{DeviceClass, DeviceProfile, HardwareTier, Permission, ProteaIdentity, ProteaMode, ProteaState, StateStore};

const APP_ID: &str = "org.protea.os.desktop";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        let store = StateStore::new(state_path());
        let state = load_or_initialize(&store);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Protea OS")
            .default_width(1100)
            .default_height(700)
            .build();

        let root = Box::new(Orientation::Vertical, 0);
        let content = Box::new(Orientation::Vertical, 18);
        content.set_margin_top(32);
        content.set_margin_bottom(32);
        content.set_margin_start(32);
        content.set_margin_end(32);

        let title = Label::new(Some("PROTEA"));
        title.add_css_class("title-1");
        let subtitle = Label::new(Some("One identity. One system. Adaptive everywhere."));
        subtitle.add_css_class("dim-label");

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

        let settings = Label::new(Some(
            &format!("Settings stored locally: {}", state.settings.len()),
        ));

        let separator = Separator::new(Orientation::Horizontal);

        content.append(&title);
        content.append(&subtitle);
        content.append(&separator);
        content.append(&status);
        content.append(&identity);
        content.append(&settings);
        content.append(&modes);
        content.append(&start);

        let taskbar = Box::new(Orientation::Horizontal, 10);
        taskbar.set_margin_start(12);
        taskbar.set_margin_end(12);
        taskbar.set_margin_top(8);
        taskbar.set_margin_bottom(8);

        let start_menu = Button::with_label("Protea");
        let clock = Label::new(Some("Protea Desktop"));
        taskbar.append(&start_menu);
        taskbar.append(&clock);

        let system = Box::new(Orientation::Horizontal, 8);
        let settings_button = Button::with_label("Settings");
        let reboot_button = Button::with_label("Restart");
        let shutdown_button = Button::with_label("Shut down");
        system.append(&settings_button);
        system.append(&reboot_button);
        system.append(&shutdown_button);

        content.append(&system);

        let settings_app = app.clone();
        settings_button.connect_clicked(move |_| {
            open_settings_window(&settings_app);
        });

        reboot_button.connect_clicked(move |_| {
            request_power_action("reboot");
        });

        shutdown_button.connect_clicked(move |_| {
            request_power_action("poweroff");
        });

        root.append(&content);
        root.append(&taskbar);
        window.set_child(Some(&root));

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

        let start_menu_app = app.clone();
        start_menu.connect_clicked(move |_| {
            open_app_launcher(&start_menu_app);
        });

        let start_app = app.clone();
        start.connect_clicked(move |_| {
            open_app_launcher(&start_app);
        });

        apply_mode_policy(state.mode);
        apply_css();
        window.present();
    });

    app.run()
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

fn apply_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        "window { background: #17120f; } .title-1 { font-weight: 800; font-size: 34px; }",
    );
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


fn open_app_launcher(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Protea Applications")
        .default_width(520)
        .default_height(620)
        .build();

    let root = Box::new(Orientation::Vertical, 12);
    root.set_margin_top(20);
    root.set_margin_bottom(20);
    root.set_margin_start(20);
    root.set_margin_end(20);

    let title = Label::new(Some("Applications"));
    title.add_css_class("title-2");
    root.append(&title);

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
            button.connect_clicked(move |_| {
                if let Err(error) = launch_application(&launch_command) {
                    eprintln!("Protea: could not launch {name}: {error}");
                }
            });
            list.append(&button);
        }
    }

    root.append(&list);
    window.set_child(Some(&root));
    window.present();
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
            '"' | '\\'' => quote = Some(ch),
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


fn open_settings_window(app: &Application) {
    let store = StateStore::new(state_path());
    let state = load_or_initialize(&store);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Protea Settings")
        .default_width(600)
        .default_height(520)
        .build();

    let root = Box::new(Orientation::Vertical, 14);
    root.set_margin_top(24);
    root.set_margin_bottom(24);
    root.set_margin_start(24);
    root.set_margin_end(24);

    let title = Label::new(Some("Protea Settings"));
    title.add_css_class("title-2");
    root.append(&title);

    root.append(&Label::new(Some(&format!(
        "Device: {:?}  •  Tier: {:?}  •  Mode: {:?}",
        state.device.class, state.device.tier, state.mode
    ))));

    let identity_text = state.identity.as_ref()
        .map(|identity| identity.display_name.as_str())
        .unwrap_or("Not configured");
    root.append(&Label::new(Some(&format!("Local identity: {identity_text}"))));

    let theme_row = Box::new(Orientation::Horizontal, 10);
    let theme_label = Label::new(Some("Theme"));
    theme_label.set_width_chars(12);
    let theme_entry = gtk::Entry::new();
    theme_entry.set_hexpand(true);
    theme_entry.set_text(state.settings.get("theme").unwrap_or("coral"));
    theme_row.append(&theme_label);
    theme_row.append(&theme_entry);
    root.append(&theme_row);

    let info = Label::new(Some(&format!(
        "{} local settings. Changes are written to the local Protea state file.",
        state.settings.len()
    )));
    info.add_css_class("dim-label");
    root.append(&info);

    let actions = Box::new(Orientation::Horizontal, 10);
    let save = Button::with_label("Save");
    save.add_css_class("suggested-action");
    let close = Button::with_label("Close");
    actions.append(&save);
    actions.append(&close);
    root.append(&actions);

    let save_store = StateStore::new(state_path());
    let save_entry = theme_entry.clone();
    let save_info = info.clone();
    save.connect_clicked(move |_| {
        let mut current = load_or_initialize(&save_store);
        let theme = save_entry.text().trim().to_string();
        if theme.is_empty() {
            save_info.set_text("Theme cannot be empty.");
            return;
        }
        current.settings.set("theme", theme);
        match save_store.save(&current) {
            Ok(()) => save_info.set_text("Settings saved locally."),
            Err(error) => save_info.set_text(&format!("Could not save settings: {error:?}")),
        }
    });

    let window_close = window.clone();
    close.connect_clicked(move |_| window_close.close());

    window.set_child(Some(&root));
    window.present();
}

fn request_power_action(action: &str) {
    let command = if action == "reboot" { "reboot" } else { "poweroff" };
    let action_name = if action == "reboot" { "restart" } else { "shut down" };

    let dialog = gtk::MessageDialog::builder()
        .text(format!("Confirm {action_name}?"))
        .secondary_text("Any unsaved application data may be lost.")
        .buttons(gtk::ButtonsType::Cancel)
        .build();
    dialog.add_button(action_name, gtk::ResponseType::Accept);

    dialog.connect_response(move |dialog, response| {
        dialog.close();
        if response != gtk::ResponseType::Accept {
            return;
        }

        let status = std::process::Command::new("loginctl")
            .arg(if action == "reboot" { "reboot" } else { "poweroff" })
            .status();

        if !matches!(status, Ok(status) if status.success()) {
            match std::process::Command::new(command).status() {
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

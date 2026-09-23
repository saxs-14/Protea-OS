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

        let app = app.clone();
        start_menu.connect_clicked(move |_| {
            open_app_launcher(&app);
        });

        let app = app.clone();
        start.connect_clicked(move |_| {
            open_app_launcher(&app);
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
                let _ = launch_application(&launch_command);
            });
            list.append(&button);
        }
    }

    root.append(&list);
    window.set_child(Some(&root));
    window.present();
}

fn discover_applications() -> Vec<(String, String)> {
    let mut apps = BTreeMap::<String, String>::new();
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
            let mut name = None;
            let mut exec = None;
            let mut is_application = false;

            for line in text.lines() {
                if line == "[Desktop Entry]" {
                    is_application = true;
                } else if is_application && line.starts_with("Name=") && name.is_none() {
                    name = Some(line[5..].trim().to_string());
                } else if is_application && line.starts_with("Exec=") && exec.is_none() {
                    exec = Some(line[5..].trim().to_string());
                } else if is_application && line.starts_with("[") {
                    break;
                }
            }

            if let (Some(name), Some(exec)) = (name, exec) {
                let command = sanitize_exec_command(&exec);
                if !command.is_empty() {
                    apps.entry(name).or_insert(command);
                }
            }
        }
    }

    apps.into_iter().collect()
}

fn sanitize_exec_command(exec: &str) -> String {
    exec.split_whitespace()
        .filter(|part| !part.starts_with('%'))
        .map(|part| part.trim_matches('"'))
        .collect::<Vec<_>>()
        .join(" ")
}

fn launch_application(command: &str) -> std::io::Result<std::process::Child> {
    let mut parts = command.split_whitespace();
    let Some(program) = parts.next() else {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "empty command"));
    };
    std::process::Command::new(program).args(parts).spawn()
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

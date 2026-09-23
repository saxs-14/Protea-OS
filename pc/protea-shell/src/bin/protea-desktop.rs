use gtk::prelude::*;
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
                status_office.set_text(&format!(
                    "Device: {:?}  •  Tier: {:?}  •  Mode: Office",
                    current.device.class, current.device.tier
                ));
            }
        });

        start_menu.connect_clicked(|_| {
            println!("Protea start surface requested");
        });

        start.connect_clicked(|_| {
            println!("Protea application surface requested");
        });

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

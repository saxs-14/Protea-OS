use std::io::{self, Write};
use protea_core::{
    DeviceClass, DeviceProfile, HardwareTier, Permission, ProteaIdentity,
    ProteaMode, ProteaState, StateStore,
};

fn main() {
    let store = StateStore::new(state_path());

    let mut state = match store.load() {
        Ok(state) => state,
        Err(_) => {
            let mut state = ProteaState::new(DeviceProfile::new(
                DeviceClass::Pc,
                HardwareTier::Minimum,
                detect_memory_mb(),
                detect_storage_mb(),
                true,
                true,
            ));
            state.set_identity(ProteaIdentity::new("local-0001", "Protea User"));
            state.settings.set("theme", "coral");
            state.permissions.grant(Permission::DeviceInformation);
            let _ = store.save(&state);
            state
        }
    };

    println!("Protea PC shell");
    println!("Type 'help' for commands.");

    loop {
        print!("protea> ");
        if io::stdout().flush().is_err() { break; }

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() { break; }

        match line.trim() {
            "help" => print_help(),
            "status" => print_status(&state),
            "gaming" => {
                state.set_mode(ProteaMode::Gaming);
                save(&store, &state);
            }
            "office" => {
                state.set_mode(ProteaMode::Office);
                save(&store, &state);
            }
            "permissions" => print_permissions(&state),
            "settings" => print_settings(&state),
            "quit" | "exit" => break,
            "" => {}
            _ => println!("Unknown command. Type 'help'."),
        }
    }
}

fn state_path() -> std::path::PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        return std::path::PathBuf::from(home).join(".local/share/protea/state");
    }
    std::path::PathBuf::from(".protea/state")
}

fn save(store: &StateStore, state: &ProteaState) {
    match store.save(state) {
        Ok(()) => println!("Mode changed to {:?}.", state.mode),
        Err(error) => eprintln!("Could not save Protea state: {:?}", error),
    }
}

fn print_status(state: &ProteaState) {
    println!("device: {:?}", state.device.class);
    println!("tier: {:?}", state.device.tier);
    println!("memory: {} MB", state.device.memory_mb);
    println!("storage: {} MB", state.device.storage_mb);
    println!("GPU: {}", state.device.has_gpu);
    println!("battery: {}", state.device.has_battery);
    println!("mode: {:?}", state.mode);
    if let Some(identity) = &state.identity {
        println!("identity: {} ({})", identity.display_name, identity.local_id);
    }
}

fn print_permissions(state: &ProteaState) {
    for permission in [
        Permission::Storage,
        Permission::Network,
        Permission::Notifications,
        Permission::DeviceInformation,
        Permission::AccountData,
    ] {
        println!("{permission:?}: {}", state.permissions.is_granted(permission));
    }
}

fn print_settings(state: &ProteaState) {
    for (key, value) in state.settings.iter() {
        println!("{key}={value}");
    }
}

fn print_help() {
    println!("status       show device and Protea state");
    println!("settings     show persisted settings");
    println!("permissions  show permissions");
    println!("gaming       switch to Gaming mode");
    println!("office       switch to Office mode");
    println!("quit         exit shell");
}

fn detect_memory_mb() -> u64 {
    if let Ok(text) = std::fs::read_to_string("/proc/meminfo") {
        if let Some(line) = text.lines().find(|line| line.starts_with("MemTotal:")) {
            if let Some(kb) = line.split_whitespace().nth(1) {
                if let Ok(value) = kb.parse::<u64>() {
                    return value / 1024;
                }
            }
        }
    }
    0
}

fn detect_storage_mb() -> u64 {
    0
}

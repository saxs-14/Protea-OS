use protea_core::{
    DeviceClass, DeviceProfile, HardwareTier, Permission, ProteaIdentity, ProteaMode, ProteaState,
    StateStore,
};
use std::env;
use std::path::PathBuf;

fn state_path() -> PathBuf {
    env::var_os("PROTEA_STATE_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("protea-state"))
}

fn new_state() -> ProteaState {
    let memory_mb = detect_memory_mb();
    let tier = HardwareTier::from_memory_mb(memory_mb);
    let device = DeviceProfile::new(
        DeviceClass::Pc,
        tier,
        memory_mb,
        0,
        detect_gpu(),
        detect_battery(),
    );

    let mut state = ProteaState::new(device);
    state.set_identity(ProteaIdentity::new("local-0001", "Protea User"));
    state.permissions.grant(Permission::DeviceInformation);
    state.permissions.grant(Permission::Network);
    state.settings.set("theme", "coral");
    state.set_mode(ProteaMode::Office);
    state
}

fn load_or_create(store: &StateStore) -> ProteaState {
    match store.load() {
        Ok(state) => state,
        Err(_) => {
            let state = new_state();
            store.save(&state).expect("unable to save initial Protea state");
            state
        }
    }
}

fn main() {
    let store = StateStore::new(state_path());
    let command = env::args().nth(1).unwrap_or_else(|| "status".into());
    let mut state = load_or_create(&store);

    match command.as_str() {
        "status" => print_status(&state),
        "settings" => {
            for (key, value) in state.settings.iter() {
                println!("{key}={value}");
            }
        }
        "permissions" => {
            for permission in [
                Permission::Storage,
                Permission::Network,
                Permission::Notifications,
                Permission::DeviceInformation,
                Permission::AccountData,
            ] {
                println!("{permission:?}={}", state.permissions.is_granted(permission));
            }
        }
        "gaming" => set_mode(&store, &mut state, ProteaMode::Gaming),
        "office" => set_mode(&store, &mut state, ProteaMode::Office),
        "help" | "--help" | "-h" => {
            println!("Protea core commands: status | settings | permissions | gaming | office");
        }
        other => {
            eprintln!("Unknown command: {other}");
            std::process::exit(2);
        }
    }
}

fn print_status(state: &ProteaState) {
    println!("Protea OS shared core");
    println!("identity: {}", state.identity.as_ref().map(|v| v.display_name.as_str()).unwrap_or("none"));
    println!("device: {:?}", state.device.class);
    println!("tier: {:?}", state.device.tier);
    println!("memory_mb: {}", state.device.memory_mb);
    println!("storage_mb: {}", state.device.storage_mb);
    println!("gpu: {}", state.device.has_gpu);
    println!("battery: {}", state.device.has_battery);
    println!("mode: {:?}", state.mode);
}

fn set_mode(store: &StateStore, state: &mut ProteaState, mode: ProteaMode) {
    state.set_mode(mode);
    store.save(state).expect("unable to save Protea state");
    println!("mode: {:?}", state.mode);
}

fn detect_memory_mb() -> u64 {
    std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|text| text.lines().find(|line| line.starts_with("MemTotal:")).map(str::to_owned))
        .and_then(|line| line.split_whitespace().nth(1).and_then(|v| v.parse::<u64>().ok()))
        .map(|kb| kb / 1024)
        .unwrap_or(2048)
}

fn detect_gpu() -> bool {
    std::fs::read_dir("/sys/class/drm")
        .ok()
        .map(|entries| entries.flatten().any(|entry| entry.file_name().to_string_lossy().starts_with("card")))
        .unwrap_or(false)
}

fn detect_battery() -> bool {
    std::fs::read_dir("/sys/class/power_supply")
        .ok()
        .map(|entries| entries.flatten().any(|entry| entry.file_name().to_string_lossy().to_ascii_uppercase().starts_with("BAT")))
        .unwrap_or(false)
}

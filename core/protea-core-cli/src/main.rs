use protea_core::{
    DeviceClass, DeviceProfile, HardwareTier, Permission, ProteaIdentity, ProteaMode, ProteaState,
};

fn main() {
    let device = DeviceProfile::new(
        DeviceClass::Pc,
        HardwareTier::Minimum,
        2048,
        32768,
        false,
        true,
    );

    let mut state = ProteaState::new(device);

    state.set_identity(ProteaIdentity::new("local-0001", "Protea User"));
    state.permissions.grant(Permission::DeviceInformation);
    state.permissions.grant(Permission::Network);
    state.settings.set("theme", "coral");
    state.set_mode(ProteaMode::Office);

    println!("Protea OS shared core");
    println!("identity: {}", state.identity.as_ref().unwrap().display_name);
    println!("mode: {:?}", state.mode);
    println!("theme: {}", state.settings.get("theme").unwrap());
    println!(
        "network permission: {}",
        state.permissions.is_granted(Permission::Network)
    );
}

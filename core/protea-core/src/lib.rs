//! Protea OS shared core.
//!
//! Standard-library-only contracts for the first local-first implementation.

mod persistence;
pub use persistence::{StateStore, StateStoreError};

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass { Pc, Phone, Watch, Tv }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareTier { Minimum, Recommended, Full }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProteaMode { Gaming, Office }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission { Storage, Network, Notifications, DeviceInformation, AccountData }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceProfile {
    pub class: DeviceClass,
    pub tier: HardwareTier,
    pub memory_mb: u64,
    pub storage_mb: u64,
    pub has_gpu: bool,
    pub has_battery: bool,
}
impl DeviceProfile {
    pub fn new(class: DeviceClass, tier: HardwareTier, memory_mb: u64, storage_mb: u64, has_gpu: bool, has_battery: bool) -> Self {
        Self { class, tier, memory_mb, storage_mb, has_gpu, has_battery }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProteaIdentity { pub local_id: String, pub display_name: String }
impl ProteaIdentity {
    pub fn new(local_id: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self { local_id: local_id.into(), display_name: display_name.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Settings { values: BTreeMap<String, String> }
impl Settings {
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) { self.values.insert(key.into(), value.into()); }
    pub fn get(&self, key: &str) -> Option<&str> { self.values.get(key).map(String::as_str) }
    pub fn contains(&self, key: &str) -> bool { self.values.contains_key(key) }
    pub fn len(&self) -> usize { self.values.len() }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.values.iter().map(|(key, value)| (key.as_str(), value.as_str()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PermissionSet { granted: Vec<Permission> }
impl PermissionSet {
    pub fn grant(&mut self, permission: Permission) { if !self.granted.contains(&permission) { self.granted.push(permission); } }
    pub fn revoke(&mut self, permission: Permission) { self.granted.retain(|item| *item != permission); }
    pub fn is_granted(&self, permission: Permission) -> bool { self.granted.contains(&permission) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModePolicy {
    pub suppress_nonessential_notifications: bool,
    pub allow_background_sync: bool,
    pub prefer_performance: bool,
    pub prefer_battery: bool,
}

impl ModePolicy {
    pub fn for_mode(mode: ProteaMode) -> Self {
        match mode {
            ProteaMode::Gaming => Self {
                suppress_nonessential_notifications: true,
                allow_background_sync: false,
                prefer_performance: true,
                prefer_battery: false,
            },
            ProteaMode::Office => Self {
                suppress_nonessential_notifications: false,
                allow_background_sync: true,
                prefer_performance: false,
                prefer_battery: true,
            },
        }
    }
}

pub struct ProteaState {
    pub identity: Option<ProteaIdentity>,
    pub settings: Settings,
    pub permissions: PermissionSet,
    pub device: DeviceProfile,
    pub mode: ProteaMode,
}
impl ProteaState {
    pub fn new(device: DeviceProfile) -> Self {
        Self { identity: None, settings: Settings::default(), permissions: PermissionSet::default(), device, mode: ProteaMode::Office }
    }
    pub fn set_identity(&mut self, identity: ProteaIdentity) { self.identity = Some(identity); }
    pub fn set_mode(&mut self, mode: ProteaMode) { self.mode = mode; }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn settings_are_keyed_and_replaceable() {
        let mut settings = Settings::default();
        settings.set("theme", "coral");
        settings.set("theme", "dark");
        assert_eq!(settings.get("theme"), Some("dark"));
        assert_eq!(settings.len(), 1);
    }
    #[test]
    fn permissions_can_be_granted_and_revoked() {
        let mut permissions = PermissionSet::default();
        permissions.grant(Permission::Network);
        assert!(permissions.is_granted(Permission::Network));
        permissions.revoke(Permission::Network);
        assert!(!permissions.is_granted(Permission::Network));
    }
    #[test]
    fn mode_policy_matches_mode() {
        let gaming = ModePolicy::for_mode(ProteaMode::Gaming);
        assert!(gaming.prefer_performance);
        assert!(gaming.suppress_nonessential_notifications);
        assert!(!gaming.allow_background_sync);

        let office = ModePolicy::for_mode(ProteaMode::Office);
        assert!(office.prefer_battery);
        assert!(office.allow_background_sync);
    }

    #[test]
    fn state_starts_in_office_mode() {
        let state = ProteaState::new(DeviceProfile::new(DeviceClass::Pc, HardwareTier::Minimum, 2048, 32768, false, true));
        assert_eq!(state.mode, ProteaMode::Office);
        assert!(state.identity.is_none());
    }
}

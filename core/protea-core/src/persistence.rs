use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use crate::{DeviceClass, DeviceProfile, HardwareTier, PermissionSet, ProteaIdentity, ProteaMode, ProteaState, Settings};

const FORMAT_VERSION: u32 = 1;

#[derive(Debug)]
pub enum StateStoreError {
    Io(io::Error),
    InvalidFormat(String),
}

impl From<io::Error> for StateStoreError {
    fn from(value: io::Error) -> Self { Self::Io(value) }
}

pub struct StateStore {
    path: PathBuf,
}

impl StateStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path { &self.path }

    pub fn save(&self, state: &ProteaState) -> Result<(), StateStoreError> {
        let data = serialize(state);
        let tmp = self.path.with_extension("tmp");

        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(&tmp)?;
        file.write_all(data.as_bytes())?;
        file.sync_all()?;
        drop(file);

        fs::rename(tmp, &self.path)?;
        Ok(())
    }

    pub fn load(&self) -> Result<ProteaState, StateStoreError> {
        let mut data = String::new();
        File::open(&self.path)?.read_to_string(&mut data)?;
        deserialize(&data)
    }
}

fn serialize(state: &ProteaState) -> String {
    let identity = state.identity.as_ref()
        .map(|v| format!("{}|{}", escape(&v.local_id), escape(&v.display_name)))
        .unwrap_or_default();

    let mut lines = vec![
        format!("format={FORMAT_VERSION}"),
        format!("device_class={:?}", state.device.class),
        format!("hardware_tier={:?}", state.device.tier),
        format!("memory_mb={}", state.device.memory_mb),
        format!("storage_mb={}", state.device.storage_mb),
        format!("has_gpu={}", state.device.has_gpu),
        format!("has_battery={}", state.device.has_battery),
        format!("mode={:?}", state.mode),
        format!("identity={identity}"),
    ];

    for (key, value) in settings_pairs(&state.settings) {
        lines.push(format!("setting={}|{}", escape(&key), escape(&value)));
    }

    for permission in [
        crate::Permission::Storage,
        crate::Permission::Network,
        crate::Permission::Notifications,
        crate::Permission::DeviceInformation,
        crate::Permission::AccountData,
    ] {
        if state.permissions.is_granted(permission) {
            lines.push(format!("permission={permission:?}"));
        }
    }

    lines.join("\n") + "\n"
}

fn settings_pairs(settings: &Settings) -> Vec<(String, String)> {
    settings.iter().map(|(key, value)| (key.to_string(), value.to_string())).collect()
}

fn deserialize(data: &str) -> Result<ProteaState, StateStoreError> {
    let mut format = None;
    let mut class = None;
    let mut tier = None;
    let mut memory = None;
    let mut storage = None;
    let mut gpu = None;
    let mut battery = None;
    let mut mode = None;
    let mut identity = None;
    let mut settings = Settings::default();
    let mut permissions = PermissionSet::default();

    for line in data.lines() {
        let (key, value) = line.split_once('=')
            .ok_or_else(|| StateStoreError::InvalidFormat(format!("invalid line: {line}")))?;

        match key {
            "format" => format = Some(parse_u32(value, "format")?),
            "device_class" => class = Some(parse_class(value)?),
            "hardware_tier" => tier = Some(parse_tier(value)?),
            "memory_mb" => memory = Some(parse_u64(value, "memory_mb")?),
            "storage_mb" => storage = Some(parse_u64(value, "storage_mb")?),
            "has_gpu" => gpu = Some(parse_bool(value, "has_gpu")?),
            "has_battery" => battery = Some(parse_bool(value, "has_battery")?),
            "mode" => mode = Some(parse_mode(value)?),
            "identity" if !value.is_empty() => {
                let (id, name) = value.split_once('|')
                    .ok_or_else(|| StateStoreError::InvalidFormat("invalid identity".into()))?;
                identity = Some(ProteaIdentity::new(unescape(id), unescape(name)));
            }
            "setting" => {
                let (key, value) = value.split_once('|')
                    .ok_or_else(|| StateStoreError::InvalidFormat("invalid setting".into()))?;
                settings.set(unescape(key), unescape(value));
            }
            "permission" => {
                permissions.grant(parse_permission(value)?);
            }
            _ => {}
        }
    }

    if format != Some(FORMAT_VERSION) {
        return Err(StateStoreError::InvalidFormat("unsupported format version".into()));
    }

    let device = DeviceProfile::new(
        class.ok_or_else(|| missing("device_class"))?,
        tier.ok_or_else(|| missing("hardware_tier"))?,
        memory.ok_or_else(|| missing("memory_mb"))?,
        storage.ok_or_else(|| missing("storage_mb"))?,
        gpu.ok_or_else(|| missing("has_gpu"))?,
        battery.ok_or_else(|| missing("has_battery"))?,
    );

    let mut state = ProteaState::new(device);
    state.identity = identity;
    state.settings = settings;
    state.permissions = permissions;
    state.mode = mode.ok_or_else(|| missing("mode"))?;
    Ok(state)
}

fn escape(value: &str) -> String {
    value.replace('%', "%25").replace('|', "%7C").replace("\n", "%0A")
}

fn unescape(value: &str) -> String {
    value.replace("%0A", "\\n").replace("%7C", "|").replace("%25", "%")
}

fn parse_u32(v: &str, k: &str) -> Result<u32, StateStoreError> { v.parse().map_err(|_| invalid(k)) }
fn parse_u64(v: &str, k: &str) -> Result<u64, StateStoreError> { v.parse().map_err(|_| invalid(k)) }
fn parse_bool(v: &str, k: &str) -> Result<bool, StateStoreError> { v.parse().map_err(|_| invalid(k)) }
fn invalid(k: &str) -> StateStoreError { StateStoreError::InvalidFormat(format!("invalid {k}")) }
fn missing(k: &str) -> StateStoreError { StateStoreError::InvalidFormat(format!("missing {k}")) }

fn parse_class(v: &str) -> Result<DeviceClass, StateStoreError> {
    match v { "Pc" => Ok(DeviceClass::Pc), "Phone" => Ok(DeviceClass::Phone), "Watch" => Ok(DeviceClass::Watch), "Tv" => Ok(DeviceClass::Tv), _ => Err(invalid("device_class")) }
}
fn parse_tier(v: &str) -> Result<HardwareTier, StateStoreError> {
    match v { "Minimum" => Ok(HardwareTier::Minimum), "Recommended" => Ok(HardwareTier::Recommended), "Full" => Ok(HardwareTier::Full), _ => Err(invalid("hardware_tier")) }
}
fn parse_mode(v: &str) -> Result<ProteaMode, StateStoreError> {
    match v { "Gaming" => Ok(ProteaMode::Gaming), "Office" => Ok(ProteaMode::Office), _ => Err(invalid("mode")) }
}
fn parse_permission(v: &str) -> Result<crate::Permission, StateStoreError> {
    match v {
        "Storage" => Ok(crate::Permission::Storage),
        "Network" => Ok(crate::Permission::Network),
        "Notifications" => Ok(crate::Permission::Notifications),
        "DeviceInformation" => Ok(crate::Permission::DeviceInformation),
        "AccountData" => Ok(crate::Permission::AccountData),
        _ => Err(invalid("permission")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn state_round_trips() {
        let dir = std::env::temp_dir();
        let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = dir.join(format!("protea-test-{id}.state"));
        let store = StateStore::new(&path);

        let mut state = ProteaState::new(DeviceProfile::new(
            DeviceClass::Pc, HardwareTier::Minimum, 2048, 32000, false, true,
        ));
        state.set_identity(ProteaIdentity::new("user-1", "Protea User"));
        state.settings.set("theme", "coral");
        state.permissions.grant(crate::Permission::Network);
        state.set_mode(ProteaMode::Gaming);

        store.save(&state).unwrap();
        let loaded = store.load().unwrap();

        assert_eq!(loaded, state);
        let _ = fs::remove_file(path);
    }
}

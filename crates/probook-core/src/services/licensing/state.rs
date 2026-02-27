use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::crypto;
use super::fingerprint::FingerprintHash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityState {
    pub install_id: String,
    pub trial_start: Option<DateTime<Utc>>,
    pub trial_end: Option<DateTime<Utc>>,
    pub last_seen_time: DateTime<Utc>,
    pub run_counter: u64,
    pub cached_fingerprint: Option<FingerprintHash>,
    pub embedded_license: Option<Vec<u8>>,
    pub license_device_bound: bool,
}

impl SecurityState {
    pub fn new() -> Self {
        Self {
            install_id: uuid::Uuid::new_v4().to_string(),
            trial_start: None,
            trial_end: None,
            last_seen_time: Utc::now(),
            run_counter: 0,
            cached_fingerprint: None,
            embedded_license: None,
            license_device_bound: false,
        }
    }
}

// ─── Persistence paths ──────────────────────────────────────────────

/// Primary storage: file in local app data directory
pub fn get_primary_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("LOCALAPPDATA").ok().map(|dir| {
            let path = PathBuf::from(dir).join("Probook");
            let _ = std::fs::create_dir_all(&path);
            path.join("security.dat")
        })
    }
    #[cfg(target_os = "macos")]
    {
        dirs_next::home_dir().map(|home| {
            let path = home.join("Library/Application Support/Probook");
            let _ = std::fs::create_dir_all(&path);
            path.join("security.dat")
        })
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        dirs_next::home_dir().map(|home| {
            let path = home.join(".probook");
            let _ = std::fs::create_dir_all(&path);
            path.join("security.dat")
        })
    }
}

// ─── Load / Save with encryption ────────────────────────────────────

/// Load state from primary file, fallback to secondary (registry/hidden file).
/// Returns None if no state exists anywhere (first launch).
pub fn load_state(key: &[u8; 32]) -> Option<SecurityState> {
    let primary = load_from_primary(key);
    let secondary = load_from_secondary(key);

    reconcile_states(primary, secondary)
}

/// Save state to both persistence locations.
pub fn save_state(key: &[u8; 32], state: &SecurityState) -> Result<(), String> {
    let json = serde_json::to_vec(state).map_err(|e| format!("Failed to serialize state: {}", e))?;
    let encrypted = crypto::encrypt_state(key, &json)?;

    // Save to primary (file)
    if let Some(path) = get_primary_path() {
        if let Err(e) = std::fs::write(&path, &encrypted) {
            eprintln!("Warning: failed to write primary state: {}", e);
        }
    }

    // Save to secondary (registry on Windows, hidden file on macOS)
    save_to_secondary(&encrypted);

    Ok(())
}

fn load_from_primary(key: &[u8; 32]) -> Option<SecurityState> {
    let path = get_primary_path()?;
    let data = std::fs::read(&path).ok()?;
    let decrypted = crypto::decrypt_state(key, &data).ok()?;
    serde_json::from_slice(&decrypted).ok()
}

// ─── Secondary storage (platform-specific) ──────────────────────────

#[cfg(target_os = "windows")]
fn load_from_secondary(key: &[u8; 32]) -> Option<SecurityState> {
    use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let subkey = hkcu.open_subkey("Software\\Probook").ok()?;
    let value: String = subkey.get_value("security_state").ok()?;
    let data = BASE64.decode(&value).ok()?;
    let decrypted = crypto::decrypt_state(key, &data).ok()?;
    serde_json::from_slice(&decrypted).ok()
}

#[cfg(target_os = "windows")]
fn save_to_secondary(encrypted: &[u8]) {
    use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((subkey, _)) = hkcu.create_subkey("Software\\Probook") {
        let encoded = BASE64.encode(encrypted);
        let _ = subkey.set_value("security_state", &encoded);
    }
}

#[cfg(target_os = "macos")]
fn load_from_secondary(key: &[u8; 32]) -> Option<SecurityState> {
    let home = dirs_next::home_dir()?;
    let path = home.join("Library/Preferences/.probook_sys");
    let data = std::fs::read(&path).ok()?;
    let decrypted = crypto::decrypt_state(key, &data).ok()?;
    serde_json::from_slice(&decrypted).ok()
}

#[cfg(target_os = "macos")]
fn save_to_secondary(encrypted: &[u8]) {
    if let Some(home) = dirs_next::home_dir() {
        let path = home.join("Library/Preferences/.probook_sys");
        let _ = std::fs::write(&path, encrypted);
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn load_from_secondary(key: &[u8; 32]) -> Option<SecurityState> {
    let home = dirs_next::home_dir()?;
    let path = home.join(".probook_sys");
    let data = std::fs::read(&path).ok()?;
    let decrypted = crypto::decrypt_state(key, &data).ok()?;
    serde_json::from_slice(&decrypted).ok()
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn save_to_secondary(encrypted: &[u8]) {
    if let Some(home) = dirs_next::home_dir() {
        let path = home.join(".probook_sys");
        let _ = std::fs::write(&path, encrypted);
    }
}

// ─── Reconciliation ─────────────────────────────────────────────────

/// If both states exist, use the one with the higher run_counter.
/// If only one exists, use it. If neither, return None.
fn reconcile_states(
    primary: Option<SecurityState>,
    secondary: Option<SecurityState>,
) -> Option<SecurityState> {
    match (primary, secondary) {
        (Some(p), Some(s)) => {
            if p.run_counter >= s.run_counter {
                Some(p)
            } else {
                Some(s)
            }
        }
        (Some(p), None) => Some(p),
        (None, Some(s)) => Some(s),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_state_new_defaults() {
        let state = SecurityState::new();
        assert!(!state.install_id.is_empty());
        assert!(state.trial_start.is_none());
        assert!(state.trial_end.is_none());
        assert_eq!(state.run_counter, 0);
        assert!(state.cached_fingerprint.is_none());
        assert!(state.embedded_license.is_none());
        assert!(!state.license_device_bound);
    }

    #[test]
    fn test_security_state_new_unique_install_id() {
        let s1 = SecurityState::new();
        let s2 = SecurityState::new();
        assert_ne!(s1.install_id, s2.install_id);
    }

    #[test]
    fn test_reconcile_both_none() {
        assert!(reconcile_states(None, None).is_none());
    }

    #[test]
    fn test_reconcile_primary_only() {
        let state = SecurityState::new();
        let id = state.install_id.clone();
        let result = reconcile_states(Some(state), None);
        assert_eq!(result.unwrap().install_id, id);
    }

    #[test]
    fn test_reconcile_secondary_only() {
        let state = SecurityState::new();
        let id = state.install_id.clone();
        let result = reconcile_states(None, Some(state));
        assert_eq!(result.unwrap().install_id, id);
    }

    #[test]
    fn test_reconcile_primary_higher_counter() {
        let mut primary = SecurityState::new();
        primary.run_counter = 10;
        let primary_id = primary.install_id.clone();

        let mut secondary = SecurityState::new();
        secondary.run_counter = 5;

        let result = reconcile_states(Some(primary), Some(secondary));
        assert_eq!(result.unwrap().install_id, primary_id);
    }

    #[test]
    fn test_reconcile_secondary_higher_counter() {
        let mut primary = SecurityState::new();
        primary.run_counter = 3;

        let mut secondary = SecurityState::new();
        secondary.run_counter = 8;
        let secondary_id = secondary.install_id.clone();

        let result = reconcile_states(Some(primary), Some(secondary));
        assert_eq!(result.unwrap().install_id, secondary_id);
    }

    #[test]
    fn test_reconcile_equal_counters_prefers_primary() {
        let mut primary = SecurityState::new();
        primary.run_counter = 5;
        let primary_id = primary.install_id.clone();

        let mut secondary = SecurityState::new();
        secondary.run_counter = 5;

        let result = reconcile_states(Some(primary), Some(secondary));
        assert_eq!(result.unwrap().install_id, primary_id);
    }

    #[test]
    fn test_encrypt_decrypt_state_roundtrip() {
        let key = crypto::derive_state_key("test-fp");
        let mut state = SecurityState::new();
        state.run_counter = 42;
        state.license_device_bound = true;

        let json = serde_json::to_vec(&state).unwrap();
        let encrypted = crypto::encrypt_state(&key, &json).unwrap();
        let decrypted = crypto::decrypt_state(&key, &encrypted).unwrap();
        let restored: SecurityState = serde_json::from_slice(&decrypted).unwrap();

        assert_eq!(restored.install_id, state.install_id);
        assert_eq!(restored.run_counter, 42);
        assert!(restored.license_device_bound);
    }
}

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use super::crypto;
use super::fingerprint::{self, FingerprintHash};
use super::license::{self, LicensePayload, LicenseType};
use super::state::{self, SecurityState};

const TRIAL_DAYS: i64 = 30;
const GRACE_DAYS: i64 = 14;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LicenseStatus {
    NoLicense,
    TrialActive { days_remaining: u32 },
    Licensed { expires_at: String, days_remaining: u32 },
    GracePeriod { days_remaining: u32 },
    Expired,
    ClockTampered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LicenseStatusTag {
    NoLicense,
    TrialActive,
    Licensed,
    GracePeriod,
    Expired,
    ClockTampered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseStatusInfo {
    pub status: LicenseStatusTag,
    pub days_remaining: Option<u32>,
    pub expires_at: Option<String>,
    pub is_write_allowed: bool,
    pub customer_name: Option<String>,
    pub license_id: Option<String>,
    pub license_type: Option<String>,
}

impl LicenseStatusInfo {
    pub fn from_status(status: &LicenseStatus, payload: Option<&LicensePayload>) -> Self {
        let (tag, days_remaining, expires_at, is_write_allowed) = match status {
            LicenseStatus::NoLicense => (LicenseStatusTag::NoLicense, None, None, false),
            LicenseStatus::TrialActive { days_remaining } => (
                LicenseStatusTag::TrialActive,
                Some(*days_remaining),
                None,
                true,
            ),
            LicenseStatus::Licensed { expires_at, days_remaining } => (
                LicenseStatusTag::Licensed,
                Some(*days_remaining),
                Some(expires_at.clone()),
                true,
            ),
            LicenseStatus::GracePeriod { days_remaining } => (
                LicenseStatusTag::GracePeriod,
                Some(*days_remaining),
                None,
                true,
            ),
            LicenseStatus::Expired => (LicenseStatusTag::Expired, None, None, false),
            LicenseStatus::ClockTampered => (LicenseStatusTag::ClockTampered, None, None, false),
        };

        LicenseStatusInfo {
            status: tag,
            days_remaining,
            expires_at,
            is_write_allowed,
            customer_name: payload.map(|p| p.customer_name.clone()),
            license_id: payload.map(|p| p.license_id.clone()),
            license_type: payload.map(|p| match p.license_type {
                LicenseType::Trial => "trial".to_string(),
                LicenseType::Annual => "annual".to_string(),
                LicenseType::Lifetime => "lifetime".to_string(),
            }),
        }
    }
}

/// Global licensing engine state — initialized once on app startup.
static ENGINE: once_cell::sync::Lazy<Mutex<LicenseEngine>> =
    once_cell::sync::Lazy::new(|| Mutex::new(LicenseEngine::new()));

struct LicenseEngine {
    state: Option<SecurityState>,
    fingerprint: Option<FingerprintHash>,
    key: Option<[u8; 32]>,
    cached_status: LicenseStatus,
    cached_payload: Option<LicensePayload>,
}

impl LicenseEngine {
    fn new() -> Self {
        Self {
            state: None,
            fingerprint: None,
            key: None,
            cached_status: LicenseStatus::NoLicense,
            cached_payload: None,
        }
    }
}

/// Initialize the licensing engine. Call once on app startup.
/// Returns the current license status.
pub fn initialize() -> Result<LicenseStatusInfo, String> {
    let mut engine = ENGINE.lock().map_err(|e| format!("Engine lock failed: {}", e))?;

    // 1. Collect device fingerprint (use cached if available from state)
    let fp = fingerprint::collect_fingerprint();
    let fp_hash = fingerprint::compute_fingerprint_hash(&fp);

    // 2. Derive encryption key
    let key = crypto::derive_state_key(&fp_hash.hash);

    // 3. Load security state from dual persistence
    let loaded = state::load_state(&key);
    let mut sec_state = loaded.unwrap_or_else(SecurityState::new);

    // 4. Clock rollback detection
    let now = Utc::now();
    if now < sec_state.last_seen_time {
        // Clock went backwards — suspicious
        let diff = sec_state.last_seen_time - now;
        if diff.num_minutes() > 5 {
            // Allow 5-minute tolerance for clock drift
            engine.cached_status = LicenseStatus::ClockTampered;
            engine.state = Some(sec_state);
            engine.fingerprint = Some(fp_hash);
            engine.key = Some(key);
            return Ok(LicenseStatusInfo::from_status(
                &engine.cached_status,
                None,
            ));
        }
    }

    // 5. Update timestamps and counter
    sec_state.last_seen_time = now;
    sec_state.run_counter += 1;

    // Cache fingerprint if not already stored
    if sec_state.cached_fingerprint.is_none() {
        sec_state.cached_fingerprint = Some(fp_hash.clone());
    }

    // 6. Determine license status
    let (status, payload) = compute_status(&sec_state, &fp_hash);

    // 7. Save updated state
    state::save_state(&key, &sec_state)?;

    engine.state = Some(sec_state);
    engine.fingerprint = Some(fp_hash);
    engine.key = Some(key);
    engine.cached_status = status.clone();
    engine.cached_payload = payload.clone();

    Ok(LicenseStatusInfo::from_status(&status, payload.as_ref()))
}

/// Start a 30-day free trial.
pub fn start_trial() -> Result<LicenseStatusInfo, String> {
    let mut engine = ENGINE.lock().map_err(|e| format!("Engine lock failed: {}", e))?;

    let key = engine.key.ok_or("Engine not initialized")?;
    let fp_hash = engine.fingerprint.clone().ok_or("No fingerprint")?;

    let mut sec_state = engine.state.clone().ok_or("No state")?;

    // Don't allow re-starting trial if one already existed
    if sec_state.trial_start.is_some() {
        return Err("Trial has already been started".to_string());
    }

    let now = Utc::now();
    sec_state.trial_start = Some(now);
    sec_state.trial_end = Some(now + Duration::days(TRIAL_DAYS));
    sec_state.last_seen_time = now;

    state::save_state(&key, &sec_state)?;

    let (status, payload) = compute_status(&sec_state, &fp_hash);
    engine.state = Some(sec_state);
    engine.cached_status = status.clone();
    engine.cached_payload = payload.clone();

    Ok(LicenseStatusInfo::from_status(&status, payload.as_ref()))
}

/// Import a .probook license file.
pub fn import_license(file_bytes: &[u8]) -> Result<LicenseStatusInfo, String> {
    let mut engine = ENGINE.lock().map_err(|e| format!("Engine lock failed: {}", e))?;

    let key = engine.key.ok_or("Engine not initialized")?;
    let fp_hash = engine.fingerprint.clone().ok_or("No fingerprint")?;

    let mut sec_state = engine.state.clone().ok_or("No state")?;

    // 1. Verify the license signature
    let payload = license::verify_license(file_bytes)?;

    // 2. Check device binding
    if let Some(ref device_hash) = payload.device_hash {
        if device_hash != &fp_hash.display_id() {
            // Check stored fingerprint 2-of-3 match
            if let Some(ref cached_fp) = sec_state.cached_fingerprint {
                if !fingerprint::matches_fingerprint(cached_fp, &fp_hash) {
                    return Err(format!(
                        "This license is bound to a different device. \
                         If this is a reinstall or hardware change, \
                         contact support with License ID: {}",
                        payload.license_id
                    ));
                }
            } else {
                return Err(format!(
                    "This license is bound to device {}. \
                     Your device ID is {}. Contact support with License ID: {}",
                    device_hash,
                    fp_hash.display_id(),
                    payload.license_id
                ));
            }
        }
    }

    // 3. Check expiry
    let now = Utc::now();
    if payload.license_type != LicenseType::Lifetime && now > payload.expires_at {
        return Err("This license has already expired".to_string());
    }

    // 4. Embed license in state
    sec_state.embedded_license = Some(file_bytes.to_vec());
    sec_state.license_device_bound = payload.device_hash.is_some();
    sec_state.last_seen_time = now;

    state::save_state(&key, &sec_state)?;

    let (status, _) = compute_status(&sec_state, &fp_hash);
    engine.state = Some(sec_state);
    engine.cached_status = status.clone();
    engine.cached_payload = Some(payload.clone());

    Ok(LicenseStatusInfo::from_status(&status, Some(&payload)))
}

/// Get current license status (no side effects).
pub fn get_status() -> Result<LicenseStatusInfo, String> {
    let engine = ENGINE.lock().map_err(|e| format!("Engine lock failed: {}", e))?;
    Ok(LicenseStatusInfo::from_status(
        &engine.cached_status,
        engine.cached_payload.as_ref(),
    ))
}

/// Check if write operations are currently allowed.
pub fn is_write_allowed() -> bool {
    let engine = ENGINE.lock().unwrap_or_else(|e| e.into_inner());
    engine.cached_status.is_write_allowed()
}

/// Get the device ID for display to the user.
pub fn get_device_id() -> Result<String, String> {
    let engine = ENGINE.lock().map_err(|e| format!("Engine lock failed: {}", e))?;
    match &engine.fingerprint {
        Some(fp) => Ok(fp.display_id()),
        None => {
            // Engine not initialized, compute on the fly
            let fp = fingerprint::collect_fingerprint();
            let hash = fingerprint::compute_fingerprint_hash(&fp);
            Ok(hash.display_id())
        }
    }
}

// ─── Internal helpers ───────────────────────────────────────────────

impl LicenseStatus {
    pub fn is_write_allowed(&self) -> bool {
        matches!(
            self,
            LicenseStatus::TrialActive { .. }
                | LicenseStatus::Licensed { .. }
                | LicenseStatus::GracePeriod { .. }
        )
    }
}

fn compute_status(
    state: &SecurityState,
    _fp_hash: &FingerprintHash,
) -> (LicenseStatus, Option<LicensePayload>) {
    let now = Utc::now();

    // Priority 1: Check embedded license
    if let Some(ref license_bytes) = state.embedded_license {
        if let Ok(payload) = license::verify_license(license_bytes) {
            // Lifetime licenses never expire
            if payload.license_type == LicenseType::Lifetime {
                return (
                    LicenseStatus::Licensed {
                        expires_at: payload.expires_at.to_rfc3339(),
                        days_remaining: u32::MAX,
                    },
                    Some(payload),
                );
            }

            if now <= payload.expires_at {
                let days_left = (payload.expires_at - now).num_days().max(0) as u32;
                return (
                    LicenseStatus::Licensed {
                        expires_at: payload.expires_at.to_rfc3339(),
                        days_remaining: days_left,
                    },
                    Some(payload),
                );
            }

            // Grace period
            let grace_end = payload.expires_at + Duration::days(GRACE_DAYS);
            if now <= grace_end {
                let days_left = (grace_end - now).num_days().max(0) as u32;
                return (
                    LicenseStatus::GracePeriod {
                        days_remaining: days_left,
                    },
                    Some(payload),
                );
            }

            // Fully expired
            return (LicenseStatus::Expired, Some(payload));
        }
    }

    // Priority 2: Check trial
    if let (Some(_trial_start), Some(trial_end)) = (state.trial_start, state.trial_end) {
        if now <= trial_end {
            let days_left = (trial_end - now).num_days().max(0) as u32;
            return (
                LicenseStatus::TrialActive {
                    days_remaining: days_left,
                },
                None,
            );
        }

        // Trial grace period
        let grace_end = trial_end + Duration::days(GRACE_DAYS);
        if now <= grace_end {
            let days_left = (grace_end - now).num_days().max(0) as u32;
            return (LicenseStatus::GracePeriod { days_remaining: days_left }, None);
        }

        // Trial fully expired
        return (LicenseStatus::Expired, None);
    }

    // No license, no trial
    (LicenseStatus::NoLicense, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn make_fp_hash() -> FingerprintHash {
        fingerprint::compute_fingerprint_hash(&fingerprint::DeviceFingerprint {
            machine_guid: Some("test-guid".to_string()),
            bios_serial: Some("test-bios".to_string()),
            volume_serial: Some("test-vol".to_string()),
        })
    }

    fn make_empty_state() -> SecurityState {
        SecurityState {
            install_id: "test-install".to_string(),
            trial_start: None,
            trial_end: None,
            last_seen_time: Utc::now(),
            run_counter: 0,
            cached_fingerprint: None,
            embedded_license: None,
            license_device_bound: false,
        }
    }

    // ── is_write_allowed tests ──

    #[test]
    fn test_is_write_allowed_trial_active() {
        assert!(LicenseStatus::TrialActive { days_remaining: 15 }.is_write_allowed());
    }

    #[test]
    fn test_is_write_allowed_licensed() {
        assert!(LicenseStatus::Licensed {
            expires_at: "2027-01-01".to_string(),
            days_remaining: 365,
        }
        .is_write_allowed());
    }

    #[test]
    fn test_is_write_allowed_grace_period() {
        assert!(LicenseStatus::GracePeriod { days_remaining: 7 }.is_write_allowed());
    }

    #[test]
    fn test_is_write_allowed_expired() {
        assert!(!LicenseStatus::Expired.is_write_allowed());
    }

    #[test]
    fn test_is_write_allowed_clock_tampered() {
        assert!(!LicenseStatus::ClockTampered.is_write_allowed());
    }

    #[test]
    fn test_is_write_allowed_no_license() {
        assert!(!LicenseStatus::NoLicense.is_write_allowed());
    }

    // ── compute_status tests ──

    #[test]
    fn test_compute_status_no_trial_no_license() {
        let state = make_empty_state();
        let fp = make_fp_hash();
        let (status, payload) = compute_status(&state, &fp);
        assert!(matches!(status, LicenseStatus::NoLicense));
        assert!(payload.is_none());
    }

    #[test]
    fn test_compute_status_active_trial() {
        let mut state = make_empty_state();
        let now = Utc::now();
        state.trial_start = Some(now - Duration::days(5));
        state.trial_end = Some(now + Duration::days(25));
        let fp = make_fp_hash();

        let (status, payload) = compute_status(&state, &fp);
        match status {
            LicenseStatus::TrialActive { days_remaining } => {
                assert!(days_remaining >= 24 && days_remaining <= 25);
            }
            other => panic!("Expected TrialActive, got {:?}", other),
        }
        assert!(payload.is_none());
    }

    #[test]
    fn test_compute_status_trial_expired_in_grace() {
        let mut state = make_empty_state();
        let now = Utc::now();
        state.trial_start = Some(now - Duration::days(35));
        state.trial_end = Some(now - Duration::days(5)); // expired 5 days ago, within 14-day grace
        let fp = make_fp_hash();

        let (status, _) = compute_status(&state, &fp);
        match status {
            LicenseStatus::GracePeriod { days_remaining } => {
                assert!(days_remaining >= 8 && days_remaining <= 9);
            }
            other => panic!("Expected GracePeriod, got {:?}", other),
        }
    }

    #[test]
    fn test_compute_status_trial_fully_expired() {
        let mut state = make_empty_state();
        let now = Utc::now();
        state.trial_start = Some(now - Duration::days(60));
        state.trial_end = Some(now - Duration::days(30)); // expired 30 days ago, past 14-day grace
        let fp = make_fp_hash();

        let (status, _) = compute_status(&state, &fp);
        assert!(matches!(status, LicenseStatus::Expired));
    }

    #[test]
    fn test_compute_status_invalid_embedded_license_falls_through() {
        let mut state = make_empty_state();
        state.embedded_license = Some(vec![0u8; 10]); // Invalid license bytes
        let fp = make_fp_hash();

        // Should fall through to no-trial check → NoLicense
        let (status, _) = compute_status(&state, &fp);
        assert!(matches!(status, LicenseStatus::NoLicense));
    }

    // ── LicenseStatusInfo::from_status tests ──

    #[test]
    fn test_status_info_from_no_license() {
        let info = LicenseStatusInfo::from_status(&LicenseStatus::NoLicense, None);
        assert!(matches!(info.status, LicenseStatusTag::NoLicense));
        assert!(!info.is_write_allowed);
        assert!(info.days_remaining.is_none());
    }

    #[test]
    fn test_status_info_from_trial_with_payload() {
        let payload = LicensePayload {
            license_id: "LIC-1".to_string(),
            customer_name: "Test".to_string(),
            license_type: LicenseType::Trial,
            issued_at: Utc::now(),
            expires_at: Utc::now() + Duration::days(30),
            device_hash: None,
            max_devices: 1,
        };

        let info = LicenseStatusInfo::from_status(
            &LicenseStatus::TrialActive { days_remaining: 25 },
            Some(&payload),
        );
        assert!(matches!(info.status, LicenseStatusTag::TrialActive));
        assert!(info.is_write_allowed);
        assert_eq!(info.days_remaining, Some(25));
        assert_eq!(info.customer_name, Some("Test".to_string()));
        assert_eq!(info.license_type, Some("trial".to_string()));
    }
}

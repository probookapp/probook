use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceFingerprint {
    pub machine_guid: Option<String>,
    pub bios_serial: Option<String>,
    pub volume_serial: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintHash {
    pub hash: String,
    pub signals: [Option<String>; 3],
}

impl FingerprintHash {
    /// Format the combined hash as a user-friendly device ID (e.g. "A7F3-B2C1-9D4E")
    pub fn display_id(&self) -> String {
        let h = &self.hash;
        if h.len() >= 12 {
            format!(
                "{}-{}-{}",
                &h[0..4].to_uppercase(),
                &h[4..8].to_uppercase(),
                &h[8..12].to_uppercase()
            )
        } else {
            h.to_uppercase()
        }
    }
}

pub fn compute_fingerprint_hash(fp: &DeviceFingerprint) -> FingerprintHash {
    let signal_values = [&fp.machine_guid, &fp.bios_serial, &fp.volume_serial];

    let signals: [Option<String>; 3] = std::array::from_fn(|i| {
        signal_values[i].as_ref().map(|v| {
            let mut hasher = Sha256::new();
            hasher.update(v.as_bytes());
            format!("{:x}", hasher.finalize())
        })
    });

    // Combined hash from all available signals
    let mut combined_hasher = Sha256::new();
    for sig in &signals {
        if let Some(h) = sig {
            combined_hasher.update(h.as_bytes());
        }
    }
    let combined = format!("{:x}", combined_hasher.finalize());

    FingerprintHash {
        hash: combined,
        signals,
    }
}

/// Check if current fingerprint matches stored one using 2-of-3 rule.
/// Returns true if at least 2 of the 3 signals match.
pub fn matches_fingerprint(stored: &FingerprintHash, current: &FingerprintHash) -> bool {
    let mut match_count = 0;
    let mut comparable = 0;

    for i in 0..3 {
        if let (Some(s), Some(c)) = (&stored.signals[i], &current.signals[i]) {
            comparable += 1;
            if s == c {
                match_count += 1;
            }
        }
    }

    // If we have fewer than 2 comparable signals, accept if all available match
    if comparable < 2 {
        return match_count == comparable && comparable > 0;
    }

    match_count >= 2
}

// ─── Platform: Windows ─────────────────────────────────────────────

#[cfg(target_os = "windows")]
pub fn collect_fingerprint() -> DeviceFingerprint {
    DeviceFingerprint {
        machine_guid: win::get_machine_guid(),
        bios_serial: win::get_bios_serial(),
        volume_serial: win::get_volume_serial(),
    }
}

#[cfg(target_os = "windows")]
mod win {
    use std::process::Command;
    use std::time::Duration;

    /// Signal 1: MachineGuid from registry (fast, no WMI)
    pub fn get_machine_guid() -> Option<String> {
        use winreg::enums::HKEY_LOCAL_MACHINE;
        use winreg::RegKey;

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key = hklm
            .open_subkey("SOFTWARE\\Microsoft\\Cryptography")
            .ok()?;
        let guid: String = key.get_value("MachineGuid").ok()?;
        Some(guid)
    }

    /// Signal 2: BIOS serial via PowerShell (with wmic fallback for older Windows)
    pub fn get_bios_serial() -> Option<String> {
        // Try PowerShell first (modern Windows 10/11)
        let ps_result = Command::new("powershell")
            .args(["-NoProfile", "-Command", "(Get-CimInstance Win32_BIOS).SerialNumber"])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .ok()
            .and_then(|child| wait_with_timeout(child, Duration::from_secs(3)));

        if let Some(output) = ps_result {
            let text = String::from_utf8_lossy(&output.stdout);
            let serial = text.trim().to_string();
            if !serial.is_empty() && serial != "To be filled by O.E.M." {
                return Some(serial);
            }
        }

        // Fallback to wmic (deprecated but still works on older systems)
        let wmic_result = Command::new("wmic")
            .args(["bios", "get", "serialnumber"])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .ok()
            .and_then(|child| wait_with_timeout(child, Duration::from_secs(1)));

        if let Some(output) = wmic_result {
            let text = String::from_utf8_lossy(&output.stdout);
            return text.lines()
                .nth(1)
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty() && s != "To be filled by O.E.M.");
        }

        None
    }

    /// Signal 3: Volume serial of system drive
    pub fn get_volume_serial() -> Option<String> {
        let output = Command::new("cmd")
            .args(["/c", "vol", "C:"])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .output()
            .ok()?;

        let text = String::from_utf8_lossy(&output.stdout);
        // Output format: " Volume Serial Number is XXXX-XXXX"
        for line in text.lines() {
            if let Some(pos) = line.find("Serial Number is") {
                let serial = line[pos + 17..].trim();
                if !serial.is_empty() {
                    return Some(serial.to_string());
                }
            }
        }
        None
    }

    fn wait_with_timeout(
        mut child: std::process::Child,
        timeout: Duration,
    ) -> Option<std::process::Output> {
        let start = std::time::Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(_status)) => {
                    let stdout = child.stdout.take().map(|mut s| {
                        let mut buf = Vec::new();
                        std::io::Read::read_to_end(&mut s, &mut buf).ok();
                        buf
                    }).unwrap_or_default();
                    let stderr = child.stderr.take().map(|mut s| {
                        let mut buf = Vec::new();
                        std::io::Read::read_to_end(&mut s, &mut buf).ok();
                        buf
                    }).unwrap_or_default();
                    return Some(std::process::Output {
                        status: _status,
                        stdout,
                        stderr,
                    });
                }
                Ok(None) => {
                    if start.elapsed() > timeout {
                        let _ = child.kill();
                        return None;
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(_) => return None,
            }
        }
    }
}

// ─── Platform: macOS ────────────────────────────────────────────────

#[cfg(target_os = "macos")]
pub fn collect_fingerprint() -> DeviceFingerprint {
    DeviceFingerprint {
        machine_guid: mac::get_platform_uuid(),
        bios_serial: mac::get_hardware_serial(),
        volume_serial: mac::get_volume_uuid(),
    }
}

#[cfg(target_os = "macos")]
mod mac {
    use std::process::Command;

    /// Signal 1: IOPlatformUUID
    pub fn get_platform_uuid() -> Option<String> {
        let output = Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
            .ok()?;
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            if line.contains("IOPlatformUUID") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 4 {
                    return Some(parts[3].to_string());
                }
            }
        }
        None
    }

    /// Signal 2: Hardware serial number
    pub fn get_hardware_serial() -> Option<String> {
        let output = Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
            .ok()?;
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            if line.contains("IOPlatformSerialNumber") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 4 {
                    return Some(parts[3].to_string());
                }
            }
        }
        None
    }

    /// Signal 3: System volume UUID
    pub fn get_volume_uuid() -> Option<String> {
        let output = Command::new("diskutil")
            .args(["info", "/"])
            .output()
            .ok()?;
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            if line.contains("Volume UUID") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    return Some(parts[1].trim().to_string());
                }
            }
        }
        None
    }
}

// ─── Fallback for other platforms (development/testing) ─────────────

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn collect_fingerprint() -> DeviceFingerprint {
    DeviceFingerprint {
        machine_guid: Some("dev-machine-guid".to_string()),
        bios_serial: Some("dev-bios-serial".to_string()),
        volume_serial: Some("dev-volume-serial".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_fingerprint(
        guid: Option<&str>,
        bios: Option<&str>,
        vol: Option<&str>,
    ) -> DeviceFingerprint {
        DeviceFingerprint {
            machine_guid: guid.map(|s| s.to_string()),
            bios_serial: bios.map(|s| s.to_string()),
            volume_serial: vol.map(|s| s.to_string()),
        }
    }

    #[test]
    fn test_display_id_formatting() {
        let hash = FingerprintHash {
            hash: "abcdef1234567890".to_string(),
            signals: [None, None, None],
        };
        assert_eq!(hash.display_id(), "ABCD-EF12-3456");
    }

    #[test]
    fn test_display_id_short_hash() {
        let hash = FingerprintHash {
            hash: "abc".to_string(),
            signals: [None, None, None],
        };
        assert_eq!(hash.display_id(), "ABC");
    }

    #[test]
    fn test_display_id_exact_12_chars() {
        let hash = FingerprintHash {
            hash: "abcdef123456".to_string(),
            signals: [None, None, None],
        };
        assert_eq!(hash.display_id(), "ABCD-EF12-3456");
    }

    #[test]
    fn test_compute_fingerprint_hash_all_signals() {
        let fp = make_fingerprint(Some("guid"), Some("bios"), Some("vol"));
        let result = compute_fingerprint_hash(&fp);
        assert!(!result.hash.is_empty());
        assert!(result.signals[0].is_some());
        assert!(result.signals[1].is_some());
        assert!(result.signals[2].is_some());
    }

    #[test]
    fn test_compute_fingerprint_hash_partial_signals() {
        let fp = make_fingerprint(Some("guid"), None, Some("vol"));
        let result = compute_fingerprint_hash(&fp);
        assert!(!result.hash.is_empty());
        assert!(result.signals[0].is_some());
        assert!(result.signals[1].is_none());
        assert!(result.signals[2].is_some());
    }

    #[test]
    fn test_compute_fingerprint_hash_deterministic() {
        let fp = make_fingerprint(Some("guid"), Some("bios"), Some("vol"));
        let h1 = compute_fingerprint_hash(&fp);
        let h2 = compute_fingerprint_hash(&fp);
        assert_eq!(h1.hash, h2.hash);
    }

    #[test]
    fn test_matches_fingerprint_all_match() {
        let fp = make_fingerprint(Some("a"), Some("b"), Some("c"));
        let stored = compute_fingerprint_hash(&fp);
        let current = compute_fingerprint_hash(&fp);
        assert!(matches_fingerprint(&stored, &current));
    }

    #[test]
    fn test_matches_fingerprint_two_of_three_match() {
        let fp1 = make_fingerprint(Some("a"), Some("b"), Some("c"));
        let fp2 = make_fingerprint(Some("a"), Some("b"), Some("DIFFERENT"));
        let stored = compute_fingerprint_hash(&fp1);
        let current = compute_fingerprint_hash(&fp2);
        assert!(matches_fingerprint(&stored, &current));
    }

    #[test]
    fn test_matches_fingerprint_one_of_three_fails() {
        let fp1 = make_fingerprint(Some("a"), Some("b"), Some("c"));
        let fp2 = make_fingerprint(Some("a"), Some("DIFF1"), Some("DIFF2"));
        let stored = compute_fingerprint_hash(&fp1);
        let current = compute_fingerprint_hash(&fp2);
        assert!(!matches_fingerprint(&stored, &current));
    }

    #[test]
    fn test_matches_fingerprint_zero_match() {
        let fp1 = make_fingerprint(Some("a"), Some("b"), Some("c"));
        let fp2 = make_fingerprint(Some("x"), Some("y"), Some("z"));
        let stored = compute_fingerprint_hash(&fp1);
        let current = compute_fingerprint_hash(&fp2);
        assert!(!matches_fingerprint(&stored, &current));
    }

    #[test]
    fn test_matches_fingerprint_one_comparable_one_match() {
        let fp1 = make_fingerprint(Some("a"), None, None);
        let fp2 = make_fingerprint(Some("a"), None, None);
        let stored = compute_fingerprint_hash(&fp1);
        let current = compute_fingerprint_hash(&fp2);
        assert!(matches_fingerprint(&stored, &current));
    }

    #[test]
    fn test_matches_fingerprint_one_comparable_no_match() {
        let fp1 = make_fingerprint(Some("a"), None, None);
        let fp2 = make_fingerprint(Some("x"), None, None);
        let stored = compute_fingerprint_hash(&fp1);
        let current = compute_fingerprint_hash(&fp2);
        assert!(!matches_fingerprint(&stored, &current));
    }

    #[test]
    fn test_matches_fingerprint_no_comparable_signals() {
        let stored = FingerprintHash {
            hash: "abc".to_string(),
            signals: [Some("a".to_string()), None, None],
        };
        let current = FingerprintHash {
            hash: "def".to_string(),
            signals: [None, Some("b".to_string()), None],
        };
        assert!(!matches_fingerprint(&stored, &current));
    }
}

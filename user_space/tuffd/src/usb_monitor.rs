use anyhow::Result;
use log::{info, debug};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use crate::events::{TuffLogEntry, LogLevel, TuffEvent};

/// Polls /dev/disk/by-id for USB devices containing a valid TUFF Key.
/// Returns the raw 32-byte key and the UUID (filename) if found.
pub async fn wait_for_key() -> Result<Option<(Vec<u8>, String)>> {
    info!("Starting USB Key polling...");

    let mut attempt_count = 0u64;

    // Polling loop
    loop {
        // Log "Searching" event every ~30 seconds (15 attempts * 2 sec)
        if attempt_count % 15 == 0 {
            TuffLogEntry::new(
                LogLevel::Info,
                TuffEvent::KeySearch {
                    status: format!("Scanning for TUFF-KEY (Attempt {})...", attempt_count),
                },
            ).log();
        }
        attempt_count += 1;

        let usb_devices = scan_usb_devices()?;

        for device_path in usb_devices {
            debug!("Checking candidate device: {:?}", device_path);

            if let Ok(Some((key, uuid))) = check_device_for_key(&device_path) {
                TuffLogEntry::new(
                    LogLevel::Audit,
                    TuffEvent::KeyDetected {
                        device: device_path.to_string_lossy().to_string(),
                        key_uuid: uuid.clone(),
                    },
                ).log();
                return Ok(Some((key, uuid)));
            }
        }

        // Wait before next scan
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

fn scan_usb_devices() -> Result<Vec<PathBuf>> {
    let mut candidates = Vec::new();
    let disk_dir = Path::new("/dev/disk/by-id");

    if !disk_dir.exists() {
        return Ok(candidates);
    }

    for entry in fs::read_dir(disk_dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();

        // Look for USB partitions (ending in -partX)
        if name.starts_with("usb-") && name.contains("-part") {
            candidates.push(entry.path());
        }
    }
    Ok(candidates)
}

fn check_device_for_key(device_path: &Path) -> Result<Option<(Vec<u8>, String)>> {
    let mount_point = Path::new("/mnt/tuff_key_check");
    fs::create_dir_all(mount_point)?;

    // 1. Mount (Read-Only)
    let status = Command::new("mount")
        .arg("-o").arg("ro")
        .arg(device_path)
        .arg(mount_point)
        .status();

    match status {
        Ok(s) if s.success() => {
            // Success, proceed to check key
        }
        Ok(s) => {
            let err_msg = format!("Mount command returned failure code: {:?}", s.code());
            TuffLogEntry::new(
                LogLevel::Warn,
                TuffEvent::MountFailure {
                    path: device_path.to_string_lossy().to_string(),
                    error: err_msg,
                },
            ).log();
            return Ok(None);
        }
        Err(e) => {
            TuffLogEntry::new(
                LogLevel::Error,
                TuffEvent::MountFailure {
                    path: device_path.to_string_lossy().to_string(),
                    error: e.to_string(),
                },
            ).log();
            return Ok(None);
        }
    }

    // 2. Search for Key
    let result = (|| -> Result<Option<(Vec<u8>, String)>> {
        let key_dir = mount_point.join("TUFF_KEYS");
        if !key_dir.exists() {
            return Ok(None);
        }

        for entry in fs::read_dir(key_dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "key" {
                    // Validate size (32 bytes)
                    let metadata = fs::metadata(&path)?;
                    if metadata.len() == 32 {
                        let key_data = fs::read(&path)?;
                        let file_stem = path.file_stem().unwrap().to_string_lossy().to_string();
                        return Ok(Some((key_data, file_stem)));
                    } else {
                        TuffLogEntry::new(
                            LogLevel::Warn,
                            TuffEvent::KeyRejected {
                                device: device_path.to_string_lossy().to_string(),
                                reason: format!("Invalid key size: {} bytes", metadata.len()),
                            },
                        ).log();
                    }
                }
            }
        }
        Ok(None)
    })();

    // 3. Unmount
    let umount_status = Command::new("umount").arg(mount_point).status();
    if let Err(e) = umount_status {
        TuffLogEntry::new(
            LogLevel::Error,
            TuffEvent::IoError {
                context: "Unmount failed".into(),
                error: e.to_string(),
            },
        ).log();
    }

    result
}

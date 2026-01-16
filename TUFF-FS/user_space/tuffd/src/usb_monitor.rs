use anyhow::{Context, Result};
use log::{info, debug};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use crate::events::{TuffLogEntry, LogLevel, TuffEvent};
use nix::mount::{mount, umount2, MntFlags, MsFlags};
use nix::errno::Errno;

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
    let sys_block = Path::new("/sys/block");

    if !sys_block.exists() {
        return Ok(candidates);
    }

    for entry in fs::read_dir(sys_block)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        let device_path = entry.path().join("device");

        if !is_usb_device(&device_path)? {
            continue;
        }

        for part in list_partitions(&entry.path(), &name)? {
            candidates.push(part);
        }
    }
    Ok(candidates)
}

fn is_usb_device(device_path: &Path) -> Result<bool> {
    if !device_path.exists() {
        return Ok(false);
    }
    let link = fs::read_link(device_path).with_context(|| {
        format!("Failed to read link {}", device_path.display())
    })?;
    Ok(link.to_string_lossy().contains("/usb"))
}

fn list_partitions(block_path: &Path, base: &str) -> Result<Vec<PathBuf>> {
    let mut parts = Vec::new();
    for entry in fs::read_dir(block_path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "device" {
            continue;
        }
        if !name.starts_with(base) || name == base {
            continue;
        }
        let dev_path = Path::new("/dev").join(&name);
        if dev_path.exists() {
            parts.push(dev_path);
        }
    }
    Ok(parts)
}

fn check_device_for_key(device_path: &Path) -> Result<Option<(Vec<u8>, String)>> {
    let mount_point = Path::new("/mnt/tuff_key_check");
    fs::create_dir_all(mount_point)?;

    // 1. Mount (Read-Only)
    if let Err(e) = mount_readonly(device_path, mount_point) {
        TuffLogEntry::new(
            LogLevel::Warn,
            TuffEvent::MountFailure {
                path: device_path.to_string_lossy().to_string(),
                error: e.to_string(),
            },
        ).log();
        return Ok(None);
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
    if let Err(e) = umount2(mount_point, MntFlags::MNT_DETACH) {
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

fn mount_readonly(device_path: &Path, mount_point: &Path) -> Result<()> {
    let fs_types = ["vfat", "exfat", "ext4", "ext3", "ext2"];
    for fs_type in fs_types.iter() {
        let res = mount(
            Some(device_path),
            mount_point,
            Some(*fs_type),
            MsFlags::MS_RDONLY,
            None::<&str>,
        );
        match res {
            Ok(()) => return Ok(()),
            Err(Errno::EINVAL) => continue,
            Err(_) => continue,
        }
    }
    Err(anyhow::anyhow!("No supported filesystem for USB key"))
}

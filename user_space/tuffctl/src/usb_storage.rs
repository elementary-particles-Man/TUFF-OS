use anyhow::{Result, Context, bail};
use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;

pub struct UsbKeyStore;

impl UsbKeyStore {
    pub fn find_usb_devices() -> Result<Vec<PathBuf>> {
        // Minimal implementation for TF-Core
        let entries = fs::read_dir("/dev/disk/by-id")?;
        let mut usbs = Vec::new();
        for entry in entries {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("usb-") && !name.contains("-part") {
                usbs.push(entry.path());
            }
        }
        Ok(usbs)
    }

    pub fn write_key_to_usb(device_path: &Path, key: &[u8; 32], uuid: &str) -> Result<()> {
        // 1. Mount
        let mount_point = Path::new("/mnt/usb_tmp");
        fs::create_dir_all(mount_point)?;

        let status = Command::new("mount")
            .arg(device_path)
            .arg(mount_point)
            .status().context("Failed to run mount command")?;

        if !status.success() {
            // Try mounting first partition if raw device failed
            let part1 = format!("{}-part1", device_path.to_string_lossy());
            let status2 = Command::new("mount").arg(&part1).arg(mount_point).status()?;
            if !status2.success() { bail!("Could not mount USB device"); }
        }

        // 2. Write Key
        let key_dir = mount_point.join("TUFF_KEYS");
        fs::create_dir_all(&key_dir)?;
        let key_file = key_dir.join(format!("{}.key", uuid));
        fs::write(&key_file, key)?;

        // 3. Verify Write
        let read_back = fs::read(&key_file)?;
        if read_back != key {
            bail!("Verification failed: Written key does not match memory key");
        }

        // 4. Unmount
        Command::new("umount").arg(mount_point).status()?;
        Ok(())
    }
}

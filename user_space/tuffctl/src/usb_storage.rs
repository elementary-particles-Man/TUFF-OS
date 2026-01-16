use anyhow::{Result, Context, bail};
use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;
use log::info;
use dialoguer::{Select, theme::ColorfulTheme};

pub struct UsbKeyStore;

impl UsbKeyStore {
    /// Detects USB mass storage devices via /dev/disk/by-id
    pub fn find_usb_devices() -> Result<Vec<PathBuf>> {
        let entries = fs::read_dir("/dev/disk/by-id")
            .context("Failed to read /dev/disk/by-id")?;

        let mut usbs = Vec::new();
        for entry in entries {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();

            // Filter for USB devices, ignore partitions (we want the disk itself or the main partition entry)
            // For simplicity in TF-Core, we look for 'usb-' prefix and no partition number first, or specific partition logic.
            // Here we look for partitions ending in -part1 to ensure we mount a filesystem.
            if name.starts_with("usb-") && name.ends_with("-part1") {
                usbs.push(entry.path());
            }
        }
        Ok(usbs)
    }

    /// Reads the system DMI Product UUID
    pub fn get_system_uuid() -> Result<String> {
        // Try reading from sysfs (requires root)
        let uuid_path = "/sys/class/dmi/id/product_uuid";
        if Path::new(uuid_path).exists() {
            let uuid = fs::read_to_string(uuid_path)?.trim().to_string();
            if !uuid.is_empty() {
                return Ok(uuid);
            }
        }

        // Fallback or Error? For TUFF-OS security, we prefer unique binding.
        // But for dev env (QEMU), we might accept a fallback if explicit.
        bail!("Could not read system UUID from /sys/class/dmi/id/product_uuid. Are you root?");
    }

    /// Interactive selection of USB device
    pub fn select_usb_device() -> Result<PathBuf> {
        let devices = Self::find_usb_devices()?;
        if devices.is_empty() {
            bail!("No USB devices (part1) found. Please insert a formatted USB drive.");
        }

        let selections: Vec<String> = devices.iter()
            .map(|d| d.to_string_lossy().to_string())
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select USB Device to store the Master Key")
            .default(0)
            .items(&selections)
            .interact()?;

        Ok(devices[selection].clone())
    }

    pub fn write_key_to_usb(device_path: &Path, key: &[u8; 32], uuid: &str) -> Result<()> {
        // 1. Mount
        let mount_point = Path::new("/mnt/usb_tmp");
        if !mount_point.exists() {
            fs::create_dir_all(mount_point)?;
        }

        // Unmount just in case it was already mounted
        let _ = Command::new("umount").arg(mount_point).status();

        let status = Command::new("mount")
            .arg(device_path)
            .arg(mount_point)
            .status().context("Failed to run mount command")?;

        if !status.success() {
            bail!("Could not mount USB device {:?}", device_path);
        }

        // 2. Write Key
        let key_dir = mount_point.join("TUFF_KEYS");
        if !key_dir.exists() {
            fs::create_dir_all(&key_dir)?;
        }

        let key_file = key_dir.join(format!("{}.key", uuid));
        fs::write(&key_file, key).context("Failed to write key file to USB")?;

        // 3. Verify Write
        let read_back = fs::read(&key_file).context("Failed to read back key for verification")?;
        if read_back != key {
            // Try to cleanup before erroring
            let _ = Command::new("umount").arg(mount_point).status();
            bail!("Verification failed: Written key does not match memory key");
        }

        // Sync to be safe
        let _ = Command::new("sync").status();

        // 4. Unmount
        let umount_status = Command::new("umount").arg(mount_point).status()?;
        if !umount_status.success() {
            eprintln!("Warning: Failed to unmount USB. Please remove safely.");
        } else {
            info!("Key written successfully to USB");
        }

        Ok(())
    }
}

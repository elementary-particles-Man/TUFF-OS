#!/bin/bash
set -e

TARGET_DEV=$1
SOURCE_EFI="bootloader/output/bootx64.efi"

if [ -z "$TARGET_DEV" ]; then
    echo "Usage: $0 <usb_partition_device> (e.g., /dev/sdb1)"
    exit 1
fi

if [ ! -f "$SOURCE_EFI" ]; then
    echo "[ERROR] Signed artifact not found at $SOURCE_EFI"
    echo "Please run ./tools/build_uefi.sh first."
    exit 1
fi

MOUNT_POINT="/mnt/tuff_deploy_tmp"

echo "[INFO] Deploying TUFF-OS Bootloader to $TARGET_DEV..."

# Mount
mkdir -p $MOUNT_POINT
sudo mount $TARGET_DEV $MOUNT_POINT

# Create Structure
sudo mkdir -p $MOUNT_POINT/EFI/BOOT

# Copy Bootloader
echo "Copying BOOTX64.EFI..."
sudo cp $SOURCE_EFI $MOUNT_POINT/EFI/BOOT/BOOTX64.EFI

# (Optional) Copy Enrollment Keys for convenience
echo "Copying Enrollment Keys to /KEYS..."
sudo mkdir -p $MOUNT_POINT/KEYS
if [ -f "keys/secure_boot/db.der" ]; then
    sudo cp keys/secure_boot/db.der $MOUNT_POINT/KEYS/
fi

# Sync & Umount
echo "Syncing..."
sync
sudo umount $MOUNT_POINT
rmdir $MOUNT_POINT

echo "[SUCCESS] Deployment complete. You can now boot from this USB."

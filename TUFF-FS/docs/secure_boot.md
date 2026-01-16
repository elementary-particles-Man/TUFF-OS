# Secure Boot Guide for TUFF-OS

## 1. Overview
TUFF-OS uses a custom Secure Boot chain. The bootloader (`tuffctl.efi`) is signed with a self-generated key (Snakeoil). To boot this on actual hardware, you must enroll your public key (`db.der`) into the UEFI firmware.

## 2. Key Generation
Run the following script to generate keys if you haven't already:
```bash
./tools/gen_sb_keys.sh
```
Artifacts created in `keys/secure_boot/`:
- `db.key`: Private key (Keep secret!)
- `db.crt`: Public certificate (X.509)
- `db.der`: Public key for UEFI enrollment (Binary format)

## 3. Preparation for Enrollment
Prepare a FAT32 formatted USB drive and copy the keys.

```bash
# Example structure on USB
/KEYS/
  └── db.der
```

## 4. UEFI Key Enrollment Steps
*Note: Menus vary by vendor (Dell, HP, Lenovo, etc.). The following is a generic flow.*

1.  **Enter BIOS/UEFI Setup** (F2, F12, Del, etc.).
2.  **Go to "Secure Boot" settings**.
3.  **Enter "Setup Mode"**:
    -   Option often named "Reset to Setup Mode", "Clear Keys", or "Delete PK".
    -   Confirm Secure Boot status changes to `Setup` or `Disabled`.
4.  **Enroll the DB Key**:
    -   Select **"Key Management"** or **"DB Management"**.
    -   Select **"Enroll Key"** (or "Append Key").
    -   Select "Install from File" and browse to your USB drive.
    -   Select `/KEYS/db.der`.
    -   Confirm import (Signature Database).
5.  **Save & Exit**:
    -   Secure Boot should be enabled automatically or manually set to `Enabled`.

## 5. Deployment (Bootloader)
To boot TUFF-OS, the signed artifact must be placed in the standard UEFI boot path.

### Directory Structure
```text
(USB Root)
 ├── EFI/
 │    └── BOOT/
 │         └── BOOTX64.EFI  <-- Signed tuffctl_efi.efi
 └── TUFF_KEYS/             <-- (Optional) If you are using this USB as a Physical Key
      └── ...
```

### Helper Script
Use `tools/deploy_usb.sh` to automate this:
```bash
sudo ./tools/deploy_usb.sh /dev/sdX1
```

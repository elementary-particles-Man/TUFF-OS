# Secure Boot Setup (Development)

This document describes the development-time Secure Boot workflow for TUFF-OS.

## Prerequisites
- `openssl`
- `sbsigntool` (provides `sbsign`)
- Rust target: `x86_64-unknown-uefi`

## 1. Generate Development Keys
Run:

```bash
./tools/gen_sb_keys.sh
```

This creates:
- `keys/secure_boot/db.key`
- `keys/secure_boot/db.crt`
- `keys/secure_boot/db.der`

## 2. Build and Sign UEFI Binary
Run:

```bash
./tools/build_uefi.sh
```

Outputs:
- Unsigned: `bootloader/output/tuffctl_efi.efi`
- Signed: `bootloader/output/bootx64.efi`

## 3. Enrolling Keys (UEFI Firmware)
This is firmware-specific. Common approaches:
- Use a vendor-provided firmware UI for Secure Boot key enrollment.
- Use a UEFI shell tool like KeyTool (if available) to enroll the `db` certificate.

For development use only. Production workflows should use properly managed keys,
access control, and revocation mechanisms.

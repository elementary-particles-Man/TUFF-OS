#!/bin/bash
set -e

OUTPUT_DIR="bootloader/output"
mkdir -p $OUTPUT_DIR

echo "[INFO] Building tuffctl_efi for x86_64-unknown-uefi..."

# Ensure target is installed
rustup target add x86_64-unknown-uefi

# Build
cargo build -p tuffctl_efi --target x86_64-unknown-uefi --release

SRC_EFI="target/x86_64-unknown-uefi/release/tuffctl_efi.efi"
DEST_EFI="$OUTPUT_DIR/tuffctl_efi.efi"
SIGNED_EFI="$OUTPUT_DIR/bootx64.efi"

cp $SRC_EFI $DEST_EFI
echo "[INFO] Artifact copied to $DEST_EFI"

# Try signing if tools and keys exist
if [ -f "tools/sign_efi.sh" ] && [ -f "keys/secure_boot/db.key" ]; then
    ./tools/sign_efi.sh $DEST_EFI $SIGNED_EFI
else
    echo "[WARN] Signing skipped (keys or script missing). Using unsigned file for bootx64.efi."
    cp $DEST_EFI $SIGNED_EFI
fi

echo "[SUCCESS] UEFI Bootloader ready at $SIGNED_EFI"

#!/bin/bash
set -e

echo "[INFO] Building tuffctl_efi for x86_64-unknown-uefi..."

# Ensure target is installed
rustup target add x86_64-unknown-uefi

# Build
cargo build -p tuffctl_efi --target x86_64-unknown-uefi --release

echo "[SUCCESS] UEFI Bootloader built at target/x86_64-unknown-uefi/release/tuffctl_efi.efi"

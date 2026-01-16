#!/bin/bash
set -e

# =============================================================================
# [WARNING] Disk Space Requirement
# This script builds a full Linux OS image using Buildroot.
# IT REQUIRES APPROX. 30GB OF FREE DISK SPACE.
#
# If the build fails due to "No space left on device":
# 1. Stop the build.
# 2. Run 'rm -rf tf_core/buildroot/output' to reclaim space.
# 3. Focus on developing 'tuffd' on the host machine instead.
# =============================================================================

BUILDROOT_VER="2024.02"
BUILDROOT_DIR="tf_core/buildroot"
CONFIG_FILE="$(pwd)/tf_core/buildroot_config/tuff_defconfig"
OVERLAY_DIR="$(pwd)/tf_core/overlay"

# 1. Setup Buildroot
if [ ! -d "$BUILDROOT_DIR" ]; then
    echo "[INFO] Downloading Buildroot $BUILDROOT_VER..."
    mkdir -p tf_core
    wget -qO- https://buildroot.org/downloads/buildroot-$BUILDROOT_VER.tar.gz | tar xz -C tf_core
    mv tf_core/buildroot-$BUILDROOT_VER $BUILDROOT_DIR
fi

# 2. Build tuffd (Host Rust -> Target Musl)
# Note: We need x86_64-unknown-linux-musl target installed
echo "[INFO] Building tuffd for TF-Core (Musl)..."
rustup target add x86_64-unknown-linux-musl || true
# Disable udev for musl build to avoid link errors if libudev not present in sysroot
cargo build -p tuffd --target x86_64-unknown-linux-musl --release --no-default-features

# 3. Prepare Overlay
chmod +x $OVERLAY_DIR/init
mkdir -p $OVERLAY_DIR/bin
# Copy binary only if build succeeded
if [ -f "target/x86_64-unknown-linux-musl/release/tuffd" ]; then
    cp target/x86_64-unknown-linux-musl/release/tuffd $OVERLAY_DIR/bin/tuffd
else
    echo "[WARN] tuffd binary not found. Skipping overlay copy."
fi

# 4. Configure Buildroot
echo "[INFO] Configuring OS..."
cd $BUILDROOT_DIR
make defconfig BR2_DEFCONFIG=$CONFIG_FILE

# Inject Overlay Path directly into .config to ensure it uses absolute path
sed -i "s|BR2_ROOTFS_OVERLAY=\"\"|BR2_ROOTFS_OVERLAY=\"$OVERLAY_DIR\"|" .config

# 5. Build
echo "[INFO] Building TF-Core Image (This takes time)..."
make

echo "[SUCCESS] OS Image built at $BUILDROOT_DIR/output/images/rootfs.squashfs"

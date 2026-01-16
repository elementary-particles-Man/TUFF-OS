# TF-Core (TUFF Foundation Core)

TF-Core is the immutable, minimal Linux environment that hosts `tuffd`.

## Development Status (2026-01-16)
**Build Skipped due to Disk Constraints.**
- The Buildroot configuration (`tuff_defconfig`) is complete and valid.
- Full OS build requires ~30GB disk space, which is currently unavailable on the dev machine.
- **Current Strategy**: Focus on developing `tuffd` and `tuffctl` (User Space) on the host machine using `cargo run`. The OS image can be built later on a machine with sufficient storage.

## Build Strategy

We use **Buildroot** to generate a tiny, read-only system image.

### 1. Components
- **Kernel**: Linux 6.6 LTS (Minimal Config, No Modules if possible)
- **Init**: `/bin/tuffd` (Directly launched as PID 1)
- **Libc**: Musl libc (Static linking preferred)
- **FS**: SquashFS (Read-Only Root)

### 2. Partition Layout (Target)
- `EFI` (FAT32): Bootloader
- `KERNEL` (Raw/EFI Stub): The TF-Core Kernel
- `ROOTFS` (SquashFS): The OS Image
- `STATE` (Ext4/LUKS): Encrypted persistence for logs/keys

# TF-Core (TUFF Foundation Core)

TF-Core is the immutable, minimal Linux environment that hosts `tuffd`.

## Build Strategy

We use **Buildroot** to generate a tiny, read-only system image.

### 1. Components
- **Kernel**: Linux 6.6 LTS (Minimal Config, No Modules if possible)
- **Init**: `/bin/tuffd` (Directly launched as PID 1)
- **Lib**: Musl libc (Static linking preferred)
- **FS**: SquashFS (Read-Only Root)

### 2. Partition Layout (Target)
- `EFI` (FAT32): Bootloader
- `KERNEL` (Raw/EFI Stub): The TF-Core Kernel
- `ROOTFS` (SquashFS): The OS Image
- `STATE` (Ext4/LUKS): Encrypted persistence for logs/keys

### 3. Next Steps
- Create `tf_core/buildroot/` config.
- Implement `tools/build_os.sh`.

# TUFF-OS

## 概要 / Overview
TUFF-OSは、TUFF-FSのための超小型Linux下層OSです。  
UEFIと上位OSの間で起動し、I/O制御と復旧に特化します。

TUFF-OS is a minimal Linux underlay OS for TUFF-FS.  
It boots between UEFI and the upper OS and focuses on I/O control and recovery.

## 目的 / Goals
- TUFF-FS領域の確保と管理  
- 4096バイト単位のI/O制御  
- 自動復旧のための最小ランタイム  

- Allocate and manage TUFF-FS storage  
- Enforce 4096-byte I/O granularity  
- Provide a minimal runtime for recovery

## リポジトリ構成 / Repository Layout
- `TUFF-FS/` : ソースコード（tuffd/tuffutl、Buildroot、カーネル設定）  
- `TUFF-OS/` : 生成物配置（kernel/rootfs/artifacts/iso）  
- `docs/` : 開発メモ・運用手順  

- `TUFF-FS/` : Source code (tuffd/tuffutl, Buildroot, kernel config)  
- `TUFF-OS/` : Build artifacts (kernel/rootfs/artifacts/iso)  
- `docs/` : Notes and operational guides

## ビルド / Build
```
cd /mnt/Ext4-Data/Develop/TUFF-OS/TUFF-FS
bash tools/build_os.sh
```

成果物は `TUFF-OS/` に配置されています。  
Artifacts are placed under `TUFF-OS/`.

## 重要な前提 / Important Notes
- ネットワーク機能は無効化しています。  
- ローカルログインは不要で、tuffdがPID1になります。  
- 既存OSがある環境への導入は、tuffutl経由でのみ行う前提です。  

- Networking is disabled.  
- Local login is not required; tuffd runs as PID1.  
- Underlay installation on existing OS is intended to be done via tuffutl only.

## Push方法 / Push Guide
詳細は `docs/push_guide.md` を参照してください。  
See `docs/push_guide.md` for SSH push instructions.


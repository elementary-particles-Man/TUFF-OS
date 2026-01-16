# TUFF-OS 進捗メモ（要点まとめ）

## 背景/前提
- TUFF-OSは「BIOS/UEFIと上位OSの間」に位置する超小型Linuxとして設計する。
- 目的はTUFF-FSの領域確保、I/O制御、自動復旧に特化する。
- 画面/UIは最小。ネットワークは不要。ローカルログイン不要。
- セクタ/チャンクは4096バイトを最小単位。
- TUFF-OS領域サイズは512MB固定。

## ビルド/配置
- `tools/build_os.sh` を改修し、カーネルフラグメントの絶対パス指定とoverlayのmerged-/usr対応を追加。
- BuildrootでOSイメージ生成（`rootfs.squashfs`, `rootfs.tar`, `bzImage`）。
- 配布向け配置は `/mnt/Ext4-Data/Develop/TUFF-OS/TUFF-OS/` に整理。
  - `kernel/bzImage`
  - `rootfs/rootfs.squashfs`
  - `artifacts/rootfs.tar`
  - `iso/EFI/BOOT` と `iso/boot` を作成（ISO作成準備用の空ディレクトリ）

## カーネル方針
- UEFI GOP向けに `simpledrm + fbcon` を有効化し、表示安定を優先。
- ネットワーク/GUI/サウンドなどは無効化方向（不要機能の排除で攻撃面削減）。
- 物理ストレージはSATA/NVMe/USBストレージを想定。

## ユーザ空間方針（完全Rust化）
- initは`tuffd`に一本化（ローカルログイン・シェル不要）。
- BusyBoxやudevなど外部コマンド依存は排除する方向。
- USB検出・mount/umountはRustで直接syscall化予定。

## 導入/セーフガード方針（最重要）
- 既存OSがある場合は、OS側に`tuffutl`を導入し`init`のみ許可。
- それ以外の導入経路は排除。
- **A最優先**: MK USBが刺さっていない限り導入・処理を進めない。
- 既存OS検出時は中断。**署名付きトークン必須**でのみ進行。
  - トークン名: `TUFF_BACKUP_TOKEN.sig`
  - 置き場所: USB直下
  - 署名検証鍵: TUFF-OSに組み込み
- 書き戻しは**ブロックイメージ方式のみ**（OS依存スナップショット禁止）。

## 実装済みセーフティ
- MK指紋照合の導入。
  - 初回は指紋保存、以降は一致必須。
  - ミスマッチ時は`FREEZE`遷移。
  - 保存先: `/var/lib/tuff/mk_fingerprint`

## 次の作業候補
- `tuffutl init` のCLI仕様とガード条件を確定。
- `tuffd` PID1化 + udev/mount依存排除。
- USBキー検出を`/sys`ベースへ移行。

#!/bin/bash
set -e

INPUT_EFI=$1
OUTPUT_EFI=$2
KEY_DIR="keys/secure_boot"

if [ -z "$INPUT_EFI" ] || [ -z "$OUTPUT_EFI" ]; then
    echo "Usage: $0 <input.efi> <output.efi>"
    exit 1
fi

if [ ! -f "$KEY_DIR/db.key" ]; then
    echo "[ERROR] Keys not found. Run tools/gen_sb_keys.sh first."
    exit 1
fi

if ! command -v sbsign &> /dev/null; then
    echo "[ERROR] 'sbsign' tool not found. Please install sbsigntool."
    exit 1
fi

echo "[INFO] Signing $INPUT_EFI..."
sbsign --key $KEY_DIR/db.key --cert $KEY_DIR/db.crt --output $OUTPUT_EFI $INPUT_EFI

echo "[SUCCESS] Signed EFI created at $OUTPUT_EFI"

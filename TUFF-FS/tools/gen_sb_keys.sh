#!/bin/bash
set -e

KEY_DIR="keys/secure_boot"
mkdir -p $KEY_DIR

if [ -f "$KEY_DIR/db.key" ]; then
    echo "[INFO] Keys already exist in $KEY_DIR. Skipping generation."
    exit 0
fi

echo "[INFO] Generating Secure Boot keys (Snakeoil) for development..."

# Generate Key and Self-Signed Certificate
openssl req -new -x509 -newkey rsa:2048 -subj "/CN=TUFF-OS Development Key/" -keyout $KEY_DIR/db.key -out $KEY_DIR/db.crt -days 3650 -nodes -sha256

# Convert to DER (EFI signature list format)
openssl x509 -in $KEY_DIR/db.crt -out $KEY_DIR/db.der -outform DER

echo "[SUCCESS] Keys generated in $KEY_DIR"

#!/bin/bash
set -e

SCHEMA_SRC="shared/schemas/src/tuff.fbs"
OUTPUT_DIR="shared/schemas/src/generated"

# Check for flatc
if ! command -v flatc &> /dev/null; then
    echo "[ERROR] 'flatc' (FlatBuffers Compiler) not found."
    echo "Please install it (e.g., 'sudo apt install flatbuffers-compiler' or from source)."
    exit 1
fi

mkdir -p $OUTPUT_DIR

echo "[INFO] Compiling schema: $SCHEMA_SRC"
flatc --rust --gen-object-api --filename-suffix _generated -o $OUTPUT_DIR $SCHEMA_SRC

echo "[SUCCESS] Rust code generated in $OUTPUT_DIR"

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

use tuff_common::paths::MK_FINGERPRINT_PATH;

pub enum FingerprintStatus {
    Stored,
    Matched,
    Mismatch,
}

pub fn verify_or_store_mk_fingerprint(key: &[u8]) -> Result<FingerprintStatus> {
    let fingerprint = sha256_hex(key);
    let path = Path::new(MK_FINGERPRINT_PATH);

    if path.exists() {
        let existing = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", MK_FINGERPRINT_PATH))?;
        let existing = existing.trim();
        if existing == fingerprint {
            return Ok(FingerprintStatus::Matched);
        }
        return Ok(FingerprintStatus::Mismatch);
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }
    fs::write(path, fingerprint.as_bytes())
        .with_context(|| format!("Failed to write {}", MK_FINGERPRINT_PATH))?;
    Ok(FingerprintStatus::Stored)
}

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let digest = hasher.finalize();
    let mut out = String::with_capacity(digest.len() * 2);
    for b in digest {
        out.push(hex_char(b >> 4));
        out.push(hex_char(b & 0x0f));
    }
    out
}

fn hex_char(v: u8) -> char {
    match v {
        0..=9 => (b'0' + v) as char,
        10..=15 => (b'a' + (v - 10)) as char,
        _ => '?',
    }
}

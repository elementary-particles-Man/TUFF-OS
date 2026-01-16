use anyhow::Result;
use std::fs;
use std::path::Path;
use tuff_common::paths::{INDEX_CHUNK_CURRENT, INDEX_CHUNK_PREV, INDEX_DIR};

// Placeholder for future FS operations implementation.
#[allow(dead_code)]
pub struct FsManager;

impl FsManager {
    pub fn load_latest_index_chunk(&self) -> Result<Option<Vec<u8>>> {
        if !Path::new(INDEX_CHUNK_CURRENT).exists() {
            return Ok(None);
        }
        let data = fs::read(INDEX_CHUNK_CURRENT)?;
        Ok(Some(data))
    }

    pub fn write_latest_index_chunk(&self, data: &[u8]) -> Result<()> {
        fs::create_dir_all(INDEX_DIR)?;

        let tmp_path = format!("{}.tmp", INDEX_CHUNK_CURRENT);
        fs::write(&tmp_path, data)?;

        if Path::new(INDEX_CHUNK_CURRENT).exists() {
            let _ = fs::rename(INDEX_CHUNK_CURRENT, INDEX_CHUNK_PREV);
        }

        fs::rename(&tmp_path, INDEX_CHUNK_CURRENT)?;
        Ok(())
    }
}

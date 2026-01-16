use anyhow::{Context, Result};

use std::time::{SystemTime, UNIX_EPOCH};
use tuff_schemas::tuff;

pub fn parse_index_chunk(buf: &[u8]) -> Result<tuff::tuff_os::IndexChunk<'_>> {
    tuff::tuff_os::root_as_index_chunk(buf)
        .context("invalid IndexChunk flatbuffer")
}

pub fn build_minimal_index_chunk(volume_name: &str, default_redundancy: u8) -> Result<Vec<u8>> {
    if volume_name.is_empty() {
        anyhow::bail!("volume_name is empty");
    }
    if default_redundancy == 0 {
        anyhow::bail!("invalid default_redundancy: {}", default_redundancy);
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("time went backwards")?
        .as_secs() as i64;

    let mut header = tuff::tuff_os::IndexChunkHeaderT::default();
    header.generation = 1;
    header.wrote_flag = true;
    header.timestamp = timestamp;
    header.default_redundancy = default_redundancy;
    header.volume_name = Some(volume_name.to_string());
    header.prev_chunk_hash = None;

    let mut chunk = tuff::tuff_os::IndexChunkT::default();
    chunk.header = Box::new(header);
    chunk.entries = Some(Vec::new());

    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let root = chunk.pack(&mut builder);
    tuff::tuff_os::finish_index_chunk_buffer(&mut builder, root);
    Ok(builder.finished_data().to_vec())
}

pub fn validate_index_chunk(buf: &[u8]) -> Result<()> {
    let chunk = parse_index_chunk(buf)?;
    let header = chunk.header();
    let generation = header.generation();
    if generation == 0 || generation > 254 {
        anyhow::bail!("invalid generation: {}", generation);
    }
    if !header.wrote_flag() {
        anyhow::bail!("index chunk not committed");
    }
    if header.timestamp() <= 0 {
        anyhow::bail!("invalid timestamp: {}", header.timestamp());
    }
    match header.volume_name() {
        Some(name) if !name.is_empty() => {}
        Some(_) => anyhow::bail!("volume_name is empty"),
        None => anyhow::bail!("missing volume_name"),
    }
    let redundancy = header.default_redundancy();
    if redundancy == 0 {
        anyhow::bail!("invalid default_redundancy: {}", redundancy);
    }
    Ok(())
}

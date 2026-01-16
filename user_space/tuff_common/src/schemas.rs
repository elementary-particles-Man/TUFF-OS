use anyhow::{Context, Result};

use tuff_schemas::tuff;

pub fn parse_index_chunk(buf: &[u8]) -> Result<tuff::tuff_os::IndexChunk<'_>> {
    tuff::tuff_os::root_as_index_chunk(buf)
        .context("invalid IndexChunk flatbuffer")
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

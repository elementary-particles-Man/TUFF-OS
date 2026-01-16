use anyhow::{Context, Result};

use tuff_schemas::tuff;

pub fn parse_index_chunk(buf: &[u8]) -> Result<tuff::tuff_os::IndexChunk<'_>> {
    tuff::tuff_os::root_as_index_chunk(buf)
        .context("invalid IndexChunk flatbuffer")
}

pub fn validate_index_chunk(buf: &[u8]) -> Result<()> {
    let chunk = parse_index_chunk(buf)?;
    let header = chunk.header().context("missing IndexChunkHeader")?;
    let generation = header.generation();
    if generation == 0 || generation > 254 {
        anyhow::bail!("invalid generation: {}", generation);
    }
    Ok(())
}

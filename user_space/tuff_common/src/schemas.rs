use anyhow::{Context, Result};

use tuff_schemas::tuff;

pub fn parse_index_chunk(buf: &[u8]) -> Result<tuff::tuff_os::IndexChunk<'_>> {
    tuff::tuff_os::root_as_index_chunk(buf)
        .context("invalid IndexChunk flatbuffer")
}

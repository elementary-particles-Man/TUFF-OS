// Re-export generated FlatBuffers modules

// Suppress warnings for auto-generated code (FlatBuffers)
#[allow(clippy::all, dead_code, mismatched_lifetime_syntaxes, unused_imports)]
pub mod tuff {
    #![allow(clippy::all, dead_code, mismatched_lifetime_syntaxes, unused_imports)]
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated/tuff_generated.rs"));
}

// Common helpers can be added here

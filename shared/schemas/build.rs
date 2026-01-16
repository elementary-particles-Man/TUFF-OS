use std::process::Command;
use std::path::Path;

fn main() {
    let schema_file = "src/tuff.fbs";
    let output_dir = "src/generated";

    // 1. Watch for schema changes
    println!("cargo:rerun-if-changed={}", schema_file);

    // 2. Ensure output directory exists
    if !Path::new(output_dir).exists() {
        std::fs::create_dir_all(output_dir).unwrap();
    }

    // 3. Run flatc
    // equivalent to: flatc --rust --gen-object-api --filename-suffix _generated -o src/generated src/tuff.fbs
    let status = Command::new("flatc")
        .arg("--rust")
        .arg("--gen-object-api")
        .arg("--filename-suffix")
        .arg("_generated")
        .arg("-o")
        .arg(output_dir)
        .arg(schema_file)
        .status();

    // Only panic if flatc is missing or fails, but provide a helpful message
    match status {
        Ok(s) if !s.success() => panic!("flatc failed. Check your schema syntax."),
        Err(_) => println!("cargo:warning=flatc not found. Skipping schema generation (assuming generated files exist)."),
        _ => {}
    }
}

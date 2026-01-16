fn main() {
    println!("cargo:rerun-if-changed=src/tuff.fbs");
    // Assume flatc is installed or handle generation manually if needed.
    // For now, we rely on the user having flatc or using a crate that bundles it.
    // Ideally, use `flatc-rust` crate logic here.
}

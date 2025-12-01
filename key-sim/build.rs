fn main() {
    // This build script runs before cargo build
    println!("cargo:rerun-if-changed=build.rs");
    
    // Add manifest embedding for Windows
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg-bins=/MANIFESTUAC:level='asInvoker'");
    }
}
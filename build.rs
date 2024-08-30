fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.lock");
    rustdoc_assets::copy_assets_folder("doc/papers");
}

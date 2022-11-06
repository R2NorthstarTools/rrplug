extern crate windres;

use windres::Build;

fn main() {
    Build::new().compile("manifest\\Resource.rc").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=r2rsplugins\\headers\\Resource.rc");
}
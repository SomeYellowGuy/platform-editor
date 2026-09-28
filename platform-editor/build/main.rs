//! The main build module for generating raw bytes from the
//! textures and font in the asset folder.

use std::{env, fs, path::Path};

mod bundled_font;
mod bundled_textures;

pub fn write_expect(result: std::io::Result<()>) {
    result.expect("Could not write to `BufWriter`")
}

fn main() {
    let out_dir_path = env::var("OUT_DIR").expect("OUT_DIR should have been defined");
    let crate_dir_path =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR should have been defined");

    let out_dir = Path::new(&out_dir_path).join("generated");
    let workspace_dir = Path::new(&crate_dir_path)
        .parent()
        .expect("Crate directory should have a parent");

    // Create the generated directory if it doesn't exist.
    if !out_dir.exists() {
        fs::create_dir_all(&out_dir)
            .expect("the output directory's parent directories should be created");
    }

    bundled_textures::write_bundled_textures(&out_dir, workspace_dir);
    bundled_font::write_bundled_font(&out_dir, workspace_dir);
}

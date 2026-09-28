//! Build module to generate the raw texture bytes for each texture,
//! which are used to load the required textures of the game.

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use walkdir::WalkDir;

use crate::write_expect;

const ASSETS_GFX_FOLDER: &str = "assets/gfx";

pub fn write_bundled_textures(out_dir: &Path, workspace_dir: &Path) {
    let dest_path = Path::new(&out_dir).join("bundled_textures.rs");
    let mut file = BufWriter::new(
        File::create(&dest_path)
            .expect("`File` for the texture bundle should be created successfully"),
    );

    let gfx_folder = workspace_dir.join(ASSETS_GFX_FOLDER);

    // Create a phf map builder to build our static map.
    let mut map = phf_codegen::Map::new();

    // Recursively read the folder for image files.
    for entry in WalkDir::new(&gfx_folder) {
        let dir_entry = entry.expect("Directory entry should be read successfully");
        let path = dir_entry.path();

        if path.is_file() {
            // Get the relative path for our key.
            let relative_path = path
                .strip_prefix(&gfx_folder)
                .expect("folder path should be prefixed");
            let relative_str = relative_path.to_string_lossy().replace("\\", "/");

            // Get the absolute path for our `include_bytes!` macro call, which will be our value.
            let absolute_str = path
                .canonicalize()
                .expect("Path should be able to be canonicalized")
                .to_string_lossy()
                .replace("\\", "/");

            // Write the key-value pair for the texture.
            map.entry(
                relative_str,
                format!("include_bytes!(r#\"{absolute_str}\"#)"),
            );
        }
    }

    // Write the final file.
    write_expect(writeln!(
        file,
        "static TEXTURES: phf::Map<&'static str, &'static [u8]> = {};",
        map.build()
    ));
}

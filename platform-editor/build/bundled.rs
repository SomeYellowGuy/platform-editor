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
const ASSETS_AUDIO_FOLDER: &str = "assets/audio";
const FONT_PATH: &str = "assets/font.ttf";

pub fn write_bundled(out_dir: &Path, workspace_dir: &Path) {
    write_bundled_textures(out_dir, workspace_dir);
    write_bundled_font(out_dir, workspace_dir);
    write_bundled_audio(out_dir, workspace_dir);
}

fn write_bundled_bytes(
    out_dir: &Path,
    workspace_dir: &Path,
    target_folder: &str,
    static_name: &str,
    bundled_file: &str,
) {
    let dest_path = Path::new(&out_dir).join(bundled_file);
    let mut file = BufWriter::new(
        File::create(&dest_path)
            .expect("`File` for the texture bundle should be created successfully"),
    );

    let folder = workspace_dir.join(target_folder);

    // Create a phf map builder to build our static map.
    let mut map = phf_codegen::Map::new();

    // Recursively read the folder for image files.
    for entry in WalkDir::new(&folder) {
        let dir_entry = entry.expect("Directory entry should be read successfully");
        let path = dir_entry.path();

        if path.is_file() {
            // Get the relative path for our key.
            let relative_path = path
                .strip_prefix(&folder)
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
        "static {static_name}: phf::Map<&'static str, &'static [u8]> = {};",
        map.build()
    ));
}

pub fn write_bundled_textures(out_dir: &Path, workspace_dir: &Path) {
    write_bundled_bytes(
        out_dir,
        workspace_dir,
        ASSETS_GFX_FOLDER,
        "TEXTURES",
        "bundled_textures.rs",
    );
}

pub fn write_bundled_font(out_dir: &Path, workspace_dir: &Path) {
    let dest_path = Path::new(&out_dir).join("bundled_font.rs");
    let mut file = BufWriter::new(
        File::create(&dest_path).expect("`File` for the font should be created successfully"),
    );

    let absolute_str = workspace_dir
        .join(FONT_PATH)
        .canonicalize()
        .expect("Path should be able to be canonicalized")
        .to_string_lossy()
        .replace("\\", "/");

    write_expect(writeln!(
        file,
        "static FONT_BYTES: &[u8] = include_bytes!(r#\"{absolute_str}\"#);"
    ));
}

pub fn write_bundled_audio(out_dir: &Path, workspace_dir: &Path) {
    write_bundled_bytes(
        out_dir,
        workspace_dir,
        ASSETS_AUDIO_FOLDER,
        "AUDIO",
        "bundled_audio.rs",
    );
}

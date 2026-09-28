//! Build module to generate the raw font bytes, which are used to
//! initialize the font in the main function.

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::write_expect;

const FONT_PATH: &str = "assets/font.ttf";

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

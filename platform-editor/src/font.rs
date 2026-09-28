//! A small module for storing the font's raw bytes and having a function
//! to create it from those bytes.

use sdl3::{
    iostream::IOStream,
    ttf::{Font, Sdl3TtfContext},
};

// Import the raw bytes of the font.
include!(concat!(env!("OUT_DIR"), "/generated/bundled_font.rs"));

pub fn load_font(context: &Sdl3TtfContext, point_size: f32) -> Result<Font<'static>, sdl3::Error> {
    let io = IOStream::from_bytes(FONT_BYTES)?;
    context.load_font_from_iostream(io, point_size)
}

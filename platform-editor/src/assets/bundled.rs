//! Stores the bundled data of the game (textures and audio), if enabled.

use platform_editor_core::audio::Sound;
use sdl3::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

// Include the built file that contains the raw bytes of our textures and audio.
include!(concat!(env!("OUT_DIR"), "/generated/bundled_textures.rs"));
include!(concat!(env!("OUT_DIR"), "/generated/bundled_audio.rs"));

pub fn load_texture_from_static_textures<'c>(
    creator: &'c TextureCreator<WindowContext>,
    path: &'static str,
) -> Result<Texture<'c>, sdl3::Error> {
    if let Some(v) = TEXTURES.get(path) {
        creator.load_texture_bytes(v)
    } else {
        panic!("No texture was bundled with the path {path}!")
    }
}

pub fn get_audio_data(sound: Sound) -> Option<&'static [u8]> {
    AUDIO.get(sound.relative_path()).copied()
}

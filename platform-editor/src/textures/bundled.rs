use sdl3::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

// Include the built file that contains the raw bytes of our textures.
include!(concat!(env!("OUT_DIR"), "/generated/bundled_textures.rs"));

pub fn load_texture_from_static_textures<'c>(
    creator: &'c TextureCreator<WindowContext>,
    path: &'static str,
) -> Result<Texture<'c>, sdl3::Error> {
    creator.load_texture_bytes(TEXTURES[path])
}

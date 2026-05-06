use sdl3::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub struct Images<'c> {
    pub strip: Texture<'c>,
    pub title: Texture<'c>,
}

impl<'c> Images<'c> {
    pub fn load(creator: &'c TextureCreator<WindowContext>) -> Result<Images<'c>, sdl3::Error> {
        Ok(Self {
            strip: creator.load_texture("assets/gfx/strip.png")?,
            title: creator.load_texture("assets/gfx/title.png")?,
        })
    }
}

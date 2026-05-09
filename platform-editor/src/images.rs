use sdl3::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    ttf::Font,
    video::WindowContext,
};

use crate::component::button::ExtractedFontTextureSets;

pub struct Textures<'c> {
    pub strip: Texture<'c>,
    pub title: Texture<'c>,
    pub button: Texture<'c>,
    pub button_icons: Texture<'c>,

    pub title_textures: ExtractedFontTextureSets<'c>,
}

impl<'c> Textures<'c> {
    fn load_texture(creator: &'c TextureCreator<WindowContext>, name: &str) -> Option<Texture<'c>> {
        creator.load_texture(name).ok()
    }

    pub fn load(creator: &'c TextureCreator<WindowContext>, font: &Font) -> Option<Textures<'c>> {
        Some(Self {
            strip: Self::load_texture(creator, "assets/gfx/strip.png")?,
            title: Self::load_texture(creator, "assets/gfx/title.png")?,
            button: Self::load_texture(creator, "assets/gfx/button.png")?,
            button_icons: Self::load_texture(creator, "assets/gfx/button_icons.png")?,

            title_textures: ExtractedFontTextureSets::new(creator, font)?,
        })
    }
}

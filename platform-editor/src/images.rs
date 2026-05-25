use sdl3::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    ttf::Font,
    video::WindowContext,
};

use crate::component::button::ExtractedFontTextureSets;

pub struct Textures<'c> {
    pub strip: Texture<'c>,

    pub title: TitleTextures<'c>,
    pub level_select: LevelSelectTextures<'c>,
}

impl<'c> Textures<'c> {
    fn load_texture(creator: &'c TextureCreator<WindowContext>, name: &str) -> Option<Texture<'c>> {
        creator.load_texture(name).ok()
    }

    pub fn load(creator: &'c TextureCreator<WindowContext>, font: &Font) -> Option<Textures<'c>> {
        Some(Self {
            strip: Self::load_texture(creator, "assets/gfx/strip.png")?,
            title: TitleTextures {
                title: Self::load_texture(creator, "assets/gfx/title/title.png")?,
                button: Self::load_texture(creator, "assets/gfx/title/button.png")?,
                button_icons: Self::load_texture(creator, "assets/gfx/title/button_icons.png")?,

                texts: ExtractedFontTextureSets::new(creator, font)?,
            },
            level_select: LevelSelectTextures {
                level_buttons: Self::load_texture(
                    creator,
                    "assets/gfx/level_select/level_buttons.png",
                )?,
                stars: Self::load_texture(creator, "assets/gfx/level_select/stars.png")?,
                perfect_star_highlight: Self::load_texture(
                    creator,
                    "assets/gfx/level_select/perfect_star_highlight.png",
                )?,
                digits: Self::load_texture(creator, "assets/gfx/level_select/digits.png")?,
            },
        })
    }
}

pub struct TitleTextures<'c> {
    pub title: Texture<'c>,
    pub button: Texture<'c>,
    pub button_icons: Texture<'c>,

    pub texts: ExtractedFontTextureSets<'c>,
}

pub struct LevelSelectTextures<'c> {
    pub level_buttons: Texture<'c>,
    pub stars: Texture<'c>,
    pub perfect_star_highlight: Texture<'c>,
    pub digits: Texture<'c>,
}

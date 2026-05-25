use sdl3::{
    image::LoadTexture,
    pixels::Color,
    render::{Texture, TextureCreator},
    ttf::Font,
    video::WindowContext,
};

use crate::component::title::button::ExtractedFontTextureSets;

pub struct Textures<'c> {
    pub strip: Texture<'c>,
    pub back_button: Texture<'c>,

    pub title: TitleTextures<'c>,
    pub level_select: LevelSelectTextures<'c>,
}

fn load_texture<'c>(creator: &'c TextureCreator<WindowContext>, name: &str) -> Option<Texture<'c>> {
    creator.load_texture(name).ok()
}

impl<'c> Textures<'c> {
    pub fn load(creator: &'c TextureCreator<WindowContext>, font: &Font) -> Option<Textures<'c>> {
        Some(Self {
            strip: load_texture(creator, "assets/gfx/strip.png")?,
            back_button: load_texture(creator, "assets/gfx/back.png")?,
            title: TitleTextures {
                title: load_texture(creator, "assets/gfx/title/title.png")?,
                button: load_texture(creator, "assets/gfx/title/button.png")?,
                button_icons: load_texture(creator, "assets/gfx/title/button_icons.png")?,

                texts: ExtractedFontTextureSets::new(creator, font)?,
            },
            level_select: LevelSelectTextures::load(creator, font)?,
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
    pub locked: Texture<'c>,

    pub header_text: Texture<'c>,
}

impl<'c> LevelSelectTextures<'c> {
    pub fn header_text(
        creator: &'c TextureCreator<WindowContext>,
        font: &Font,
    ) -> Option<Texture<'c>> {
        let surface = font
            .render("Level Select")
            .blended(Color::RGB(255, 255, 255))
            .ok()?;
        creator.create_texture_from_surface(&surface).ok()
    }

    pub fn load(creator: &'c TextureCreator<WindowContext>, font: &Font) -> Option<Self> {
        Some(Self {
            level_buttons: load_texture(creator, "assets/gfx/level_select/level_buttons.png")?,
            stars: load_texture(creator, "assets/gfx/level_select/stars.png")?,
            perfect_star_highlight: load_texture(
                creator,
                "assets/gfx/level_select/perfect_star_highlight.png",
            )?,
            digits: load_texture(creator, "assets/gfx/level_select/digits.png")?,
            locked: load_texture(creator, "assets/gfx/level_select/locked.png")?,
            header_text: Self::header_text(creator, font)?,
        })
    }
}

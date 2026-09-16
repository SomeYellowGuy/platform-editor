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
    pub level: LevelTextures<'c>,
}

fn load_texture<'c>(creator: &'c TextureCreator<WindowContext>, name: &str) -> Option<Texture<'c>> {
    if let Ok(t) = creator.load_texture(name) {
        Some(t)
    } else {
        println!("Coul not load texture: {name}");
        None
    }
}

fn load_tile_texture<'c>(
    creator: &'c TextureCreator<WindowContext>,
    name: &str,
) -> Option<Texture<'c>> {
    load_texture(creator, &("assets/gfx/tiles/".to_string() + name))
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
            level: LevelTextures::load(creator, font)?,
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

pub type TileTextures<'c> = platform_editor_core::textures::TileTextures<Texture<'c>>;
pub type DirectionalTextures<'c> = platform_editor_core::textures::DirectionalTextures<Texture<'c>>;
pub type PlacedBlockTextures<'c> = platform_editor_core::textures::PlacedBlockTextures<Texture<'c>>;
pub type MovingPlacedBlockTextures<'c> =
    platform_editor_core::textures::MovingPlacedBlockTextures<Texture<'c>>;

pub struct LevelTextures<'c> {
    pub tiles: TileTextures<'c>,

    pub player: Texture<'c>,
    pub flags: [Texture<'c>; 9],

    pub end_dialog: Texture<'c>,

    pub items_text: Texture<'c>,
}

impl<'c> LevelTextures<'c> {
    pub fn items_text(
        creator: &'c TextureCreator<WindowContext>,
        font: &Font,
    ) -> Option<Texture<'c>> {
        let surface = font
            .render("ITEMS")
            .blended(Color::RGB(255, 255, 255))
            .ok()?;
        creator.create_texture_from_surface(&surface).ok()
    }

    pub fn load(creator: &'c TextureCreator<WindowContext>, font: &Font) -> Option<Self> {
        let flags: Vec<_> = (1..=9)
            .filter_map(|n| load_texture(creator, &format!("assets/gfx/level/flag/{n}.png")))
            .collect();
        Some(Self {
            tiles: Self::load_tile_textures(creator)?,
            player: load_texture(creator, "assets/gfx/level/player.png")?,
            flags: flags.try_into().ok()?,
            end_dialog: load_texture(creator, "assets/gfx/level/end_dialog.png")?,
            items_text: Self::items_text(creator, font)?,
        })
    }

    pub fn load_tile_textures(
        creator: &'c TextureCreator<WindowContext>,
    ) -> Option<TileTextures<'c>> {
        Some(TileTextures {
            empty: load_tile_texture(creator, "empty.png")?,
            spikes: DirectionalTextures::new(
                Some(load_tile_texture(creator, "spike_up.png")?),
                Some(load_tile_texture(creator, "spike_down.png")?),
                None,
                None,
            ),
            shooters: DirectionalTextures::new(
                Some(load_tile_texture(creator, "shooter_up.png")?),
                None,
                Some(load_tile_texture(creator, "shooter_left.png")?),
                Some(load_tile_texture(creator, "shooter_right.png")?),
            ),
            placed_blocks: PlacedBlockTextures {
                moving: MovingPlacedBlockTextures {
                    single: DirectionalTextures::new(
                        Some(load_tile_texture(creator, "placedblock_m_up.png")?),
                        None,
                        Some(load_tile_texture(creator, "placedblock_m_left.png")?),
                        Some(load_tile_texture(creator, "placedblock_m_right.png")?),
                    ),
                    vertical: load_tile_texture(creator, "placedblock_m_vertical.png")?,
                    horizontal: load_tile_texture(creator, "placedblock_m_horizontal.png")?,
                },
                permanent: load_tile_texture(creator, "placedblock.png")?,
                timed: [
                    load_tile_texture(creator, "placedblock_t1.png")?,
                    load_tile_texture(creator, "placedblock_t2.png")?,
                    load_tile_texture(creator, "placedblock_t3.png")?,
                    load_tile_texture(creator, "placedblock_t4.png")?,
                    load_tile_texture(creator, "placedblock_t5.png")?,
                ],
            },
            grass: [
                load_tile_texture(creator, "grass_g.png")?,
                load_tile_texture(creator, "grass_g1.png")?,
                load_tile_texture(creator, "grass_g2.png")?,
                load_tile_texture(creator, "grass_g3.png")?,
            ],
            dirt: [
                load_tile_texture(creator, "grass_d.png")?,
                load_tile_texture(creator, "grass_d1.png")?,
                load_tile_texture(creator, "grass_d2.png")?,
                load_tile_texture(creator, "grass_d3.png")?,
                load_tile_texture(creator, "grass_d4.png")?,
            ],
            top_slab: load_tile_texture(creator, "block_slab_r.png")?,
            bottom_slab: load_tile_texture(creator, "block_slab.png")?,
            block: load_tile_texture(creator, "block.png")?,
        })
    }
}

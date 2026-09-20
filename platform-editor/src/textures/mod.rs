use sdl3::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use crate::component::title::button::ExtractedFontTextureSets;

mod dynamic;

pub use dynamic::{DynamicText, TextAlignment};

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
        println!("Could not load texture: {name}");
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
    pub fn load(creator: &'c TextureCreator<WindowContext>) -> Option<Textures<'c>> {
        Some(Self {
            strip: load_texture(creator, "assets/gfx/strip.png")?,
            back_button: load_texture(creator, "assets/gfx/back.png")?,
            title: TitleTextures {
                title: load_texture(creator, "assets/gfx/title/title.png")?,
                button: load_texture(creator, "assets/gfx/title/button.png")?,
                button_icons: load_texture(creator, "assets/gfx/title/button_icons.png")?,

                texts: ExtractedFontTextureSets::new(creator),
            },
            level_select: LevelSelectTextures::load(creator)?,
            level: LevelTextures::load(creator)?,
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

    pub header_text: DynamicText<'c>,
    pub stars_text: DynamicText<'c>,
}

impl<'c> LevelSelectTextures<'c> {
    pub fn load(creator: &'c TextureCreator<WindowContext>) -> Option<Self> {
        Some(Self {
            level_buttons: load_texture(creator, "assets/gfx/level_select/level_buttons.png")?,
            stars: load_texture(creator, "assets/gfx/level_select/stars.png")?,
            perfect_star_highlight: load_texture(
                creator,
                "assets/gfx/level_select/perfect_star_highlight.png",
            )?,
            digits: load_texture(creator, "assets/gfx/level_select/digits.png")?,
            locked: load_texture(creator, "assets/gfx/level_select/locked.png")?,
            header_text: DynamicText::new(creator),
            stars_text: DynamicText::new(creator),
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
    pub hit_flag: Texture<'c>,

    pub end_dialog: EndDialogTextures<'c>,

    pub items_text: DynamicText<'c>,
}

impl<'c> LevelTextures<'c> {
    pub fn load(creator: &'c TextureCreator<WindowContext>) -> Option<Self> {
        let flags: Vec<_> = (1..=9)
            .filter_map(|n| load_texture(creator, &format!("assets/gfx/level/flag/{n}.png")))
            .collect();
        Some(Self {
            tiles: Self::load_tile_textures(creator)?,
            player: load_texture(creator, "assets/gfx/level/player.png")?,
            flags: flags.try_into().ok()?,
            hit_flag: load_texture(creator, "assets/gfx/level/flag/hit.png")?,
            end_dialog: EndDialogTextures::load(creator)?,
            items_text: DynamicText::new(creator),
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

pub struct EndDialogTextures<'c> {
    pub base: Texture<'c>,
    pub nice_text: DynamicText<'c>,
    pub level_text: DynamicText<'c>,
}

impl<'c> EndDialogTextures<'c> {
    pub fn load(creator: &'c TextureCreator<WindowContext>) -> Option<Self> {
        Some(Self {
            base: load_texture(creator, "assets/gfx/level/end_dialog/base.png")?,
            nice_text: DynamicText::new(creator),
            level_text: DynamicText::new(creator),
        })
    }
}

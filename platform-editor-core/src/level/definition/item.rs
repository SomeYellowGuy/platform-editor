use crate::{
    level::definition::{CollectibleType, MovingBlockItem, Tile},
    textures::level::{CollectibleTextures, TileTextures},
};

/// A type of something that can be placed by a player.
#[derive(Debug, Clone)]
pub enum Item {
    Block,
    TimedBlock(i32),
    Moving(MovingBlockItem),
    GravityOrb,
    Star,
}

impl Item {
    pub fn icon_texture<'a, T>(&self, textures: &'a TileTextures<T>) -> Option<&'a T> {
        match self {
            Self::Block => Some(&textures.placed_blocks.permanent),
            Self::TimedBlock(t) => Some(textures.placed_blocks.timed_texture(*t as usize)),
            Self::Moving(moving_block_item) => moving_block_item.texture(textures),
            // TODO
            Self::GravityOrb => None,
            Self::Star => None,
        }
    }

    pub fn icon_texture_mut<'a, T>(
        &self,
        tile_textures: &'a mut TileTextures<T>,
        collectible_textures: &'a mut CollectibleTextures<T>,
    ) -> Option<&'a mut T> {
        match self {
            Self::Block => Some(&mut tile_textures.placed_blocks.permanent),
            Self::TimedBlock(t) => Some(tile_textures.placed_blocks.timed_texture_mut(*t as usize)),
            Self::Moving(moving_block_item) => moving_block_item.texture_mut(tile_textures),
            // TODO
            Self::GravityOrb => Some(&mut collectible_textures.gravity_orb),
            Self::Star => Some(&mut collectible_textures.star),
        }
    }

    pub fn icon_texture_scale(&self) -> f32 {
        match self {
            Self::GravityOrb | Self::Star => 0.8,
            _ => 1.0,
        }
    }

    pub fn place_outcome(&self) -> ItemPlaceOutcome {
        match self {
            Self::Block => ItemPlaceOutcome::Tile(Tile::PlacedBlock),
            Self::TimedBlock(t) => ItemPlaceOutcome::Tile(Tile::PlacedTimedBlock(*t as f32)),
            Self::Moving(moving_block_item) => ItemPlaceOutcome::Moving(moving_block_item.clone()),
            Self::GravityOrb => ItemPlaceOutcome::Collectible(CollectibleType::GravityOrb),
            Self::Star => ItemPlaceOutcome::Collectible(CollectibleType::Star(usize::MAX)),
        }
    }
}

pub enum ItemPlaceOutcome {
    Tile(Tile),
    Moving(MovingBlockItem),
    Collectible(CollectibleType),
}

/// Represents a type of item (which can be placed) and its remaining count.
#[derive(Debug, Clone)]
pub struct ItemStack {
    pub item: Item,
    pub count: u32,
}

impl ItemStack {
    pub const fn new(item: Item, count: u32) -> Self {
        Self { item, count }
    }
}

use crate::{
    level::definition::{MovingBlockItem, Tile},
    textures::level::TileTextures,
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

    pub fn icon_texture_mut<'a, T>(&self, textures: &'a mut TileTextures<T>) -> Option<&'a mut T> {
        match self {
            Self::Block => Some(&mut textures.placed_blocks.permanent),
            Self::TimedBlock(t) => Some(textures.placed_blocks.timed_texture_mut(*t as usize)),
            Self::Moving(moving_block_item) => moving_block_item.texture_mut(textures),
            // TODO
            Self::GravityOrb => None,
            Self::Star => None,
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
            Self::GravityOrb => todo!(),
            Self::Star => todo!(),
        }
    }
}

pub enum ItemPlaceOutcome {
    Tile(Tile),
    Moving(MovingBlockItem),
    // TODO: Add collectible outcome
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

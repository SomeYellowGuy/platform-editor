use crate::{common_util::Direction, textures::level::TileTextures};

/// A structure that describes a type of moving block.
#[derive(Debug, Clone)]
pub enum MovingBlockItem {
    Single(Direction),
    Horizontal,
    Vertical,
}

impl MovingBlockItem {
    pub fn texture<'a, T>(&self, textures: &'a TileTextures<T>) -> Option<&'a T> {
        match self {
            Self::Single(d) => textures.placed_blocks.moving.single.get(*d),
            Self::Horizontal => Some(&textures.placed_blocks.moving.horizontal),
            Self::Vertical => Some(&textures.placed_blocks.moving.vertical),
        }
    }

    pub fn texture_mut<'a, T>(&self, textures: &'a mut TileTextures<T>) -> Option<&'a mut T> {
        match self {
            Self::Single(d) => textures.placed_blocks.moving.single.get_mut(*d),
            Self::Horizontal => Some(&mut textures.placed_blocks.moving.horizontal),
            Self::Vertical => Some(&mut textures.placed_blocks.moving.vertical),
        }
    }
}

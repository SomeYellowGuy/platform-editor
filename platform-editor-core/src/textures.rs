use crate::common_util::Direction;

/// A structure that may or may not hold a texture or image for each direction.
pub struct DirectionalTextures<T> {
    textures: [Option<T>; 4]
}

impl<T> DirectionalTextures<T> {
    pub fn new(
        up: Option<T>,
        down: Option<T>,
        left: Option<T>,
        right: Option<T>
    ) -> Self {
        Self {
            textures: [
                up, down, left, right
            ]
        }
    }

    pub fn get(&self, direction: Direction) -> Option<&T> {
        match direction {
            Direction::Up => self.textures[0].as_ref(),
            Direction::Down => self.textures[1].as_ref(),
            Direction::Left => self.textures[2].as_ref(),
            Direction::Right => self.textures[3].as_ref(),
        }
    }
}
pub struct PlacedBlockTextures<T> {
    pub moving: MovingPlacedBlockTextures<T>,
    pub permanent: T,
    pub timed: [T; 5]
}

pub struct MovingPlacedBlockTextures<T> {
    pub single: DirectionalTextures<T>,
    pub vertical: T,
    pub horizontal: T,
}

pub struct TileTextures<T> {
    pub spikes: DirectionalTextures<T>,
    pub shooters: DirectionalTextures<T>,
    pub placed_blocks: PlacedBlockTextures<T>,
    pub grass: [T; 4],
    pub dirt: [T; 5],
    pub top_slab: T,
    pub bottom_slab: T,
    pub block: T
}
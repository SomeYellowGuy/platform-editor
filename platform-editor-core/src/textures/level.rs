use crate::{common_util::Direction, level::Tile};

/// A structure that may or may not hold a texture or image for each direction.
pub struct DirectionalTextures<T> {
    textures: [Option<T>; 4],
}

impl<T> DirectionalTextures<T> {
    pub fn new(up: Option<T>, down: Option<T>, left: Option<T>, right: Option<T>) -> Self {
        Self {
            textures: [up, down, left, right],
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

    pub fn get_mut(&mut self, direction: Direction) -> Option<&mut T> {
        match direction {
            Direction::Up => self.textures[0].as_mut(),
            Direction::Down => self.textures[1].as_mut(),
            Direction::Left => self.textures[2].as_mut(),
            Direction::Right => self.textures[3].as_mut(),
        }
    }
}
pub struct PlacedBlockTextures<T> {
    pub moving: MovingPlacedBlockTextures<T>,
    pub permanent: T,
    pub timed: [T; 5],
}

impl<T> PlacedBlockTextures<T> {
    /// Returns a reference to the texture used for a hypothetical timed placed block.
    pub fn timed_texture(&self, timer: i32) -> &T {
        &self.timed[(timer as usize).min(self.timed.len() - 1)]
    }

    /// Returns a mutable reference to the texture used for a hypothetical timed placed block.
    pub fn timed_texture_mut(&mut self, timer: i32) -> &mut T {
        &mut self.timed[(timer as usize).min(self.timed.len() - 1)]
    }

    /// Returns the texture used for an already-placed timed block.
    pub fn runtime_timed_texture(&self, timer: f32) -> &T {
        &self.timed[(timer.ceil() as usize).min(self.timed.len() - 1)]
    }
}

pub struct MovingPlacedBlockTextures<T> {
    pub single: DirectionalTextures<T>,
    pub vertical: T,
    pub horizontal: T,
}

pub struct TileTextures<T> {
    pub empty: T,
    pub void: T,

    pub spikes: DirectionalTextures<T>,
    pub shooters: DirectionalTextures<T>,
    pub placed_blocks: PlacedBlockTextures<T>,
    pub grass: [T; 4],
    pub dirt: [T; 5],
    pub top_slab: T,
    pub bottom_slab: T,
    pub block: T,
}

impl<T> TileTextures<T> {
    pub fn texture_from_tile(&self, tile: &Tile) -> Option<&T> {
        match tile {
            Tile::Empty => None,
            Tile::Block => Some(&self.block),
            Tile::TopSlab => Some(&self.top_slab),
            Tile::BottomSlab => Some(&self.bottom_slab),
            Tile::Grass(i) => Some(&self.grass[*i]),
            Tile::Dirt(i) => Some(&self.dirt[*i]),
            Tile::PlacedBlock => Some(&self.placed_blocks.permanent),
            Tile::PlacedTimedBlock(t) => {
                let shown = t.floor() as usize;
                if shown < self.placed_blocks.timed.len() {
                    Some(&self.placed_blocks.timed[shown])
                } else {
                    None
                }
            }
            Tile::Shooter(direction) => self.shooters.get(*direction),
            Tile::Spike(direction) => self.spikes.get(*direction),
        }
    }

    pub fn texture_from_tile_mut(&mut self, tile: &Tile) -> Option<&mut T> {
        match tile {
            Tile::Empty => None,
            Tile::Block => Some(&mut self.block),
            Tile::TopSlab => Some(&mut self.top_slab),
            Tile::BottomSlab => Some(&mut self.bottom_slab),
            Tile::Grass(i) => Some(&mut self.grass[*i]),
            Tile::Dirt(i) => Some(&mut self.dirt[*i]),
            Tile::PlacedBlock => Some(&mut self.placed_blocks.permanent),
            Tile::PlacedTimedBlock(t) => {
                let shown = t.floor() as usize;
                if shown < self.placed_blocks.timed.len() {
                    Some(&mut self.placed_blocks.timed[shown])
                } else {
                    None
                }
            }
            Tile::Shooter(direction) => self.shooters.get_mut(*direction),
            Tile::Spike(direction) => self.spikes.get_mut(*direction),
        }
    }
}

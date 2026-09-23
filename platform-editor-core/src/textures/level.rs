use crate::{
    common_util::Direction,
    level::{CollectibleType, Tile},
};

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
    fn timer_index(&self, timer: f32) -> Option<usize> {
        let ceil = usize::try_from(timer.ceil() as i32 - 1).ok()?;
        if ceil >= self.timed.len() {
            None
        } else {
            Some(ceil)
        }
    }

    /// Returns a reference to the texture used for a hypothetical timed placed block.
    pub fn timed_texture(&self, timer: usize) -> &T {
        &self.timed[timer - 1]
    }

    /// Returns a mutable reference to the texture used for a hypothetical timed placed block (if any).
    pub fn timed_texture_mut(&mut self, timer: usize) -> &mut T {
        &mut self.timed[timer - 1]
    }

    /// Returns a reference to the texture used for an already-placed timed block (if any).
    pub fn runtime_timed_texture(&self, timer: f32) -> Option<&T> {
        self.timer_index(timer).map(|t| &self.timed[t])
    }

    /// Returns a mutable reference to the texture used for an already-placed timed block.
    pub fn runtime_timed_texture_mut(&mut self, timer: f32) -> Option<&mut T> {
        self.timer_index(timer).map(|t| &mut self.timed[t])
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
            Tile::PlacedTimedBlock(t) => self.placed_blocks.runtime_timed_texture(*t),
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
            Tile::PlacedTimedBlock(t) => self.placed_blocks.runtime_timed_texture_mut(*t),
            Tile::Shooter(direction) => self.shooters.get_mut(*direction),
            Tile::Spike(direction) => self.spikes.get_mut(*direction),
        }
    }
}

pub struct CollectibleTextures<T> {
    pub star: T,
    pub gravity_orb: T,
}

impl<T> CollectibleTextures<T> {
    pub fn texture_from_collectible(&self, collectible: CollectibleType) -> Option<&T> {
        match collectible {
            CollectibleType::Star(_) => Some(&self.star),
            CollectibleType::GravityOrb => Some(&self.gravity_orb),
            _ => None,
        }
    }

    pub fn texture_from_collectible_mut(&mut self, collectible: CollectibleType) -> Option<&mut T> {
        match collectible {
            CollectibleType::Star(_) => Some(&mut self.star),
            CollectibleType::GravityOrb => Some(&mut self.gravity_orb),
            _ => None,
        }
    }
}

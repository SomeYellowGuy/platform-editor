use crate::{common_util::{Direction, FPos, Pos}, level::scratch::StoredScratchLevel};

pub mod scratch;


#[derive(Debug, Clone, Copy)]
pub enum LockColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue
}

/// A tile at runtime. Locks are not stored here,
/// but are rather stored separately.
pub enum Tile {
    Empty,
    Block,
    TopSlab,
    BottomSlab,
    Grass(usize),
    Dirt(usize),

    PlacedBlock,
    PlacedTimeBlock(f32),

    Shooter(Direction),
    Spike(Direction)
}

pub struct StoredLock {
    pub pos: FPos,
    pub size: FPos
}

pub struct LevelState {
    pub tiles: Vec<Tile>,
    pub size: Pos<usize>
}

impl LevelState {
    pub fn tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y * self.size.y + x]
    }

    pub fn tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.tiles[y * self.size.y + x]
    }

    pub fn load_scratch_level(&mut self, index: usize) {
        let level = &crate::level::scratch::levels::LEVELS[index];
        for x in 0..StoredScratchLevel::WIDTH {
            for y in 0..StoredScratchLevel::HEIGHT {
                let i = y * self.size.y + x;
                if let Some(tile) = level.tiles[i].to_tile_state(i) {
                    *self.tile_mut(x, y) = tile
                }
            }
        }
    }
}
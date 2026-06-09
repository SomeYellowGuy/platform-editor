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
#[derive(Debug, Default, Clone)]
pub enum Tile {
    #[default]
    Empty,
    Block,
    TopSlab,
    BottomSlab,
    Grass(usize),
    Dirt(usize),

    PlacedBlock,
    PlacedTimedBlock(f32),

    Shooter(Direction),
    Spike(Direction)
}

pub struct StoredLock {
    pub pos: FPos,
    pub size: FPos
}

#[derive(Debug, Default, Clone)]
pub struct LevelState {
    pub tiles: Vec<Tile>,
    pub size: Pos<usize>
}

impl LevelState {
    /// Creates a new state, into which a level can be loaded.
    pub fn new() -> Self {
        LevelState::default()
    }
    
    pub fn tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y * self.size.x + x]
    }

    pub fn tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.tiles[y * self.size.x + x]
    }

    pub fn load_scratch_level(&mut self, index: usize) {
        let level = &crate::level::scratch::levels::LEVELS[index];
        let mut tiles = Vec::new();
        self.size = Pos::new(StoredScratchLevel::WIDTH, StoredScratchLevel::HEIGHT);
        for y in 0..StoredScratchLevel::HEIGHT {
            for x in 0..StoredScratchLevel::WIDTH {
                let mut set_tile = Tile::Empty;
                let i = y * self.size.x + x;
                if let Some(tile) = level.tiles[i].to_tile_state(i) {
                    set_tile = tile
                }
                tiles.push(set_tile);
            }
        }
        self.tiles = tiles;
    }
}
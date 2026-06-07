use crate::common_util::{Direction, FPos};

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
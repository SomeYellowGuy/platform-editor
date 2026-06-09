use crate::{common_util::Pos, level::scratch::{StoredScratchLevel, StoredScratchTile}};

macro_rules! level {
    ($level:literal, $tiles:literal, $start_pos:expr, $collectibles: expr) => {
        StoredScratchLevel {
            tiles: StoredScratchTile::tiles_from_bytes($tiles),
            collectibles: &[],
            start_pos: $start_pos
        }
    };
}

/// The total number of Scratch levels in the game.
pub const LEVEL_COUNT: usize = 30;

pub const LEVELS: [StoredScratchLevel; 1] = [
    level!(
        1, br"00000000000000000000000000000000000000000000000000000000000000222000000000033322222222223333333333333333",
        Pos::new(1.5, 4.5),
        []
    )
];
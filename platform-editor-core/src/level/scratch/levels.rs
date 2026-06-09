use crate::{
    common_util::Vec2,
    level::scratch::{StoredScratchLevel, StoredScratchTile},
};

macro_rules! level {
    ($level:literal, $tiles:literal, $start_pos:expr, $collectibles: expr) => {
        StoredScratchLevel {
            tiles: StoredScratchTile::tiles_from_bytes($tiles),
            collectibles: &[],
            start_pos: $start_pos,
        }
    };
}

/// The total number of Scratch levels in the game.
pub const LEVEL_COUNT: usize = 30;

pub const LEVELS: [StoredScratchLevel; 3] = [
    level!(
        1, br"00000000000000000000000000000000000000000000000000000000000000222000000000033322222222223333333333333333",
        Vec2::new(1.5, 4.5),
        []
    ),
    level!(
        2, br"00000000000000000000000000000000000002200000000000330000000000033000000000003322000000000333300000000033",
        Vec2::new(1.0, 4.5),
        []
    ),
    level!(
        3, br"00000000000000000000000000000000000000022200000000003330000001000333000002222233300000333333330000033333",
        Vec2::new(1.5, 1.8),
        []
    )
];

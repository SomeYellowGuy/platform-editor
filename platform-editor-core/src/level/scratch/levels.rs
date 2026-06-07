use crate::level::scratch::{StoredScratchLevel, StoredScratchTile};

macro_rules! level {
    ($level:literal, $tiles:literal, $collectibles: expr) => {
        StoredScratchLevel {
            tiles: StoredScratchTile::tiles_from_str($tiles),
            collectibles: &[],
        }
    };
}

pub const LEVELS: [StoredScratchLevel; 1] = [
    level!(
        1, "00000000000000000000000000000000000000000000000000000000000000222000000000033322222222223333333333333333",
        []
    )
];
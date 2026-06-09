use crate::{common_util::{Direction, FPos, digit_count}, level::{LockColor, Tile}};

pub mod levels;

#[derive(Debug, Clone, Copy)]
pub enum ShooterDirection {
    Left,
    Right,
    Up
}

impl From<ShooterDirection> for Direction {
    fn from(value: ShooterDirection) -> Self {
        match value {
            ShooterDirection::Left => Self::Left,
            ShooterDirection::Right => Self::Right,
            ShooterDirection::Up => Self::Up,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SpikeDirection {
    Up,
    Down
}

impl From<SpikeDirection> for Direction {
    fn from(value: SpikeDirection) -> Self {
        match value {
            SpikeDirection::Up => Self::Up,
            SpikeDirection::Down => Self::Down,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LockAxis {
    X,
    Y
}

/// A tile for an original Scratch level.
#[derive(Debug, Clone, Copy)]
pub enum StoredScratchTile {
    Empty,
    Block,
    TopSlab,
    BottomSlab,
    Grass,
    Dirt,

    Shooter(ShooterDirection),
    Spike(SpikeDirection),
    Lock(LockColor, LockAxis)
}

impl StoredScratchTile {
    /// Converts a `u8` byte to a tile from the original Scratch game.
    pub const fn from_byte(ch: u8) -> Option<Self> {
        // Taken from the Scratch game.
        match ch {
            b'0' => Some(Self::Empty),
            b'1' => Some(Self::Block),
            b'2' => Some(Self::Grass),
            b'3' => Some(Self::Dirt),
            b'4' | b'5' => Some(Self::Spike(SpikeDirection::Up)),
            b'6' => Some(Self::Shooter(ShooterDirection::Left)),
            b'7' => Some(Self::Shooter(ShooterDirection::Right)),
            b'8' => Some(Self::Shooter(ShooterDirection::Up)),
            b'9' => Some(Self::Spike(SpikeDirection::Down)),
            b'A' => Some(Self::Lock(LockColor::Red, LockAxis::Y)),
            b'B' => Some(Self::Lock(LockColor::Orange, LockAxis::Y)),
            b'C' => Some(Self::Lock(LockColor::Yellow, LockAxis::Y)),
            b'D' => Some(Self::Lock(LockColor::Green, LockAxis::Y)),
            b'E' => Some(Self::BottomSlab),
            b'F' => Some(Self::Lock(LockColor::Red, LockAxis::X)),
            b'G' => Some(Self::TopSlab),
            b'H' => Some(Self::Lock(LockColor::Orange, LockAxis::X)),
            b'I' => Some(Self::Lock(LockColor::Blue, LockAxis::Y)),
            b'J' => Some(Self::Lock(LockColor::Blue, LockAxis::X)),
            b'K' => Some(Self::Lock(LockColor::Yellow, LockAxis::X)),
            _ => None
        }
    }

    pub const fn tiles_from_bytes<'a>(bytes: &'a [u8; StoredScratchLevel::TILE_COUNT]) -> [StoredScratchTile; StoredScratchLevel::TILE_COUNT] {
        let mut out = [StoredScratchTile::Empty; StoredScratchLevel::TILE_COUNT];

        let mut i = 0;
        while i < StoredScratchLevel::TILE_COUNT {
            out[i] = Self::from_byte(bytes[i]).unwrap();
            i += 1;
        }

        out
    }

    /// Calculates a number for a tile space. This is used to give different
    /// tile types for grass an dirt.
    fn tile_seed(n: usize) -> usize {
        (n * n + 4 * digit_count(n)) % 15
    }

    pub fn to_tile_state(&self, index: usize) -> Option<Tile> {
        match self {
            Self::Empty => Some(Tile::Empty),
            Self::Block => Some(Tile::Block),
            Self::TopSlab => Some(Tile::TopSlab),
            Self::BottomSlab => Some(Tile::BottomSlab),
            Self::Grass => {
                let seed = Self::tile_seed(index + 1);
                Some(Tile::Grass(
                    if seed < 4 {
                        seed
                    } else {
                        0
                    }
                ))
            },
            Self::Dirt => {
                let seed = Self::tile_seed(index + 1);
                Some(Tile::Dirt(
                    if seed < 5 {
                        seed
                    } else {
                        0
                    }
                ))
            },
            Self::Shooter(shooter_direction) => Some(Tile::Shooter((*shooter_direction).into())),
            Self::Spike(spike_direction) => Some(Tile::Shooter((*spike_direction).into())),
            Self::Lock(_, _) => None,
        }
    }
}

// An original Scratch level, which can be loaded in the game.
#[derive(Debug, Clone, Copy)]
pub struct StoredScratchLevel {
    pub tiles: [StoredScratchTile; StoredScratchLevel::WIDTH * StoredScratchLevel::HEIGHT],
    pub collectibles: &'static [StoredScratchCollectible],

    // The player's start position.
    pub start_pos: FPos
}

impl StoredScratchLevel {
    pub const WIDTH: usize = 13;
    pub const HEIGHT: usize = 8;
    pub const TILE_COUNT: usize = Self::WIDTH * Self::HEIGHT;
}

#[derive(Debug, Clone, Copy)]
pub struct StoredScratchCollectible {
    pub pos: FPos,
    pub kind: ScratchCollectible
}

#[derive(Debug, Clone, Copy)]
pub enum ScratchCollectible {
    Orb,
    Key(i8)
}
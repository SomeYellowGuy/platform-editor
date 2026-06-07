use crate::{common_util::FPos, level::LockColor};

pub mod levels;

#[derive(Debug, Clone, Copy)]
pub enum ShooterDirection {
    Left,
    Right,
    Up
}

#[derive(Debug, Clone, Copy)]
pub enum SpikeDirection {
    Up,
    Down
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

    pub const fn tiles_from_str<'a>(str: &'a str) -> [StoredScratchTile; 13 * 8] {
        let bytes = str.as_bytes();
        let mut out = [StoredScratchTile::Empty; 13 * 8];

        let mut i = 0;
        while i < 13 * 8 {
            out[i] = Self::from_byte(bytes[i]).unwrap();
            i += 1;
        }

        out
    }
}

// An original Scratch level, which can be loaded in the game.
pub struct StoredScratchLevel {
    pub tiles: [StoredScratchTile; 13 * 8],
    pub collectibles: &'static [StoredScratchCollectible]
}

pub struct StoredScratchCollectible {
    pub pos: FPos,
    pub kind: ScratchCollectible
}

pub enum ScratchCollectible {
    Orb,
    Key(i8)
}
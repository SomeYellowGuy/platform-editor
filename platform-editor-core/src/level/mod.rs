use crate::common_util::{Direction, Rectf};

pub mod definition;
mod lock_color_map;
pub mod scratch;
pub mod state;

pub use lock_color_map::{FilledIter, FilledIterMut, FilledLockColorMap, LockColorMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LockColor {
    Red = 0,
    Orange = 1,
    Yellow = 2,
    Green = 3,
    Blue = 4,
}

impl LockColor {
    pub const ALL: [Self; 5] = [
        Self::Red,
        Self::Orange,
        Self::Yellow,
        Self::Green,
        Self::Blue,
    ];

    pub const fn border_color(self) -> u32 {
        match self {
            Self::Red => 0xff52_0000,
            Self::Orange => 0xff52_1c00,
            Self::Yellow => 0xff52_4f00,
            Self::Green => 0xff05_5200,
            Self::Blue => 0xff00_3152,
        }
    }

    pub const fn gradient(self) -> (u32, u32) {
        match self {
            Self::Red => (0xffff_0000, 0xffff_0000),
            Self::Orange => (0xffff_5600, 0xffff_c600),
            Self::Yellow => (0xfff4_ff00, 0xffff_d400),
            Self::Green => (0xff0_0ff2c, 0xff00_ff93),
            Self::Blue => (0xff00_c4ff, 0xff00_8bff),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockBorderType {
    None,
    OnlyHorizontal,
    OnlyVertical,
    Both,
}

impl LockBorderType {
    pub fn vertical(self) -> bool {
        matches!(self, Self::OnlyVertical | Self::Both)
    }

    pub fn horizontal(self) -> bool {
        matches!(self, Self::OnlyHorizontal | Self::Both)
    }

    pub fn shrink_rect(self, mut rect: Rectf, thickness: f32) -> Rectf {
        let horizontal = self.horizontal();
        let vertical = self.vertical();

        if horizontal {
            rect.pos.y += thickness / 2.0;
            rect.dimensions.y -= thickness;
        }
        if vertical {
            rect.pos.x += thickness / 2.0;
            rect.dimensions.x -= thickness;
        }

        rect
    }
}

/// An extra condition for a star to be collected when a level is finished.
#[derive(Debug, Clone)]
pub enum StarCondition {
    /// The star of the given index must be collected (it is present somewhere in the level space).
    Collect(usize),
    /// The level must be finished in the stored number of seconds.
    Time(u32),
    /// The number of items placed must at most be equal to this value.
    Items(u32),
    /// At least this number of enemies must be defeated.
    EnemiesDefeated(u32),
    /// At least this number of enemies must be left alive.
    EnemiesLeft(u32),
    /// The player must be in this gravity state.
    Gravity(Direction),
}

impl StarCondition {
    pub fn number_display(&self) -> Option<u32> {
        match self {
            Self::Time(n) | Self::Items(n) | Self::EnemiesDefeated(n) | Self::EnemiesLeft(n) => {
                Some(*n)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Normal,
    Slow,
}

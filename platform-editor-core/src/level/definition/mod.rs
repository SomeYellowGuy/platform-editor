mod collectible;
mod flag;
mod item;
mod moving;
mod tile;
mod unlocking;

pub use collectible::{Collectible, CollectibleType};
pub use flag::FlagState;
pub use item::{Item, ItemPlaceOutcome, ItemStack};
pub use moving::MovingBlockItem;
pub use tile::Tile;
pub use unlocking::StoredLock;

use crate::{
    common_util::{Direction, Vec2f},
    level::EnemyType,
};

#[derive(Debug, Clone)]
pub struct Enemy {
    pub ty: EnemyType,
    pub pos: Vec2f,
}

impl Enemy {
    pub const fn new(ty: EnemyType, pos: Vec2f) -> Self {
        Self { ty, pos }
    }

    pub const fn normal(pos: Vec2f) -> Self {
        Self::new(EnemyType::Normal, pos)
    }

    pub const fn slow(pos: Vec2f) -> Self {
        Self::new(EnemyType::Slow, pos)
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
    Enemies(u32),
    /// At least this number of enemies must be left alive.
    EnemiesLeft(u32),
    /// The player must be in this gravity state.
    Gravity(Direction),
}

impl StarCondition {
    pub fn number_display(&self) -> Option<u32> {
        match self {
            Self::Time(n) | Self::Items(n) | Self::Enemies(n) | Self::EnemiesLeft(n) => Some(*n),
            _ => None,
        }
    }
}

use crate::{
    component::{Hold, HoldBase},
    hold_impl,
};

/// Provides level select level button bahavior.
#[derive(Debug, Clone)]
pub struct LevelSelectButtonBase {
    /// The level this button represents.
    pub level: usize,

    base: HoldBase,
}

hold_impl!(LevelSelectButtonBase: 600_000_000);

impl LevelSelectButtonBase {
    pub const MAX_DISPLAY_HOLD_TIME: u32 = 300_000_000;

    pub fn scale_multiplier(&self) -> f32 {
        let capped_time = self.base.hold_time.min(Self::MAX_DISPLAY_HOLD_TIME);
        let gradient = 1.0 - capped_time as f32 / Self::MAX_DISPLAY_HOLD_TIME as f32;
        1.0 + 0.1 * (1.0 - gradient * gradient)
    }

    pub const fn new(level: usize) -> Self {
        Self {
            level,
            base: HoldBase::new(),
        }
    }
}

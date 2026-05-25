use crate::component::Hold;

/// Provides level select level button bahavior.
#[derive(Debug, Clone, Copy)]
pub struct LevelSelectButtonBase {
    /// The level this button represents.
    pub level: usize,

    /// The held time for this button (in nanoseconds), which increases when held and decreases when released
    /// by the current delta time.
    ///
    /// This will be from 0 to 600,000,000.
    pub hold_time: u32,

    /// Whether this button is currently being held.
    pub held: bool,
}

impl Hold for LevelSelectButtonBase {
    const MAX_HOLD_TIME: u32 = 600_000_000;

    fn hold_time(&self) -> u32 {
        self.hold_time
    }

    fn set_hold_time(&mut self, new_time: u32) {
        self.hold_time = new_time;
    }
}

impl LevelSelectButtonBase {
    pub const MAX_DISPLAY_HOLD_TIME: u32 = 300_000_000;

    pub fn scale_multiplier(&self) -> f32 {
        let capped_time = self.hold_time.min(Self::MAX_DISPLAY_HOLD_TIME);
        let gradient = 1.0 - capped_time as f32 / Self::MAX_DISPLAY_HOLD_TIME as f32;
        1.0 + 0.1 * (1.0 - gradient * gradient)
    }

    pub const fn new(level: usize) -> Self {
        Self {
            level,
            hold_time: 0,
            held: false,
        }
    }
}

use std::time::Instant;

use crate::component::Hold;

/// The bottom bar of the level screen, displaying the current time,
/// level number and FPS (possibly).
#[derive(Debug, Default)]
pub struct BottomBarBase {
    pub state: TimeState,
}

/// The time state stored by a bottom bar.
#[derive(Debug, Default, Clone, Copy)]
pub enum TimeState {
    /// The player hasn't moved yet.
    #[default]
    Stationary,
    /// The level is ongoing, but not finished.
    /// The `Instant` is the go instant.
    Active(Instant),
    /// The level is finished.
    /// The `u32` is the finally displayed time value. It may be capped.
    Finished(u32),
}

impl TimeState {
    pub fn time_counter(&self) -> u32 {
        match self {
            Self::Stationary => 0,
            Self::Active(instant) => instant
                .elapsed()
                .as_secs()
                .min(BottomBarBase::MAX_DISPLAY_TIME) as u32,
            Self::Finished(time) => *time,
        }
    }
}

impl BottomBarBase {
    pub const MAX_DISPLAY_TIME: u64 = 999;

    pub fn new() -> Self {
        Self::default()
    }
}

/// Represents a button on the bottom of the level screen (bottom bar).
pub struct BottomBarButtonBase {
    pub hold_time: u32,
    pub ty: BottomBarButtonType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BottomBarButtonType {
    Reset,
    Options,
    LevelSelect,
}

impl BottomBarButtonType {
    pub const ALL: [Self; 3] = [Self::Reset, Self::Options, Self::LevelSelect];
}

impl Hold for BottomBarButtonBase {
    const MAX_HOLD_TIME: u32 = 400_000_000;

    fn hold_time(&self) -> u32 {
        self.hold_time
    }

    fn set_hold_time(&mut self, new_time: u32) {
        self.hold_time = new_time
    }
}

impl BottomBarButtonBase {
    pub const FADE_IN_TIME: f32 = 0.5;

    pub fn new(ty: BottomBarButtonType) -> Self {
        Self { ty, hold_time: 0 }
    }

    pub const fn scale_multiplier(&self) -> f32 {
        let gradient = 1.0 - self.hold_time as f32 / Self::MAX_HOLD_TIME as f32;
        0.9 + 0.1 * (1.0 - gradient * gradient)
    }
}

use std::time::Instant;

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

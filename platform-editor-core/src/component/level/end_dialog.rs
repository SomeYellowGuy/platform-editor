use std::time::Instant;

use crate::{
    component::Hold,
    level::{StarCondition, state::LevelState},
};

/// The pop-up that shows when you beat a level.
#[derive(Debug, Clone)]
pub struct EndDialogBase {
    pub start: Instant,
    /// All statuses of the stars starting from the second (the first is guaranteed to be collected).
    pub star_statuses: Vec<StarStatus>,
}

impl EndDialogBase {
    pub const DELAY: f32 = 1.0;
    pub const FADE_IN_TIME: f32 = 0.5;

    pub const STAR_DELAY: f32 = 1.0;
    pub const STAR_ANIMATION_DURATION: f32 = 1.6;
}

#[derive(Debug, Clone)]
pub struct StarStatus {
    pub collected: bool,
    pub condition: StarCondition,
}

impl EndDialogBase {
    pub fn from_level_state(state: &LevelState) -> Self {
        Self {
            start: state.finish_instant.unwrap_or_else(Instant::now),
            star_statuses: state.star_statuses(),
        }
    }

    /// The lifetime of this dialog in seconds.
    pub fn lifetime(&self) -> f32 {
        self.start.elapsed().as_secs_f32()
    }
}

/// Represents a button on the end dialog.
pub struct EndDialogButtonBase {
    pub start: Instant,
    pub hold_time: u32,
    pub ty: EndDialogButtonType,
}

impl EndDialogButtonBase {
    pub const TOTAL_DELAY: f32 = EndDialogBase::DELAY
        + EndDialogBase::FADE_IN_TIME
        + EndDialogBase::STAR_DELAY
        + EndDialogBase::STAR_ANIMATION_DURATION;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EndDialogButtonType {
    LevelSelect,
    Next,
    Retry,
}

impl EndDialogButtonType {
    pub const ALL: [Self; 3] = [Self::LevelSelect, Self::Next, Self::Retry];
}

impl Hold for EndDialogButtonBase {
    const MAX_HOLD_TIME: u32 = 400_000_000;

    fn hold_time(&self) -> u32 {
        self.hold_time
    }

    fn set_hold_time(&mut self, new_time: u32) {
        self.hold_time = new_time
    }
}

impl EndDialogButtonBase {
    pub const FADE_IN_TIME: f32 = 0.5;

    pub fn new(state: &LevelState, ty: EndDialogButtonType) -> Self {
        Self {
            start: state.finish_instant.unwrap_or_else(Instant::now),
            ty,
            hold_time: 0,
        }
    }

    pub const fn scale_multiplier(&self) -> f32 {
        let gradient = 1.0 - self.hold_time as f32 / Self::MAX_HOLD_TIME as f32;
        0.9 + 0.1 * (1.0 - gradient * gradient)
    }
}

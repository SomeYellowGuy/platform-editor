use std::time::Instant;

use crate::level::{StarCondition, state::LevelState};

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
}

#[derive(Debug, Clone)]
pub struct StarStatus {
    pub collected: bool,
    pub condition: StarCondition,
}

impl EndDialogBase {
    pub fn from_level_state(state: &LevelState) -> Self {
        Self {
            start: Instant::now(),
            star_statuses: state.star_statuses(),
        }
    }

    /// The lifetime of this dialog in seconds.
    pub fn lifetime(&self) -> f32 {
        self.start.elapsed().as_secs_f32()
    }
}

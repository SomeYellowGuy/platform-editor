use crate::level::state::LevelState;

/// The main part of a level, and the area of the actual level.
pub struct BoardBase {
    pub state: LevelState,
}

impl BoardBase {
    pub fn new() -> Self {
        Self {
            state: LevelState::new(),
        }
    }
}

impl Default for BoardBase {
    fn default() -> Self {
        Self::new()
    }
}

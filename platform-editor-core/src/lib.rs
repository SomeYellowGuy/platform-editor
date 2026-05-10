//! A part of *Platform Editor* that doesn't require SDL.

use std::time::Instant;

use crate::options::Options;

pub mod component;
pub mod options;
pub mod transition;

/// Stores "save data" in the game.
/// `E` is the extra, platform-specific data.
pub struct AppData<E: Default> {
    #[allow(unused)]
    pub options: Options,
    pub start: Instant,
    pub extra: E,
}

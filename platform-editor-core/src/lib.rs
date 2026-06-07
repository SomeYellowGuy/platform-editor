//! A part of *Platform Editor* that doesn't require SDL.

use std::time::Instant;

use crate::options::Options;

pub mod common_util;
pub mod component;
pub mod level;
pub mod options;
pub mod screen;
pub mod textures;

/// Stores "save data" in the game.
/// `E` is the extra, platform-specific data.
pub struct AppData<E: Default> {
    #[allow(unused)]
    pub options: Options,
    pub start: Instant,

    pub level_select_scroll: f32,
    pub level_select_scroll_velocity: f32,

    pub extra: E,
}

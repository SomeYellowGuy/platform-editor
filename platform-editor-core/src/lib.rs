//! A part of *Platform Editor* that doesn't require SDL.

use std::time::Instant;

use crate::{
    component::{Event, QueuedComponent},
    level::state::LevelState,
    options::Options,
    save::campaign::LevelSave,
};

pub mod audio;
pub mod common_util;
pub mod component;
pub mod level;
pub mod options;
pub mod save;
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

    pub previous_selected_item: Option<usize>,
    pub level_state: Option<LevelState>,

    pub extra: E,

    pub level: LevelSave,
}

#[derive(Debug, Clone)]
pub struct QueuedData<C> {
    /// Any events to handle after the current logical tick.
    pub events: Vec<Event>,

    /// Any components to add after the current logical tick.
    pub components: Vec<QueuedComponent<C>>,
}

impl<C> QueuedData<C> {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            components: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn add_component(&mut self, component: QueuedComponent<C>) {
        self.components.push(component);
    }

    pub fn extract(&mut self) -> (Vec<Event>, Vec<QueuedComponent<C>>) {
        (
            std::mem::take(&mut self.events),
            std::mem::take(&mut self.components),
        )
    }
}

impl<C> Default for QueuedData<C> {
    fn default() -> Self {
        Self::new()
    }
}

use std::default;

/// Options in the game.
#[derive(Debug)]
pub struct Options {}

/// A mode deciding the maximum FPS or VSync.
#[derive(Debug, Copy, Clone)]
pub enum PresentMode {
    /// No VSync, and the frame rate is capped to the provided number.
    Capped(u32),
    /// No VSync, and the frame rate is uncapped.
    Uncapped,
    /// VSync, so the frame rate is matched to that of the monitor.
    Vsync,
}

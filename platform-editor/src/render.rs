use std::time::Instant;

use sdl3::{Error, render::Canvas, video::Window};

use crate::{App, images::Images};

pub type DrawResult = Result<(), Error>;

/// Useful data for rendering.
pub struct RenderData<'window, 'i, 'c> {
    pub canvas: &'window mut Canvas<Window>,
    pub images: &'i mut Images<'c>,
    pub start: Instant,
}

impl<'window, 'i, 'c> RenderData<'window, 'i, 'c> {
    pub fn new(app: &'window mut App, images: &'i mut Images<'c>) -> Self {
        Self {
            canvas: &mut app.canvas,
            images,
            start: app.start,
        }
    }
}

/// A trait telling that something can be rendered.
pub trait Render {
    /// Renders this object.
    fn render(&self, data: RenderData) -> DrawResult;
}

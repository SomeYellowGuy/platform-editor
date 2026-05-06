use std::{collections::HashMap, time::Instant};

use sdl3::{
    Error,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{App, component::Component, images::Images};

pub type DrawResult = Result<(), Error>;

/// Useful data for rendering.
pub struct RenderData<'window, 'i, 'c> {
    pub canvas: &'window mut Canvas<Window>,
    pub images: &'i mut Images<'c>,
    pub start: Instant,
}

impl<'window, 'i, 'c> RenderData<'window, 'i, 'c> {
    /// Creates some new `RenderData` with the provided app and images.
    pub fn new(app: &'window mut App, images: &'i mut Images<'c>) -> Self {
        Self {
            canvas: &mut app.canvas,
            images,
            start: app.start,
        }
    }

    /// Returns the oscillation angle at the current instant with the provided multiplier.
    ///
    /// # Notes
    ///
    /// This is used to provide animations to the background and title, for example.
    pub fn oscillation_angle(&self, multiplier: f64) -> f64 {
        self.start.elapsed().as_nanos() as f64 / 1_000_000_000.0 * multiplier
    }
}

/// A trait telling that something can be rendered.
pub trait Render {
    /// Renders this object.
    fn render(&self, data: &mut RenderData) -> DrawResult;
}

/// The game's background, which is always drawn.
pub struct Background;

impl Render for Background {
    fn render(&self, mut data: &mut RenderData) -> DrawResult {
        data.images.strip.set_alpha_mod(170);
        let oscillation_angle = data.oscillation_angle(2.1);

        for i in 0..7 {
            Self::draw_strip(oscillation_angle, i, &mut data)?;
        }

        Ok(())
    }
}

impl Background {
    fn draw_strip(angle: f64, i: usize, data: &mut RenderData) -> DrawResult {
        const SIZE: u32 = 1060;
        const SPACING: i32 = 400;
        const ANGLE_SPEED: f64 = 100.0;

        let offset =
            (i as i32 - 3) * SPACING + ((angle * ANGLE_SPEED) as i32 % SPACING - SPACING / 2);
        let center = Point::new(
            crate::WIDTH as i32 / 2 + offset,
            crate::HEIGHT as i32 / 2 + 10,
        );

        data.canvas.copy_ex(
            &data.images.strip,
            None,
            Rect::from_center(center, SIZE, SIZE),
            6.0_f64 * angle.sin() - 3.0_f64,
            Some(center.into()),
            false,
            false,
        )?;

        Ok(())
    }
}

/// A map storing each component (via an ID) and giving each one a priority value to be rendered.
///
/// A higher priority means appearing later in the `iter` and `iter_mut` methods.
pub struct ComponentMap {
    components: HashMap<String, Component>,
    render_priorities: HashMap<String, i32>,
}

impl ComponentMap {
    /// Creates a new map.
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            render_priorities: HashMap::new(),
        }
    }

    /// Inserts a new [`Component`] in this map.
    pub fn insert(&mut self, id: &str, component: Component, priority: i32) {
        self.components.insert(id.to_string(), component);
        self.render_priorities.insert(id.to_string(), priority);
    }

    /// Removes a [`Component`], with the provided key, from this map and returns the `Component`
    /// if one could be removed.
    pub fn remove(&mut self, id: &str) -> Option<Component> {
        self.render_priorities.remove(id);
        self.components.remove(id)
    }

    /// Provides an [`Iterator`], starting from the lowest-prioritized `Component`, where
    /// each item is a reference to a `Component`.
    ///
    /// # Notes
    ///
    /// This is useful for rendering.
    pub fn ascending_iter(&self) -> impl Iterator<Item = (&String, &Component)> {
        let mut items: Vec<_> = self.components.iter().collect();
        items.sort_by_key(|(s, _)| *self.render_priorities.get(*s).unwrap());
        items.into_iter()
    }

    /// Provides an [`Iterator`], starting from the highest-prioritized `Component`, where
    /// each item is a mutable reference to a `Component`.
    ///
    /// # Notes
    ///
    /// This is useful for executing logic.
    pub fn descending_iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Component)> {
        let mut items: Vec<_> = self.components.iter_mut().collect();
        items.sort_by_key(|(s, _)| -*self.render_priorities.get(*s).unwrap());
        items.into_iter()
    }
}

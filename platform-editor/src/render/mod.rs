use std::time::Instant;

use sdl3::{
    Error,
    rect::{Point, Rect},
    render::Canvas,
    ttf::Font,
    video::Window,
};

use crate::{App, AppData, ExtractedData, images::Textures};

pub type DrawResult = Result<(), Error>;

/// Useful data for rendering.
pub struct RenderData<'window, 'i, 'c> {
    pub canvas: &'window mut Canvas<Window>,
    pub images: &'i mut Textures<'c>,
    pub font: &'static Font<'static>,
    pub start: Instant,
    /// The current transition's data, if any.
    pub transition_time: Option<u64>,
    pub extracted_data: &'i ExtractedData,
}

impl<'window, 'i, 'c> RenderData<'window, 'i, 'c> {
    /// Creates some new `RenderData` with the provided app and images.
    pub fn new(
        app: &'window mut App,
        data: &'window AppData,
        images: &'i mut Textures<'c>,
        font: &'static Font,
        transition_time: Option<u64>,
        extracted: &'i ExtractedData,
    ) -> Self {
        Self {
            canvas: &mut app.canvas,
            images,
            start: data.start,
            font,
            transition_time,
            extracted_data: extracted,
        }
    }

    /// Returns the oscillation angle at the current instant with the provided multiplier.
    ///
    /// # Notes
    ///
    /// This is used to provide animations to the background and title, for example.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn oscillation_angle(&self, multiplier: f64) -> f64 {
        self.start.elapsed().as_nanos() as f64 / 1_000_000_000.0 * multiplier
    }

    /// Returns a transition offset at the current instant with the provided multiplier.
    ///
    /// # Notes
    ///
    /// This is used to provide animations to the buttons and title when
    /// transitioning for example.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn transition_offset(&self, multiplier: f32) -> f32 {
        (self.transition_time.unwrap_or(0) / 1_000_000) as f32 / 40.0 * multiplier
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
    fn render(&self, data: &mut RenderData) -> DrawResult {
        data.images.strip.set_alpha_mod(60);
        let oscillation_angle = data.oscillation_angle(2.6);

        for i in 0..7 {
            Self::draw_strip(oscillation_angle, i, data)?;
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

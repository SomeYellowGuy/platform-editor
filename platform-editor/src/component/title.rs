use sdl3::rect::{Point, Rect};

use crate::render::{Render, RenderData};

/// Represents the animating title on the title screen.
pub struct TitleBase;

impl Render for TitleBase {
    fn render(&self, data: &mut RenderData) -> crate::render::DrawResult {
        let oscillation_angle = data.oscillation_angle(2.1);
        let sine = oscillation_angle.sin();

        let scale_multiplier = 1.05 - (0.15 * sine).abs();

        data.canvas.copy_ex(
            &data.images.title,
            None,
            Rect::from_center(
                Point::new(
                    crate::WIDTH as i32 / 2 + (60.0 * sine) as i32,
                    110 + (30.0 * sine).abs() as i32,
                ),
                (920.0 * scale_multiplier) as u32,
                (150.0 * scale_multiplier) as u32,
            ),
            6.0_f64 * sine,
            None,
            false,
            false,
        )?;

        // nothing
        Ok(())
    }
}

use platform_editor_core::component::level::end_dialog::EndDialogBase;
use sdl3::{
    pixels::Color,
    render::{BlendMode, FPoint},
};

use crate::{HEIGHT, WIDTH, logic::Logic, render::Render, util::frect_from_center};

impl Render for EndDialogBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let elapsed = self.start.elapsed().as_secs_f32();

        if elapsed < Self::DELAY {
            return Ok(());
        }

        data.canvas.set_blend_mode(BlendMode::Blend);

        let alpha = (elapsed - Self::DELAY).min(Self::FADE_IN_TIME) * (255.0 / Self::FADE_IN_TIME);
        data.textures.level.end_dialog.set_alpha_mod(alpha as u8);
        data.textures.level.end_dialog.set_color_mod(190, 190, 190);

        data.canvas
            .set_draw_color(Color::RGBA(0, 0, 0, (alpha * 0.5) as u8));
        data.canvas.fill_rect(None)?;

        data.canvas.copy_ex(
            &data.textures.level.end_dialog,
            None,
            frect_from_center(
                FPoint::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
                550.0,
                660.0,
            ),
            0.0,
            None,
            false,
            false,
        )?;

        Ok(())
    }
}

impl Logic for EndDialogBase {
    fn run_logic(&mut self, _data: &mut crate::logic::LogicData) {}
}

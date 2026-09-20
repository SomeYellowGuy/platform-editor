use platform_editor_core::component::level::end_dialog::EndDialogBase;
use sdl3::{
    pixels::Color,
    render::{BlendMode, FPoint, FRect},
};

use crate::{HEIGHT, WIDTH, logic::Logic, render::Render, textures::TextAlignment, util::FRectExt};

impl Render for EndDialogBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let elapsed = self.start.elapsed().as_secs_f32();

        if elapsed < Self::DELAY {
            return Ok(());
        }

        data.canvas.set_blend_mode(BlendMode::Blend);

        let alpha = (elapsed - Self::DELAY).min(Self::FADE_IN_TIME) * (255.0 / Self::FADE_IN_TIME);
        let alpha_mod = alpha as u8;
        data.textures.level.end_dialog.base.set_alpha_mod(alpha_mod);
        data.textures
            .level
            .end_dialog
            .base
            .set_color_mod(190, 190, 190);

        data.canvas
            .set_draw_color(Color::RGBA(0, 0, 0, (alpha * 0.5) as u8));
        data.canvas.fill_rect(None)?;

        pub const DIALOG_CENTER: FPoint = FPoint {
            x: WIDTH as f32 / 2.0,
            y: HEIGHT as f32 / 2.0,
        };

        // Draw the end dialog base (plate).
        data.canvas.copy_ex(
            &data.textures.level.end_dialog.base,
            None,
            FRect::from_center(DIALOG_CENTER, 550.0, 660.0),
            0.0,
            None,
            false,
            false,
        )?;

        pub const DIALOG_NICE_CENTER: FPoint = FPoint {
            x: DIALOG_CENTER.x,
            y: DIALOG_CENTER.y - 270.0,
        };
        pub const DIALOG_LEVEL_CENTER: FPoint = FPoint {
            x: DIALOG_CENTER.x,
            y: DIALOG_CENTER.y + 80.0,
        };

        // The NICE and level text.
        data.textures
            .level
            .end_dialog
            .nice_text
            .set_alpha_mod(alpha_mod);

        data.textures
            .level
            .end_dialog
            .nice_text
            .update(data.canvas, data.font, "NICE!")?;
        data.textures.level.end_dialog.nice_text.draw(
            data.canvas,
            TextAlignment::Center,
            DIALOG_NICE_CENTER,
            1.5,
        )?;

        data.textures
            .level
            .end_dialog
            .level_text
            .set_alpha_mod(alpha_mod);
        let tex = &mut data.textures.level.end_dialog.level_text;
        tex.update(
            data.canvas,
            data.font,
            format!("Level {}", data.extracted_data.playing_level + 1),
        )?;
        tex.draw(data.canvas, TextAlignment::Center, DIALOG_LEVEL_CENTER, 0.9)?;

        Ok(())
    }
}

impl Logic for EndDialogBase {
    fn run_logic(&mut self, _data: &mut crate::logic::LogicData) {}
}

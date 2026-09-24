use platform_editor_core::component::level_select::LevelSelectHeaderBase;
use sdl3::{
    pixels::Color,
    rect::Rect,
    render::{FPoint, FRect},
};

use crate::{HEIGHT, WIDTH, logic::Logic, render::Render, textures::TextAlignment};

pub const HEADER_HEIGHT: u32 = 90;

impl Render for LevelSelectHeaderBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let t = (data.transition_offset(1.2) - 10.0).max(0.0);
        let header_offset = ((t * t) / 1.4) as i32;

        data.canvas.set_blend_mode(sdl3::render::BlendMode::Blend);

        data.canvas.set_draw_color(Color::RGBA(10, 10, 10, 230));
        data.canvas
            .fill_rect(Rect::new(0, -header_offset, WIDTH, HEADER_HEIGHT))?;

        const LINE_THICKNESS: u32 = 8;
        data.canvas.set_draw_color(Color::RGBA(60, 60, 60, 150));
        data.canvas.fill_rect(Rect::new(
            0,
            HEADER_HEIGHT as i32 - LINE_THICKNESS as i32 / 2 - header_offset,
            WIDTH,
            LINE_THICKNESS,
        ))?;

        const TEXT_SCALE: f32 = 1.5;
        let texture = &mut data.textures.level_select.header_text;

        let text_y = HEADER_HEIGHT as f32 / 2.0 - 5.0 - header_offset as f32;

        // Draw the "Level Select" text.
        texture.update(data.canvas, data.font, "Level Select")?;
        texture.draw(
            data.canvas,
            TextAlignment::Left,
            FPoint::new(20.0, text_y),
            TEXT_SCALE,
        )?;

        const STAR_INFO_OFFSET: f32 = 270.0;

        // Draw the star icon.
        data.canvas.copy_ex(
            &data.textures.level_select.stars,
            FRect::new(40.0, 0.0, 40.0, 40.0),
            FRect::new(
                (WIDTH / 2) as f32 + STAR_INFO_OFFSET,
                7.0 - header_offset as f32,
                70.0,
                70.0,
            ),
            10.0,
            None,
            false,
            false,
        )?;

        // Draw the star count.
        data.textures
            .level_select
            .stars_text
            .update(data.canvas, data.font, "0/90")?;

        let star_count_pos = FPoint::new((WIDTH / 2) as f32 + STAR_INFO_OFFSET + 90.0, text_y);
        data.textures.level_select.stars_text.draw(
            data.canvas,
            TextAlignment::Left,
            star_count_pos,
            TEXT_SCALE,
        )?;

        data.canvas.set_draw_color(Color::RGBA(10, 10, 10, 130));
        data.canvas.fill_rect(Rect::new(
            0,
            (HEIGHT - HEADER_HEIGHT) as i32 + header_offset,
            WIDTH,
            HEADER_HEIGHT,
        ))?;

        data.canvas.set_draw_color(Color::RGBA(0, 0, 0, 240));
        data.canvas.fill_rect(Rect::new(
            0,
            (HEIGHT - HEADER_HEIGHT) as i32 - LINE_THICKNESS as i32 / 2 + header_offset,
            WIDTH,
            LINE_THICKNESS,
        ))?;

        Ok(())
    }
}

impl Logic for LevelSelectHeaderBase {}

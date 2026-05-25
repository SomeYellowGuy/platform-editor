use platform_editor_core::component::level_select::LevelSelectHeaderBase;
use sdl3::{pixels::Color, rect::Rect, render::FRect};

use crate::{HEIGHT, WIDTH, logic::Logic, render::Render};

impl Render for LevelSelectHeaderBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let t = (data.transition_offset(1.2) - 10.0).max(0.0);
        let header_offset = ((t * t) / 1.4) as i32;

        const HEADER_HEIGHT: u32 = 90;
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
        let texture = &data.textures.level_select.header_text;
        let height = texture.height() as f32 * TEXT_SCALE;

        // Draw the "Level Select" text.
        data.canvas.copy_ex(
            texture,
            None,
            FRect::new(
                20.0,
                (HEADER_HEIGHT as f32 - height) / 2.0 - 5.0 - header_offset as f32,
                texture.width() as f32 * TEXT_SCALE,
                height,
            ),
            0.0,
            None,
            false,
            false,
        )?;

        const STAR_INFO_OFFSET: f32 = 270.0;

        // Draw the star icon.
        data.canvas.copy_ex(
            &data.textures.level_select.stars,
            FRect::new(40.0, 0.0, 40.0, 40.0),
            FRect::new((WIDTH / 2) as f32 + STAR_INFO_OFFSET, 7.0 - header_offset as f32, 70.0, 70.0),
            10.0,
            None,
            false,
            false,
        )?;

        // Draw the star count.
        if let Ok(surface) = data.font.render("0/90").blended(Color::RGB(255, 255, 255))
            && let Ok(tex) = data
                .canvas
                .texture_creator()
                .create_texture_from_surface(&surface)
        {
            data.canvas.copy_ex(
                &tex,
                None,
                FRect::new(
                    (WIDTH / 2) as f32 + STAR_INFO_OFFSET + 90.0,
                    -7.0 - header_offset as f32,
                    tex.width() as f32 * TEXT_SCALE,
                    tex.height() as f32 * TEXT_SCALE,
                ),
                0.0,
                None,
                false,
                false,
            )?;
        }

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

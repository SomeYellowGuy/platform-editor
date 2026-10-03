use platform_editor_core::{
    common_util::Vec2f, component::level_select::LevelSelectHeaderBase,
    level::scratch::levels::LEVEL_COUNT,
};
use sdl3::{
    pixels::Color,
    rect::Rect,
    render::{FPoint, FRect},
};

use crate::{
    HEIGHT, WIDTH,
    assets::textures::TextAlignment,
    logic::Logic,
    render::Render,
    util::{DualImageDimensions, IntoFPoint},
};

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

        let star_dimensions = DualImageDimensions::new(&data.textures.level_select.stars);
        let star_src = star_dimensions.frect(true);

        // Draw the star icon.
        data.canvas.copy_ex(
            &data.textures.level_select.stars,
            star_src,
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

        let stars = data.extracted_data.level_save.stars_collected();

        let total_stars = (LEVEL_COUNT - 1) * 3;

        let mut star_count_pos = Vec2f::new((WIDTH / 2) as f32 + STAR_INFO_OFFSET + 90.0, text_y);

        // Draw the star count.
        let all_stars_collected = stars == total_stars as u32;
        let stars_text = &mut data.textures.level_select.stars_text;
        stars_text.update(data.canvas, data.font, stars.to_string())?;
        stars_text.set_color_mod(if all_stars_collected {
            Color::RGB(255, 255, 200)
        } else {
            Color::WHITE
        });
        stars_text.draw(
            data.canvas,
            TextAlignment::Left,
            star_count_pos.into_fpoint(),
            TEXT_SCALE,
        )?;
        if all_stars_collected {
            stars_text.draw(
                data.canvas,
                TextAlignment::Left,
                star_count_pos.add_x(-2.0).into_fpoint(),
                TEXT_SCALE,
            )?;
        }

        star_count_pos = star_count_pos.add_x(stars_text.width() * TEXT_SCALE);
        stars_text.update(data.canvas, data.font, format!("/{total_stars}"))?;
        stars_text.set_color_mod(Color::WHITE);
        stars_text.draw(
            data.canvas,
            TextAlignment::Left,
            star_count_pos.into_fpoint(),
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

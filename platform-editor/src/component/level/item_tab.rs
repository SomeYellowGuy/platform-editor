use platform_editor_core::component::level::item_tab::ItemTabBase;
use sdl3::{pixels::Color, rect::Rect, render::FPoint};

use crate::{WIDTH, logic::Logic, render::Render, textures::TextAlignment};

impl Render for ItemTabBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let t = data.transition_offset(1.8);
        let offset = t * t / 2.0;

        const LINE_THICKNESS: u32 = 8;
        const HEADER_HEIGHT: u32 = 120;

        data.canvas.set_draw_color(Color::RGBA(30, 30, 30, 30));
        data.canvas
            .fill_rect(Rect::new(0, -offset as i32, WIDTH, HEADER_HEIGHT))?;

        data.canvas.set_draw_color(Color::RGBA(10, 10, 10, 60));
        data.canvas.fill_rect(Rect::new(
            0,
            HEADER_HEIGHT as i32 - LINE_THICKNESS as i32 / 2 - offset as i32,
            WIDTH,
            LINE_THICKNESS,
        ))?;

        const TEXT_SCALE: f32 = 1.5;
        let tex = &mut data.textures.level.items_text;

        tex.update(data.canvas, data.font, "ITEMS")?;
        tex.draw(
            data.canvas,
            TextAlignment::Left,
            FPoint::new(20.0, HEADER_HEIGHT as f32 / 2.0 - 5.0 - offset),
            TEXT_SCALE,
        )?;

        Ok(())
    }
}

impl Logic for ItemTabBase {
    fn run_logic(&mut self, _data: &mut crate::logic::LogicData) {}
}

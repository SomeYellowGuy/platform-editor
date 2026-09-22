use platform_editor_core::{common_util::Vec2f, component::level::ItemTabBase, level::ItemStack};
use sdl3::{
    pixels::Color,
    render::{FPoint, FRect},
};

use crate::{
    WIDTH,
    logic::Logic,
    render::{DrawResult, Render, RenderData},
    textures::{DynamicText, TextAlignment},
    util::{FRectExt, IntoFPoint},
};

const HEADER_HEIGHT: f32 = 120.0;
const ITEM_BOX_SIZE: f32 = 90.0;
const ITEM_BOX_GAP: f32 = ITEM_BOX_SIZE + 20.0;

impl Render for ItemTabBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let Some(state) = data.extracted_data.level_state else {
            // There should be the level state.
            return Ok(());
        };

        let t = data.transition_offset(1.8);
        let offset = t * t / 2.0;

        const LINE_THICKNESS: f32 = 8.0;

        // Draw the tab itself (the plate).
        data.canvas.set_draw_color(Color::RGBA(30, 30, 30, 30));
        data.canvas
            .fill_rect(FRect::new(0.0, -offset, WIDTH as f32, HEADER_HEIGHT))?;

        data.canvas.set_draw_color(Color::RGBA(10, 10, 10, 60));
        data.canvas.fill_rect(FRect::new(
            0.0,
            HEADER_HEIGHT - LINE_THICKNESS / 2.0 - offset,
            WIDTH as f32,
            LINE_THICKNESS,
        ))?;

        const TEXT_SCALE: f32 = 1.5;
        let tex = &mut data.textures.level.items_text;

        tex.update(data.canvas, data.font, "ITEMS")?;
        tex.draw(
            data.canvas,
            TextAlignment::Left,
            FPoint::new(20.0, HEADER_HEIGHT / 2.0 - 5.0 - offset),
            TEXT_SCALE,
        )?;

        // Draw the items.
        render_items(data, offset, &state.items, state.selected_item)?;

        Ok(())
    }
}

fn render_items(
    data: &mut RenderData,
    offset: f32,
    items: &[ItemStack],
    selected: Option<usize>,
) -> DrawResult {
    let box_texture = &data.textures.level.item_box;
    let mut text = DynamicText::new(&data.canvas.texture_creator());

    for (i, stack) in items.iter().rev().enumerate() {
        let hitbox = item_hitbox(i, offset, selected.is_some_and(|s| s == i));
        // Draw the box itself.
        data.canvas.copy(box_texture, None, hitbox)?;
        // Draw the icon.
        const ITEM_BOX_ICON_SIZE: f32 = 55.0;
        const ITEM_BOX_ICON_OFFSET: f32 = 2.0;
        const ITEM_BOX_COUNT_OFFSET: Vec2f = Vec2f::new(30.0, 13.0);
        let center = item_box_center(offset);
        if let Some(texture) = stack.item.icon_texture(&data.textures.level.tiles) {
            let icon_center = center - Vec2f::new(ITEM_BOX_ICON_OFFSET, ITEM_BOX_ICON_OFFSET);
            data.canvas.copy(
                texture,
                None,
                FRect::from_center(
                    icon_center.into_fpoint(),
                    ITEM_BOX_ICON_SIZE,
                    ITEM_BOX_ICON_SIZE,
                ),
            )?;
        }
        // Draw the stack count.
        text.update(data.canvas, data.font, stack.count.to_string())?;
        let text_center = center + ITEM_BOX_COUNT_OFFSET;
        text.draw(
            data.canvas,
            TextAlignment::Right,
            text_center.into_fpoint(),
            0.92,
        )?;
    }

    Ok(())
}

fn item_box_center(offset: f32) -> Vec2f {
    const CORNER_DISTANCE: f32 = HEADER_HEIGHT / 2.0;
    Vec2f::new(WIDTH as f32 - CORNER_DISTANCE, CORNER_DISTANCE - offset)
}

fn item_hitbox(i: usize, offset: f32, selected: bool) -> FRect {
    let size = if selected {
        ITEM_BOX_SIZE + 10.0
    } else {
        ITEM_BOX_SIZE
    };
    FRect::from_center(
        (item_box_center(offset) + Vec2f::new(i as f32 * ITEM_BOX_GAP, 0.0)).into_fpoint(),
        size,
        size,
    )
}

impl Logic for ItemTabBase {
    fn run_logic(&mut self, _data: &mut crate::logic::LogicData) {}
}

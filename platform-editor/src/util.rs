use platform_editor_core::common_util::Vec2f;
use sdl3::{
    pixels::Color,
    rect::Rect,
    render::{FPoint, FRect, Texture},
};

use crate::render::{DrawResult, RenderData};

/// Creates an [`FRect`] from a center position and a width and height.
pub fn frect_from_center(center: FPoint, width: f32, height: f32) -> FRect {
    FRect::new(
        center.x - width / 2.0,
        center.y - height / 2.0,
        width,
        height,
    )
}

/// Casts an [`FRect`] to a [`Rect`].
pub fn frect_to_rect(rect: FRect) -> Rect {
    Rect::new(rect.x as i32, rect.y as i32, rect.w as u32, rect.h as u32)
}

pub fn fpos_to_fpoint(pos: Vec2f) -> FPoint {
    FPoint::new(pos.x, pos.y)
}

pub fn create_font_texture_and(
    data: &mut RenderData,
    text: &str,
    color: impl Into<Color>,
    f: impl FnOnce(&mut RenderData, Texture) -> DrawResult,
) -> DrawResult {
    if let Ok(surface) = data.font.render(text).blended(color)
        && let Ok(tex) = data
            .canvas
            .texture_creator()
            .create_texture_from_surface(&surface)
    {
        f(data, tex)
    } else {
        Ok(())
    }
}

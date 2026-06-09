use platform_editor_core::common_util::Vec2f;
use sdl3::{
    rect::Rect,
    render::{FPoint, FRect},
};

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

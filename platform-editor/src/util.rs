use platform_editor_core::common_util::Vec2;
use sdl3::{
    rect::Rect,
    render::{FPoint, FRect},
};

/// An external trait that provides extraneous [`FRect`] methods.
pub trait FRectExt {
    /// Creates this rectangle from a center position and a width and height.
    fn from_center(center: FPoint, width: f32, height: f32) -> FRect;

    /// Returns whether this rectangle contains the given point.
    fn contains_point(self, point: FPoint) -> bool;

    /// Converts this rectangle into a [`Rect`].
    fn into_rect(self) -> Rect;
}

impl FRectExt for FRect {
    fn from_center(center: FPoint, width: f32, height: f32) -> FRect {
        FRect::new(
            center.x - width / 2.0,
            center.y - height / 2.0,
            width,
            height,
        )
    }

    fn contains_point(self, point: FPoint) -> bool {
        self.x <= point.x
            && self.x + self.w >= point.x
            && self.y <= point.y
            && self.y + self.h >= point.y
    }

    fn into_rect(self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }
}

/// An external trait to convert a struct into an [`FPoint`].
pub trait IntoFPoint {
    /// Converts this point into an [`FPoint`].
    fn into_fpoint(self) -> FPoint;
}

impl IntoFPoint for FPoint {
    fn into_fpoint(self) -> FPoint {
        FPoint::new(self.x, self.y)
    }
}

impl IntoFPoint for Vec2<f32> {
    fn into_fpoint(self) -> FPoint {
        FPoint::new(self.x, self.y)
    }
}
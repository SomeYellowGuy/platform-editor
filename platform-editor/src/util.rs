use platform_editor_core::common_util::{Rectf, Vec2, Vec2f};
use sdl3::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, FPoint, FRect, RenderTarget, Vertex, VertexIndices},
};

use crate::render::DrawResult;

/// An external trait that provides extraneous [`FRect`] methods.
pub trait FRectExt {
    /// Creates this rectangle from a center position and a width and height.
    fn from_center(center: FPoint, width: f32, height: f32) -> FRect;

    /// Returns whether this rectangle contains the given point.
    fn contains_point(self, point: impl IntoFPoint) -> bool;

    /// Converts this rectangle into a [`Rect`].
    fn into_rect(self) -> Rect;

    /// Converts this rectangle into a [`Rectf`].
    fn into_rectf(self) -> Rectf;
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

    fn contains_point(self, point: impl IntoFPoint) -> bool {
        let point = point.into_fpoint();
        (self.x..=(self.x + self.w)).contains(&point.x)
            && (self.y..=(self.y + self.h)).contains(&point.y)
    }

    fn into_rect(self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }

    fn into_rectf(self) -> Rectf {
        Rectf::from_xy_and_dimensions(self.x, self.y, self.w, self.h)
    }
}

/// An external trait to convert a struct into an [`FPoint`].
pub trait IntoFPoint {
    /// Converts this point into an [`FPoint`].
    fn into_fpoint(self) -> FPoint;
}

impl IntoFPoint for FPoint {
    fn into_fpoint(self) -> FPoint {
        self
    }
}

impl IntoFPoint for Vec2<f32> {
    fn into_fpoint(self) -> FPoint {
        FPoint::new(self.x, self.y)
    }
}

/// An external trait to convert a struct into an [`FRect`].
pub trait IntoFRect {
    /// Converts this point into an [`FRect`].
    fn into_frect(self) -> FRect;
}

impl IntoFRect for Rectf {
    fn into_frect(self) -> FRect {
        FRect::new(self.pos.x, self.pos.y, self.dimensions.x, self.dimensions.y)
    }
}

pub struct Corners<T> {
    pub top_left: T,
    pub top_right: T,
    pub bottom_left: T,
    pub bottom_right: T,
}

/// An external trait that provides extraneous [`Canvas`] methods.
pub trait CanvasExt {
    fn fill_gradient_rect(&mut self, corners: Corners<Color>, rect: FRect) -> DrawResult;
}

fn vertex(color: Color, position: Vec2f) -> Vertex {
    Vertex {
        position: position.into_fpoint(),
        color: color.into(),
        tex_coord: FPoint::new(0.0, 0.0),
    }
}

impl<T: RenderTarget> CanvasExt for Canvas<T> {
    fn fill_gradient_rect(&mut self, corners: Corners<Color>, rect: FRect) -> DrawResult {
        const INDICES: [u8; 6] = [0, 1, 2, 2, 3, 0];

        let rect = rect.into_rectf();
        let vertices = &[
            vertex(corners.top_left, rect.pos),
            vertex(corners.top_right, rect.pos.add_x(rect.dimensions.x)),
            vertex(corners.bottom_right, rect.pos + rect.dimensions),
            vertex(corners.bottom_left, rect.pos.add_y(rect.dimensions.y)),
        ];
        self.render_geometry(vertices, None, VertexIndices::U8(&INDICES))
    }
}

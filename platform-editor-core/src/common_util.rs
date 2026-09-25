//! Common utility methods for Platform Editor implementations.

use std::{
    f32::consts::PI,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// Returns the number of digits of a `usize`.
pub fn digit_count(int: usize) -> usize {
    if int == 0 {
        return 1;
    }
    let mut number = int;
    let mut digits = 0;
    while number > 0 {
        number /= 10;
        digits += 1;
    }
    digits
}

pub struct ScrollInfo<'a> {
    pub held: bool,
    pub last_pos: &'a mut Option<f32>,
    pub pos: f32,
    pub velocity: &'a mut f32,
    pub scroll: &'a mut f32,
    pub delta_seconds: f32,

    pub starting_level_select_scroll: f32,
    pub spacing: f32,
    pub levels: usize,
    pub levels_per_row: usize,
    pub extra_end_scroll: f32,
}

pub fn scroll(info: ScrollInfo<'_>) {
    if info.held {
        if let Some(prev_pos) = info.last_pos {
            let delta = info.pos - *prev_pos;
            *info.velocity = delta;
        }
        *info.last_pos = Some(info.pos);
    } else {
        let max_scroll = info.starting_level_select_scroll
            - info.spacing * (info.levels.div_ceil(info.levels_per_row).saturating_sub(2)) as f32
            - info.extra_end_scroll;
        // Push the scroll towards the level buttons if it is dragged out of bounds.
        let (drag_value, out_by): (f32, f32) = if *info.scroll > info.starting_level_select_scroll {
            *info.velocity -= info.delta_seconds * 100.0;
            (0.01, info.starting_level_select_scroll - *info.scroll)
        } else if *info.scroll < max_scroll {
            *info.velocity += info.delta_seconds * 100.0;
            (0.01, max_scroll - *info.scroll)
        } else {
            (0.01, 0.0)
        };
        *info.velocity *= drag_value.powf(info.delta_seconds);
        // Max out the velocity if needed.
        *info.velocity = info
            .velocity
            .clamp(-10.0 + out_by / 10.0, 10.0 + out_by / 10.0);
        *info.last_pos = None;
    }
    *info.scroll += *info.velocity * info.delta_seconds * 40.0;
}

/// A two-dimensional vector of some type `T`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    /// Creates a [`Pos`] with two components.
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn get(self, bidirection: Bidirection) -> T {
        match bidirection {
            Bidirection::Horizontal => self.x,
            Bidirection::Vertical => self.y,
        }
    }

    pub fn get_mut(&mut self, bidirection: Bidirection) -> &mut T {
        match bidirection {
            Bidirection::Horizontal => &mut self.x,
            Bidirection::Vertical => &mut self.y,
        }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Vec2<U> {
        Vec2::new(f(self.x), f(self.y))
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> From<Vec2<T>> for (T, T) {
    fn from(value: Vec2<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Mul<U, Output = T>, U: Clone> Mul<U> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: U) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs,
        }
    }
}

impl<T: Div<U, Output = T>, U: Clone> Div<U> for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: U) -> Self::Output {
        Self {
            x: self.x / rhs.clone(),
            y: self.y / rhs,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Add<Output = T>> Vec2<T> {
    /// Adds `dx` to the x-component of this vector, returning the new vector.
    pub fn add_x(self, dx: T) -> Self {
        Vec2::new(self.x + dx, self.y)
    }

    /// Adds `dy` to the y-component of this vector, returning the new vector.
    pub fn add_y(self, dy: T) -> Self {
        Vec2::new(self.x, self.y + dy)
    }
}

impl Vec2<usize> {
    /// Converts this vector to a `Vec2f`.
    pub fn to_vec2f(self) -> Vec2f {
        Vec2f::new(self.x as f32, self.y as f32)
    }

    /// Converts this vector to a centered `Vec2f` (the point is shifted by `(0.5, 0.5)` up and right).
    pub fn to_center_vec2f(self) -> Vec2f {
        Vec2f::new(self.x as f32 + 0.5, self.y as f32 + 0.5)
    }
}

macro_rules! assign {
    ($tr:ty, $func:ident, $op_tr:ident, $op:tt) => {
        impl<T: Clone + $op_tr<Output = T>> $tr for Vec2<T> {
            fn $func(&mut self, other: Self) {
                *self = Self {
                    x: self.x.clone() $op other.x,
                    y: self.y.clone() $op other.y,
                };
            }
        }
    };
}

assign!(std::ops::AddAssign, add_assign, Add, +);
assign!(std::ops::SubAssign, sub_assign, Sub, -);
assign!(std::ops::MulAssign, mul_assign, Mul, *);
assign!(std::ops::DivAssign, div_assign, Div, /);

macro_rules! impl_distance_methods {
    ($( $(| $sqrt:ident |)? $ty:ty),+) => {
        $(
            impl Vec2<$ty> {
                $(
                    /// Returns the distance between the point represented by this
                    /// vector and that represented by `other`.
                    pub fn distance(self, other: Self) -> $ty {
                        self.distance_sqr(other).$sqrt()
                    }
                )?

                /// Returns the square of the distance between the point
                /// represented by this vector and that represented by `other`.
                pub fn distance_sqr(self, other: Self) -> $ty {
                    let dx = self.x - other.x;
                    let dy = self.y - other.y;
                    dx * dx + dy * dy
                }
            }
        )+
    };
}

impl_distance_methods!(|sqrt| f32, |sqrt| f64, i8, i16, i32, i64);

pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;

/// A structure representing an axis-aligned rectangle in space.
///
/// `pos` is the rectangle's top left vertex,
/// while `dimensions` represents the rectangle's dimensions.
#[derive(Debug, Copy, Clone)]
pub struct Rectf {
    pub pos: Vec2f,
    pub dimensions: Vec2f,
}

impl Rectf {
    /// Creates a [`Rectf`] with the provided top-left position vector and dimensions.
    #[must_use]
    pub const fn new(pos: Vec2f, dimensions: Vec2f) -> Self {
        Self { pos, dimensions }
    }

    /// Creates a [`Rectf`] with the provided center position vector and dimensions.
    #[must_use]
    pub fn from_center(center: Vec2f, dimensions: Vec2f) -> Self {
        Self::new(center - dimensions / 2.0, dimensions)
    }

    /// Creates a [`Rectf`] with the provided top-left position vector, width and height.
    #[must_use]
    pub const fn from_dimensions(pos: Vec2f, width: f32, height: f32) -> Self {
        Self::new(pos, Vec2f::new(width, height))
    }

    /// Creates a [`Rectf`] with the provided top-left coordinates, width and height.
    #[must_use]
    pub const fn from_xy_and_dimensions(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self::from_dimensions(Vec2f::new(x, y), width, height)
    }

    /// Returns whether this `Rectf` contains the given point.
    #[must_use]
    pub fn contains(self, point: Vec2f) -> bool {
        point.x > self.pos.x
            && point.x < self.pos.x + self.dimensions.x
            && point.y > self.pos.y
            && point.y < self.pos.y + self.dimensions.y
    }

    /// Returns whether this `Rectf` intersects with the other provided `Rectf`.
    #[must_use]
    pub fn intersects(self, other: Self) -> bool {
        !(self.pos.x > other.pos.x + other.dimensions.x
            || self.pos.x + self.dimensions.x < other.pos.x
            || self.pos.y > other.pos.y + other.dimensions.y
            || self.pos.y + self.dimensions.y < other.pos.y)
    }

    /// Returns whether this `Rectf` completely contains the given `Rectf`.
    pub fn contains_rect(self, other: Self) -> bool {
        self.pos.x <= other.pos.x
            && self.pos.y <= other.pos.y
            && self.pos.x + self.dimensions.x >= other.pos.x + other.dimensions.x
            && self.pos.y + self.dimensions.y >= other.pos.y + other.dimensions.y
    }

    /// Returns whether this `Rectf` touches the line `x = <x>`.
    pub fn touches_x_line(self, x: f32) -> bool {
        (self.pos.x..=(self.pos.x + self.dimensions.x)).contains(&x)
    }

    /// Returns whether this `Rectf` touches the line `y = <y>`.
    pub fn touches_y_line(self, y: f32) -> bool {
        (self.pos.y..=(self.pos.y + self.dimensions.y)).contains(&y)
    }

    pub fn inflate(self, by: f32) -> Self {
        Rectf::from_dimensions(self.pos, self.dimensions.x + by, self.dimensions.y + by)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const fn unit_vec2f(self) -> Vec2f {
        match self {
            Self::Up => Vec2f::new(0.0, -1.0),
            Self::Down => Vec2f::new(0.0, 1.0),
            Self::Left => Vec2f::new(-1.0, 0.0),
            Self::Right => Vec2f::new(1.0, 0.0),
        }
    }

    pub const fn bidirection(self) -> Bidirection {
        match self {
            Self::Up | Self::Down => Bidirection::Vertical,
            Self::Left | Self::Right => Bidirection::Horizontal,
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub const fn angle(self) -> f32 {
        match self {
            Direction::Up => -PI / 2.0,
            Direction::Down => PI / 2.0,
            Direction::Left => PI,
            Direction::Right => 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Bidirection {
    Horizontal,
    Vertical,
}

impl Bidirection {
    pub const fn other(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

/// Helper function to convert 3 RGB values to a single packed RGBA color.
pub const fn rgb(red: u8, green: u8, blue: u8) -> u32 {
    0xff00_0000 | (((red as u32) << 16) + ((green as u32) << 8) + blue as u32)
}

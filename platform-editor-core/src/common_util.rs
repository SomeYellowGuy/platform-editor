//! Common utility methods for Platform Editor implementations.

use std::ops::{Add, Div, Mul, Neg, Sub};

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
            - info.spacing * (info.levels.div_ceil(info.levels_per_row) - 2) as f32
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
#[derive(Debug, Clone, Copy)]
pub struct Pos<T> {
    pub x: T,
    pub y: T
}

impl<T> Pos<T> {
    /// Creates a [`Pos`] with two components.
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Pos<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> From<Pos<T>> for (T, T) {
    fn from(value: Pos<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T: Add<Output = T>> Add for Pos<T> {
    type Output = Pos<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Pos<T> {
    type Output = Pos<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Mul<U, Output = T>, U: Clone> Mul<U> for Pos<T> {
    type Output = Pos<T>;

    fn mul(self, rhs: U) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs,
        }
    }
}

impl<T: Div<U, Output = T>, U: Clone> Div<U> for Pos<T> {
    type Output = Pos<T>;

    fn div(self, rhs: U) -> Self::Output {
        Self {
            x: self.x / rhs.clone(),
            y: self.y / rhs,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Pos<T> {
    type Output = Pos<T>;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

macro_rules! assign {
    ($tr:ty, $func:ident, $op_tr:ident, $op:tt) => {
        impl<T: Clone + $op_tr<Output = T>> $tr for Pos<T> {
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

pub type FPos = Pos<f32>;
pub type IPos = Pos<i32>;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}
use std::f64::consts::PI;

use crate::{
    common_util::{Direction, Vec2f},
    level::LockColor,
};

/// Represents a type of collectible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectibleType {
    Star(usize),
    GravityOrb,
    Key(LockColor),
}

impl CollectibleType {
    pub fn collectible_rect_scale(self) -> Vec2f {
        match self {
            CollectibleType::Key(_) => Vec2f::new(0.8, 1.2),
            _ => Vec2f::new(1.0, 1.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Collectible {
    pub pos: Vec2f,
    pub ty: CollectibleType,
    pub track: Option<Track>,
}

impl Collectible {
    pub fn with_track_option(pos: Vec2f, ty: CollectibleType, track: Option<Track>) -> Self {
        Self { pos, ty, track }
    }
    pub fn new(pos: Vec2f, ty: CollectibleType) -> Self {
        Self {
            pos,
            ty,
            track: None,
        }
    }

    pub fn with_track(pos: Vec2f, ty: CollectibleType, track: Track) -> Self {
        Self {
            pos,
            ty,
            track: Some(track),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    pub ty: TrackType,
    pub time_period: f64,
}

impl Track {
    pub fn offset(&self, time: u128) -> Vec2f {
        let time =
            ((time % (self.time_period * 1_000_000_000.0) as u128) / 1_000) as f64 / 1_000_000.0;
        match self.ty {
            TrackType::Line {
                direction,
                amplitude,
            } => {
                let angle = time / self.time_period * 2.0 * PI;
                direction.unit_vec2f() * (amplitude * angle.sin() as f32)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TrackType {
    Line {
        direction: Direction,
        amplitude: f32,
    },
}

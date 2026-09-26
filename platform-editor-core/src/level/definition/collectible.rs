use crate::{common_util::Vec2f, level::LockColor};

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
}

impl Collectible {
    pub fn new(pos: Vec2f, ty: CollectibleType) -> Self {
        Self { pos, ty }
    }
}

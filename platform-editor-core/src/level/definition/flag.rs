use crate::common_util::{Direction, Rectf, Vec2, Vec2f};

/// The stat of a flag, the object to touch to finish a level.
///
/// It stores a position and a direction (the cardinal direction of its pole).
#[derive(Debug, Clone, Copy)]
pub struct FlagState {
    pub pos: Vec2f,
    pub direction: Direction,
}

impl FlagState {
    pub const HITBOX_SIZE: f32 = 0.8;

    pub const fn new(pos: Vec2f) -> Self {
        Self::with_direction(pos, Direction::Down)
    }

    pub const fn with_direction(pos: Vec2f, direction: Direction) -> Self {
        Self { pos, direction }
    }

    pub fn hitbox(self) -> Rectf {
        Rectf::from_center(self.pos, Vec2::new(Self::HITBOX_SIZE, Self::HITBOX_SIZE))
    }
}

impl Default for FlagState {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            direction: Direction::Down,
        }
    }
}

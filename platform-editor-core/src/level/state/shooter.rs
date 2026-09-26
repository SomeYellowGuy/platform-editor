use std::time::{Duration, Instant};

use crate::{
    common_util::{Bidirection, Direction, Rectf, Vec2f},
    level::{
        definition::Tile,
        state::{CollisionContext, entity::CollisionType},
    },
};

#[derive(Debug, Clone)]
pub struct ShooterState {
    pub pos: Vec2f,
    pub cooldown: f32,
    pub direction: Direction,
    pub speed_multiplier: f32,
    last_shot: Option<Instant>,
}

impl ShooterState {
    pub const COOLDOWN: f32 = 1.25;

    /// The minimum difference in the perpendicular component to a shooter's
    /// direction of the positions of the player and the shooter for the shooter
    /// to trigger.
    ///
    /// For example, for a shooter in the right direction, this would be the
    /// minimum difference in the y-component of the position vectors of both.
    pub const ACTIVATION_RANGE: f32 = 0.35;

    /// The minimum distance from the point `1.0` units from a shooter, along its
    /// direction of the player for the shooter to trigger.
    pub const ACTIVATION_RADIUS: f32 = 1.0;

    pub const GLOW_SIZE: f32 = 0.9;

    pub fn new(pos: Vec2f, direction: Direction, speed_multiplier: f32) -> Self {
        Self {
            pos,
            direction,
            speed_multiplier,
            cooldown: 0.0,
            last_shot: None,
        }
    }

    /// Ticks this shooter, returning the shot bullet if any.
    pub fn tick(&mut self, delta: f32, player_pos: Vec2f) -> Option<ShooterBullet> {
        const ACTIVATION_RADIUS_SQR: f32 =
            ShooterState::ACTIVATION_RADIUS * ShooterState::ACTIVATION_RADIUS;

        self.cooldown = (self.cooldown - delta).max(0.0);
        let should_shoot = {
            if self.cooldown < f32::EPSILON {
                // Check whether the player is in range.
                let radius_check_pos = self.pos + self.direction.unit_vec2f();
                let is_in_range = match self.direction.bidirection() {
                    Bidirection::Horizontal => {
                        (self.pos.y - player_pos.y).abs() <= Self::ACTIVATION_RANGE
                    }
                    Bidirection::Vertical => {
                        (self.pos.x - player_pos.x).abs() <= Self::ACTIVATION_RANGE
                    }
                };
                is_in_range || radius_check_pos.distance_sqr(player_pos) < ACTIVATION_RADIUS_SQR
            } else {
                false
            }
        };

        if should_shoot {
            self.cooldown = Self::COOLDOWN;
            self.last_shot = Some(Instant::now())
        }

        should_shoot.then(|| ShooterBullet::new(self))
    }

    pub fn glow(&self) -> Option<f32> {
        self.last_shot.and_then(|i| {
            let elapsed = i.elapsed().as_secs_f32();
            if elapsed > ShooterBullet::IMMUNE_TIME {
                None
            } else {
                Some(1.0 - elapsed / ShooterBullet::IMMUNE_TIME)
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct ShooterBullet {
    pub pos: Vec2f,
    pub direction: Direction,
    pub speed_multiplier: f32,
    instant: Instant,
}

impl ShooterBullet {
    pub const SPEED: f32 = 4.5;

    pub const HIT_RADIUS: f32 = 0.2;
    pub const SIZE: f32 = 2.0 * Self::HIT_RADIUS;

    pub const IMMUNE_TIME: f32 = (0.6 / Self::SPEED) + 0.04;

    pub fn new(shooter: &ShooterState) -> Self {
        Self {
            pos: shooter.pos,
            direction: shooter.direction,
            instant: Instant::now(),
            speed_multiplier: shooter.speed_multiplier,
        }
    }

    pub fn hitbox(&self) -> Rectf {
        Rectf::from_center(self.pos, Vec2f::new(Self::SIZE, Self::SIZE))
    }

    pub fn is_immune_to_tiles(&self) -> bool {
        self.instant.elapsed() < Duration::from_secs_f32(Self::IMMUNE_TIME)
    }

    /// Ticks this bullet, returning whether it should survive.
    pub fn tick(&mut self, delta: f32, context: CollisionContext) -> bool {
        let ty = context.is_colliding(self.hitbox());
        let should_survive = match ty {
            CollisionType::None => true,
            CollisionType::Tile(t) => {
                self.is_immune_to_tiles() && matches!(t, Tile::Shooter { .. })
            }
            CollisionType::Moving(_) | CollisionType::Border | CollisionType::Lock => false,
        };
        if should_survive {
            self.pos += self.direction.unit_vec2f() * Self::SPEED * delta * self.speed_multiplier
        }
        should_survive
    }

    pub fn is_touching(&self, pos: Vec2f, entity_radius: f32) -> bool {
        let max_distance = entity_radius + Self::HIT_RADIUS;
        self.pos.distance_sqr(pos) < (max_distance * max_distance)
    }
}

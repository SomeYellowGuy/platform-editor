use std::f32::consts::PI;

use crate::{
    common_util::{Direction, Rectf, Vec2, Vec2f},
    level::{
        definition::Tile,
        state::{
            CollectibleState, LockState, Moving, TileCollision, TileState,
            moving::MovingHitboxSnapshot,
        },
    },
};

#[derive(Debug, Clone, Copy)]
pub struct CollisionContext<'a> {
    pub tiles: &'a TileState,
    pub locks: &'a LockState,
    pub moving: &'a [Moving],
    pub moving_snapshot: &'a MovingHitboxSnapshot,

    pub delta: f32,
}

impl<'a> CollisionContext<'a> {
    pub fn new(
        tiles: &'a TileState,
        locks: &'a LockState,
        moving: &'a [Moving],
        moving_snapshot: &'a MovingHitboxSnapshot,
        delta: f32,
    ) -> Self {
        Self {
            tiles,
            locks,
            moving,
            moving_snapshot,
            delta,
        }
    }

    pub fn is_colliding(&self, hitbox: Rectf) -> CollisionType {
        if let Some(v) = self.colliding_with_moving(hitbox) {
            CollisionType::Moving(v)
        } else if self.locks.is_colliding(hitbox) {
            CollisionType::Lock
        } else {
            match self.tiles.tile_collision_with_hitbox(hitbox) {
                TileCollision::Tile(t) => CollisionType::Tile(t.clone()),
                TileCollision::Border => CollisionType::Border,
                _ => CollisionType::None,
            }
        }
    }

    /// Returns the velocity of the first moving platform the provided hitbox touches.
    pub fn colliding_with_moving(&self, hitbox: Rectf) -> Option<Vec2f> {
        self.moving.iter().enumerate().find_map(|(i, m)| {
            m.hitbox()
                .intersects(hitbox)
                .then(|| m.velocity(i, self.tiles, self.moving_snapshot, self.delta))
        })
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub pos: Vec2f,
    pub velocity: Vec2f,
    pub gravity_direction: Direction,

    is_falling: bool,
    platform_move_vector: Option<Vec2f>,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            velocity: Default::default(),
            gravity_direction: Direction::Down,
            is_falling: Default::default(),
            platform_move_vector: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CollisionType {
    None,
    Border,
    Tile(Tile),
    Lock,
    Moving(Vec2f),
}

impl CollisionType {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Controls(u8);

impl Controls {
    const JUMP_BIT: u8 = 4;
    const RIGHT_BIT: u8 = 2;
    const LEFT_BIT: u8 = 1;

    pub fn new(left: bool, right: bool, jump: bool) -> Self {
        let mut b = 0;
        if left {
            b |= Self::LEFT_BIT;
        }
        if right {
            b |= Self::RIGHT_BIT;
        }
        if jump {
            b |= Self::JUMP_BIT;
        }
        Controls(b)
    }

    pub fn left(self) -> bool {
        (self.0 & Self::LEFT_BIT) != 0
    }

    pub fn right(self) -> bool {
        (self.0 & Self::RIGHT_BIT) != 0
    }

    pub fn jump(self) -> bool {
        (self.0 & Self::JUMP_BIT) != 0
    }
}

impl Entity {
    /// The size of an entity relative to the measurement of a tile (whose size is considered to be `1.0`).
    pub const SIZE: f32 = Self::RADIUS * 2.0;

    /// The radius of an entity relative to measurement of a tile (whose size is considered to be `1.0`).
    pub const RADIUS: f32 = 0.375;

    /// The acceleration due to gravity.
    pub const GRAVITY: f32 = 0.47;

    /// The velocity of an entity if they jump.
    pub const JUMP_VELOCITY: f32 = 0.16;

    /// The lateral velocity of an entity if they move.
    pub const MOVE_VELOCITY: f32 = 0.6;

    pub fn hitbox(&self) -> Rectf {
        Self::hitbox_from_pos(self.pos)
    }

    pub fn hitbox_from_pos(pos: Vec2f) -> Rectf {
        Rectf::from_center(pos, Vec2::new(Self::SIZE, Self::SIZE))
    }

    pub fn gravity(&self) -> Vec2f {
        self.gravity_direction.unit_vec2f() * Self::GRAVITY
    }

    /// Updates this entity to take the velocity of the first moving platform it is
    /// found to touch (or almost touch).
    ///
    /// This velocity, if any, will be reflected in [`Entity::tick`].
    pub fn update_platform_move_vector(&mut self, context: CollisionContext) {
        // Check for moving platform collisions.
        self.platform_move_vector = context.colliding_with_moving(self.hitbox().inflate(0.01));
    }

    /// Ticks this entity.
    ///
    /// Returns whether this entity is still alive.
    pub fn tick(&mut self, controls: Controls, context: CollisionContext) -> bool {
        // If this entity is touching a moving platform, make it move
        // along its instantaneous velocity.
        if let Some(v) = self.platform_move_vector {
            self.pos += v;
            self.platform_move_vector = None;
        }

        // Move the entity.
        self.move_in_steps(context);

        // Apply gravity.
        self.velocity += self.gravity() * context.delta;

        // Apply controls.
        self.apply_controls(controls, context.delta);

        // Apply friction.
        let non_gravity_component = self
            .velocity
            .get_mut(self.gravity_direction.bidirection().other());
        *non_gravity_component *= 0.85_f32.powf(context.delta * 30.0);

        // Check for any compression that might defeat this entity.
        if self.is_colliding(context).is_some() {
            if let Some(pos) = Self::find_closest_space(self.pos, context, 20) {
                // A suitable position has been found.
                self.pos = pos;
            } else {
                // This entity is defeated.
                return false;
            }
        }

        // Check if the entity is too close to the void, or is touching something deadly.
        !self.is_touching_void(context.tiles) && !self.is_touching_deadly_area(context.tiles)
    }

    fn apply_controls(&mut self, controls: Controls, delta: f32) {
        let horizontal = -(controls.left() as i8) + (controls.right() as i8);
        self.velocity.x += horizontal as f32 * delta * Self::MOVE_VELOCITY;

        if !self.is_falling && controls.jump() {
            let new_velocity = -Self::JUMP_VELOCITY * self.gravity_multiplier();
            let gravity_component = self.velocity.get_mut(self.gravity_direction.bidirection());
            *gravity_component = new_velocity;
        }
    }

    pub fn move_in_steps(&mut self, context: CollisionContext) {
        let quality = ((self.velocity.x.abs() + self.velocity.y.abs()).ceil() * 5.0) as usize;
        self.is_falling = true;

        for _ in 0..quality {
            let x = self.pos.x;
            self.pos.x += self.velocity.x / quality as f32;
            if self.is_colliding(context).is_some() {
                self.pos.x = x;
                self.velocity.x = 0.0;
                break;
            }
        }
        for _ in 0..quality {
            let y = self.pos.y;
            self.pos.y += self.velocity.y / quality as f32;
            if self.is_colliding(context).is_some() {
                if self.velocity.y * self.gravity_multiplier() > 0.0 {
                    self.is_falling = false;
                }
                self.pos.y = y;
                self.velocity.y = 0.0;
                break;
            }
        }
    }

    pub fn is_touching_void(&self, tiles: &TileState) -> bool {
        self.hitbox().touches_y_line(tiles.size.y as f32 - 0.1)
    }

    fn gravity_multiplier(&self) -> f32 {
        match self.gravity_direction {
            Direction::Up | Direction::Left => -1.0,
            Direction::Down | Direction::Right => 1.0,
        }
    }

    pub fn is_colliding(&self, context: CollisionContext) -> CollisionType {
        context.is_colliding(self.hitbox())
    }

    pub fn is_colliding_with_tiles(&self, tiles: &TileState) -> bool {
        tiles.is_colliding_with_hitbox(self.hitbox())
    }

    /// Checks for collisions with moving platforms, returning the velocity
    /// of the first moving platform to be found touching this entity.
    pub fn colliding_with_moving(&self, context: CollisionContext) -> Option<Vec2f> {
        context.colliding_with_moving(self.hitbox())
    }

    pub fn is_touching_deadly_area(&self, tiles: &TileState) -> bool {
        let hitbox = self.hitbox();

        for y in 0..tiles.size.y {
            for x in 0..tiles.size.x {
                let tile = tiles.tile(x, y);
                if tile.is_deadly_for_hitbox(Vec2::new(x, y), hitbox) {
                    return true;
                }
            }
        }
        false
    }

    /// Tries to move the given position around to find a position where the hitbox
    /// at it does not collide with the provided [`CollisionContext`].
    ///
    /// If such a position can be found, this returns a `Some`. Otherwise, this returns
    /// a `None`.
    pub fn find_closest_space(
        mut pos: Vec2f,
        context: CollisionContext,
        max_loops: usize,
    ) -> Option<Vec2f> {
        const INITIAL_DELTA: f32 = 0.1;
        const DIRECTIONS_PER_LOOP: usize = 32;

        pos.y -= INITIAL_DELTA;
        if context.is_colliding(Self::hitbox_from_pos(pos)).is_none() {
            return Some(pos);
        }
        pos.y += 2.0 * INITIAL_DELTA;
        if context.is_colliding(Self::hitbox_from_pos(pos)).is_none() {
            return Some(pos);
        }
        pos.y -= INITIAL_DELTA;
        let mut angle = 0.0f32;
        let mut distance = 0.04;

        for _ in 0..max_loops {
            for _ in 0..DIRECTIONS_PER_LOOP {
                let target = pos + Vec2f::new(angle.cos(), angle.sin()) * distance;
                if context
                    .is_colliding(Self::hitbox_from_pos(target))
                    .is_none()
                {
                    return Some(target);
                }
                angle += 2.0 * PI / DIRECTIONS_PER_LOOP as f32;
            }
            distance += 0.025;
            println!("{distance}")
        }

        None
    }

    /// Checks for any possible collectibles this entity is colliding with, returning
    /// the mutable references to the collided collectibles.
    pub fn check_collectibles<'a>(
        &self,
        collectibles: &'a mut [CollectibleState],
    ) -> Vec<&'a mut CollectibleState> {
        collectibles
            .iter_mut()
            .filter_map(|c| {
                (!c.is_collected() && c.is_touching(self.pos, Self::RADIUS)).then_some(c)
            })
            .collect()
    }
}

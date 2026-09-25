use std::f32::consts::PI;

use crate::{
    common_util::{Direction, Rectf, Vec2, Vec2f},
    level::{
        Tile,
        state::{CollectibleState, Moving, TileCollision, TileState},
    },
};

#[derive(Debug, Clone, Copy)]
pub struct CollisionContext<'a> {
    pub tiles: &'a TileState,
    pub moving: &'a [Moving],

    pub delta: f32,
}

impl<'a> CollisionContext<'a> {
    pub fn new(tiles: &'a TileState, moving: &'a [Moving], delta: f32) -> Self {
        Self {
            tiles,
            moving,
            delta,
        }
    }

    pub fn is_colliding(&self, hitbox: Rectf) -> CollisionType {
        if let Some(v) = self.colliding_with_moving(hitbox) {
            CollisionType::Moving(v)
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
        self.moving.iter().find_map(|m| {
            m.hitbox()
                .intersects(hitbox)
                .then(|| m.velocity(self.tiles, self.delta))
        })
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub pos: Vec2f,
    pub velocity: Vec2f,
    pub gravity_direction: Direction,

    is_falling: bool,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            velocity: Default::default(),
            gravity_direction: Direction::Down,
            is_falling: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CollisionType {
    None,
    Border,
    Tile(Tile),
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

    /// Ticks this entity.
    ///
    /// Returns whether this entity is still alive.
    pub fn tick(&mut self, context: CollisionContext) -> bool {
        // Apply gravity.
        self.velocity += self.gravity() * context.delta;

        // Apply friction.
        let non_gravity_component = self
            .velocity
            .get_mut(self.gravity_direction.bidirection().other());
        *non_gravity_component *= 0.85_f32.powf(context.delta * 30.0);

        // Check if the entity is too close to the void, or is touching something deadly.
        let alive =
            !self.is_touching_void(context.tiles) && !self.is_touching_deadly_area(context.tiles);

        if !alive {
            return false;
        }

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

        // Move the entity.
        self.move_in_steps(context);

        // Check for moving platform collisions.
        if let Some(v) = self.colliding_with_moving(context) {
            self.pos += v;
        }

        alive
    }

    pub fn apply_controls(&mut self, left: bool, right: bool, jump: bool, delta: f32) {
        let horizontal = -(left as i8) + (right as i8);
        self.velocity.x += horizontal as f32 * delta * Self::MOVE_VELOCITY;

        if !self.is_falling && jump {
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
        context.colliding_with_moving(self.hitbox().inflate(0.001))
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
        const DIRECTIONS_PER_LOOP: usize = 16;

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
                if context.is_colliding(Self::hitbox_from_pos(pos)).is_none() {
                    return Some(target);
                }
                angle += 2.0 * PI / DIRECTIONS_PER_LOOP as f32;
            }
            distance += 0.025;
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

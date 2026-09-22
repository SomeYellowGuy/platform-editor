use crate::{
    common_util::{Direction, Rectf, Vec2, Vec2f},
    level::{Tile, state::TileState},
};

/// The size of an entity relative to a tile (whose size is considered to be `1.0`).
pub const ENTITY_SIZE: f32 = 0.75;

/// The acceleration due to gravity.
pub const GRAVITY: f32 = 0.47;

/// The velocity of an entity if they jump.
pub const JUMP_VELOCITY: f32 = 0.16;

/// The lateral velocity of an entity if they move.
pub const MOVE_VELOCITY: f32 = 0.6;

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

impl Entity {
    pub fn hitbox(&self) -> Rectf {
        Rectf::from_center(self.pos, Vec2::new(ENTITY_SIZE, ENTITY_SIZE))
    }

    pub fn gravity(&self) -> Vec2f {
        self.gravity_direction.unit_vec2f() * GRAVITY
    }

    /// Ticks this entity.
    ///
    /// Returns whether this entity is still alive.
    pub fn tick(&mut self, tiles: &TileState, delta: f32) -> bool {
        // Apply gravity.
        self.velocity += self.gravity() * delta;

        // Apply friction.
        let non_gravity_component = self
            .velocity
            .get_mut(self.gravity_direction.bidirection().other());
        *non_gravity_component *= 0.85_f32.powf(delta * 30.0);

        // Move the entity.
        self.move_in_steps(tiles);

        // Check if the entity is too close to the void.
        !self.is_touching_void(tiles)
    }

    pub fn apply_controls(&mut self, left: bool, right: bool, jump: bool, delta: f32) {
        let horizontal = -(left as i8) + (right as i8);
        self.velocity.x += horizontal as f32 * delta * MOVE_VELOCITY;

        if !self.is_falling && jump {
            let new_velocity = -JUMP_VELOCITY * self.gravity_multiplier();
            let gravity_component = self.velocity.get_mut(self.gravity_direction.bidirection());
            *gravity_component = new_velocity;
        }
    }

    pub fn move_in_steps(&mut self, tiles: &TileState) {
        let quality = ((self.velocity.x.abs() + self.velocity.y.abs()).ceil() * 5.0) as usize;
        self.is_falling = true;

        for _ in 0..quality {
            let x = self.pos.x;
            self.pos.x += self.velocity.x / quality as f32;
            if self.is_colliding_with_tiles(tiles) {
                self.pos.x = x;
                self.velocity.x = 0.0;
                break;
            }
        }
        for _ in 0..quality {
            let y = self.pos.y;
            self.pos.y += self.velocity.y / quality as f32;
            if self.is_colliding_with_tiles(tiles) {
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
        self.hitbox()
            .touches_y_line(tiles.size.y as f32 - f32::EPSILON)
    }

    fn gravity_multiplier(&self) -> f32 {
        match self.gravity_direction {
            Direction::Up | Direction::Left => -1.0,
            Direction::Down | Direction::Right => 1.0,
        }
    }

    pub fn is_colliding_with_tiles(&self, tiles: &TileState) -> bool {
        let hitbox = self.hitbox();

        if !Rectf::new(
            Vec2f::new(0.0, 0.0),
            Vec2f::new(tiles.size.x as f32, tiles.size.y as f32),
        )
        .contains_rect(hitbox)
        {
            return true;
        }

        for y in 0..tiles.size.y {
            for x in 0..tiles.size.x {
                if *tiles.tile(x, y) == Tile::Empty {
                    continue;
                }
                let tile_hitbox = Rectf::from_xy_and_dimensions(x as f32, y as f32, 1.0, 1.0);
                if tile_hitbox.intersects(hitbox) {
                    return true;
                }
            }
        }
        false
    }
}

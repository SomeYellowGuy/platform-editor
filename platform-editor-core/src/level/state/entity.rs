use crate::{
    common_util::{Direction, Rectf, Vec2, Vec2f},
    level::state::{CollectibleState, TileState},
};

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
    /// The size of an entity relative to the measurement of a tile (whose size is considered to be `1.0`).
    pub const SIZE: f32 = Self::RADIUS * 2.0;

    /// The radius of an entity relative to measurement of a tile (whose size is considered to be `1.0`).
    pub const RADIUS: f32 = 0.375;

    pub fn hitbox(&self) -> Rectf {
        Rectf::from_center(self.pos, Vec2::new(Self::SIZE, Self::SIZE))
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

        // Check if the entity is too close to the void, or is touching something deadly.
        let alive = !self.is_touching_void(tiles) && !self.is_touching_deadly_area(tiles);

        // Move the entity.
        self.move_in_steps(tiles);

        alive
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
        self.hitbox().touches_y_line(tiles.size.y as f32 - 0.1)
    }

    fn gravity_multiplier(&self) -> f32 {
        match self.gravity_direction {
            Direction::Up | Direction::Left => -1.0,
            Direction::Down | Direction::Right => 1.0,
        }
    }

    pub fn is_colliding_with_tiles(&self, tiles: &TileState) -> bool {
        let hitbox = self.hitbox();

        if !Rectf::new(Vec2f::new(0.0, 0.0), tiles.size.map(|u| u as f32)).contains_rect(hitbox) {
            return true;
        }

        for y in 0..tiles.size.y {
            for x in 0..tiles.size.x {
                let tile = tiles.tile(x, y);
                let tile_hitbox = tile.hitbox(Vec2f::new(x as f32, y as f32));
                if let Some(tile_hitbox) = tile_hitbox
                    && tile_hitbox.intersects(hitbox)
                {
                    return true;
                }
            }
        }
        false
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

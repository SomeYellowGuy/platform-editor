use crate::{
    common_util::{Direction, Rect, Vec2, Vec2f},
    level::scratch::StoredScratchLevel,
};

pub mod scratch;

pub const ENTITY_SIZE: f32 = 0.75;
pub const GRAVITY: f32 = 0.47;
pub const JUMP_VELOCITY: f32 = 0.16;
pub const MOVE_VELOCITY: f32 = 0.6;

#[derive(Debug, Clone, Copy)]
pub enum LockColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
}

/// A tile at runtime. Locks are not stored here,
/// but are rather stored separately.
#[derive(Debug, Default, Clone, PartialEq)]
pub enum Tile {
    #[default]
    Empty,
    Block,
    TopSlab,
    BottomSlab,
    Grass(usize),
    Dirt(usize),

    PlacedBlock,
    PlacedTimedBlock(f32),

    Shooter(Direction),
    Spike(Direction),
}

#[derive(Debug, Default, Clone)]
pub struct StoredLock {
    pub pos: Vec2f,
    pub size: Vec2f,
}

#[derive(Debug, Default, Clone)]
pub struct Entity {
    pub pos: Vec2f,
    pub velocity: Vec2f,
    pub reversed_gravity: bool,

    is_falling: bool,
}

impl Entity {
    pub fn hitbox(&self) -> Rect {
        Rect::from_center(self.pos, Vec2::new(ENTITY_SIZE, ENTITY_SIZE))
    }

    pub fn tick(&mut self, tiles: &TileState, delta: f32) {
        // Apply gravity.
        self.velocity -= Vec2f::new(0.0, -GRAVITY) * delta * self.gravity_multiplier();

        // Apply friction.
        self.velocity.x *= 0.85_f32.powf(delta * 30.0);

        // Move the entity.
        self.move_in_steps(tiles);
    }

    pub fn apply_controls(&mut self, left: bool, right: bool, jump: bool, delta: f32) {
        let horizontal = -(left as i8) + (right as i8);
        self.velocity.x += horizontal as f32 * delta * MOVE_VELOCITY;

        if !self.is_falling && jump {
            self.velocity.y = -JUMP_VELOCITY * self.gravity_multiplier();
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

    fn gravity_multiplier(&self) -> f32 {
        if self.reversed_gravity { -1.0 } else { 1.0 }
    }

    pub fn is_colliding_with_tiles(&self, tiles: &TileState) -> bool {
        let hitbox = self.hitbox();

        if !Rect::new(
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
                let tile_hitbox = Rect::from_xy_and_dimensions(x as f32, y as f32, 1.0, 1.0);
                if tile_hitbox.intersects(hitbox) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Default, Clone)]
pub struct LevelState {
    pub player: Entity,
    pub tile_state: TileState,
}

#[derive(Debug, Default, Clone)]
pub struct TileState {
    pub tiles: Vec<Tile>,
    pub size: Vec2<usize>,
}

impl TileState {
    pub fn tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y * self.size.x + x]
    }

    pub fn tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.tiles[y * self.size.x + x]
    }
}

impl LevelState {
    /// Creates a new state, into which a level can be loaded.
    pub fn new() -> Self {
        LevelState::default()
    }

    pub fn load_scratch_level(&mut self, index: usize) {
        let level = &crate::level::scratch::levels::LEVELS[index];
        let mut tiles = Vec::new();
        self.tile_state.size = Vec2::new(StoredScratchLevel::WIDTH, StoredScratchLevel::HEIGHT);
        for y in 0..StoredScratchLevel::HEIGHT {
            for x in 0..StoredScratchLevel::WIDTH {
                let mut set_tile = Tile::Empty;
                let i = y * self.tile_state.size.x + x;
                if let Some(tile) = level.tiles[i].to_tile_state(i) {
                    set_tile = tile
                }
                tiles.push(set_tile);
            }
        }
        self.tile_state.tiles = tiles;
        self.player.pos = level.start_pos;
        self.player.velocity = Vec2f::new(0.0, 0.0);
        self.player.reversed_gravity = false;
    }

    pub fn tick(&mut self, delta: f32) {
        self.player.tick(&self.tile_state, delta);
    }
}

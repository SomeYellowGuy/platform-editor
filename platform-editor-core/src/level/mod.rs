use crate::{
    common_util::{Direction, Rectf, Vec2, Vec2f},
    level::state::TileState,
    textures::level::TileTextures,
};

pub mod scratch;
pub mod state;

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

#[derive(Debug, Clone)]
pub struct StoredLock {
    pub pos: Vec2f,
    pub size: Vec2f,
}

/// An extra condition for a star to be collected when a level is finished.
#[derive(Debug, Clone)]
pub enum StarCondition {
    /// The star of the given index must be collected (it is present somewhere in the level space).
    Collect(usize),
    /// The level must be finished in the stored number of seconds.
    Time(u32),
    /// The number of items placed must at most be equal to this value.
    Items(u32),
    /// At least this number of enemies must be defeated.
    Enemies(u32),
    /// At least this number of enemies must be left alive.
    EnemiesLeft(u32),
    /// The player must be in this gravity state.
    Gravity(Direction),
}

impl StarCondition {
    pub fn number_display(&self) -> Option<u32> {
        match self {
            Self::Time(n) | Self::Items(n) | Self::Enemies(n) | Self::EnemiesLeft(n) => Some(*n),
            _ => None,
        }
    }
}

/// A structure that describes a type of moving block.
#[derive(Debug, Clone)]
pub enum MovingBlockItem {
    Single(Direction),
    Horizontal,
    Vertical,
}

impl MovingBlockItem {
    pub fn icon_texture<'a, T>(&self, textures: &'a TileTextures<T>) -> Option<&'a T> {
        match self {
            Self::Single(d) => textures.placed_blocks.moving.single.get(*d),
            Self::Horizontal => Some(&textures.placed_blocks.moving.horizontal),
            Self::Vertical => Some(&textures.placed_blocks.moving.vertical),
        }
    }
}

/// A type of something that can be placed by a player.
#[derive(Debug, Clone)]
pub enum Item {
    Block,
    TimedBlock(i32),
    Moving(MovingBlockItem),
    GravityOrb,
    Star,
}

impl Item {
    pub fn icon_texture<'a, T>(&self, textures: &'a TileTextures<T>) -> Option<&'a T> {
        match self {
            Self::Block => Some(&textures.placed_blocks.permanent),
            Self::TimedBlock(t) => Some(textures.placed_blocks.timed_texture(*t)),
            Self::Moving(moving_block_item) => moving_block_item.icon_texture(textures),
            // TODO
            Self::GravityOrb => None,
            Self::Star => None,
        }
    }
}

/// Represents a type of item (which can be placed) and its remaining count.
#[derive(Debug, Clone)]
pub struct ItemStack {
    pub item: Item,
    pub count: u32,
}

impl ItemStack {
    pub const fn new(item: Item, count: u32) -> Self {
        Self { item, count }
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

impl Entity {
    pub fn hitbox(&self) -> Rectf {
        Rectf::from_center(self.pos, Vec2::new(ENTITY_SIZE, ENTITY_SIZE))
    }

    pub fn gravity(&self) -> Vec2f {
        self.gravity_direction.unit_vec2f() * GRAVITY
    }

    pub fn tick(&mut self, tiles: &TileState, delta: f32) {
        // Apply gravity.
        self.velocity += self.gravity() * delta;

        // Apply friction.
        let non_gravity_component = self
            .velocity
            .get_mut(self.gravity_direction.bidirection().other());
        *non_gravity_component *= 0.85_f32.powf(delta * 30.0);

        // Move the entity.
        self.move_in_steps(tiles);
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

use crate::{
    common_util::{Direction, Rectf, Vec2, Vec2f},
    textures::level::TileTextures,
};

pub mod scratch;
pub mod state;

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

    pub fn icon_texture_mut<'a, T>(&self, textures: &'a mut TileTextures<T>) -> Option<&'a mut T> {
        match self {
            Self::Single(d) => textures.placed_blocks.moving.single.get_mut(*d),
            Self::Horizontal => Some(&mut textures.placed_blocks.moving.horizontal),
            Self::Vertical => Some(&mut textures.placed_blocks.moving.vertical),
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

    pub fn icon_texture_mut<'a, T>(&self, textures: &'a mut TileTextures<T>) -> Option<&'a mut T> {
        match self {
            Self::Block => Some(&mut textures.placed_blocks.permanent),
            Self::TimedBlock(t) => Some(textures.placed_blocks.timed_texture_mut(*t)),
            Self::Moving(moving_block_item) => moving_block_item.icon_texture_mut(textures),
            // TODO
            Self::GravityOrb => None,
            Self::Star => None,
        }
    }

    pub fn icon_texture_scale(&self) -> f32 {
        match self {
            Self::GravityOrb | Self::Star => 0.8,
            _ => 1.0,
        }
    }

    pub fn place_outcome(&self) -> ItemPlaceOutcome {
        match self {
            Self::Block => ItemPlaceOutcome::Tile(Tile::PlacedBlock),
            Self::TimedBlock(t) => ItemPlaceOutcome::Tile(Tile::PlacedTimedBlock(*t as f32)),
            Self::Moving(moving_block_item) => ItemPlaceOutcome::Moving(moving_block_item.clone()),
            Self::GravityOrb => todo!(),
            Self::Star => todo!(),
        }
    }
}

pub enum ItemPlaceOutcome {
    Tile(Tile),
    Moving(MovingBlockItem),
    // TODO: Add collectable outcome
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

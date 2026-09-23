pub mod entity;

use std::time::Instant;

use crate::{
    common_util::{Direction, Vec2, Vec2f, Vec2i},
    component::level::end_dialog::StarStatus,
    level::{
        FlagState, ItemPlaceOutcome, ItemStack, StarCondition, Tile, scratch::StoredScratchLevel,
        state::entity::Entity,
    },
};

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

    pub fn is_within_bounds(&self, tile: Vec2<i32>) -> bool {
        (0..self.size.x as i32).contains(&tile.x) && (0..self.size.y as i32).contains(&tile.y)
    }

    pub fn clamp(&self, mut tile: Vec2i) -> Vec2<usize> {
        tile.x = tile.x.clamp(0, (self.size.x - 1) as i32);
        tile.y = tile.y.clamp(0, (self.size.y - 1) as i32);
        Vec2::new(tile.x as usize, tile.y as usize)
    }
}

#[derive(Debug, Default, Clone)]
pub struct LevelState {
    pub finish_instant: Option<Instant>,
    pub go_instant: Option<Instant>,

    pub player: Entity,
    pub tile_state: TileState,
    pub flag: FlagState,

    /// The star conditions for all stars except the first star.
    pub star_conditions: Vec<StarCondition>,
    /// The state of items that can be placed in a level.
    pub items: Vec<ItemStack>,
    /// The selected item stack index.
    pub selected_item: Option<usize>,
    /// The number of items placed currently.
    pub placed_items: u32,
}

pub enum LevelStateOutcome {
    None,
    Win,
    Lose,
}

impl LevelState {
    /// Creates a new state into which a level can be loaded.
    pub fn new() -> Self {
        LevelState::default()
    }

    pub fn load_scratch_level(&mut self, index: usize, initial_selected_item: Option<usize>) {
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
        self.player.gravity_direction = Direction::Down;
        self.flag = level.flag;

        self.star_conditions = level.star_conditions.to_vec();
        self.items = level.items.to_vec();
        self.finish_instant = None;

        self.selected_item = initial_selected_item;
    }

    pub fn is_finished(&self) -> bool {
        self.finish_instant.is_some()
    }

    pub fn tick(&mut self, delta: f32) -> LevelStateOutcome {
        if !self.player.tick(&self.tile_state, delta) {
            return LevelStateOutcome::Lose;
        }

        // Check if the player touched the flag.
        if !self.is_finished() && self.flag.hitbox().intersects(self.player.hitbox()) {
            LevelStateOutcome::Win
        } else {
            LevelStateOutcome::None
        }
    }

    pub fn star_statuses(&self) -> Vec<StarStatus> {
        let mut statuses = Vec::new();
        for condition in &self.star_conditions {
            statuses.push(StarStatus {
                collected: self.is_satisfied(condition),
                condition: condition.clone(),
            });
        }
        statuses
    }

    pub fn is_satisfied(&self, condition: &StarCondition) -> bool {
        match condition {
            StarCondition::Time(t) => self
                .go_instant
                .is_none_or(|s| s.elapsed().as_secs() <= *t as u64),
            StarCondition::Items(items) => self.placed_items <= *items,
            _ => true, // TODO
        }
    }

    pub fn mark_finished(&mut self) {
        if self.finish_instant.is_none() {
            self.finish_instant = Some(Instant::now());
        }
    }

    pub fn go(&mut self) -> (Instant, bool) {
        let instant = Instant::now();
        if self.go_instant.is_none() {
            self.go_instant = Some(instant);
            return (instant, true);
        }
        (instant, false)
    }

    /// Tries to place the currently-selected item on the board, returning `true`
    /// if it was successful.
    pub fn try_place_item(&mut self, pos: Vec2<i32>) -> bool {
        if !self.tile_state.is_within_bounds(pos) {
            return false;
        }
        let pos = pos.map(|i| i as usize);
        if self.tile_state.tile(pos.x, pos.y) != &Tile::Empty {
            return false;
        }
        let Some(stack) = self.selected_item() else {
            return false;
        };
        if stack.count == 0 {
            return false;
        }
        stack.count -= 1;
        let outcome = stack.item.place_outcome();
        // Apply the outcome.
        let outcome_is_successful = self.apply_outcome(pos, outcome);
        if outcome_is_successful {
            self.placed_items += 1;
        }
        outcome_is_successful
    }

    pub fn apply_outcome(&mut self, pos: Vec2<usize>, outcome: ItemPlaceOutcome) -> bool {
        match outcome {
            ItemPlaceOutcome::Tile(tile) => {
                let old_tile = std::mem::replace(self.tile_state.tile_mut(pos.x, pos.y), tile);
                // Check if the player is colliding with a tile.
                if self.player.is_colliding_with_tiles(&self.tile_state) {
                    // Reverse the placement.
                    *self.tile_state.tile_mut(pos.x, pos.y) = old_tile;
                    return false;
                }
                true
            }
            ItemPlaceOutcome::Moving(_) => todo!(),
        }
    }

    pub fn selected_item(&mut self) -> Option<&mut ItemStack> {
        self.selected_item.map(|i| &mut self.items[i])
    }
}

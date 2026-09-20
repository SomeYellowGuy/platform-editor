use std::time::Instant;

use crate::{
    common_util::{Vec2, Vec2f},
    component::level::end_dialog::StarStatus,
    level::{Entity, FlagState, StarCondition, Tile, scratch::StoredScratchLevel},
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
}

impl LevelState {
    /// Creates a new state into which a level can be loaded.
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
        self.flag = level.flag;

        self.star_conditions = level.star_conditions.to_vec();

        self.finish_instant = None;
    }

    pub fn is_finished(&self) -> bool {
        self.finish_instant.is_some()
    }

    pub fn tick(&mut self, delta: f32) -> bool {
        self.player.tick(&self.tile_state, delta);

        // Check if the player touched the flag.
        if !self.is_finished() {
            self.flag.hitbox().intersects(self.player.hitbox())
        } else {
            false
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
}

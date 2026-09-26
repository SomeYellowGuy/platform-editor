mod collectible;
mod entity;
mod moving;
mod shooter;

use std::time::Instant;

use crate::{
    common_util::{Direction, Rectf, Vec2, Vec2f, Vec2i},
    component::level::end_dialog::StarStatus,
    level::{
        StarCondition,
        definition::{CollectibleType, FlagState, ItemPlaceOutcome, ItemStack, Tile},
        scratch::StoredScratchLevel,
        state::moving::MovingHitboxSnapshot,
    },
};

pub use {
    collectible::CollectibleState,
    entity::{CollisionContext, Controls, Entity},
    moving::Moving,
    shooter::{ShooterBullet, ShooterState},
};

#[derive(Debug, Default, Clone)]
pub struct TileState {
    pub tiles: Vec<Tile>,
    pub size: Vec2<usize>,
}

#[derive(Debug, Clone)]
pub enum TileCollision<'a> {
    None,
    Border,
    Tile(&'a Tile),
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

    pub fn tick(&mut self, delta: f32) {
        for tile in &mut self.tiles {
            if let Tile::PlacedTimedBlock(t) = tile {
                *t -= delta;
                if *t <= 0.0 {
                    // Destroy the tile.
                    *tile = Tile::Empty
                }
            }
        }
    }

    pub fn is_colliding_with_hitbox(&self, hitbox: Rectf) -> bool {
        matches!(
            self.tile_collision_with_hitbox(hitbox),
            TileCollision::Border | TileCollision::Tile(_)
        )
    }

    pub fn tile_collision_with_hitbox(&self, hitbox: Rectf) -> TileCollision<'_> {
        if !Rectf::new(Vec2f::new(0.0, 0.0), self.size.map(|u| u as f32)).contains_rect(hitbox) {
            return TileCollision::Border;
        }

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let tile = self.tile(x, y);
                let tile_hitbox = tile.hitbox(Vec2f::new(x as f32, y as f32));
                if let Some(tile_hitbox) = tile_hitbox
                    && tile_hitbox.intersects(hitbox)
                {
                    return TileCollision::Tile(tile);
                }
            }
        }
        TileCollision::None
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
    placed_items: u32,
    /// The current collectibles in the level.
    pub collectibles: Vec<CollectibleState>,
    /// The indices of the collected stars in the level.
    collected_star_indices: Vec<usize>,

    /// The active shooter instances in the level.
    pub shooters: Vec<ShooterState>,
    /// The active shooter bullets in the level.
    pub shooter_bullets: Vec<ShooterBullet>,

    /// The active moving platforms in the level.
    pub moving: Vec<Moving>,
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

    pub fn load_scratch_level(&mut self, index: usize, initially_selected_item: Option<usize>) {
        let level = &crate::level::scratch::levels::LEVELS[index];
        let mut tiles = Vec::new();

        self.shooters.clear();
        self.shooter_bullets.clear();
        self.tile_state.size = Vec2::new(StoredScratchLevel::WIDTH, StoredScratchLevel::HEIGHT);
        for y in 0..StoredScratchLevel::HEIGHT {
            for x in 0..StoredScratchLevel::WIDTH {
                let mut set_tile = Tile::Empty;
                let i = y * self.tile_state.size.x + x;
                if let Some(tile) = level.tiles[i].to_tile_state(i) {
                    set_tile = tile
                }
                if let Tile::Shooter {
                    direction,
                    speed_multiplier,
                } = &set_tile
                {
                    self.shooters.push(ShooterState::new(
                        Vec2f::new(x as f32 + 0.5, y as f32 + 0.5),
                        *direction,
                        *speed_multiplier,
                    ));
                }
                tiles.push(set_tile);
            }
        }
        self.tile_state.tiles = tiles;
        self.player.pos = level.start_pos;
        self.player.velocity = Vec2f::new(0.0, 0.0);
        self.player.gravity_direction = Direction::Down;
        self.flag = level.flag;

        self.star_conditions = level
            .star_conditions
            .iter()
            .map(|c| StarCondition::from(*c))
            .collect();
        self.items = level.items.to_vec();
        self.finish_instant = None;
        self.selected_item = if let Some(i) = initially_selected_item
            && i >= self.items.len()
        {
            // To avoid going out of bounds, reset the selected item index.
            None
        } else {
            initially_selected_item
        };
        self.collectibles = level
            .collectibles
            .iter()
            .map(|c| CollectibleState::new((*c).into()))
            .collect();
    }

    pub fn is_finished(&self) -> bool {
        self.finish_instant.is_some()
    }

    pub fn tick(&mut self, player_controls: Controls, delta: f32) -> LevelStateOutcome {
        // Tick the tile state.
        self.tile_state.tick(delta);

        // Get a snapshot of all moving platform hitboxes.
        let snapshot = MovingHitboxSnapshot::new(&self.moving);
        // Tick moving platforms.
        for (i, moving) in &mut self.moving.iter_mut().enumerate() {
            moving.tick(i, &self.tile_state, &snapshot, delta);
        }

        let collision_context =
            CollisionContext::new(&self.tile_state, &self.moving, &snapshot, delta);

        // Update the moving platform vectors for the player.
        self.player.update_platform_move_vector(collision_context);

        // Tick shooter bullets.
        if self
            .shooter_bullets
            .iter()
            .any(|bullet| bullet.is_touching(self.player.pos, Entity::RADIUS))
        {
            return LevelStateOutcome::Lose;
        }

        self.shooter_bullets
            .retain_mut(|bullet| bullet.tick(delta, collision_context));

        // Check for shooters.
        for shooter in &mut self.shooters {
            if let Some(bullet) = shooter.tick(delta, self.player.pos) {
                self.shooter_bullets.push(bullet);
            }
        }

        // Tick the player.
        if !self.player.tick(player_controls, collision_context) {
            return LevelStateOutcome::Lose;
        }

        // Check for any collected collectibles.
        for collectible in self.player.check_collectibles(&mut self.collectibles) {
            collectible.mark_collected();
            if let CollectibleType::Star(i) = collectible.ty {
                self.collected_star_indices.push(i);
            }
        }

        // Check for collectibles to destroy.
        self.collectibles.retain(|s| !s.should_be_destroyed());

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
            StarCondition::Collect(i) => self.collected_star_indices.contains(i),
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
    pub fn try_place_item(&mut self, pos: Vec2<i32>, delta: f32) -> bool {
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
        let outcome_is_successful = self.apply_outcome(pos, outcome, delta);
        if outcome_is_successful {
            self.placed_items += 1;
        }
        outcome_is_successful
    }

    pub fn apply_outcome(
        &mut self,
        pos: Vec2<usize>,
        outcome: ItemPlaceOutcome,
        delta: f32,
    ) -> bool {
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
            ItemPlaceOutcome::Moving(ty) => {
                let moving_platform = Moving::new(ty, pos.to_center_vec2f());
                // Check if the player is colliding with a non-empty tile.
                if self
                    .tile_state
                    .is_colliding_with_hitbox(moving_platform.hitbox())
                {
                    return false;
                }
                self.moving.push(moving_platform);

                let snapshot = MovingHitboxSnapshot::new(&self.moving);
                let context =
                    CollisionContext::new(&self.tile_state, &self.moving, &snapshot, delta);
                // Check if the player is colliding with the moving platform.
                if self.player.colliding_with_moving(context).is_some() {
                    // Reverse the placement.
                    self.moving.pop();
                    return false;
                }
                true
            }
        }
    }

    pub fn selected_item(&mut self) -> Option<&mut ItemStack> {
        self.selected_item.map(|i| &mut self.items[i])
    }
}

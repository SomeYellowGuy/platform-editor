mod collectible;
mod entity;
mod moving;
mod shooter;

use std::time::Instant;

use crate::{
    audio::{AudioPlay, Sound},
    common_util::{Direction, Rectf, Vec2, Vec2f, Vec2i},
    component::level::end_dialog::StarStatus,
    level::{
        LockBorderType, LockColor, LockColorMap, StarCondition,
        definition::{
            Collectible, CollectibleType, FlagState, Item, ItemPlaceOutcome, ItemStack, StoredLock,
            Tile,
        },
        scratch::{ScratchTileState, StoredScratchLevel},
        state::{
            entity::{EntityAudioData, HazardContext},
            moving::MovingHitboxSnapshot,
        },
    },
};

pub use {
    collectible::CollectibleState,
    entity::{CollisionContext, Controls, Enemy, Entity},
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

    pub fn tick(&mut self, delta: f32, audio_play: &mut impl AudioPlay) {
        for tile in &mut self.tiles {
            if let Tile::PlacedTimedBlock(t) = tile {
                *t -= delta;
                if *t <= 0.0 {
                    // Destroy the tile.
                    *tile = Tile::Empty;
                    audio_play.play(Sound::Disintegrate);
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

#[derive(Debug, Clone)]
pub struct Lock {
    pub rect: Rectf,
    pub keyhole_offset: Vec2f,
    pub border_type: LockBorderType,
}

impl Lock {
    pub const BORDER_THICKNESS: f32 = 0.11;
    pub const KEYHOLE_SIZE: f32 = 0.35;

    pub const FADE_TIME: f32 = 0.2;
}

#[derive(Debug, Default, Clone)]
pub struct LockState {
    /// The active locks in the level.
    locks: LockColorMap<Vec<Lock>>,
    /// The [`Instants`] when a key of each color has been collected.
    key_collect_instants: LockColorMap<Instant>,
}

impl LockState {
    pub fn new() -> Self {
        Self {
            locks: LockColorMap::new(),
            key_collect_instants: LockColorMap::new(),
        }
    }

    pub fn clear(&mut self) {}

    /// Gets the instant when a key of the provided color was collected.
    pub fn key_collect_instant(&self, color: LockColor) -> Option<Instant> {
        self.key_collect_instants.get(color).copied()
    }

    /// Adds a lock to this lock state.
    pub fn add_lock(&mut self, lock: Lock, color: LockColor) {
        if let Some(v) = self.locks.get_mut(color) {
            v.push(lock);
        } else {
            self.locks.insert(color, vec![lock]);
        }
    }

    /// Adds a stored lock to this lock state.
    pub fn add_stored_lock(&mut self, lock: StoredLock) {
        if let Some(v) = self.locks.get_mut(lock.color) {
            v.push(lock.into());
        } else {
            self.locks.insert(lock.color, vec![lock.into()]);
        }
    }

    pub fn locks(&self) -> &LockColorMap<Vec<Lock>> {
        &self.locks
    }

    /// Unlocks all locks of the provided color.
    pub fn unlock(&mut self, color: LockColor) {
        if !self.key_collect_instants.contains_key(color) {
            self.key_collect_instants.insert(color, Instant::now());
        }
    }

    /// Returns whether the locks of the given color are unlocked.
    pub fn is_unlocked(&self, color: LockColor) -> bool {
        self.key_collect_instants.contains_key(color)
    }

    /// Returns whether the provided hitbox is colliding with any active
    /// lock in the level.
    pub fn is_colliding(&self, hitbox: Rectf) -> bool {
        for (color, locks) in &self.locks {
            if self.is_unlocked(color) {
                continue;
            }
            if locks.iter().any(|l| l.rect.intersects(hitbox)) {
                return true;
            }
        }
        false
    }

    pub fn color_alpha(&self, color: LockColor) -> f32 {
        self.key_collect_instant(color).map_or(0.0, |i| {
            (i.elapsed().as_secs_f32() / Lock::FADE_TIME).min(1.0)
        })
    }
}

#[derive(Debug, Clone)]
pub struct LevelState {
    pub finish_instant: Option<Instant>,
    pub go_instant: Option<Instant>,

    start_instant: Instant,

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

    /// The active lock state in the level.
    pub lock_state: LockState,

    /// The enemies in the level.
    pub enemies: Vec<Enemy>,
    pub enemies_at_start: usize,
}

pub enum LevelStateOutcome {
    None,
    Win,
    Lose,
}

impl Default for LevelState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceError {
    OutOfItems,
    OutOfBounds,
    Collision,
    NoSelectedItem,
}

impl LevelState {
    /// Creates a new state into which a level can be loaded.
    pub fn new() -> Self {
        LevelState {
            finish_instant: None,
            go_instant: None,
            start_instant: Instant::now(),
            player: Entity::default(),
            tile_state: TileState::default(),
            flag: FlagState::default(),
            star_conditions: Vec::new(),
            items: Vec::new(),
            selected_item: None,
            placed_items: 0,
            collectibles: Vec::new(),
            collected_star_indices: Vec::new(),
            shooters: Vec::new(),
            shooter_bullets: Vec::new(),
            moving: Vec::new(),
            lock_state: LockState::new(),
            enemies: Vec::new(),
            enemies_at_start: 0,
        }
    }

    pub fn load_scratch_level(
        &mut self,
        index: usize,
        initially_selected_item: Option<usize>,
        last_level_stars: Option<u32>,
    ) {
        let level = &crate::level::scratch::levels::LEVELS[index];
        let mut tiles = Vec::new();

        self.lock_state.clear();
        self.shooters.clear();
        self.shooter_bullets.clear();
        self.tile_state.size = Vec2::new(StoredScratchLevel::WIDTH, StoredScratchLevel::HEIGHT);
        for y in 0..StoredScratchLevel::HEIGHT {
            for x in 0..StoredScratchLevel::WIDTH {
                let mut set_tile = Tile::Empty;
                let i = y * self.tile_state.size.x + x;
                match level.tiles[i].to_tile_state(i) {
                    ScratchTileState::Tile(t) => set_tile = t,
                    ScratchTileState::Lock(lock) => self
                        .lock_state
                        .add_lock(lock.into_lock(Vec2::new(x, y)), lock.color),
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
        self.player.reset();
        self.enemies = level
            .enemies
            .iter()
            .map(|e| Enemy::new(e.ty, e.pos, e.gravity_direction))
            .collect();
        self.enemies_at_start = self.enemies.len();
        self.player.gravity_direction = Direction::Down;
        self.flag = level.flag;

        self.star_conditions = level
            .star_conditions
            .iter()
            .map(|c| StarCondition::from(*c))
            .collect();
        self.items = if let Some(stars) = last_level_stars {
            vec![ItemStack::new(Item::Star, stars)]
        } else {
            level.items.to_vec()
        };
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

    pub fn tick(
        &mut self,
        audio_play: &mut impl AudioPlay,
        player_controls: Controls,
        delta: f32,
    ) -> LevelStateOutcome {
        // Tick the tile state.
        self.tile_state.tick(delta, audio_play);

        // Get a snapshot of all moving platform hitboxes.
        let snapshot = MovingHitboxSnapshot::new(&self.moving);
        // Tick moving platforms.
        for (i, moving) in &mut self.moving.iter_mut().enumerate() {
            moving.tick(i, &self.tile_state, &self.lock_state, &snapshot, delta);
        }

        let collision_context = CollisionContext::new(
            &self.tile_state,
            &self.lock_state,
            &self.moving,
            &snapshot,
            delta,
        );

        // Update the moving platform vectors for the player.
        self.player.update_platform_move_vector(collision_context);

        self.shooter_bullets
            .retain_mut(|bullet| bullet.tick(delta, collision_context));

        // Check for shooters to shoot anything.
        for shooter in &mut self.shooters {
            if let Some(bullet) = shooter.tick(delta, self.player.pos) {
                self.shooter_bullets.push(bullet);
                audio_play.play(Sound::Shoot);
            }
        }

        let hazard_context = HazardContext::new(&self.shooter_bullets);

        // Tick the player.
        if !self.player.tick(
            player_controls,
            collision_context,
            hazard_context,
            EntityAudioData::new(audio_play, 0.7),
        ) {
            return LevelStateOutcome::Lose;
        }

        // Tick the enemies.
        self.enemies.retain_mut(|e| {
            let alive = e.tick(
                self.player.pos,
                collision_context,
                hazard_context,
                audio_play,
            );

            if !alive {
                audio_play.play(Sound::EnemyDefeat);
            }

            alive
        });

        // Check if any enemies touch the player.
        let player_hitbox = self.player.hitbox();
        for enemy in &self.enemies {
            if enemy
                .entity
                .hitbox()
                .inflate(-0.2)
                .intersects(player_hitbox)
            {
                return LevelStateOutcome::Lose;
            }
        }

        let time = self.nanos_since_start();

        // Check for any collected collectibles.
        let mut collected_colors: Vec<LockColor> = Vec::new();
        let mut collectible_collected = false;
        for collectible in self.player.check_collectibles(time, &mut self.collectibles) {
            collectible.mark_collected();
            collectible_collected = true;
            match collectible.ty {
                CollectibleType::Star(i) => self.collected_star_indices.push(i),
                CollectibleType::Key(color) => {
                    self.lock_state.unlock(color);
                    // "Collect" all the other keys of the same color for later.
                    collected_colors.push(color);
                }
                CollectibleType::GravityOrb => self.player.invert_gravity(),
            }
        }

        for collectible in &mut self.collectibles {
            if let CollectibleType::Key(color) = collectible.ty
                && collected_colors.contains(&color)
            {
                collectible_collected = true;
                collectible.mark_collected();
            }
        }

        if collectible_collected {
            audio_play.play(Sound::Collect);
        }

        // Check for any orbs that enemies have collected.
        for enemy in &mut self.enemies {
            for collectible in enemy
                .entity
                .check_collectibles(time, &mut self.collectibles)
            {
                if let CollectibleType::GravityOrb = collectible.ty {
                    enemy.entity.invert_gravity();
                    collectible.mark_collected();
                }
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
            StarCondition::EnemiesDefeated(e) => {
                self.enemies_at_start - self.enemies.len() >= *e as usize
            }
            StarCondition::EnemiesLeft(e) => self.enemies.len() >= *e as usize,
            StarCondition::Gravity(d) => self.player.gravity_direction == *d,
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
    pub fn try_place_item(&mut self, pos: Vec2<i32>) -> Result<(), PlaceError> {
        if !self.tile_state.is_within_bounds(pos) {
            return Err(PlaceError::OutOfBounds);
        }
        let pos = pos.map(|i| i as usize);
        if self.tile_state.tile(pos.x, pos.y) != &Tile::Empty {
            return Err(PlaceError::NoSelectedItem);
        }
        let Some(stack) = self.selected_item() else {
            return Err(PlaceError::NoSelectedItem);
        };
        if stack.count == 0 {
            return Err(PlaceError::OutOfItems);
        }
        let outcome = stack.item.place_outcome();
        // Apply the outcome.
        let outcome_is_successful = self.apply_outcome(pos, outcome);
        if outcome_is_successful == Ok(()) {
            let stack = self.selected_item_mut().unwrap();
            stack.count -= 1;
            self.placed_items += 1;
        }
        outcome_is_successful
    }

    fn entities(&self) -> impl Iterator<Item = &Entity> {
        std::iter::once(&self.player).chain(self.enemies.iter().map(|e| &e.entity))
    }

    fn collides_with_collectibles(&self, hitbox: Rectf) -> bool {
        self.collectibles.iter().any(|c| {
            c.tile_place_hitbox(self.nanos_since_start())
                .intersects(hitbox)
        })
    }

    fn collides_with_level_check(&self, hitbox: Rectf) -> bool {
        // Check if this hitbox collides with a collectible.
        if self.collides_with_collectibles(hitbox) {
            return false;
        }

        // Check if any entity is colliding with the hitbox.
        if self.entities().any(|e| e.hitbox().intersects(hitbox)) {
            return false;
        }

        // Check if any moving platform is colliding with the hitbox.
        if self.moving.iter().any(|m| m.hitbox().intersects(hitbox)) {
            return false;
        }

        true
    }

    /// Returns the time, in nanoseconds, since this level state was created.
    ///
    /// The creation time of this state can predate its first tick.
    pub fn nanos_since_start(&self) -> u128 {
        self.start_instant.elapsed().as_nanos()
    }

    pub fn apply_outcome(
        &mut self,
        pos: Vec2<usize>,
        outcome: ItemPlaceOutcome,
    ) -> Result<(), PlaceError> {
        match outcome {
            ItemPlaceOutcome::Tile(tile) => {
                if let Some(hitbox) = tile.hitbox(pos.to_vec2f())
                    && !self.collides_with_level_check(hitbox)
                {
                    return Err(PlaceError::Collision);
                }
                *self.tile_state.tile_mut(pos.x, pos.y) = tile;
                Ok(())
            }
            ItemPlaceOutcome::Moving(ty) => {
                let moving_platform = Moving::new(ty, pos.to_center_vec2f());
                let hitbox = moving_platform.hitbox();
                if !self.collides_with_level_check(hitbox) {
                    return Err(PlaceError::Collision);
                }
                self.moving.push(moving_platform);
                Ok(())
            }
            ItemPlaceOutcome::Collectible(ty) => {
                let collectible =
                    CollectibleState::new(Collectible::new(pos.to_center_vec2f(), ty));
                if !self.collides_with_level_check(
                    collectible.tile_place_hitbox(self.nanos_since_start()),
                ) {
                    return Err(PlaceError::Collision);
                }
                self.collectibles.push(collectible);
                Ok(())
            }
        }
    }

    pub fn selected_item(&self) -> Option<&ItemStack> {
        self.selected_item.map(|i| &self.items[i])
    }

    pub fn selected_item_mut(&mut self) -> Option<&mut ItemStack> {
        self.selected_item.map(|i| &mut self.items[i])
    }
}

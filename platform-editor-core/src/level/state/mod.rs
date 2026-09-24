pub mod entity;

use std::time::{Duration, Instant};

use crate::{
    common_util::{Bidirection, Direction, Rectf, Vec2, Vec2f, Vec2i},
    component::level::end_dialog::StarStatus,
    level::{
        Collectible, CollectibleType, FlagState, ItemPlaceOutcome, ItemStack, StarCondition, Tile,
        scratch::StoredScratchLevel, state::entity::Entity,
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
        if !Rectf::new(Vec2f::new(0.0, 0.0), self.size.map(|u| u as f32)).contains_rect(hitbox) {
            return true;
        }

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let tile = self.tile(x, y);
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
}

#[derive(Debug, Clone)]
pub struct CollectibleState {
    pub pos: Vec2<f32>,
    pub ty: CollectibleType,
    pub collect_time: Option<Instant>,
}

impl CollectibleState {
    pub const HITBOX_RADIUS: f32 = 0.25;
    pub const FADE_TIME: f32 = 0.5;

    pub fn new(collectible: Collectible) -> Self {
        Self {
            pos: collectible.pos,
            ty: collectible.ty,
            collect_time: None,
        }
    }

    pub fn is_collected(&self) -> bool {
        self.collect_time.is_some()
    }

    pub fn mark_collected(&mut self) {
        if self.collect_time.is_none() {
            self.collect_time = Some(Instant::now())
        }
    }

    pub fn is_touching(&self, entity_pos: Vec2f, entity_radius: f32) -> bool {
        let max_distance = entity_radius + Self::HITBOX_RADIUS;
        entity_pos.distance_sqr(self.pos) < max_distance * max_distance
    }

    pub fn should_be_destroyed(&self) -> bool {
        self.collect_time
            .is_some_and(|t| t.elapsed().as_secs_f32() > Self::FADE_TIME)
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
                if let Tile::Shooter(d) = &set_tile {
                    self.shooters.push(ShooterState::new(
                        Vec2f::new(x as f32 + 0.5, y as f32 + 0.5),
                        *d,
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

    pub fn tick(&mut self, delta: f32) -> LevelStateOutcome {
        // Tick the tile state.
        self.tile_state.tick(delta);

        // Tick shooter bullets.
        if self
            .shooter_bullets
            .iter()
            .any(|bullet| bullet.is_touching(self.player.pos, Entity::RADIUS))
        {
            return LevelStateOutcome::Lose;
        }

        self.shooter_bullets
            .retain_mut(|bullet| bullet.tick(delta, &self.tile_state));

        // Check for shooters.
        for shooter in &mut self.shooters {
            if let Some(bullet) = shooter.tick(delta, self.player.pos) {
                self.shooter_bullets.push(bullet);
            }
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

        // Check if the player touched the void.
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

#[derive(Debug, Clone)]
pub struct ShooterState {
    pub pos: Vec2f,
    pub cooldown: f32,
    pub direction: Direction,
    last_shot: Option<Instant>,
}

impl ShooterState {
    pub const COOLDOWN: f32 = 1.25;
    pub const ACTIVATION_RANGE: f32 = 0.35;

    pub const GLOW_SIZE: f32 = 0.9;

    pub fn new(pos: Vec2f, direction: Direction) -> Self {
        Self {
            pos,
            direction,
            cooldown: 0.0,
            last_shot: None,
        }
    }

    /// Ticks this shooter, returning the shot bullet if any.
    pub fn tick(&mut self, delta: f32, player_pos: Vec2f) -> Option<ShooterBullet> {
        self.cooldown = (self.cooldown - delta).max(0.0);
        let should_shoot = {
            if self.cooldown < f32::EPSILON {
                // Check whether the player is in range.
                match self.direction.bidirection() {
                    Bidirection::Horizontal => {
                        (self.pos.y - player_pos.y).abs() <= Self::ACTIVATION_RANGE
                    }
                    Bidirection::Vertical => {
                        (self.pos.x - player_pos.x).abs() <= Self::ACTIVATION_RANGE
                    }
                }
            } else {
                false
            }
        };

        if should_shoot {
            self.cooldown = Self::COOLDOWN;
            self.last_shot = Some(Instant::now())
        }

        should_shoot.then(|| ShooterBullet::new(self))
    }

    pub fn glow(&self) -> Option<f32> {
        self.last_shot.and_then(|i| {
            let elapsed = i.elapsed().as_secs_f32();
            if elapsed > ShooterBullet::IMMUNE_TIME {
                None
            } else {
                Some(1.0 - elapsed / ShooterBullet::IMMUNE_TIME)
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct ShooterBullet {
    pub pos: Vec2f,
    pub direction: Direction,
    instant: Instant,
}

impl ShooterBullet {
    pub const SPEED: f32 = 5.0;

    pub const HIT_RADIUS: f32 = 0.2;
    pub const SIZE: f32 = 2.0 * Self::HIT_RADIUS;

    pub const IMMUNE_TIME: f32 = (0.6 / Self::SPEED) + 0.04;

    pub fn new(shooter: &ShooterState) -> Self {
        Self {
            pos: shooter.pos,
            direction: shooter.direction,
            instant: Instant::now(),
        }
    }

    pub fn hitbox(&self) -> Rectf {
        Rectf::from_center(self.pos, Vec2f::new(Self::SIZE, Self::SIZE))
    }

    pub fn is_immune_to_tiles(&self) -> bool {
        self.instant.elapsed() < Duration::from_secs_f32(Self::IMMUNE_TIME)
    }

    /// Ticks this bullet, returning whether it should survive.
    pub fn tick(&mut self, delta: f32, tile_state: &TileState) -> bool {
        if !self.is_immune_to_tiles() && tile_state.is_colliding_with_hitbox(self.hitbox()) {
            return false;
        }

        self.pos += self.direction.unit_vec2f() * Self::SPEED * delta;

        true
    }

    pub fn is_touching(&self, pos: Vec2f, entity_radius: f32) -> bool {
        let max_distance = entity_radius + Self::HIT_RADIUS;
        self.pos.distance_sqr(pos) < (max_distance * max_distance)
    }
}

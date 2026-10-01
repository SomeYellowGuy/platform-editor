use std::time::Instant;

use crate::{
    common_util::{Rectf, Vec2f},
    level::definition::{Collectible, CollectibleType, Track},
};

#[derive(Debug, Clone)]
pub struct CollectibleState {
    pos: Vec2f,
    pub ty: CollectibleType,
    pub track: Option<Track>,
    pub collect_time: Option<Instant>,
}

impl CollectibleState {
    pub const HITBOX_RADIUS: f32 = 0.25;
    pub const FADE_TIME: f32 = 0.5;

    pub fn new(collectible: Collectible) -> Self {
        Self {
            pos: collectible.pos,
            ty: collectible.ty,
            track: collectible.track,
            collect_time: None,
        }
    }

    pub fn pos(&self, time: u128) -> Vec2f {
        self.pos
            + self
                .track
                .as_ref()
                .map_or(Vec2f::new(0.0, 0.0), |t| t.offset(time))
    }

    pub fn tile_place_hitbox(&self, time: u128) -> Rectf {
        Rectf::from_center(
            self.pos(time),
            Vec2f::new(Self::HITBOX_RADIUS, Self::HITBOX_RADIUS),
        )
    }

    pub fn is_collected(&self) -> bool {
        self.collect_time.is_some()
    }

    pub fn mark_collected(&mut self) {
        if self.collect_time.is_none() {
            self.collect_time = Some(Instant::now())
        }
    }

    pub fn is_touching(&self, time: u128, entity_pos: Vec2f, entity_radius: f32) -> bool {
        let max_distance = entity_radius + Self::HITBOX_RADIUS;
        entity_pos.distance_sqr(self.pos(time)) < max_distance * max_distance
    }

    pub fn should_be_destroyed(&self) -> bool {
        self.collect_time
            .is_some_and(|t| t.elapsed().as_secs_f32() > Self::FADE_TIME)
    }
}

use crate::{
    common_util::{Direction, Rectf, Vec2f},
    level::{
        definition::MovingBlockItem,
        state::{CollisionContext, TileState},
    },
};

#[derive(Debug, Clone)]
pub struct Moving {
    pub ty: MovingBlockItem,

    pub pos: Vec2f,
    pub direction: Direction,

    pub flip_cooldown: f32,
}

/// A snapshot of all hitboxes of the moving platforms in a level.
/// This is used to let moving platforms collide with other moving platforms
/// without issues with Rust's borrow checker.
#[derive(Debug, Clone)]
pub struct MovingHitboxSnapshot {
    hitboxes: Vec<Rectf>,
}

impl MovingHitboxSnapshot {
    pub fn new(moving: &[Moving]) -> Self {
        Self {
            hitboxes: moving.iter().map(Moving::hitbox).collect(),
        }
    }

    pub fn is_colliding(&self, index: usize, hitbox: Rectf) -> bool {
        self.hitboxes
            .iter()
            .enumerate()
            .any(|(i, h)| i != index && h.intersects(hitbox))
    }
}

impl Moving {
    pub const SPEED: f32 = 2.0;
    const HITBOX_SIZE: f32 = 0.99;

    pub fn new(ty: MovingBlockItem, pos: Vec2f) -> Self {
        Self {
            pos,
            direction: Self::initial_direction(&ty),
            ty,
            flip_cooldown: 0.0,
        }
    }

    fn initial_direction(ty: &MovingBlockItem) -> Direction {
        match ty {
            MovingBlockItem::Single(d) => *d,
            MovingBlockItem::Horizontal => Direction::Right,
            MovingBlockItem::Vertical => Direction::Up,
        }
    }

    pub fn hitbox(&self) -> Rectf {
        Rectf::from_center(self.pos, Vec2f::new(Self::HITBOX_SIZE, Self::HITBOX_SIZE))
    }

    pub fn target_velocity(&self, delta: f32) -> Vec2f {
        self.direction.unit_vec2f() * Self::SPEED * delta
    }

    pub fn velocity(
        &self,
        index: usize,
        tiles: &TileState,
        snapshot: &MovingHitboxSnapshot,
        delta: f32,
    ) -> Vec2f {
        let mut pos = self.pos;
        let velocity = self.target_velocity(delta);
        let quality = ((velocity.x.abs() + velocity.y.abs()).ceil() * 5.0) as usize;
        let hitbox = self.hitbox();
        for _ in 0..quality {
            let x = pos.x;
            pos.x += velocity.x / quality as f32;
            if Self::is_colliding(hitbox, index, tiles, snapshot) {
                pos.x = x;
                break;
            }
        }
        for _ in 0..quality {
            let y = pos.y;
            pos.y += velocity.y / quality as f32;
            if Self::is_colliding(hitbox, index, tiles, snapshot) {
                pos.y = y;
                break;
            }
        }

        pos - self.pos
    }

    fn is_colliding(
        hitbox: Rectf,
        index: usize,
        tiles: &TileState,
        snapshot: &MovingHitboxSnapshot,
    ) -> bool {
        snapshot.is_colliding(index, hitbox) || tiles.is_colliding_with_hitbox(hitbox)
    }

    pub fn tick(
        &mut self,
        index: usize,
        tiles: &TileState,
        snapshot: &MovingHitboxSnapshot,
        delta: f32,
    ) {
        self.flip_cooldown = (self.flip_cooldown - delta).max(0.0);

        let mut velocity = self.target_velocity(delta);
        let quality = ((velocity.x.abs() + velocity.y.abs()).ceil() * 5.0) as usize;

        for _ in 0..quality {
            let x = self.pos.x;
            self.pos.x += velocity.x / quality as f32;
            if Self::is_colliding(self.hitbox(), index, tiles, snapshot) {
                self.pos.x = x;
                self.update_x_velocity();
                velocity = self.target_velocity(delta);
            }
        }
        for _ in 0..quality {
            let y = self.pos.y;
            self.pos.y += velocity.y / quality as f32;
            if Self::is_colliding(self.hitbox(), index, tiles, snapshot) {
                self.pos.y = y;
                self.update_y_velocity();
                velocity = self.target_velocity(delta);
            }
        }
    }

    fn update_x_velocity(&mut self) {
        if let MovingBlockItem::Horizontal = self.ty {
            self.direction = self.direction.opposite();
        }
    }

    fn update_y_velocity(&mut self) {
        if let MovingBlockItem::Vertical = self.ty {
            self.direction = self.direction.opposite();
        }
    }

    pub fn colliding_with_moving(context: CollisionContext, hitbox: Rectf) -> Option<Vec2f> {
        context.moving.iter().enumerate().find_map(|(i, m)| {
            m.hitbox()
                .intersects(hitbox)
                .then(|| m.velocity(i, context.tiles, context.moving_snapshot, context.delta))
        })
    }
}

use crate::{
    common_util::{Bidirection, Direction, Rectf, Vec2f},
    level::{
        MovingBlockItem,
        state::{CollisionContext, TileState},
    },
};

#[derive(Debug, Clone)]
pub struct Moving {
    pub ty: MovingBlockItem,

    pub pos: Vec2f,
    pub direction: Direction,
}

impl Moving {
    pub const SPEED: f32 = 3.0;
    const HITBOX_SIZE: f32 = 0.99;

    pub fn new(ty: MovingBlockItem, pos: Vec2f) -> Self {
        Self {
            pos,
            direction: Self::initial_direction(&ty),
            ty,
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

    pub fn velocity(&self, tiles: &TileState, delta: f32) -> Vec2f {
        let mut pos = self.pos;
        let velocity = self.direction.unit_vec2f() * Self::SPEED * delta;
        let quality = ((velocity.x.abs() + velocity.y.abs()).ceil() * 5.0) as usize;
        let hitbox = self.hitbox();
        for _ in 0..quality {
            let x = pos.x;
            pos.x += velocity.x / quality as f32;
            if tiles.is_colliding_with_hitbox(hitbox) {
                pos.x = x;
                break;
            }
        }
        for _ in 0..quality {
            let y = pos.y;
            pos.y += velocity.y / quality as f32;
            if tiles.is_colliding_with_hitbox(hitbox) {
                pos.y = y;
                break;
            }
        }

        pos - self.pos
    }

    pub fn tick(&mut self, tiles: &TileState, delta: f32) {
        let velocity = self.direction.unit_vec2f() * Self::SPEED * delta;
        let quality = ((velocity.x.abs() + velocity.y.abs()).ceil() * 5.0) as usize;
        let hitbox = self.hitbox();
        for _ in 0..quality {
            let x = self.pos.x;
            self.pos.x += velocity.x / quality as f32;
            if tiles.is_colliding_with_hitbox(hitbox) {
                self.pos.x = x;
                self.update_x_velocity();
                break;
            }
        }
        for _ in 0..quality {
            let y = self.pos.y;
            self.pos.y += velocity.y / quality as f32;
            if tiles.is_colliding_with_hitbox(hitbox) {
                self.pos.y = y;
                self.update_y_velocity();
                break;
            }
        }
    }

    fn update_x_velocity(&mut self) {
        if let MovingBlockItem::Horizontal = self.ty
            && self.direction.bidirection() == Bidirection::Horizontal
        {
            self.direction = self.direction.opposite()
        }
    }

    fn update_y_velocity(&mut self) {
        if let MovingBlockItem::Vertical = self.ty
            && self.direction.bidirection() == Bidirection::Vertical
        {
            self.direction = self.direction.opposite()
        }
    }

    pub fn colliding_with_moving(context: CollisionContext, hitbox: Rectf) -> Option<Vec2f> {
        context.moving.iter().find_map(|m| {
            m.hitbox()
                .intersects(hitbox)
                .then(|| m.velocity(context.tiles, context.delta))
        })
    }
}

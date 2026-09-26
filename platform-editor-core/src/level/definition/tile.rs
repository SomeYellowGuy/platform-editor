use crate::common_util::{Direction, Rectf, Vec2, Vec2f};

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

    Shooter {
        direction: Direction,
        speed_multiplier: f32,
    },
    Spike(Direction),
}

impl Tile {
    pub const SLAB_THICKNESS: f32 = 0.55;
    pub const SPIKE_SLAB_THICKNESS: f32 = 0.25;

    pub const SPIKE_KILL_HITBOX_DIMENSIONS: Vec2f = Vec2f::new(0.2, 0.4);

    fn slab_hitbox(top_left: Vec2f, direction: Direction, thickness: f32) -> Rectf {
        match direction {
            Direction::Up => Rectf::new(top_left, Vec2f::new(1.0, thickness)),
            Direction::Down => {
                Rectf::new(top_left.add_y(1.0 - thickness), Vec2f::new(1.0, thickness))
            }
            Direction::Left => Rectf::new(top_left, Vec2f::new(thickness, 1.0)),
            Direction::Right => Rectf::new(
                top_left + Vec2f::new(1.0 - thickness, 1.0),
                Vec2f::new(thickness, 1.0),
            ),
        }
    }

    /// Returns the colliding hitbox of this tile.
    pub fn hitbox(&self, top_left: Vec2f) -> Option<Rectf> {
        match self {
            Self::Empty => None,

            Self::TopSlab => Some(Self::slab_hitbox(
                top_left,
                Direction::Up,
                Self::SLAB_THICKNESS,
            )),
            Self::BottomSlab => Some(Self::slab_hitbox(
                top_left,
                Direction::Down,
                Self::SLAB_THICKNESS,
            )),
            Self::Spike(d) => Some(Self::slab_hitbox(
                top_left,
                d.opposite(),
                Self::SPIKE_SLAB_THICKNESS,
            )),

            _ => Some(Rectf::new(top_left, Vec2f::new(1.0, 1.0))),
        }
    }

    /// Returns the [`Rectf`] used to draw the texture of this tile, if any.
    pub fn rendering_rect(&self, top_left: Vec2f) -> Option<Rectf> {
        match self {
            Self::Spike(_) => Some(Rectf::new(top_left, Vec2f::new(1.0, 1.0))),
            _ => self.hitbox(top_left),
        }
    }

    fn spike_half_hitbox(pos: Vec2<usize>, x_center: f32, direction: Direction) -> Rectf {
        match direction {
            Direction::Up => Rectf::new(
                Vec2f::new(
                    x_center - (Self::SPIKE_KILL_HITBOX_DIMENSIONS.x / 2.0),
                    1.0 - Self::SPIKE_KILL_HITBOX_DIMENSIONS.y - Self::SPIKE_SLAB_THICKNESS,
                ) + pos.to_vec2f(),
                Self::SPIKE_KILL_HITBOX_DIMENSIONS,
            ),
            Direction::Down => Rectf::new(
                Vec2f::new(
                    x_center - (Self::SPIKE_KILL_HITBOX_DIMENSIONS.x / 2.0),
                    Self::SPIKE_KILL_HITBOX_DIMENSIONS.y + Self::SPIKE_SLAB_THICKNESS,
                ) + pos.to_vec2f(),
                Self::SPIKE_KILL_HITBOX_DIMENSIONS,
            ),
            _ => todo!(),
        }
    }

    pub fn spike_hitboxes(pos: Vec2<usize>, direction: Direction) -> [Rectf; 2] {
        [
            Self::spike_half_hitbox(pos, 0.275, direction),
            Self::spike_half_hitbox(pos, 0.75, direction),
        ]
    }

    /// Returns whether an entity with the provided hitbox would be
    /// defeated due to touching this tile's kill hitbox.
    pub fn is_deadly_for_hitbox(&self, pos: Vec2<usize>, hitbox: Rectf) -> bool {
        if let Self::Spike(direction) = self {
            let hitboxes = Self::spike_hitboxes(pos, *direction);
            hitboxes[0].intersects(hitbox) || hitboxes[1].intersects(hitbox)
        } else {
            false
        }
    }
}

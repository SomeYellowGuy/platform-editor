use crate::{
    common_util::{Rectf, Vec2f},
    level::{LockBorderType, LockColor, state::Lock},
};

#[derive(Debug, Clone, Copy)]
pub struct StoredLock {
    pub rect: Rectf,
    pub color: LockColor,
}

impl From<StoredLock> for Lock {
    fn from(value: StoredLock) -> Self {
        Self {
            rect: value.rect,
            keyhole_offset: Vec2f::new(0.0, 0.0),
            border_type: match (value.rect.dimensions.x, value.rect.dimensions.y) {
                (w, h) if w > h => LockBorderType::OnlyHorizontal,
                (w, h) if w < h => LockBorderType::OnlyVertical,
                _ => LockBorderType::Both,
            },
        }
    }
}

use crate::{common_util::Rectf, level::LockColor};

#[derive(Debug, Clone)]
pub struct StoredLock {
    pub area: Rectf,
    pub color: LockColor,
}

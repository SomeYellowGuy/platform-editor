use crate::common_util::rgb;

pub mod bottom_bar;
pub mod end_dialog;

/// The main part of a level, and the area of the actual level.
#[derive(Debug, Default)]
pub struct BoardBase;

/// The placeable items of a level.
#[derive(Debug, Default)]
pub struct ItemTabBase;

impl ItemTabBase {
    pub const EMPTY_ITEM_STACK_COLOR: u32 = rgb(255, 127, 127);
}

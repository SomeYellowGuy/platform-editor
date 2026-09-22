use crate::{
    common_util::Vec2,
    level::{Item, ItemStack, StarCondition, scratch::StoredScratchLevel},
};

/// The total number of Scratch levels in the game.
pub const LEVEL_COUNT: usize = LEVELS.len();

pub const LEVELS: &[StoredScratchLevel] = &[
    // Level 1
    StoredScratchLevel::new(br"00000000000000000000000000000000000000000000000000000000000000222000000000033322222222223333333333333333")
        .start_pos(Vec2::new(1.5, 4.5))
        .flag_pos(Vec2::new(11.5, 3.5))
        .items(&[ItemStack::new(Item::Block, 1)])
        .stars(StarCondition::Time(12), StarCondition::Time(8)),
    // Level 2
    StoredScratchLevel::new(br"00000000000000000000000000000000000002200000000000330000000000033000000000003322000000000333300000000033")
        .start_pos(Vec2::new(1.0, 4.5))
        .flag_pos(Vec2::new(11.5, 3.5))
        .items(&[ItemStack::new(Item::Block, 4)])
        .stars(StarCondition::Time(12), StarCondition::Items(3)),
    // Level 3
    StoredScratchLevel::new(br"00000000000000000000000000000000000000022200000000003330000001000333000002222233300000333333330000033333")
        .start_pos(Vec2::new(1.5, 1.8))
        .flag_pos(Vec2::new(11.5, 3.5))
        .items(&[ItemStack::new(Item::TimedBlock(3), 1)])
        .stars(StarCondition::Collect(0), StarCondition::Items(0)),
];

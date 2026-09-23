use crate::{
    common_util::{Vec2, Vec2f},
    level::{
        Item, ItemStack,
        scratch::{
            ScratchCollectible as Collectible, ScratchCollectibleType as CollectableType,
            ScratchStarCondition as StarCondition, StoredScratchLevel as Level,
        },
    },
};

/// The total number of Scratch levels in the game.
pub const LEVEL_COUNT: usize = LEVELS.len();

pub const LEVELS: &[Level] = &[
    // Level 1
    Level::builder(br"00000000000000000000000000000000000000000000000000000000000000222000000000033322222222223333333333333333")
        .start_pos(Vec2::new(1.5, 4.5))
        .flag_pos(Vec2::new(11.5, 3.5))
        .items(&[ItemStack::new(Item::Block, 1)])
        .stars(StarCondition::Time(12), StarCondition::Time(8))
        .build(),
    // Level 2
    Level::builder(br"00000000000000000000000000000000000002200000000000330000000000033000000000003322000000000333300000000033")
        .start_pos(Vec2::new(1.0, 4.5))
        .flag_pos(Vec2::new(12.0, 1.5))
        .items(&[ItemStack::new(Item::Block, 4)])
        .stars(StarCondition::Time(12), StarCondition::Items(3))
        .build(),
    // Level 3
    Level::builder(br"00000000000000000000000000000000000000022200000000003330000001000333000002222233300000333333330000033333")
        .start_pos(Vec2::new(1.5, 1.8))
        .flag_pos(Vec2::new(11.5, 4.5))
        .items(&[ItemStack::new(Item::TimedBlock(3), 1)])
        .stars(StarCondition::Collect, StarCondition::Items(0))
        .collectibles(&[Collectible::new(Vec2f::new(7.5, 0.75), CollectableType::Star)])
        .build(),
];

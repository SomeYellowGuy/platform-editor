use crate::{
    common_util::{Direction, Vec2, Vec2f},
    level::{
        definition::{Enemy, Item, ItemStack, MovingBlockItem},
        scratch::{
            ScratchCollectible as Collectible, ScratchCollectibleType as CollectibleType,
            ScratchLockColor as LockColor, ScratchStarCondition as StarCondition,
            StoredScratchLevel as Level,
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
        .collectibles(&[Collectible::new(Vec2f::new(11.0, 1.0), CollectibleType::Star)])
        .stars(StarCondition::Collect, StarCondition::Items(0))
        .build(),
    // Level 4
    Level::builder(br"00000000000000000000000000110000000000055555555550000000000000000000000000000000000000000002200000000000")
        .start_pos(Vec2::new(1.0, 6.2))
        .flag_pos(Vec2::new(1.0, 1.5))
        .items(&[ItemStack::new(Item::Block, 9)])
        .stars(StarCondition::Items(7), StarCondition::Time(25))
        .build(),
    // Level 5
    Level::builder(br"00000000000000000000000000000000000000000000000006000000000006222000000006233300010002233332222222333333")
        .start_pos(Vec2::new(1.0, 6.0))
        .flag_pos(Vec2::new(12.0, 3.5))
        .items(&[ItemStack::new(Item::TimedBlock(3), 4), ItemStack::new(Item::TimedBlock(2), 1)])
        .collectibles(&[Collectible::new(Vec2f::new(5.5, 1.5), CollectibleType::Star)])
        .stars(StarCondition::Collect, StarCondition::Items(3))
        .build(),
    // Level 6
    Level::builder(br"00003300000000000330000000220063002200033006300330003300330063222370000003333333000000333333322222233333")
        .start_pos(Vec2::new(1.0, 1.2))
        .flag_pos(Vec2::new(12.0, 3.5))
        .items(&[ItemStack::new(Item::Block, 3), ItemStack::new(Item::TimedBlock(2), 4)])
        .stars(StarCondition::Items(5), StarCondition::Time(22))
        .build(),
    // Level 7
    Level::builder(br"000GGG9GGG00000000000000000000000000000000000000000000000000000000000000000000000885E08850022GGGG1GGGG22")
        .start_pos(Vec2::new(1.0, 5.9))
        .flag_pos(Vec2::new(12.0, 6.5))
        .items(&[ItemStack::new(Item::Moving(MovingBlockItem::Single(Direction::Up)), 2)])
        .collectibles(&[Collectible::new(Vec2f::new(11.5, 0.5), CollectibleType::Star)])
        .stars(StarCondition::Collect, StarCondition::Time(9))
        .build(),
    // Level 8
    Level::builder(br"99999999900000000000000000000000000092229900000000003000000000000000000000002200000000000332288888888833")
        .start_pos(Vec2::new(1.0, 6.0))
        .flag_pos(Vec2::new(12.0, 1.5))
        .items(&[
            ItemStack::new(Item::TimedBlock(2), 8),
            ItemStack::new(Item::Moving(MovingBlockItem::Single(Direction::Left)), 1),
            ItemStack::new(Item::Moving(MovingBlockItem::Single(Direction::Right)), 1)
        ])
        .stars(StarCondition::Items(4), StarCondition::Time(24))
        .build(),
    // Level 9
    Level::builder(br"00000000000000000000000000000000000500000000000010000000000001000000000000100010000000010002222222222222")
        .start_pos(Vec2::new(1.5, 4.5))
        .flag_pos(Vec2::new(11.5, 6.5))
        .items(&[
            ItemStack::new(Item::TimedBlock(1), 5)
        ])
        .stars(StarCondition::Items(3), StarCondition::Time(10))
        .build(),
    // Level 10
    Level::builder(br"000000000000000000000000000000000000222000000000233300000000033330000000003300000000000AB002222222222222")
        .start_pos(Vec2::new(1.5, 4.5))
        .flag_pos(Vec2::new(12.0, 6.5))
        .items(&[
            ItemStack::new(Item::TimedBlock(2), 1),
            ItemStack::new(Item::TimedBlock(4), 3)
        ])
        .stars(StarCondition::Collect, StarCondition::Time(16))
        .collectibles(&[
            Collectible::key(Vec2f::new(2.5, 2.0), LockColor::Orange),
            Collectible::key(Vec2f::new(6.5, 1.5), LockColor::Red),
            Collectible::new(Vec2f::new(11.5, 1.0), CollectibleType::Star)
        ])
        .build(),
    // Level 11
    Level::builder(br"00000000900600000000000200000002222232000000B0000030E00002222223020000A000000032000222222223300033333333")
        .start_pos(Vec2::new(0.5, 3.8))
        .flag_pos(Vec2::new(10.3, 3.5))
        .items(&[
            ItemStack::new(Item::Moving(MovingBlockItem::Vertical), 1),
            ItemStack::new(Item::TimedBlock(2), 4)
        ])
        .stars(StarCondition::Items(4), StarCondition::Time(20))
        .collectibles(&[
            Collectible::key(Vec2f::new(9.5, 0.5), LockColor::Red),
            Collectible::key(Vec2f::new(12.5, 0.8), LockColor::Orange)
        ])
        .build(),
    // Level 12
    Level::builder(br"000000000000000000000000000000000000000000000EE000000000009100000000000000000000001000000002222222222222")
        .start_pos(Vec2::new(1.5, 4.5))
        .flag_pos(Vec2::new(11.5, 6.5))
        .items(&[
            ItemStack::new(Item::TimedBlock(3), 1)
        ])
        .enemies(&[Enemy::normal(Vec2f::new(9.5, 6.5))])
        .stars(StarCondition::Enemies(1), StarCondition::Time(9))
        .build(),
    // Level 13
    Level::builder(br"000000000000000000000000000000000000000000000000002200000000000A0222200000022210000000000333222222222233")
        .start_pos(Vec2::new(1.5, 3.5))
        .flag_pos(Vec2::new(12.0, 2.5))
        .items(&[
            ItemStack::new(Item::Moving(MovingBlockItem::Vertical), 1),
            ItemStack::new(Item::Moving(MovingBlockItem::Single(Direction::Left)), 1)
        ])
        .enemies(&[
            Enemy::normal(Vec2f::new(7.0, 6.5)),
            Enemy::normal(Vec2f::new(8.5, 6.5)),
            Enemy::normal(Vec2f::new(12.5, 4.5))
        ])
        .stars(StarCondition::EnemiesLeft(3), StarCondition::Enemies(3))
        .collectibles(&[Collectible::key(Vec2f::new(6.5, 2.0), LockColor::Red)])
        .build()
];

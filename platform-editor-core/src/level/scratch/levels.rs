use crate::{
    common_util::{Direction, Vec2, Vec2f},
    level::{
        definition::{Enemy, FlagState, Item, ItemStack, MovingBlockItem},
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
    Level::builder(br"33999000000003300001111100330000G9999003300000000000330000000000000002218888000000330000000222233EEEEEEE")
        .start_pos(Vec2::new(1.0, 5.6))
        .flag_pos(Vec2::new(7.6, 7.0))
        .items(&[
            ItemStack::new(Item::Moving(MovingBlockItem::Single(Direction::Right)), 1),
            ItemStack::new(Item::Moving(MovingBlockItem::Single(Direction::Up)), 1)
        ])
        .collectibles(&[Collectible::new(Vec2f::new(11.5, 0.5), CollectibleType::Star)])
        .stars(StarCondition::Collect, StarCondition::Time(15))
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
    Level::builder(br"000000000033300000000003000000000000B00000000000022222220000E00A0EEE100001E02210000000000333222222222233")
        .start_pos(Vec2::new(1.5, 3.5))
        .flag_pos(Vec2::new(12.0, 2.5))
        .items(&[
            ItemStack::new(Item::TimedBlock(2), 1),
            ItemStack::new(Item::Moving(MovingBlockItem::Single(Direction::Left)), 1)
        ])
        .enemies(&[
            Enemy::normal(Vec2f::new(7.0, 6.5)),
            Enemy::normal(Vec2f::new(8.5, 6.5)),
            Enemy::normal(Vec2f::new(12.5, 4.5))
        ])
        .stars(StarCondition::EnemiesLeft(3), StarCondition::Enemies(3))
        .collectibles(&[
            Collectible::key(Vec2f::new(6.5, 4.8), LockColor::Red),
            Collectible::key(Vec2f::new(3.5, 6.5), LockColor::Orange)
        ])
        .build(),
    // Level 14
    Level::builder(br"0001000000300000B000000C0022222000002221017B000000B61017A000000A61F1330000002200000000000A02222211111111")
        .start_pos(Vec2::new(1.0, 6.5))
        .flag_pos(Vec2::new(12.0, 1.5))
        .items(&[
            ItemStack::new(Item::Moving(MovingBlockItem::Horizontal), 1),
            ItemStack::new(Item::Block, 2),
            ItemStack::new(Item::TimedBlock(3), 6)
        ])
        .stars(StarCondition::Collect, StarCondition::Time(20))
        .collectibles(&[
            Collectible::new(Vec2f::new(0.8, 0.6), CollectibleType::Star),
            Collectible::key(Vec2f::new(8.3, 1.4), LockColor::Red),
            Collectible::key(Vec2f::new(1.5, 4.8), LockColor::Orange),
            Collectible::key(Vec2f::new(12.5, 6.5), LockColor::Yellow),
        ])
        .build(),
    // Level 15
    Level::builder(br"99900000000000000000000EEE00000000001002222000000C003333000000222000B00000003333111G88883333333333333333")
        .start_pos(Vec2::new(1.5, 2.2))
        .flag_pos(Vec2::new(12.0, 3.5))
        .items(&[
            ItemStack::new(Item::Moving(MovingBlockItem::Single(Direction::Right)), 3),
            ItemStack::new(Item::Moving(MovingBlockItem::Single(Direction::Left)), 1)
        ])
        .collectibles(&[
            Collectible::key(Vec2f::new(12.0, 0.8), LockColor::Orange),
            Collectible::key(Vec2f::new(0.5, 5.5), LockColor::Yellow)
        ])
        .enemies(&[Enemy::normal(Vec2f::new(1.5, 5.5))])
        .stars(StarCondition::Items(3), StarCondition::Time(21))
        .build(),
    // Level 16
    Level::builder(br"00030000100000003000030000000B00013000022220000300000B070000D0000019999992222000000000000002200000022222")
        .start_pos(Vec2::new(1.0, 6.5))
        .flag_pos(Vec2::new(2.5, 4.5))
        .items(&[
            ItemStack::new(Item::TimedBlock(4), 1),
            ItemStack::new(Item::TimedBlock(3), 2),
            ItemStack::new(Item::Moving(MovingBlockItem::Vertical), 1),
            ItemStack::new(Item::Moving(MovingBlockItem::Horizontal), 1)
        ])
        .collectibles(&[
            Collectible::key(Vec2f::new(6.6, 1.6), LockColor::Orange),
            Collectible::key(Vec2f::new(12.5, 0.5), LockColor::Green)
        ])
        .enemies(&[Enemy::normal(Vec2f::new(0.5, 2.5))])
        .stars(StarCondition::Items(4), StarCondition::Time(30))
        .build(),
    // Level 17
    Level::builder(br"111333333333000000000000000000000000000221F111000000333010100000033301010000113330C02000111333EEE3000G11")
        .start_pos(Vec2::new(1.5, 2.0))
        .flag(FlagState::with_direction(Vec2::new(5.5, 4.5), Direction::Up))
        .items(&[
            ItemStack::new(Item::Moving(MovingBlockItem::Vertical), 1),
            ItemStack::new(Item::TimedBlock(2), 1)
        ])
        .collectibles(&[
            Collectible::new(Vec2f::new(6.5, 2.5), CollectibleType::Orb),
            Collectible::new(Vec2f::new(12.5, 2.5), CollectibleType::Orb),
            Collectible::key(Vec2f::new(12.5, 0.5), LockColor::Red),
            Collectible::key(Vec2f::new(11.5, 3.8), LockColor::Yellow)
        ])
        .stars(StarCondition::Time(15), StarCondition::InvertedGravity)
        .build(),
    // Level 18
    Level::builder(br"00031E0000000000D011000000000200000000022232000002200000000000G000000000000D0000000000002222220000000333")
        .start_pos(Vec2::new(1.0, 5.7))
        .flag_pos(Vec2::new(1.7, 2.5))
        .items(&[
            ItemStack::new(Item::Moving(MovingBlockItem::Horizontal), 1),
            ItemStack::new(Item::GravityOrb, 2)
        ])
        .collectibles(&[
            Collectible::new(Vec2f::new(11.5, 1.2), CollectibleType::Orb),
            Collectible::key(Vec2f::new(12.5, 3.5), LockColor::Green),
            Collectible::new(Vec2f::new(9.5, 7.0), CollectibleType::Star)
        ])
        .enemies(&[Enemy::normal_with_direction(Vec2::new(7.8, 0.3), Direction::Up)])
        .stars(StarCondition::Collect, StarCondition::Enemies(1))
        .build(),
    // Level 19
    Level::builder(br"101111E9999991F100B00000001H1009000001100000000000000000000000000000000000000022000000000003300000000000")
        .start_pos(Vec2::new(1.0, 5.5))
        .flag(FlagState::with_direction(Vec2::new(1.5, 0.5), Direction::Up))
        .items(&[
            ItemStack::new(Item::GravityOrb, 2),
            ItemStack::new(Item::Moving(MovingBlockItem::Single(Direction::Left)), 1),
            ItemStack::new(Item::TimedBlock(2), 1)
        ])
        .collectibles(&[
            Collectible::key(Vec2f::new(8.5, 5.5), LockColor::Red),
            Collectible::key(Vec2f::new(4.5, 1.5), LockColor::Orange),
            Collectible::new(Vec2f::new(11.9, 5.9), CollectibleType::Star)
        ])
        .enemies(&[Enemy::normal_with_direction(Vec2::new(3.5, 1.5), Direction::Up)])
        .stars(StarCondition::Time(20), StarCondition::Collect)
        .build(),
    // Level 20
    Level::builder(br"00000000900A00000000000EEE0000000000006GGGGGGGGGGIGG00000000000C000000000000E000000000000A0E000000002G3E")
        .start_pos(Vec2::new(10.5, 0.5))
        .flag_pos(Vec2::new(12.5, 1.0))
        .items(&[
            ItemStack::new(Item::Moving(MovingBlockItem::Horizontal), 1),
            ItemStack::new(Item::Moving(MovingBlockItem::Vertical), 1),
            ItemStack::new(Item::TimedBlock(3), 2)
        ])
        .collectibles(&[
            Collectible::key(Vec2f::new(12.5, 7.0), LockColor::Red),
            Collectible::key(Vec2f::new(0.5, 7.0), LockColor::Yellow),
            Collectible::key(Vec2f::new(0.5, 0.5), LockColor::Blue),
            Collectible::new(Vec2f::new(9.5, 6.5), CollectibleType::Orb),
            Collectible::new(Vec2f::new(12.5, 4.5), CollectibleType::Orb)
        ])
        .stars(StarCondition::InvertedGravity, StarCondition::Items(3))
        .build(),
    // Level 21
    Level::builder(br"9900099999911000000000000G000000000000000000000000G00055000000000001150000000000B0A0000122222221G5555333")
        .start_pos(Vec2::new(1.0, 6.0))
        .flag_pos(Vec2::new(11.5, 2.5))
        .items(&[
            ItemStack::new(Item::GravityOrb, 4)
        ])
        .collectibles(&[
            Collectible::new(Vec2f::new(1.0, 4.5), CollectibleType::Orb),
            Collectible::new(Vec2f::new(2.0, 2.5), CollectibleType::Orb),
            Collectible::key(Vec2f::new(12.5, 5.5), LockColor::Orange),
            Collectible::new(Vec2f::new(3.2, 6.5), CollectibleType::Star),
            Collectible::key(Vec2f::new(3.8, 6.5), LockColor::Red),
        ])
        .stars(StarCondition::Items(1), StarCondition::Collect)
        .build(),
    // Level 22
    Level::builder(br"00001090000010000B0000000600001E0000222000000000063300000000003330000000000G132200000000DB03300000000222")
        .start_pos(Vec2::new(1.0, 5.6))
        .flag_pos(Vec2::new(12.5, 6.5))
        .items(&[
            ItemStack::new(Item::Block, 2),
            ItemStack::new(Item::TimedBlock(4), 1),
            ItemStack::new(Item::Moving(MovingBlockItem::Horizontal), 2),
            ItemStack::new(Item::Moving(MovingBlockItem::Vertical), 1)
        ])
        .collectibles(&[
            Collectible::key(Vec2f::new(1.0, 1.0), LockColor::Green),
            Collectible::key(Vec2f::new(10.5, 1.0), LockColor::Orange)
        ])
        .enemies(&[Enemy::normal_with_direction(Vec2::new(3.5, 2.5), Direction::Up)])
        .stars(StarCondition::Items(5), StarCondition::Time(36))
        .build(),
    // Level 23
    Level::builder(br"0G000000000000C00000000000G10000000000200000000002230000000022333000000223333322002233333333322333333333")
        .start_pos(Vec2::new(1.0, 5.5))
        .flag_pos(Vec2::new(0.7, 1.5))
        .items(&[
            ItemStack::new(Item::TimedBlock(5), 3),
            ItemStack::new(Item::TimedBlock(3), 6),
            ItemStack::new(Item::GravityOrb, 3)
        ])
        .collectibles(&[Collectible::key(Vec2f::new(12.5, 1.0), LockColor::Yellow)])
        .enemies(&[
            Enemy::normal_with_direction(Vec2::new(5.5, 0.5), Direction::Up),
            Enemy::normal_with_direction(Vec2::new(7.5, 0.5), Direction::Up),
            Enemy::normal(Vec2::new(7.5, 4.5)),
            Enemy::normal(Vec2::new(9.5, 3.5)),
            Enemy::normal(Vec2::new(11.5, 2.5))
        ])
        .stars(StarCondition::InvertedGravity, StarCondition::Time(36))
        .build(),
    // Level 24
    Level::builder(br"0000000000006000000000000600000000000060000000000006000000000000600000001111GG0000000ABCDJ02222222222212")
        .start_pos(Vec2::new(1.0, 0.5))
        .flag_pos(Vec2::new(12.5, 6.5))
        .items(&[
            ItemStack::new(Item::TimedBlock(4), 3),
            ItemStack::new(Item::Moving(MovingBlockItem::Vertical), 1)
        ])
        .collectibles(&[
            Collectible::key(Vec2f::new(2.5, 0.5), LockColor::Red),
            Collectible::key(Vec2f::new(4.5, 1.5), LockColor::Orange),
            Collectible::key(Vec2f::new(6.5, 2.5), LockColor::Yellow),
            Collectible::key(Vec2f::new(8.5, 3.5), LockColor::Green),
            Collectible::key(Vec2f::new(10.5, 4.5), LockColor::Blue)
        ])
        .stars(StarCondition::Items(2), StarCondition::Time(24))
        .build(),
    // Level 25
    Level::builder(br"0A000100010000E0001000100010000700060000000010001F1000000G000G00000000D000J00022000EE00EE003388821882188")
        .start_pos(Vec2::new(1.0, 5.5))
        .flag(FlagState::with_direction(Vec2::new(0.5, 0.6), Direction::Left))
        .items(&[
            ItemStack::new(Item::TimedBlock(5), 2),
            ItemStack::new(Item::TimedBlock(4), 4),
            ItemStack::new(Item::Moving(MovingBlockItem::Vertical), 3)
        ])
        .collectibles(&[
            Collectible::key(Vec2f::new(4.0, 1.0), LockColor::Green),
            Collectible::key(Vec2f::new(7.5, 1.0), LockColor::Blue),
            Collectible::key(Vec2f::new(11.0, 1.0), LockColor::Red)
        ])
        .stars(StarCondition::Items(8), StarCondition::Time(60))
        .build(),
    // Level 26
    Level::builder(br"0A000000000A01700000000061100000000000110000000000010000000000E110000000000DC022000000002223350000005333")
        .start_pos(Vec2::new(1.0, 5.5))
        .flag_pos(Vec2::new(12.5, 5.5))
        .items(&[
            ItemStack::new(Item::Moving(MovingBlockItem::Horizontal), 2),
            ItemStack::new(Item::Moving(MovingBlockItem::Vertical), 1),
            ItemStack::new(Item::Block, 2)
        ])
        .collectibles(&[
            Collectible::key(Vec2f::new(3.5, 1.0), LockColor::Yellow),
            Collectible::key(Vec2f::new(9.5, 1.0), LockColor::Green),

            Collectible::key_moving(Vec2f::new(5.5, 2.5), LockColor::Red),
            Collectible::key_moving(Vec2f::new(7.5, 2.5), LockColor::Red),
            Collectible::key_moving(Vec2f::new(5.5, 4.5), LockColor::Red),
            Collectible::key_moving(Vec2f::new(7.5, 4.5), LockColor::Red),
            Collectible::key_moving(Vec2f::new(5.5, 6.5), LockColor::Red),
            Collectible::key_moving(Vec2f::new(7.5, 6.5), LockColor::Red)
        ])
        .enemies(&[
            Enemy::normal(Vec2f::new(0.43, 0.5)),
            Enemy::normal(Vec2f::new(0.5, 0.5)),
            Enemy::normal(Vec2f::new(0.57, 0.5)),

            Enemy::normal(Vec2f::new(12.43, 0.5)),
            Enemy::normal(Vec2f::new(12.5, 0.5)),
            Enemy::normal(Vec2f::new(12.57, 0.5))
        ])
        .stars(StarCondition::Items(3), StarCondition::EnemiesLeft(6))
        .build(),
    // Level 27
    Level::builder(br"1E00000000A000100000000E00K2222222001000D00000300200222022230032200009000003330000000000AJ62222222222222")
        .start_pos(Vec2::new(1.5, 5.5))
        .flag_pos(Vec2::new(12.0, 3.5))
        .items(&[
            ItemStack::new(Item::Moving(MovingBlockItem::Vertical), 1),
            ItemStack::new(Item::TimedBlock(3), 4)
        ])
        .collectibles(&[
            Collectible::new(Vec2f::new(4.5, 6.2), CollectibleType::Orb),
            Collectible::new(Vec2f::new(11.5, 6.5), CollectibleType::Star),

            Collectible::key(Vec2f::new(0.5, 1.5), LockColor::Red),
            Collectible::key(Vec2f::new(2.5, 0.5), LockColor::Yellow),
            Collectible::key_moving(Vec2f::new(4.5, 3.5), LockColor::Green),
            Collectible::key_moving(Vec2f::new(4.3, 0.5), LockColor::Blue),
            Collectible::key_moving(Vec2f::new(7.3, 1.5), LockColor::Blue),
        ])
        .enemies(&[Enemy::slow(Vec2f::new(0.5, 3.5))])
        .stars(StarCondition::Time(30), StarCondition::Collect)
        .build(),
    // Level 28
    Level::builder(br"70000010000A0000000B0000102220001GGGI1000D00000000GG00300000G0000006GG1000000007C00B00000003332210000222")
        .start_pos(Vec2::new(1.5, 1.5))
        .flag_pos(Vec2::new(12.5, 2.5))
        .items(&[
            ItemStack::new(Item::TimedBlock(3), 2),
            ItemStack::new(Item::TimedBlock(5), 2),
        ])
        .collectibles(&[
            Collectible::new(Vec2f::new(11.5, 5.0), CollectibleType::Orb),

            Collectible::key(Vec2f::new(4.0, 6.5), LockColor::Red),
            Collectible::key(Vec2f::new(8.0, 1.0), LockColor::Orange),
            Collectible::key_moving(Vec2f::new(10.5, 4.5), LockColor::Yellow),
            Collectible::key(Vec2f::new(7.5, 6.5), LockColor::Green),
            Collectible::key(Vec2f::new(0.5, 6.5), LockColor::Blue),
        ])
        .enemies(&[Enemy::slow(Vec2f::new(9.5, 1.0))])
        .stars(StarCondition::Items(3), StarCondition::Time(50))
        .build(),
    // Level 29
    Level::builder(br"10A0000000A01GG700000006GG0000000000000500220002200522K3300033H2200003E0E30000000002I200000EEEE11E11EEEE")
        .start_pos(Vec2::new(6.5, 5.5))
        .flag_pos(Vec2::new(6.5, 7.0))
        .items(&[
            ItemStack::new(Item::Moving(MovingBlockItem::Single(Direction::Right)), 1),
            ItemStack::new(Item::Moving(MovingBlockItem::Vertical), 3),
            ItemStack::new(Item::TimedBlock(3), 1),
            ItemStack::new(Item::TimedBlock(5), 2)
        ])
        .collectibles(&[
            Collectible::new(Vec2f::new(6.5, 0.65), CollectibleType::Star),
            Collectible::key_moving(Vec2f::new(6.5, 1.5), LockColor::Yellow),

            Collectible::key(Vec2f::new(1.5, 3.5), LockColor::Red),
            Collectible::key(Vec2f::new(11.5, 3.5), LockColor::Red),
            Collectible::key(Vec2f::new(0.5, 5.5), LockColor::Red),
            Collectible::key(Vec2f::new(12.5, 5.5), LockColor::Red),

            Collectible::key(Vec2f::new(0.5, 7.0), LockColor::Orange),
            Collectible::key(Vec2f::new(12.5, 7.0), LockColor::Blue),
        ])
        .enemies(&[
            Enemy::normal(Vec2f::new(1.5, 0.5)),
            Enemy::normal(Vec2f::new(11.5, 0.5)),
            Enemy::normal(Vec2f::new(10.5, 6.5)),
            Enemy::normal(Vec2f::new(2.5, 6.5))
        ])
        .stars(StarCondition::Collect, StarCondition::Time(30))
        .build(),
    // Level 30
    Level::builder(br"GGG0000000GG000J0000000CD01G7000000011G0000000000000000000000000000000000001G882K201H800A0023E30G090EEEE")
        .start_pos(Vec2::new(1.5, 5.5))
        .flag_pos(Vec2::new(12.5, 1.5))
        .items(&[
            ItemStack::new(Item::Moving(MovingBlockItem::Vertical), 1),
            ItemStack::new(Item::TimedBlock(3), 1)
        ])
        .collectibles(&[
            Collectible::key(Vec2f::new(6.5, 6.8), LockColor::Red),
            Collectible::key(Vec2f::new(1.0, 1.5), LockColor::Orange),
            Collectible::key(Vec2f::new(12.5, 7.0), LockColor::Yellow),
            Collectible::key(Vec2f::new(6.5, 0.5), LockColor::Green),
            Collectible::key(Vec2f::new(12.7, 3.0), LockColor::Blue),

            Collectible::new(Vec2f::new(2.5, 7.0), CollectibleType::Orb),
        ])
        .enemies(&[Enemy::slow(Vec2f::new(6.5, 5.5))])
        .stars(StarCondition::InvertedGravity, StarCondition::Time(60))
        .build(),
    // Level 31 (the Congratulations level)
    Level::builder(br"00000000000000000000000000000000000000000000000000000000011100000000011111000022222222222223333333333333")
        .start_pos(Vec2::new(1.5, 4.5))
        .flag_pos(Vec2::new(1000.0, 1000.0))
        // The items are hardcoded.
        .stars(StarCondition::Time(1), StarCondition::Time(2))
        .build(),
];

use crate::seed_generation::{game_world::GameWorld, seed_settings::{BossGarageRequirements, FinalOxideUnlock, RelicTime}};

#[derive(Debug)]
pub struct GameSetup {
    pub game_world: GameWorld,
    pub settings: Vec<(SettingID, SettingValue)>,
}

#[derive(PartialEq, PartialOrd, Eq, Hash, Copy, Clone, Ord, Debug)]
#[repr(u32)]
pub enum LevelID {
    DingoCanyon = 0,
    DragonMines = 1,
    BlizzardBluff = 2,
    CrashCove = 3,
    TigerTemple = 4,
    PapusPyramid = 5,
    RoosTubes = 6,
    HotAirSkyway = 7,
    SewerSpeedway = 8,
    MysteryCaves = 9,
    CortexCastle = 10,
    NGinLabs = 11,
    PolarPass = 12,
    OxideStation = 13,
    CocoPark = 14,
    TinyArena = 15,
    SlideColiseum = 16,
    TurboTrack = 17,
    NitroCourt = 18,
    RampageRuins = 19,
    SkullRock = 21,
    RockyRoad = 23,
    CupRed = 100,
    CupGreen = 101,
    CupBlue = 102,
    CupYellow = 103,
    CupPurple = 104,
}

impl std::fmt::Display for LevelID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            LevelID::DingoCanyon => String::from("DingoCanyon"),
            LevelID::DragonMines => String::from("DragonMines"),
            LevelID::BlizzardBluff => String::from("BlizzardBluff"),
            LevelID::CrashCove => String::from("CrashCove"),
            LevelID::TigerTemple => String::from("TigerTemple"),
            LevelID::PapusPyramid => String::from("PapusPyramid"),
            LevelID::RoosTubes => String::from("RoosTubes"),
            LevelID::HotAirSkyway => String::from("HotAirSkyway"),
            LevelID::SewerSpeedway => String::from("SewerSpeedway"),
            LevelID::MysteryCaves => String::from("MysteryCaves"),
            LevelID::CortexCastle => String::from("CortexCastle"),
            LevelID::NGinLabs => String::from("NGinLabs"),
            LevelID::PolarPass => String::from("PolarPass"),
            LevelID::OxideStation => String::from("OxideStation"),
            LevelID::CocoPark => String::from("CocoPark"),
            LevelID::TinyArena => String::from("TinyArena"),
            LevelID::SlideColiseum => String::from("SlideColiseum"),
            LevelID::TurboTrack => String::from("TurboTrack"),
            LevelID::NitroCourt => String::from("NitroCourt"),
            LevelID::RampageRuins => String::from("RampageRuins"),
            LevelID::SkullRock => String::from("SkullRock"),
            LevelID::RockyRoad => String::from("RockyRoad"),
            LevelID::CupRed => String::from("RedGemCup"),
            LevelID::CupGreen => String::from("GreenGemCup"),
            LevelID::CupBlue => String::from("BlueGemCup"),
            LevelID::CupYellow => String::from("YellowGemCup"),
            LevelID::CupPurple => String::from("PurpleGemCup"),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum UnlockStage {
    One,
    Two,
}

#[derive(Debug, Clone)]
pub enum UnlockRequirement {
    Item(UnlockRequirementItem),
    LevelList(Vec<LevelID>)
}

#[derive(Debug, Clone, Copy)]
pub struct UnlockRequirementItem {
    pub item_type: RequiredItem,
    pub count: u8,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum RequiredItem {
    Trophy = 98,
    RedCtrToken = 381,
    GreenCtrToken = 637,
    BlueCtrToken = 893,
    YellowCtrToken = 1149,
    PurpleCtrToken = 1405,
    AnyCtrToken = 1917,
    SapphireRelic = 97,
    GoldRelic = 353,
    PlatinumRelic = 609,
    AnyRelic = 1889,
    Key = 99,
    RedGem = 95,
    GreenGem = 351,
    BlueGem = 607,
    YellowGem = 863,
    PurpleGem = 1119,
    AnyGem = 1887,
}

impl std::fmt::Display for RequiredItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            RequiredItem::Trophy => String::from("Trophy"),
            RequiredItem::RedCtrToken => String::from("Red CTR Token"),
            RequiredItem::GreenCtrToken => String::from("Green CTR Token"),
            RequiredItem::BlueCtrToken => String::from("Blue CTR Token"),
            RequiredItem::YellowCtrToken => String::from("Yellow CTR Token"),
            RequiredItem::PurpleCtrToken => String::from("Purple CTR Token"),
            RequiredItem::AnyCtrToken => String::from("Any CTR Token"),
            RequiredItem::SapphireRelic => String::from("Sapphire Relic"),
            RequiredItem::GoldRelic => String::from("Gold Relic"),
            RequiredItem::PlatinumRelic => String::from("Platinum Relic"),
            RequiredItem::AnyRelic => String::from("Any Relic"),
            RequiredItem::Key => String::from("Key"),
            RequiredItem::RedGem => String::from("Red Gem"),
            RequiredItem::GreenGem => String::from("Green Gem"),
            RequiredItem::BlueGem => String::from("Blue Gem"),
            RequiredItem::YellowGem => String::from("Yellow Gem"),
            RequiredItem::PurpleGem => String::from("Purple Gem"),
            RequiredItem::AnyGem => String::from("Any Gem"),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct ItemLocation {
    pub levelid: LevelID,
    pub racetype: RaceType,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
#[repr(u16)]
pub enum RaceType {
    TrophyRace = 98,
    CtrOrCrystalChallenge = 125,
    RelicRaceSapphire = 97,
    RelicRaceGold = 353,
    RelicRacePlatinum = 609,
    BossRace = 99,
    GemCup = 95,
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum RaceReward {
    Trophy = 98,
    RedCtrToken = 381,
    GreenCtrToken = 637,
    BlueCtrToken = 893,
    YellowCtrToken = 1149,
    PurpleCtrToken = 1405,
    SapphireRelic = 97,
    GoldRelic = 353,
    PlatinumRelic = 609,
    Key = 99,
    RedGem = 95,
    GreenGem = 351,
    BlueGem = 607,
    YellowGem = 863,
    PurpleGem = 1119,
    BeatTheGame = 0,
}

impl std::fmt::Display for RaceReward {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            RaceReward::Trophy => String::from("Trophy"),
            RaceReward::RedCtrToken => String::from("Red CTR Token"),
            RaceReward::GreenCtrToken => String::from("Green CTR Token"),
            RaceReward::BlueCtrToken => String::from("Blue CTR Token"),
            RaceReward::YellowCtrToken => String::from("Yellow CTR Token"),
            RaceReward::PurpleCtrToken => String::from("Purple CTR Token"),
            RaceReward::SapphireRelic => String::from("Sapphire Relic"),
            RaceReward::GoldRelic => String::from("Gold Relic"),
            RaceReward::PlatinumRelic => String::from("Platinum Relic"),
            RaceReward::Key => String::from("Key"),
            RaceReward::RedGem => String::from("Red Gem"),
            RaceReward::GreenGem => String::from("Green Gem"),
            RaceReward::BlueGem => String::from("Blue Gem"),
            RaceReward::YellowGem => String::from("Yellow Gem"),
            RaceReward::PurpleGem => String::from("Purple Gem"),
            RaceReward::BeatTheGame => String::from("YOU WIN"),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SettingID {
    RelicDifficulty = 0,
    RelicNeedsPerfect = 1,
    BossGarageRequirements = 2,
    QolSkipMaskhints = 3,
    QolSkipPodium = 4,
    QolSkipMaskcongrats = 5,
    OxideRequiredRelics = 6,
    SeedHash1 = 7,
    SeedHash2 = 8,
    HelperTiziano = 9,
    HelperTA = 10,
}

#[derive(Debug)]
pub enum SettingValue {
    Boolean(bool),
    RelicDifficulty(RelicTime),
    BossGarageRequirements(BossGarageRequirements),
    OxideRequiredRelics(FinalOxideUnlock),
    SeedHashPart(u16),
}

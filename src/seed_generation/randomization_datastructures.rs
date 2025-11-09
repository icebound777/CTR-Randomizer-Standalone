use std::collections::HashMap;

use crate::seed_generation::seed_settings::{BossGarageRequirements, FinalOxideUnlock, RelicTime};

#[derive(Debug)]
pub struct GameSetup {
    pub warppad_links: HashMap<LevelID, LevelID>,
    pub warppad_unlocks: Vec<(LevelID, UnlockStage, UnlockRequirement)>,
    pub race_rewards: Vec<(LevelID, RaceType, RaceReward)>,
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

#[derive(Debug, PartialEq)]
pub enum UnlockStage {
    One,
    Two,
}

#[derive(Debug)]
pub struct UnlockRequirement {
    pub item_type: RequiredItem,
    pub count: u8,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum SettingValue {
    Boolean(bool),
    RelicDifficulty(RelicTime),
    BossGarageRequirements(BossGarageRequirements),
    OxideRequiredRelics(FinalOxideUnlock),
    SeedHashPart(u16),
}

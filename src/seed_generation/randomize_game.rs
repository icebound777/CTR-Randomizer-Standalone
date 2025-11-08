use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

use crate::seed_generation::{
    randomization_datastructures::{
        LevelID, RaceReward, RaceType, RandomizedGame, RequiredItem, SettingID, SettingValue,
        UnlockRequirement, UnlockStage,
    },
    seed_settings::{BossGarageRequirements, FinalOxideUnlock, RelicTime, SeedSettings},
};

fn get_vanilla_game() -> RandomizedGame {
    RandomizedGame {
        warppad_links: HashMap::from([
            (LevelID::DingoCanyon, LevelID::DingoCanyon),
            (LevelID::DragonMines, LevelID::DragonMines),
            (LevelID::BlizzardBluff, LevelID::BlizzardBluff),
            (LevelID::CrashCove, LevelID::CrashCove),
            (LevelID::TigerTemple, LevelID::TigerTemple),
            (LevelID::PapusPyramid, LevelID::PapusPyramid),
            (LevelID::RoosTubes, LevelID::RoosTubes),
            (LevelID::HotAirSkyway, LevelID::HotAirSkyway),
            (LevelID::SewerSpeedway, LevelID::SewerSpeedway),
            (LevelID::MysteryCaves, LevelID::MysteryCaves),
            (LevelID::CortexCastle, LevelID::CortexCastle),
            (LevelID::NGinLabs, LevelID::NGinLabs),
            (LevelID::PolarPass, LevelID::PolarPass),
            (LevelID::OxideStation, LevelID::OxideStation),
            (LevelID::CocoPark, LevelID::CocoPark),
            (LevelID::TinyArena, LevelID::TinyArena),
            (LevelID::SlideColiseum, LevelID::SlideColiseum),
            (LevelID::TurboTrack, LevelID::TurboTrack),
            (LevelID::NitroCourt, LevelID::NitroCourt),
            (LevelID::RampageRuins, LevelID::RampageRuins),
            (LevelID::SkullRock, LevelID::SkullRock),
            (LevelID::RockyRoad, LevelID::RockyRoad),
            (LevelID::CupRed, LevelID::CupRed),
            (LevelID::CupGreen, LevelID::CupGreen),
            (LevelID::CupBlue, LevelID::CupBlue),
            (LevelID::CupYellow, LevelID::CupYellow),
            (LevelID::CupPurple, LevelID::CupPurple),
        ]),
        warppad_unlocks: vec![
            (
                LevelID::CrashCove,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 0,
                },
            ),
            (
                LevelID::CrashCove,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 1,
                },
            ),
            (
                LevelID::RoosTubes,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 0,
                },
            ),
            (
                LevelID::RoosTubes,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 1,
                },
            ),
            (
                LevelID::MysteryCaves,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 1,
                },
            ),
            (
                LevelID::MysteryCaves,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 1,
                },
            ),
            (
                LevelID::SewerSpeedway,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 3,
                },
            ),
            (
                LevelID::SewerSpeedway,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 1,
                },
            ),
            (
                LevelID::SkullRock,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 1,
                },
            ),
            (
                LevelID::CocoPark,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 4,
                },
            ),
            (
                LevelID::CocoPark,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 2,
                },
            ),
            (
                LevelID::TigerTemple,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 4,
                },
            ),
            (
                LevelID::TigerTemple,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 2,
                },
            ),
            (
                LevelID::PapusPyramid,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 6,
                },
            ),
            (
                LevelID::PapusPyramid,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 2,
                },
            ),
            (
                LevelID::DingoCanyon,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 7,
                },
            ),
            (
                LevelID::DingoCanyon,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 2,
                },
            ),
            (
                LevelID::RampageRuins,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 2,
                },
            ),
            (
                LevelID::BlizzardBluff,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 8,
                },
            ),
            (
                LevelID::BlizzardBluff,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 3,
                },
            ),
            (
                LevelID::DragonMines,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 9,
                },
            ),
            (
                LevelID::DragonMines,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 3,
                },
            ),
            (
                LevelID::PolarPass,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 10,
                },
            ),
            (
                LevelID::PolarPass,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 3,
                },
            ),
            (
                LevelID::TinyArena,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 11,
                },
            ),
            (
                LevelID::TinyArena,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 3,
                },
            ),
            (
                LevelID::RockyRoad,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 3,
                },
            ),
            (
                LevelID::CortexCastle,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 12,
                },
            ),
            (
                LevelID::CortexCastle,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 4,
                },
            ),
            (
                LevelID::NGinLabs,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 12,
                },
            ),
            (
                LevelID::NGinLabs,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 4,
                },
            ),
            (
                LevelID::HotAirSkyway,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 14,
                },
            ),
            (
                LevelID::HotAirSkyway,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 4,
                },
            ),
            (
                LevelID::OxideStation,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Trophy,
                    count: 15,
                },
            ),
            (
                LevelID::OxideStation,
                UnlockStage::Two,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 4,
                },
            ),
            (
                LevelID::NitroCourt,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::Key,
                    count: 4,
                },
            ),
            (
                LevelID::SlideColiseum,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::SapphireRelic,
                    count: 10,
                },
            ),
            (
                LevelID::TurboTrack,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::AnyGem,
                    count: 5,
                },
            ),
            (
                LevelID::CupRed,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::RedCtrToken,
                    count: 4,
                },
            ),
            (
                LevelID::CupGreen,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::GreenCtrToken,
                    count: 4,
                },
            ),
            (
                LevelID::CupBlue,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::BlueCtrToken,
                    count: 4,
                },
            ),
            (
                LevelID::CupYellow,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::YellowCtrToken,
                    count: 4,
                },
            ),
            (
                LevelID::CupPurple,
                UnlockStage::One,
                UnlockRequirement {
                    item_type: RequiredItem::PurpleCtrToken,
                    count: 4,
                },
            ),
        ],
        race_rewards: vec![
            (LevelID::CrashCove, RaceType::TrophyRace, RaceReward::Trophy),
            (
                LevelID::CrashCove,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::RedCtrToken,
            ),
            (
                LevelID::CrashCove,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::CrashCove,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::CrashCove,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (LevelID::RoosTubes, RaceType::TrophyRace, RaceReward::Trophy),
            (
                LevelID::RoosTubes,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::GreenCtrToken,
            ),
            (
                LevelID::RoosTubes,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::RoosTubes,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::RoosTubes,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (LevelID::RoosTubes, RaceType::BossRace, RaceReward::Key),
            (
                LevelID::MysteryCaves,
                RaceType::TrophyRace,
                RaceReward::Trophy,
            ),
            (
                LevelID::MysteryCaves,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::RedCtrToken,
            ),
            (
                LevelID::MysteryCaves,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::MysteryCaves,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::MysteryCaves,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (
                LevelID::SewerSpeedway,
                RaceType::TrophyRace,
                RaceReward::Trophy,
            ),
            (
                LevelID::SewerSpeedway,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::BlueCtrToken,
            ),
            (
                LevelID::SewerSpeedway,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::SewerSpeedway,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::SewerSpeedway,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (LevelID::CocoPark, RaceType::TrophyRace, RaceReward::Trophy),
            (
                LevelID::CocoPark,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::GreenCtrToken,
            ),
            (
                LevelID::CocoPark,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::CocoPark,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::CocoPark,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (
                LevelID::TigerTemple,
                RaceType::TrophyRace,
                RaceReward::Trophy,
            ),
            (
                LevelID::TigerTemple,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::BlueCtrToken,
            ),
            (
                LevelID::TigerTemple,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::TigerTemple,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::TigerTemple,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (
                LevelID::PapusPyramid,
                RaceType::TrophyRace,
                RaceReward::Trophy,
            ),
            (
                LevelID::PapusPyramid,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::RedCtrToken,
            ),
            (
                LevelID::PapusPyramid,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::PapusPyramid,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::PapusPyramid,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (LevelID::PapusPyramid, RaceType::BossRace, RaceReward::Key),
            (
                LevelID::DingoCanyon,
                RaceType::TrophyRace,
                RaceReward::Trophy,
            ),
            (
                LevelID::DingoCanyon,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::YellowCtrToken,
            ),
            (
                LevelID::DingoCanyon,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::DingoCanyon,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::DingoCanyon,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (
                LevelID::BlizzardBluff,
                RaceType::TrophyRace,
                RaceReward::Trophy,
            ),
            (
                LevelID::BlizzardBluff,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::RedCtrToken,
            ),
            (
                LevelID::BlizzardBluff,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::BlizzardBluff,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::BlizzardBluff,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (
                LevelID::DragonMines,
                RaceType::TrophyRace,
                RaceReward::Trophy,
            ),
            (
                LevelID::DragonMines,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::BlueCtrToken,
            ),
            (
                LevelID::DragonMines,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::DragonMines,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::DragonMines,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (LevelID::DragonMines, RaceType::BossRace, RaceReward::Key),
            (LevelID::PolarPass, RaceType::TrophyRace, RaceReward::Trophy),
            (
                LevelID::PolarPass,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::GreenCtrToken,
            ),
            (
                LevelID::PolarPass,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::PolarPass,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::PolarPass,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (LevelID::TinyArena, RaceType::TrophyRace, RaceReward::Trophy),
            (
                LevelID::TinyArena,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::YellowCtrToken,
            ),
            (
                LevelID::TinyArena,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::TinyArena,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::TinyArena,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (
                LevelID::CortexCastle,
                RaceType::TrophyRace,
                RaceReward::Trophy,
            ),
            (
                LevelID::CortexCastle,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::GreenCtrToken,
            ),
            (
                LevelID::CortexCastle,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::CortexCastle,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::CortexCastle,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (LevelID::NGinLabs, RaceType::TrophyRace, RaceReward::Trophy),
            (
                LevelID::NGinLabs,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::BlueCtrToken,
            ),
            (
                LevelID::NGinLabs,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::NGinLabs,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::NGinLabs,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (
                LevelID::HotAirSkyway,
                RaceType::TrophyRace,
                RaceReward::Trophy,
            ),
            (
                LevelID::HotAirSkyway,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::YellowCtrToken,
            ),
            (
                LevelID::HotAirSkyway,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::HotAirSkyway,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::HotAirSkyway,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (LevelID::HotAirSkyway, RaceType::BossRace, RaceReward::Key),
            (
                LevelID::OxideStation,
                RaceType::TrophyRace,
                RaceReward::Trophy,
            ),
            (
                LevelID::OxideStation,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::YellowCtrToken,
            ),
            (
                LevelID::OxideStation,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::OxideStation,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::OxideStation,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (
                LevelID::SlideColiseum,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::SlideColiseum,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::SlideColiseum,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (
                LevelID::TurboTrack,
                RaceType::RelicRaceSapphire,
                RaceReward::SapphireRelic,
            ),
            (
                LevelID::TurboTrack,
                RaceType::RelicRaceGold,
                RaceReward::GoldRelic,
            ),
            (
                LevelID::TurboTrack,
                RaceType::RelicRacePlatinum,
                RaceReward::PlatinumRelic,
            ),
            (
                LevelID::SkullRock,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::PurpleCtrToken,
            ),
            (
                LevelID::RampageRuins,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::PurpleCtrToken,
            ),
            (
                LevelID::RockyRoad,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::PurpleCtrToken,
            ),
            (
                LevelID::NitroCourt,
                RaceType::CtrOrCrystalChallenge,
                RaceReward::PurpleCtrToken,
            ),
            (LevelID::CupRed, RaceType::GemCup, RaceReward::RedGem),
            (LevelID::CupGreen, RaceType::GemCup, RaceReward::GreenGem),
            (LevelID::CupBlue, RaceType::GemCup, RaceReward::BlueGem),
            (LevelID::CupYellow, RaceType::GemCup, RaceReward::YellowGem),
            (LevelID::CupPurple, RaceType::GemCup, RaceReward::PurpleGem),
        ],
        settings: vec![
            (
                SettingID::RelicDifficulty,
                SettingValue::RelicDifficulty(RelicTime::SapphireTime),
            ),
            (SettingID::RelicNeedsPerfect, SettingValue::Boolean(false)),
            (
                SettingID::BossGarageRequirements,
                SettingValue::BossGarageRequirements(BossGarageRequirements::Original4Tracks),
            ),
            (SettingID::QolSkipMaskhints, SettingValue::Boolean(true)),
            (SettingID::QolSkipPodium, SettingValue::Boolean(false)),
            (SettingID::QolSkipMaskcongrats, SettingValue::Boolean(false)),
            (
                SettingID::OxideRequiredRelics,
                SettingValue::OxideRequiredRelics(FinalOxideUnlock::SappireRelics18),
            ),
            (SettingID::SeedHash1, SettingValue::SeedHashPart(0x0000)),
            (SettingID::SeedHash2, SettingValue::SeedHashPart(0x0000)),
        ],
    }
}

pub fn get_randomized_game(seed: ChaCha8Rng, seed_as_number: u32, chosen_settings: &SeedSettings) -> RandomizedGame {
    let vanilla_game = get_vanilla_game();
    let mut new_warppads = vanilla_game.warppad_links;
    let mut new_warppad_unlocks = vanilla_game.warppad_unlocks;
    let mut new_race_rewards = vanilla_game.race_rewards;

    let overwrite_seed_hash_1;
    let overwrite_seed_hash_2;

    if chosen_settings.randomization.shuffle_adventure {
        overwrite_seed_hash_1 = (seed_as_number >> 16) as u16;
        overwrite_seed_hash_2 = (seed_as_number & 0xFFFF) as u16;

        // Warppads
        if let Some(warppad_shuffle) = &chosen_settings.randomization.warppad_shuffle {
            new_warppads = get_shuffled_warppads(
                seed,
                new_warppads,
                warppad_shuffle.include_battle_arenas,
                warppad_shuffle.include_gem_cups,
            );
        }

        // Warppad Unlocks
        // Race Rewards
    } else {
        overwrite_seed_hash_1 = 0u16;
        overwrite_seed_hash_2 = 0u16;
    }

    //todo sort relic from best item to worst, sapphire to plat?

    RandomizedGame {
        warppad_links: new_warppads,
        warppad_unlocks: new_warppad_unlocks,
        race_rewards: new_race_rewards,
        settings: vec![
            (
                SettingID::RelicDifficulty,
                SettingValue::RelicDifficulty(chosen_settings.general.rr_required_minimum_time),
            ),
            (
                SettingID::RelicNeedsPerfect,
                SettingValue::Boolean(chosen_settings.general.rr_require_perfects),
            ),
            (
                SettingID::BossGarageRequirements,
                SettingValue::BossGarageRequirements(
                    chosen_settings.randomization.bossgarage_unlock_requirements,
                ),
            ),
            (
                SettingID::QolSkipMaskhints,
                SettingValue::Boolean(chosen_settings.qol.skip_mask_hints),
            ),
            (
                SettingID::QolSkipPodium,
                SettingValue::Boolean(chosen_settings.qol.autoskip_podium_cutscenes),
            ),
            (
                SettingID::QolSkipMaskcongrats,
                SettingValue::Boolean(chosen_settings.qol.skip_mask_congrats),
            ),
            (
                SettingID::OxideRequiredRelics,
                SettingValue::OxideRequiredRelics(
                    chosen_settings.general.oxide_final_challenge_unlock,
                ),
            ),
            (
                SettingID::SeedHash1,
                SettingValue::SeedHashPart(overwrite_seed_hash_1),
            ),
            (
                SettingID::SeedHash2,
                SettingValue::SeedHashPart(overwrite_seed_hash_2),
            ),
        ],
    }
}

fn get_shuffled_warppads(
    mut seed: ChaCha8Rng,
    original_warppads: HashMap<LevelID, LevelID>,
    include_battle_arenas: bool,
    include_gem_cups: bool
) -> HashMap<LevelID, LevelID> {
    let mut randomized_levels: HashMap<LevelID, LevelID> = original_warppads.clone();

    let mut untouched_levels: HashMap<LevelID, LevelID> = HashMap::new();

    if !include_battle_arenas {
        for level_key in original_warppads.keys() {
            if *level_key >= LevelID::NitroCourt && *level_key <= LevelID::RockyRoad {
                untouched_levels.insert(*level_key, *level_key);
            }
        }
    }

    if !include_gem_cups {
        for level_key in original_warppads.keys() {
            if *level_key >= LevelID::CupRed && *level_key <= LevelID::CupPurple {
                untouched_levels.insert(*level_key, *level_key);
            }
        }
    }

    for level_key in untouched_levels.keys() {
        let _ = randomized_levels.remove(level_key);
    }

    let randomized_levels_clone = randomized_levels.clone();
    let mut level_values: Vec<&LevelID> = randomized_levels_clone.values().collect::<Vec<_>>();
    level_values.sort();
    level_values.shuffle(&mut seed);

    let mut level_keys: Vec<&LevelID>  = randomized_levels_clone.keys().collect();
    level_keys.sort();
    for level_key in level_keys {
        randomized_levels.insert(*level_key, *level_values.pop().expect("Same size as target vec."));
    }

    randomized_levels.extend(untouched_levels);

    randomized_levels
}

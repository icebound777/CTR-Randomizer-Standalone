use std::collections::HashMap;

use crate::seed_generation::randomization_datastructures::{LevelID, RaceReward, RaceType, RequiredItem, UnlockRequirement, UnlockStage};

#[derive(Debug)]
pub struct GameWorld {
    hub_1: GenericHub,
    hub_2: GenericHub,
    hub_3: GenericHub,
    hub_4: GenericHub,
    gemstone_valley: GemStoneValleyHub,
}

impl GameWorld {
    pub fn get_warppad_links(&self) -> HashMap<LevelID, LevelID> {
        HashMap::from([
            // Hub 1 - N. Sanity Beach
            (LevelID::CrashCove, self.hub_1.warppad_1.level_id),
            (LevelID::RoosTubes, self.hub_1.warppad_2.level_id),
            (LevelID::MysteryCaves, self.hub_1.warppad_3.level_id),
            (LevelID::SewerSpeedway, self.hub_1.warppad_4.level_id),
            (LevelID::SkullRock, self.hub_1.warppad_arena.level_id),

            // Hub 2 - Lost Temple
            (LevelID::CocoPark, self.hub_2.warppad_1.level_id),
            (LevelID::TigerTemple, self.hub_2.warppad_2.level_id),
            (LevelID::PapusPyramid, self.hub_2.warppad_3.level_id),
            (LevelID::DingoCanyon, self.hub_2.warppad_4.level_id),
            (LevelID::RampageRuins, self.hub_2.warppad_arena.level_id),

            // Hub 3 - Glacial Park
            (LevelID::BlizzardBluff, self.hub_3.warppad_1.level_id),
            (LevelID::DragonMines, self.hub_3.warppad_2.level_id),
            (LevelID::PolarPass, self.hub_3.warppad_3.level_id),
            (LevelID::TinyArena, self.hub_3.warppad_4.level_id),
            (LevelID::RockyRoad, self.hub_3.warppad_arena.level_id),

            // Hub 4 - Citadel City
            (LevelID::NGinLabs, self.hub_4.warppad_1.level_id),
            (LevelID::CortexCastle, self.hub_4.warppad_2.level_id),
            (LevelID::HotAirSkyway, self.hub_4.warppad_3.level_id),
            (LevelID::OxideStation, self.hub_4.warppad_4.level_id),
            (LevelID::NitroCourt, self.hub_4.warppad_arena.level_id),

            // Hub 5? - Gem Stone Valley
            (LevelID::TurboTrack, self.gemstone_valley.warppad_1.level_id),
            (LevelID::SlideColiseum, self.gemstone_valley.warppad_2.level_id),
            (LevelID::CupRed, self.gemstone_valley.cup_warppad_1.level_id),
            (LevelID::CupGreen, self.gemstone_valley.cup_warppad_2.level_id),
            (LevelID::CupBlue, self.gemstone_valley.cup_warppad_3.level_id),
            (LevelID::CupYellow, self.gemstone_valley.cup_warppad_4.level_id),
            (LevelID::CupPurple, self.gemstone_valley.cup_warppad_5.level_id),
        ])
    }

    pub fn get_warppad_unlocks(&self) -> Vec<(LevelID, UnlockStage, Option<UnlockRequirement>)> {
        fn add_single_warppad_unlocks(all_unlocks: &mut Vec<(LevelID, UnlockStage, Option<UnlockRequirement>)>, warp_pad: WarpPad) {
            all_unlocks.push((warp_pad.level_id, UnlockStage::One, warp_pad.unlock_1.requirement));
            if warp_pad.unlock_2.is_some() {
                all_unlocks.push((warp_pad.level_id, UnlockStage::Two, warp_pad.unlock_2.expect("checked by if").requirement));
            }
        }

        let mut warppad_unlocks = Vec::new();

        // Hub 1 - N. Sanity Beach
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_1.warppad_1);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_1.warppad_2);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_1.warppad_3);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_1.warppad_4);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_1.warppad_arena);

        // Hub 2 - Lost Temple
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_2.warppad_1);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_2.warppad_2);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_2.warppad_3);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_2.warppad_4);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_2.warppad_arena);

        // Hub 3 - Glacial Park
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_3.warppad_1);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_3.warppad_2);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_3.warppad_3);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_3.warppad_4);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_3.warppad_arena);

        // Hub 4 - Citadel City
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_4.warppad_1);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_4.warppad_2);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_4.warppad_3);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_4.warppad_4);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.hub_4.warppad_arena);

        // Hub 5? - Gem Stone Valley
        add_single_warppad_unlocks(&mut warppad_unlocks, self.gemstone_valley.warppad_1);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.gemstone_valley.warppad_2);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.gemstone_valley.cup_warppad_1);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.gemstone_valley.cup_warppad_2);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.gemstone_valley.cup_warppad_3);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.gemstone_valley.cup_warppad_4);
        add_single_warppad_unlocks(&mut warppad_unlocks, self.gemstone_valley.cup_warppad_5);

        warppad_unlocks
    }

    pub fn get_race_rewards(&self) -> Vec<(LevelID, RaceType, RaceReward)> {
        fn add_single_warppad_rewards(all_warppad_rewards: &mut Vec<(LevelID, RaceType, RaceReward)>, warp_pad: WarpPad) {
            fn add_single_reward(all_warppad_rewards: &mut Vec<(LevelID, RaceType, RaceReward)>, level: LevelID, unlock: RaceUnlock) {
                match unlock.reward {
                    Rewards::TrophyRaceRewards(TrophyRaceRewards { trophy_reward: rew }) => {
                        all_warppad_rewards.push((level, RaceType::TrophyRace, rew));
                    },
                    Rewards::TokensAndRelicRewards(TokensAndRelicRewards {
                        token_reward,
                        relic_sapphire_reward,
                        relic_gold_reward,
                        relic_platinum_reward
                    }) => {
                        all_warppad_rewards.push((level, RaceType::CtrOrCrystalChallenge, token_reward));
                        all_warppad_rewards.push((level, RaceType::RelicRaceSapphire, relic_sapphire_reward));
                        all_warppad_rewards.push((level, RaceType::RelicRaceGold, relic_gold_reward));
                        all_warppad_rewards.push((level, RaceType::RelicRacePlatinum, relic_platinum_reward));
                    },
                    Rewards::BattleArenaRewards(BattleArenaRewards { single_reward: rew}) => {
                        all_warppad_rewards.push((level, RaceType::CtrOrCrystalChallenge, rew));
                    },
                    Rewards::BossRaceRewards(BossRaceRewards { single_reward: rew }) => {
                        all_warppad_rewards.push((level, RaceType::BossRace, rew));
                    },
                    Rewards::RelicRaceOnlyRewards(RelicRaceOnlyRewards {
                        relic_sapphire_reward,
                        relic_gold_reward,
                        relic_platinum_reward
                    }) => {
                        all_warppad_rewards.push((level, RaceType::RelicRaceSapphire, relic_sapphire_reward));
                        all_warppad_rewards.push((level, RaceType::RelicRaceGold, relic_gold_reward));
                        all_warppad_rewards.push((level, RaceType::RelicRacePlatinum, relic_platinum_reward));
                    },
                    Rewards::GemCupRewards(GemCupRewards { single_reward: rew }) => {
                        all_warppad_rewards.push((level, RaceType::GemCup, rew));
                    }
                };
            }

            add_single_reward(all_warppad_rewards, warp_pad.level_id,  warp_pad.unlock_1);
            if warp_pad.unlock_2.is_some() {
                add_single_reward(all_warppad_rewards, warp_pad.level_id,  warp_pad.unlock_2.expect("checked by if"));
            }
        }

        let mut race_rewards = Vec::new();

        // Hub 1 - N. Sanity Beach
        add_single_warppad_rewards(&mut race_rewards, self.hub_1.warppad_1);
        add_single_warppad_rewards(&mut race_rewards, self.hub_1.warppad_2);
        add_single_warppad_rewards(&mut race_rewards, self.hub_1.warppad_3);
        add_single_warppad_rewards(&mut race_rewards, self.hub_1.warppad_4);
        add_single_warppad_rewards(&mut race_rewards, self.hub_1.warppad_arena);
        if let Rewards::BossRaceRewards(BossRaceRewards { single_reward: rew }) = self.hub_1.boss_garage.reward {
            race_rewards.push((LevelID::RoosTubes, RaceType::BossRace, rew));
        };

        // Hub 2 - Lost Temple
        add_single_warppad_rewards(&mut race_rewards, self.hub_2.warppad_1);
        add_single_warppad_rewards(&mut race_rewards, self.hub_2.warppad_2);
        add_single_warppad_rewards(&mut race_rewards, self.hub_2.warppad_3);
        add_single_warppad_rewards(&mut race_rewards, self.hub_2.warppad_4);
        add_single_warppad_rewards(&mut race_rewards, self.hub_2.warppad_arena);
        if let Rewards::BossRaceRewards(BossRaceRewards { single_reward: rew }) = self.hub_2.boss_garage.reward {
            race_rewards.push((LevelID::PapusPyramid, RaceType::BossRace, rew));
        };

        // Hub 3 - Glacial Park
        add_single_warppad_rewards(&mut race_rewards, self.hub_3.warppad_1);
        add_single_warppad_rewards(&mut race_rewards, self.hub_3.warppad_2);
        add_single_warppad_rewards(&mut race_rewards, self.hub_3.warppad_3);
        add_single_warppad_rewards(&mut race_rewards, self.hub_3.warppad_4);
        add_single_warppad_rewards(&mut race_rewards, self.hub_3.warppad_arena);
        if let Rewards::BossRaceRewards(BossRaceRewards { single_reward: rew }) = self.hub_3.boss_garage.reward {
            race_rewards.push((LevelID::DragonMines, RaceType::BossRace, rew));
        };

        // Hub 4 - Citadel City
        add_single_warppad_rewards(&mut race_rewards, self.hub_4.warppad_1);
        add_single_warppad_rewards(&mut race_rewards, self.hub_4.warppad_2);
        add_single_warppad_rewards(&mut race_rewards, self.hub_4.warppad_3);
        add_single_warppad_rewards(&mut race_rewards, self.hub_4.warppad_4);
        add_single_warppad_rewards(&mut race_rewards, self.hub_4.warppad_arena);
        if let Rewards::BossRaceRewards(BossRaceRewards { single_reward: rew }) = self.hub_4.boss_garage.reward {
            race_rewards.push((LevelID::HotAirSkyway, RaceType::BossRace, rew));
        };

        // Hub 5? - Gem Stone Valley
        add_single_warppad_rewards(&mut race_rewards, self.gemstone_valley.warppad_1);
        add_single_warppad_rewards(&mut race_rewards, self.gemstone_valley.warppad_2);
        add_single_warppad_rewards(&mut race_rewards, self.gemstone_valley.cup_warppad_1);
        add_single_warppad_rewards(&mut race_rewards, self.gemstone_valley.cup_warppad_2);
        add_single_warppad_rewards(&mut race_rewards, self.gemstone_valley.cup_warppad_3);
        add_single_warppad_rewards(&mut race_rewards, self.gemstone_valley.cup_warppad_4);
        add_single_warppad_rewards(&mut race_rewards, self.gemstone_valley.cup_warppad_5);

        race_rewards
    }
}

#[derive(Debug)]
pub struct GenericHub {
    requirement: Option<UnlockRequirement>,
    warppad_1: WarpPad,
    warppad_2: WarpPad,
    warppad_3: WarpPad,
    warppad_4: WarpPad,
    boss_garage: BossGarage,
    warppad_arena: WarpPad,
}

#[derive(Debug)]
pub struct GemStoneValleyHub {
    requirement: Option<UnlockRequirement>,
    warppad_1: WarpPad,
    warppad_2: WarpPad,
    cup_warppad_1: WarpPad,
    cup_warppad_2: WarpPad,
    cup_warppad_3: WarpPad,
    cup_warppad_4: WarpPad,
    cup_warppad_5: WarpPad,
    boss_garage: BossGarage,
}

#[derive(Debug, Clone, Copy)]
pub struct WarpPad {
    level_id: LevelID,
    unlock_1: RaceUnlock,
    unlock_2: Option<RaceUnlock>,
}

#[derive(Debug)]
pub struct BossGarage {
    requirement: Vec<UnlockRequirement>,
    reward: Rewards,
}

#[derive(Debug, Clone, Copy)]
pub struct RaceUnlock {
    requirement: Option<UnlockRequirement>,
    reward: Rewards,
}

#[derive(Debug, Clone, Copy)]
pub enum Rewards {
    TrophyRaceRewards(TrophyRaceRewards),
    TokensAndRelicRewards(TokensAndRelicRewards),
    BattleArenaRewards(BattleArenaRewards),
    BossRaceRewards(BossRaceRewards),
    RelicRaceOnlyRewards(RelicRaceOnlyRewards),
    GemCupRewards(GemCupRewards),
}

#[derive(Debug, Clone, Copy)]
pub struct TrophyRaceRewards {
    trophy_reward: RaceReward,
}

#[derive(Debug, Clone, Copy)]
pub struct TokensAndRelicRewards {
    token_reward: RaceReward,
    relic_sapphire_reward: RaceReward,
    relic_gold_reward: RaceReward,
    relic_platinum_reward: RaceReward,
}

#[derive(Debug, Clone, Copy)]
pub struct BattleArenaRewards {
    single_reward: RaceReward,
}

#[derive(Debug, Clone, Copy)]
pub struct BossRaceRewards {
    single_reward: RaceReward,
}

#[derive(Debug, Clone, Copy)]
pub struct RelicRaceOnlyRewards {
    relic_sapphire_reward: RaceReward,
    relic_gold_reward: RaceReward,
    relic_platinum_reward: RaceReward,
}

#[derive(Debug, Clone, Copy)]
pub struct GemCupRewards {
    single_reward: RaceReward,
}

pub fn get_vanilla_gameworld() -> GameWorld {
    GameWorld {
        hub_1: GenericHub {
            requirement: None,
            warppad_1: WarpPad {
                level_id: LevelID::CrashCove,
                unlock_1: RaceUnlock {
                    requirement: None,
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 1 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::RedCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            warppad_2: WarpPad {
                level_id: LevelID::RoosTubes,
                unlock_1: RaceUnlock {
                    requirement: None,
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 1 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::GreenCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            warppad_3: WarpPad {
                level_id: LevelID::MysteryCaves,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Trophy, count: 1 }),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 1 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::RedCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            warppad_4: WarpPad {
                level_id: LevelID::SewerSpeedway,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Trophy, count: 3 }),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 1 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::BlueCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            boss_garage: BossGarage {
                requirement: vec![
                    UnlockRequirement {
                        item_type: RequiredItem::Trophy,
                        count: 1
                    },
                    UnlockRequirement {
                        item_type: RequiredItem::Trophy,
                        count: 3
                    }
                ],
                reward: Rewards::BossRaceRewards(BossRaceRewards {
                    single_reward: RaceReward::Key
                })
            },
            warppad_arena: WarpPad {
                level_id: LevelID::SkullRock,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 1 }),
                    reward: Rewards::BattleArenaRewards(BattleArenaRewards {
                        single_reward: RaceReward::PurpleCtrToken
                    })
                },
                unlock_2: None,
            },
        },
        hub_2: GenericHub {
            requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 1 }),
            warppad_1: WarpPad {
                level_id: LevelID::CocoPark,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::Trophy, count: 4}),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 2 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::GreenCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            warppad_2: WarpPad {
                level_id: LevelID::TigerTemple,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::Trophy, count: 4}),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 2 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::BlueCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            warppad_3: WarpPad {
                level_id: LevelID::PapusPyramid,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::Trophy, count: 6}),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 2 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::RedCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            warppad_4: WarpPad {
                level_id: LevelID::DingoCanyon,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::Trophy, count: 7}),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 2 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::YellowCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            boss_garage: BossGarage {
                requirement: vec![
                    UnlockRequirement { item_type: RequiredItem::Trophy, count: 4 },
                    UnlockRequirement { item_type: RequiredItem::Trophy, count: 4 },
                    UnlockRequirement { item_type: RequiredItem::Trophy, count: 6 },
                    UnlockRequirement { item_type: RequiredItem::Trophy, count: 7 }
                ],
                reward: Rewards::BossRaceRewards(BossRaceRewards { single_reward: RaceReward::Key })
            },
            warppad_arena: WarpPad {
                level_id: LevelID::RampageRuins,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 2 }),
                    reward: Rewards::BattleArenaRewards(BattleArenaRewards {
                        single_reward: RaceReward::PurpleCtrToken
                    })
                },
                unlock_2: None,
            }
        },
        hub_3: GenericHub {
            requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 2 }),
            warppad_1: WarpPad {
                level_id: LevelID::BlizzardBluff,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::Trophy, count: 8}),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 3 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::RedCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            warppad_2: WarpPad {
                level_id: LevelID::DragonMines,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::Trophy, count: 9}),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 3 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::BlueCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            warppad_3: WarpPad {
                level_id: LevelID::PolarPass,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::Trophy, count: 10}),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 3 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::GreenCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            warppad_4: WarpPad {
                level_id: LevelID::TinyArena,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::Trophy, count: 11}),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 3 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::YellowCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            boss_garage: BossGarage {
                requirement: vec![
                    UnlockRequirement { item_type: RequiredItem::Trophy, count: 8 },
                    UnlockRequirement { item_type: RequiredItem::Trophy, count: 9 },
                    UnlockRequirement { item_type: RequiredItem::Trophy, count: 10 },
                    UnlockRequirement { item_type: RequiredItem::Trophy, count: 11 }
                ],
                reward: Rewards::BossRaceRewards(BossRaceRewards { single_reward: RaceReward::Key })
            },
            warppad_arena: WarpPad {
                level_id: LevelID::RockyRoad,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 3 }),
                    reward: Rewards::BattleArenaRewards(BattleArenaRewards {
                        single_reward: RaceReward::PurpleCtrToken
                    })
                },
                unlock_2: None,
            }
        },
        hub_4: GenericHub {
            requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 3 }),
            warppad_1: WarpPad {
                level_id: LevelID::NGinLabs,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::Trophy, count: 12}),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 4 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::BlueCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            warppad_2: WarpPad {
                level_id: LevelID::CortexCastle,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::Trophy, count: 12}),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 4 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::GreenCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            warppad_3: WarpPad {
                level_id: LevelID::HotAirSkyway,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::Trophy, count: 14}),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 4 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::YellowCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            warppad_4: WarpPad {
                level_id: LevelID::OxideStation,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::Trophy, count: 15}),
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards {
                        trophy_reward: RaceReward::Trophy
                    })
                },
                unlock_2: Some(RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 4 }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: RaceReward::YellowCtrToken,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
            boss_garage: BossGarage {
                requirement: vec![
                    UnlockRequirement { item_type: RequiredItem::Trophy, count: 12 },
                    UnlockRequirement { item_type: RequiredItem::Trophy, count: 12 },
                    UnlockRequirement { item_type: RequiredItem::Trophy, count: 14 },
                    UnlockRequirement { item_type: RequiredItem::Trophy, count: 15 }
                ],
                reward: Rewards::BossRaceRewards(BossRaceRewards { single_reward: RaceReward::Key })
            },
            warppad_arena: WarpPad {
                level_id: LevelID::NitroCourt,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 4 }),
                    reward: Rewards::BattleArenaRewards(BattleArenaRewards {
                        single_reward: RaceReward::PurpleCtrToken
                    })
                },
                unlock_2: None,
            }
        },
        gemstone_valley: GemStoneValleyHub {
            requirement: Some(UnlockRequirement { item_type: RequiredItem::Key, count: 1 }),
            warppad_1: WarpPad {
                level_id: LevelID::TurboTrack,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::AnyGem, count: 5}),
                    reward: Rewards::RelicRaceOnlyRewards(RelicRaceOnlyRewards {
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic
                    })
                },
                unlock_2: None
            },
            warppad_2: WarpPad {
                level_id: LevelID::SlideColiseum,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::SapphireRelic, count: 10}),
                    reward: Rewards::RelicRaceOnlyRewards(RelicRaceOnlyRewards {
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic
                    })
                },
                unlock_2: None
            },
            cup_warppad_1: WarpPad {
                level_id: LevelID::CupRed,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::RedCtrToken, count: 4}),
                    reward: Rewards::GemCupRewards(GemCupRewards {
                        single_reward: RaceReward::RedGem
                    })
                },
                unlock_2: None
            },
            cup_warppad_2: WarpPad {
                level_id: LevelID::CupGreen,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::GreenCtrToken, count: 4}),
                    reward: Rewards::GemCupRewards(GemCupRewards {
                        single_reward: RaceReward::GreenGem
                    })
                },
                unlock_2: None
            },
            cup_warppad_3: WarpPad {
                level_id: LevelID::CupBlue,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::BlueCtrToken, count: 4}),
                    reward: Rewards::GemCupRewards(GemCupRewards {
                        single_reward: RaceReward::BlueGem
                    })
                },
                unlock_2: None
            },
            cup_warppad_4: WarpPad {
                level_id: LevelID::CupYellow,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::YellowCtrToken, count: 4}),
                    reward: Rewards::GemCupRewards(GemCupRewards {
                        single_reward: RaceReward::YellowGem
                    })
                },
                unlock_2: None
            },
            cup_warppad_5: WarpPad {
                level_id: LevelID::CupPurple,
                unlock_1: RaceUnlock {
                    requirement: Some(UnlockRequirement{item_type: RequiredItem::PurpleCtrToken, count: 4}),
                    reward: Rewards::GemCupRewards(GemCupRewards {
                        single_reward: RaceReward::PurpleGem
                    })
                },
                unlock_2: None
            },
            boss_garage: BossGarage {
                requirement: vec![
                    UnlockRequirement { item_type: RequiredItem::Key, count: 4 }
                ],
                reward: Rewards::BossRaceRewards(BossRaceRewards { single_reward: RaceReward::BeatTheGame })
            },
        },
    }
}

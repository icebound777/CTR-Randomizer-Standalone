use std::collections::HashMap;

use crate::seed_generation::randomization_datastructures::{LevelID, RaceReward, RaceType, RequiredItem, UnlockRequirement, UnlockRequirementItem, UnlockStage};

#[derive(Debug, Clone)]
pub struct GameWorld {
    pub hub_1: GenericHub,
    pub hub_2: GenericHub,
    pub hub_3: GenericHub,
    pub hub_4: GenericHub,
    pub gemstone_valley: GemStoneValleyHub,
}

impl GameWorld {
    pub fn get_warppad_links(&self) -> HashMap<LevelID, LevelID> {
        HashMap::from([
            // Hub 1 - N. Sanity Beach
            (self.hub_1.warppad_1.original_level_id, self.hub_1.warppad_1.current_level_id),
            (self.hub_1.warppad_2.original_level_id, self.hub_1.warppad_2.current_level_id),
            (self.hub_1.warppad_3.original_level_id, self.hub_1.warppad_3.current_level_id),
            (self.hub_1.warppad_4.original_level_id, self.hub_1.warppad_4.current_level_id),
            (self.hub_1.warppad_arena.original_level_id, self.hub_1.warppad_arena.current_level_id),

            // Hub 2 - Lost Temple
            (self.hub_2.warppad_1.original_level_id, self.hub_2.warppad_1.current_level_id),
            (self.hub_2.warppad_2.original_level_id, self.hub_2.warppad_2.current_level_id),
            (self.hub_2.warppad_3.original_level_id, self.hub_2.warppad_3.current_level_id),
            (self.hub_2.warppad_4.original_level_id, self.hub_2.warppad_4.current_level_id),
            (self.hub_2.warppad_arena.original_level_id, self.hub_2.warppad_arena.current_level_id),

            // Hub 3 - Glacial Park
            (self.hub_3.warppad_1.original_level_id, self.hub_3.warppad_1.current_level_id),
            (self.hub_3.warppad_2.original_level_id, self.hub_3.warppad_2.current_level_id),
            (self.hub_3.warppad_3.original_level_id, self.hub_3.warppad_3.current_level_id),
            (self.hub_3.warppad_4.original_level_id, self.hub_3.warppad_4.current_level_id),
            (self.hub_3.warppad_arena.original_level_id, self.hub_3.warppad_arena.current_level_id),

            // Hub 4 - Citadel City
            (self.hub_4.warppad_1.original_level_id, self.hub_4.warppad_1.current_level_id),
            (self.hub_4.warppad_2.original_level_id, self.hub_4.warppad_2.current_level_id),
            (self.hub_4.warppad_3.original_level_id, self.hub_4.warppad_3.current_level_id),
            (self.hub_4.warppad_4.original_level_id, self.hub_4.warppad_4.current_level_id),
            (self.hub_4.warppad_arena.original_level_id, self.hub_4.warppad_arena.current_level_id),

            // Hub 5? - Gem Stone Valley
            (self.gemstone_valley.warppad_1.original_level_id, self.gemstone_valley.warppad_1.current_level_id),
            (self.gemstone_valley.warppad_2.original_level_id, self.gemstone_valley.warppad_2.current_level_id),
            (self.gemstone_valley.cup_warppad_1.original_level_id, self.gemstone_valley.cup_warppad_1.current_level_id),
            (self.gemstone_valley.cup_warppad_2.original_level_id, self.gemstone_valley.cup_warppad_2.current_level_id),
            (self.gemstone_valley.cup_warppad_3.original_level_id, self.gemstone_valley.cup_warppad_3.current_level_id),
            (self.gemstone_valley.cup_warppad_4.original_level_id, self.gemstone_valley.cup_warppad_4.current_level_id),
            (self.gemstone_valley.cup_warppad_5.original_level_id, self.gemstone_valley.cup_warppad_5.current_level_id),
        ])
    }

    pub fn get_warppad_unlocks(&self) -> HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>> {
        fn add_single_warppad_unlocks(
            all_unlocks: &mut HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>>,
            warp_pad: WarpPad
        ) {
            all_unlocks.insert((warp_pad.current_level_id, UnlockStage::One), warp_pad.unlock_1.requirement);
            if warp_pad.unlock_2.is_some() {
                all_unlocks.insert(
                    (warp_pad.current_level_id, UnlockStage::Two),
                    warp_pad.unlock_2.expect("checked by if").requirement
                );
            }
        }

        let mut warppad_unlocks = HashMap::new();

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
                }
            }

            add_single_reward(all_warppad_rewards, warp_pad.current_level_id,  warp_pad.unlock_1);
            if warp_pad.unlock_2.is_some() {
                add_single_reward(all_warppad_rewards, warp_pad.current_level_id,  warp_pad.unlock_2.expect("checked by if"));
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
        }

        // Hub 2 - Lost Temple
        add_single_warppad_rewards(&mut race_rewards, self.hub_2.warppad_1);
        add_single_warppad_rewards(&mut race_rewards, self.hub_2.warppad_2);
        add_single_warppad_rewards(&mut race_rewards, self.hub_2.warppad_3);
        add_single_warppad_rewards(&mut race_rewards, self.hub_2.warppad_4);
        add_single_warppad_rewards(&mut race_rewards, self.hub_2.warppad_arena);
        if let Rewards::BossRaceRewards(BossRaceRewards { single_reward: rew }) = self.hub_2.boss_garage.reward {
            race_rewards.push((LevelID::PapusPyramid, RaceType::BossRace, rew));
        }

        // Hub 3 - Glacial Park
        add_single_warppad_rewards(&mut race_rewards, self.hub_3.warppad_1);
        add_single_warppad_rewards(&mut race_rewards, self.hub_3.warppad_2);
        add_single_warppad_rewards(&mut race_rewards, self.hub_3.warppad_3);
        add_single_warppad_rewards(&mut race_rewards, self.hub_3.warppad_4);
        add_single_warppad_rewards(&mut race_rewards, self.hub_3.warppad_arena);
        if let Rewards::BossRaceRewards(BossRaceRewards { single_reward: rew }) = self.hub_3.boss_garage.reward {
            race_rewards.push((LevelID::DragonMines, RaceType::BossRace, rew));
        }

        // Hub 4 - Citadel City
        add_single_warppad_rewards(&mut race_rewards, self.hub_4.warppad_1);
        add_single_warppad_rewards(&mut race_rewards, self.hub_4.warppad_2);
        add_single_warppad_rewards(&mut race_rewards, self.hub_4.warppad_3);
        add_single_warppad_rewards(&mut race_rewards, self.hub_4.warppad_4);
        add_single_warppad_rewards(&mut race_rewards, self.hub_4.warppad_arena);
        if let Rewards::BossRaceRewards(BossRaceRewards { single_reward: rew }) = self.hub_4.boss_garage.reward {
            race_rewards.push((LevelID::HotAirSkyway, RaceType::BossRace, rew));
        }

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

    pub fn set_warppad_links(&mut self, new_links: HashMap<LevelID, LevelID>) {
        for (key, value) in new_links {
            match key {
                // Hub 1 - N. Sanity Beach
                LevelID::CrashCove => {self.hub_1.warppad_1 = WarpPad::new(LevelID::CrashCove, value)},
                LevelID::RoosTubes => {self.hub_1.warppad_2 = WarpPad::new(LevelID::RoosTubes, value)},
                LevelID::MysteryCaves => {self.hub_1.warppad_3 = WarpPad::new(LevelID::MysteryCaves, value)},
                LevelID::SewerSpeedway => {self.hub_1.warppad_4 = WarpPad::new(LevelID::SewerSpeedway, value)},
                LevelID::SkullRock => {self.hub_1.warppad_arena = WarpPad::new(LevelID::SkullRock, value)},

                // Hub 2 - Lost Temple
                LevelID::CocoPark => {self.hub_2.warppad_1 = WarpPad::new(LevelID::CocoPark, value)},
                LevelID::TigerTemple => {self.hub_2.warppad_2 = WarpPad::new(LevelID::TigerTemple, value)},
                LevelID::PapusPyramid => {self.hub_2.warppad_3 = WarpPad::new(LevelID::PapusPyramid, value)},
                LevelID::DingoCanyon => {self.hub_2.warppad_4 = WarpPad::new(LevelID::DingoCanyon, value)},
                LevelID::RampageRuins => {self.hub_2.warppad_arena = WarpPad::new(LevelID::RampageRuins, value)},

                // Hub 3 - Glacial Park
                LevelID::BlizzardBluff => {self.hub_3.warppad_1 = WarpPad::new(LevelID::BlizzardBluff, value)},
                LevelID::DragonMines => {self.hub_3.warppad_2 = WarpPad::new(LevelID::DragonMines, value)},
                LevelID::PolarPass => {self.hub_3.warppad_3 = WarpPad::new(LevelID::PolarPass, value)},
                LevelID::TinyArena => {self.hub_3.warppad_4 = WarpPad::new(LevelID::TinyArena, value)},
                LevelID::RockyRoad => {self.hub_3.warppad_arena = WarpPad::new(LevelID::RockyRoad, value)},

                // Hub 4 - Citadel City
                LevelID::NGinLabs => {self.hub_4.warppad_1 = WarpPad::new(LevelID::NGinLabs, value)},
                LevelID::CortexCastle => {self.hub_4.warppad_2 = WarpPad::new(LevelID::CortexCastle, value)},
                LevelID::HotAirSkyway => {self.hub_4.warppad_3 = WarpPad::new(LevelID::HotAirSkyway, value)},
                LevelID::OxideStation => {self.hub_4.warppad_4 = WarpPad::new(LevelID::OxideStation, value)},
                LevelID::NitroCourt => {self.hub_4.warppad_arena = WarpPad::new(LevelID::NitroCourt, value)},

                // Hub 5? - Gem Stone Valley
                LevelID::TurboTrack => {self.gemstone_valley.warppad_1 = WarpPad::new(LevelID::TurboTrack, value)},
                LevelID::SlideColiseum => {self.gemstone_valley.warppad_2 = WarpPad::new(LevelID::SlideColiseum, value)},
                LevelID::CupRed => {self.gemstone_valley.cup_warppad_1 = WarpPad::new(LevelID::CupRed, value)},
                LevelID::CupGreen => {self.gemstone_valley.cup_warppad_2 = WarpPad::new(LevelID::CupGreen, value)},
                LevelID::CupBlue => {self.gemstone_valley.cup_warppad_3 = WarpPad::new(LevelID::CupBlue, value)},
                LevelID::CupYellow => {self.gemstone_valley.cup_warppad_4 = WarpPad::new(LevelID::CupYellow, value)},
                LevelID::CupPurple => {self.gemstone_valley.cup_warppad_5 = WarpPad::new(LevelID::CupPurple, value)},
            }
        }
    }

    pub fn set_warppad_unlocks(
        &mut self,
        warppad_unlocks: HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>>
    ) {
        // For each tuple in the `warppad_unlocks`-vec, the LevelID does not reference
        // the actual warp pad level target for which we have to modify the warp pad,
        // but instead references the warp pad location in the adventure arena.
        // For example, if the LevelID equals LevelID::CrashCove, then we do not have
        // to modify the unlock requirements for the CrashCove warp pad no matter
        // where it is. Instead we have to modify the warp pad that is in the location
        // where originally CrashCove could be found (no matter which level that
        // warp pad sends the player to now).
        //
        // It can happen that the number of unlock stages differ between the pad we
        // have to modify and the unlock requirements we want to set (this mostly
        // happens when trying to set vanilla unlock requirements).
        // In this case, if the warp pad has only one unlock stage but we try to set
        // two, then we just drop the second unlock stage.
        // If the warp pad has two unlock stages but we only have one to set, then
        // we set the second stage requirements to equal the first stage requirements,
        // so both unlock stages can be served by that single requirement.
        let mut modifies_second_unlock: Vec<LevelID> = Vec::new();
        for ((levelid, stage), _) in warppad_unlocks.clone().into_iter() {
            if stage == UnlockStage::Two {
                modifies_second_unlock.push(levelid);
            }
        }

        for ((levelid, stage), unlock_req) in warppad_unlocks.into_iter() {
            let warppad_to_modify = match levelid {
                LevelID::CrashCove  => {
                    &mut self.hub_1.warppad_1
                },
                LevelID::RoosTubes  => {
                    &mut self.hub_1.warppad_2
                },
                LevelID::MysteryCaves  => {
                    &mut self.hub_1.warppad_3
                },
                LevelID::SewerSpeedway  => {
                    &mut self.hub_1.warppad_4
                },
                LevelID::SkullRock  => {
                    &mut self.hub_1.warppad_arena
                },
                LevelID::CocoPark  => {
                    &mut self.hub_2.warppad_1
                },
                LevelID::TigerTemple  => {
                    &mut self.hub_2.warppad_2
                },
                LevelID::PapusPyramid  => {
                    &mut self.hub_2.warppad_3
                },
                LevelID::DingoCanyon  => {
                    &mut self.hub_2.warppad_4
                },
                LevelID::RampageRuins  => {
                    &mut self.hub_2.warppad_arena
                },
                LevelID::BlizzardBluff  => {
                    &mut self.hub_3.warppad_1
                },
                LevelID::DragonMines  => {
                    &mut self.hub_3.warppad_2
                },
                LevelID::PolarPass  => {
                    &mut self.hub_3.warppad_3
                },
                LevelID::TinyArena  => {
                    &mut self.hub_3.warppad_4
                },
                LevelID::RockyRoad  => {
                    &mut self.hub_3.warppad_arena
                },
                LevelID::NGinLabs  => {
                    &mut self.hub_4.warppad_1
                },
                LevelID::CortexCastle  => {
                    &mut self.hub_4.warppad_2
                },
                LevelID::HotAirSkyway  => {
                    &mut self.hub_4.warppad_3
                },
                LevelID::OxideStation  => {
                    &mut self.hub_4.warppad_4
                },
                LevelID::NitroCourt  => {
                    &mut self.hub_4.warppad_arena
                },
                LevelID::TurboTrack  => {
                    &mut self.gemstone_valley.warppad_1
                },
                LevelID::SlideColiseum  => {
                    &mut self.gemstone_valley.warppad_2
                },
                LevelID::CupRed  => {
                    &mut self.gemstone_valley.cup_warppad_1
                },
                LevelID::CupGreen  => {
                    &mut self.gemstone_valley.cup_warppad_2
                },
                LevelID::CupBlue  => {
                    &mut self.gemstone_valley.cup_warppad_3
                },
                LevelID::CupYellow  => {
                    &mut self.gemstone_valley.cup_warppad_4
                },
                LevelID::CupPurple  => {
                    &mut self.gemstone_valley.cup_warppad_5
                },
            };

            match stage {
                UnlockStage::One => {
                    warppad_to_modify.set_unlock_1(unlock_req.expect("stage one should always exist"));

                    if warppad_to_modify.get_unlock_2().is_some() && !modifies_second_unlock.contains(&levelid) {
                        warppad_to_modify.set_unlock_2(unlock_req.expect("stage one should always exist"));
                    }
                },
                UnlockStage::Two => {
                    if warppad_to_modify.get_unlock_2().is_some() {
                        warppad_to_modify.set_unlock_2(unlock_req.expect("checked by if"));
                    }
                }
            }
        }
    }

    pub fn set_garage_unlocks(&mut self, garage_unlocks: HashMap<BossCharacter, UnlockRequirement>) {
        for (boss, req) in garage_unlocks {
            match boss {
                BossCharacter::RipperRoo => {
                    self.hub_1.boss_garage.requirement = req;
                },
                BossCharacter::PapuPapu => {
                    self.hub_2.boss_garage.requirement = req;
                },
                BossCharacter::KomodoJoe => {
                    self.hub_3.boss_garage.requirement = req;
                },
                BossCharacter::Pinstripe => {
                    self.hub_4.boss_garage.requirement = req;
                },
                BossCharacter::NOxide => {
                    self.gemstone_valley.boss_garage.requirement = req;
                },
            };
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericHub {
    requirement: Option<UnlockRequirementItem>,
    pub warppad_1: WarpPad,
    pub warppad_2: WarpPad,
    pub warppad_3: WarpPad,
    pub warppad_4: WarpPad,
    pub boss_garage: BossGarage,
    pub warppad_arena: WarpPad,
}

impl GenericHub {
    pub fn get_requirement(&self) -> Option<UnlockRequirementItem> {
        self.requirement
    }
}

#[derive(Debug, Clone)]
pub struct GemStoneValleyHub {
    requirement: Option<UnlockRequirementItem>,
    pub warppad_1: WarpPad,
    pub warppad_2: WarpPad,
    pub cup_warppad_1: WarpPad,
    pub cup_warppad_2: WarpPad,
    pub cup_warppad_3: WarpPad,
    pub cup_warppad_4: WarpPad,
    pub cup_warppad_5: WarpPad,
    pub boss_garage: BossGarage,
}

impl GemStoneValleyHub {
    pub fn get_requirement(&self) -> Option<UnlockRequirementItem> {
        self.requirement
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WarpPad {
    original_level_id: LevelID,
    current_level_id: LevelID,
    unlock_1: RaceUnlock,
    unlock_2: Option<RaceUnlock>,
}

impl WarpPad {
    pub fn new(original_level_id: LevelID, current_level_id: LevelID) -> Self {
        let unlock_1 = match current_level_id {
            // Cups
            LevelID::CupRed => {
                RaceUnlock {
                    requirement: Some(UnlockRequirementItem { item_type:RequiredItem::RedCtrToken, count: 4 }),
                    reward: Rewards::GemCupRewards(GemCupRewards { single_reward:RaceReward::RedGem })
                }
            },
            LevelID::CupGreen => {
                RaceUnlock {
                    requirement: Some(UnlockRequirementItem { item_type:RequiredItem::GreenCtrToken, count: 4 }),
                    reward: Rewards::GemCupRewards(GemCupRewards { single_reward:RaceReward::GreenGem })
                }
            },
            LevelID::CupBlue => {
                RaceUnlock {
                    requirement: Some(UnlockRequirementItem { item_type:RequiredItem::BlueCtrToken, count: 4 }),
                    reward: Rewards::GemCupRewards(GemCupRewards { single_reward:RaceReward::BlueGem })
                }
            },
            LevelID::CupYellow => {
                RaceUnlock {
                    requirement: Some(UnlockRequirementItem { item_type:RequiredItem::YellowCtrToken, count: 4 }),
                    reward: Rewards::GemCupRewards(GemCupRewards { single_reward:RaceReward::YellowGem })
                }
            },
            LevelID::CupPurple => {
                RaceUnlock {
                    requirement: Some(UnlockRequirementItem { item_type:RequiredItem::PurpleCtrToken, count: 4 }),
                    reward: Rewards::GemCupRewards(GemCupRewards { single_reward:RaceReward::PurpleGem })
                }
            },
            // Battle Arenas
            x @ (LevelID::SkullRock | LevelID::RampageRuins | LevelID::RockyRoad | LevelID::NitroCourt) => {
                RaceUnlock {
                    requirement: Some(UnlockRequirementItem {
                        item_type: RequiredItem::Key,
                        count: if x == LevelID::SkullRock {
                            1
                        } else if x == LevelID::RampageRuins {
                            2
                        } else if x == LevelID::RockyRoad {
                            3
                        } else {
                            4
                        }
                    }),
                    reward: Rewards::GemCupRewards(GemCupRewards { single_reward:RaceReward::PurpleCtrToken })
                }
            },
            // Relic Race only tracks
            x @ (LevelID::TurboTrack | LevelID::SlideColiseum) => {
                RaceUnlock {
                    requirement: if x == LevelID::TurboTrack {
                        Some(UnlockRequirementItem { item_type:RequiredItem::AnyGem, count: 5 })
                    } else {
                        Some(UnlockRequirementItem { item_type:RequiredItem::SapphireRelic, count: 10 })
                    },
                    reward: Rewards::RelicRaceOnlyRewards(RelicRaceOnlyRewards {
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic
                    })
                }
            },
            // Trophy tracks
            x => {
                RaceUnlock {
                    reward: Rewards::TrophyRaceRewards(TrophyRaceRewards { trophy_reward: RaceReward::Trophy }),
                    requirement: Some(UnlockRequirementItem { item_type: RequiredItem::Trophy, count:
                        if x == LevelID::CrashCove || x == LevelID::RoosTubes {
                            0
                        } else if x == LevelID::MysteryCaves {
                            1
                        } else if x == LevelID::SewerSpeedway {
                            3
                        } else if x == LevelID::CocoPark || x == LevelID::TigerTemple {
                            4
                        } else if x == LevelID::PapusPyramid {
                            6
                        } else if x == LevelID::DingoCanyon {
                            7
                        } else if x == LevelID::BlizzardBluff {
                            8
                        } else if x == LevelID::DragonMines {
                            9
                        } else if x == LevelID::PolarPass {
                            10
                        } else if x == LevelID::TinyArena {
                            11
                        } else if x == LevelID::NGinLabs || x == LevelID::CortexCastle {
                            12
                        } else if x == LevelID::HotAirSkyway {
                            14
                        } else { //if x == LevelID::OxideStation {
                            15
                        }
                    })
                }
            }
        };

        let unlock_2 = match current_level_id {
            // Gem Cups, Battle Arenas, TT / SC
            LevelID::SlideColiseum | LevelID::TurboTrack | LevelID::SkullRock | LevelID::RampageRuins | LevelID::RockyRoad | LevelID::NitroCourt | LevelID::CupRed | LevelID::CupGreen | LevelID::CupBlue | LevelID::CupYellow | LevelID::CupPurple => None,

            x => {
                let key_count: u8 = if x == LevelID::CrashCove || x == LevelID::RoosTubes || x == LevelID::MysteryCaves || x == LevelID::SewerSpeedway {
                    1
                } else if x == LevelID::CocoPark || x == LevelID::TigerTemple || x == LevelID::PapusPyramid || x == LevelID::DingoCanyon {
                    2
                } else if x == LevelID::BlizzardBluff || x == LevelID::DragonMines || x == LevelID::PolarPass || x == LevelID::TinyArena {
                    3
                } else {
                    4
                };

                let token_color: RaceReward = if x == LevelID::CrashCove || x == LevelID::MysteryCaves || x == LevelID::PapusPyramid || x == LevelID::BlizzardBluff {
                    RaceReward::RedCtrToken
                } else if x == LevelID::RoosTubes || x == LevelID::CocoPark || x == LevelID::PolarPass || x == LevelID::CortexCastle {
                    RaceReward::GreenCtrToken
                } else if x == LevelID::SewerSpeedway || x == LevelID::TigerTemple || x == LevelID::DragonMines || x == LevelID::NGinLabs {
                    RaceReward::BlueCtrToken
                } else {
                    RaceReward::YellowCtrToken
                };

                Some(RaceUnlock {
                    requirement: Some(UnlockRequirementItem { item_type: RequiredItem::Key, count: key_count }),
                    reward: Rewards::TokensAndRelicRewards(TokensAndRelicRewards{
                        token_reward: token_color,
                        relic_sapphire_reward: RaceReward::SapphireRelic,
                        relic_gold_reward: RaceReward::GoldRelic,
                        relic_platinum_reward: RaceReward::PlatinumRelic,
                    })
                })
            },
        };

        WarpPad { original_level_id, current_level_id, unlock_1, unlock_2 }
    }

    pub fn get_levelid(&self) -> LevelID {
        self.current_level_id
    }

    pub fn get_unlock_1(&self) -> RaceUnlock {
        self.unlock_1
    }

    pub fn set_unlock_1(&mut self, requirement: UnlockRequirementItem) {
        self.unlock_1.requirement = Some(requirement);
    }

    pub fn get_unlock_2(&self) -> Option<RaceUnlock> {
        self.unlock_2
    }

    pub fn set_unlock_2(&mut self, requirement: UnlockRequirementItem) {
        if self.unlock_2.is_some()
        {
            self.unlock_2.unwrap().requirement = Some(requirement);
        } else {
            panic!(
                "Attempting to set unlock_2 on warppad of levelID {} that doesn't have an unlock_2",
                self.get_levelid()
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct BossGarage {
    pub boss: BossCharacter,
    pub requirement: UnlockRequirement,
    pub reward: Rewards,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BossCharacter {
    RipperRoo,
    PapuPapu,
    KomodoJoe,
    Pinstripe,
    NOxide,
}

#[derive(Debug, Clone, Copy)]
pub struct RaceUnlock {
    pub requirement: Option<UnlockRequirementItem>,
    pub reward: Rewards,
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
    pub trophy_reward: RaceReward,
}

#[derive(Debug, Clone, Copy)]
pub struct TokensAndRelicRewards {
    pub token_reward: RaceReward,
    pub relic_sapphire_reward: RaceReward,
    pub relic_gold_reward: RaceReward,
    pub relic_platinum_reward: RaceReward,
}

#[derive(Debug, Clone, Copy)]
pub struct BattleArenaRewards {
    pub single_reward: RaceReward,
}

#[derive(Debug, Clone, Copy)]
pub struct BossRaceRewards {
    pub single_reward: RaceReward,
}

#[derive(Debug, Clone, Copy)]
pub struct RelicRaceOnlyRewards {
    pub relic_sapphire_reward: RaceReward,
    pub relic_gold_reward: RaceReward,
    pub relic_platinum_reward: RaceReward,
}

#[derive(Debug, Clone, Copy)]
pub struct GemCupRewards {
    pub single_reward: RaceReward,
}

pub fn get_vanilla_gameworld() -> GameWorld {
    GameWorld {
        hub_1: GenericHub {
            requirement: None,
            warppad_1: WarpPad::new(LevelID::CrashCove, LevelID::CrashCove),
            warppad_2: WarpPad::new(LevelID::RoosTubes, LevelID::RoosTubes),
            warppad_3: WarpPad::new(LevelID::MysteryCaves, LevelID::MysteryCaves),
            warppad_4: WarpPad::new(LevelID::SewerSpeedway, LevelID::SewerSpeedway),
            boss_garage: BossGarage {
                boss: BossCharacter::RipperRoo,
                requirement: UnlockRequirement::LevelList(
                    vec![
                        LevelID::CrashCove,
                        LevelID::RoosTubes,
                        LevelID::MysteryCaves,
                        LevelID::SewerSpeedway
                    ]
                ),
                reward: Rewards::BossRaceRewards(BossRaceRewards {
                    single_reward: RaceReward::Key
                })
            },
            warppad_arena: WarpPad::new(LevelID::SkullRock, LevelID::SkullRock),
        },
        hub_2: GenericHub {
            requirement: Some(UnlockRequirementItem { item_type: RequiredItem::Key, count: 1 }),
            warppad_1: WarpPad::new(LevelID::CocoPark, LevelID::CocoPark),
            warppad_2: WarpPad::new(LevelID::TigerTemple, LevelID::TigerTemple),
            warppad_3: WarpPad::new(LevelID::PapusPyramid, LevelID::PapusPyramid),
            warppad_4: WarpPad::new(LevelID::DingoCanyon, LevelID::DingoCanyon),
            boss_garage: BossGarage {
                boss: BossCharacter::PapuPapu,
                requirement: UnlockRequirement::LevelList(
                    vec![
                        LevelID::CocoPark,
                        LevelID::TigerTemple,
                        LevelID::PapusPyramid,
                        LevelID::DingoCanyon
                    ]
                ),
                reward: Rewards::BossRaceRewards(BossRaceRewards { single_reward: RaceReward::Key })
            },
            warppad_arena: WarpPad::new(LevelID::RampageRuins, LevelID::RampageRuins)
        },
        hub_3: GenericHub {
            requirement: Some(UnlockRequirementItem { item_type: RequiredItem::Key, count: 2 }),
            warppad_1: WarpPad::new(LevelID::BlizzardBluff, LevelID::BlizzardBluff),
            warppad_2: WarpPad::new(LevelID::DragonMines, LevelID::DragonMines),
            warppad_3: WarpPad::new(LevelID::PolarPass, LevelID::PolarPass),
            warppad_4: WarpPad::new(LevelID::TinyArena, LevelID::TinyArena),
            boss_garage: BossGarage {
                boss: BossCharacter::KomodoJoe,
                requirement: UnlockRequirement::LevelList(
                    vec![
                        LevelID::BlizzardBluff,
                        LevelID::DragonMines,
                        LevelID::PolarPass,
                        LevelID::TinyArena
                    ]
                ),
                reward: Rewards::BossRaceRewards(BossRaceRewards { single_reward: RaceReward::Key })
            },
            warppad_arena: WarpPad::new(LevelID::RockyRoad, LevelID::RockyRoad)
        },
        hub_4: GenericHub {
            requirement: Some(UnlockRequirementItem { item_type: RequiredItem::Key, count: 3 }),
            warppad_1: WarpPad::new(LevelID::NGinLabs, LevelID::NGinLabs),
            warppad_2: WarpPad::new(LevelID::CortexCastle, LevelID::CortexCastle),
            warppad_3: WarpPad::new(LevelID::HotAirSkyway, LevelID::HotAirSkyway),
            warppad_4: WarpPad::new(LevelID::OxideStation, LevelID::OxideStation),
            boss_garage: BossGarage {
                boss: BossCharacter::Pinstripe,
                requirement: UnlockRequirement::LevelList(
                    vec![
                        LevelID::NGinLabs,
                        LevelID::CortexCastle,
                        LevelID::HotAirSkyway,
                        LevelID::OxideStation
                    ]
                ),
                reward: Rewards::BossRaceRewards(BossRaceRewards { single_reward: RaceReward::Key })
            },
            warppad_arena: WarpPad::new(LevelID::NitroCourt, LevelID::NitroCourt)
        },
        gemstone_valley: GemStoneValleyHub {
            requirement: Some(UnlockRequirementItem { item_type: RequiredItem::Key, count: 1 }),
            warppad_1: WarpPad::new(LevelID::TurboTrack, LevelID::TurboTrack),
            warppad_2: WarpPad::new(LevelID::SlideColiseum, LevelID::SlideColiseum),
            cup_warppad_1: WarpPad::new(LevelID::CupRed, LevelID::CupRed),
            cup_warppad_2: WarpPad::new(LevelID::CupGreen, LevelID::CupGreen),
            cup_warppad_3: WarpPad::new(LevelID::CupBlue, LevelID::CupBlue),
            cup_warppad_4: WarpPad::new(LevelID::CupYellow, LevelID::CupYellow),
            cup_warppad_5: WarpPad::new(LevelID::CupPurple, LevelID::CupPurple),
            boss_garage: BossGarage {
                boss: BossCharacter::NOxide,
                requirement: UnlockRequirement::Item(UnlockRequirementItem{
                    item_type: RequiredItem::Key,
                    count: 4
                }),
                reward: Rewards::BossRaceRewards(BossRaceRewards { single_reward: RaceReward::BeatTheGame })
            },
        },
    }
}

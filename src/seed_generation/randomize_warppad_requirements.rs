use std::collections::{HashMap, HashSet};

use rand::{seq::IndexedRandom, Rng};
use rand_chacha::ChaCha8Rng;

use crate::seed_generation::{
    game_world::{get_vanilla_gameworld, BossCharacter, Hubs},
    item_randomization::{
        player_inventory::PlayerInventory,
        randomize_items::{get_location_list, get_shuffled_rewards},
    },
    randomization_datastructures::{
        ItemLocation, LevelID, RaceReward, RaceType, RequiredItem, UnlockRequirement,
        UnlockRequirementItem, UnlockStage,
    },
    seed_settings::{RewardShuffle, WarppadUnlockRequirements},
};

pub fn get_random_warppad_unlocks(
    seed: &mut ChaCha8Rng,
    requirement_setting: &WarppadUnlockRequirements,
    opt_reward_shuffle: &Option<RewardShuffle>,
    warppad_links: HashMap<LevelID, LevelID>,
    warppad_unlocks: HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>>,
    bossgarage_requirements: HashMap<BossCharacter, UnlockRequirement>,
    hub_requirements: HashMap<Hubs, Option<UnlockRequirementItem>>,
) -> Result<HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>>, ()> {
    fn get_unlock_stage(location: ItemLocation) -> UnlockStage {
        match location.racetype {
            RaceType::TrophyRace | RaceType::BossRace | RaceType::GemCup => UnlockStage::One,
            RaceType::CtrOrCrystalChallenge => match location.levelid {
                LevelID::SkullRock
                | LevelID::RampageRuins
                | LevelID::RockyRoad
                | LevelID::NitroCourt => UnlockStage::One,
                _ => UnlockStage::Two,
            },
            RaceType::RelicRaceSapphire | RaceType::RelicRaceGold | RaceType::RelicRacePlatinum => {
                match location.levelid {
                    LevelID::SlideColiseum | LevelID::TurboTrack => UnlockStage::One,
                    _ => UnlockStage::Two,
                }
            }
        }
    }

    //
    let mut free_warppads_warppad_unlocks: HashMap<
        (LevelID, UnlockStage),
        Option<UnlockRequirementItem>,
    > = HashMap::new();
    for ((levelid, stage), _) in warppad_unlocks {
        free_warppads_warppad_unlocks.insert(
            (levelid, stage),
            Some(UnlockRequirementItem {
                item_type: RequiredItem::Trophy,
                count: 0,
            }),
        );
    }

    let res_zeroed_out_item_placement = if let Some(reward_shuffle) = opt_reward_shuffle {
        get_shuffled_rewards(
            seed,
            reward_shuffle,
            &warppad_links,
            free_warppads_warppad_unlocks.clone(),
            bossgarage_requirements.clone(),
            hub_requirements.clone(),
            true,
        )
    } else {
        Ok(get_vanilla_gameworld().get_race_rewards())
    };

    if res_zeroed_out_item_placement.is_err() {
        return Err(());
    }

    let mut zeroed_out_item_placement = res_zeroed_out_item_placement.unwrap();

    // We only have item placements, but are missing static unlock requirements,
    // so we have to generate those after the fact now.
    // Sadly `get_shuffled_rewards` does not return its location list too
    let mut location_list = get_location_list(
        &warppad_links,
        free_warppads_warppad_unlocks,
        bossgarage_requirements,
        hub_requirements,
    );

    // Filter out 2nd stage unlocks
    // They're currently also set to "free", but we have to set the requirements
    // for the 1st stage first.
    let mut second_stage_unlocks = location_list.clone();
    second_stage_unlocks.retain(|f, _| matches!(get_unlock_stage(*f), UnlockStage::Two));
    location_list.retain(|f, _| matches!(get_unlock_stage(*f), UnlockStage::One));

    // Now, with item placement that was made on a game world without any
    // warp pad unlock requirements:
    // 1) Decide on the free warp pads in hub 1, and
    // 2) Starting from there, check which items those races give us, and
    //    randomly choose requirements for the next warp pad(s) from those items
    let mut random_unlocks: HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>> =
        HashMap::new();

    // 1) Decide on the free warp pads in hub 1
    let num_starting_warppads = [(1, 10), (2, 30), (3, 30), (4, 15), (5, 15)]
        .choose_weighted(seed, |x| x.1)
        .unwrap()
        .0;
    let starting_warppads = [
        LevelID::CrashCove,
        LevelID::RoosTubes,
        LevelID::MysteryCaves,
        LevelID::SewerSpeedway,
        LevelID::SkullRock,
    ]
    .choose_multiple(seed, num_starting_warppads)
    .collect::<Vec<_>>();

    for x in starting_warppads {
        let actual_level = warppad_links.get(x).expect("Links should have every level");
        random_unlocks.insert(
            (*actual_level, UnlockStage::One),
            Some(UnlockRequirementItem {
                item_type: RequiredItem::Trophy,
                count: 0,
            }),
        );
    }

    // 2) Starting from there, check which items those races give us, and
    //    randomly choose requirements for the next warp pad(s) from those items
    let mut inventory = PlayerInventory::new();
    let mut filled_locations: Vec<ItemLocation> = Vec::new();

    println!("{:?}", zeroed_out_item_placement);
    for (levelid, _) in random_unlocks.keys() {
        inventory.add_track(*levelid);

        match levelid {
            LevelID::CupRed
            | LevelID::CupGreen
            | LevelID::CupBlue
            | LevelID::CupYellow
            | LevelID::CupPurple => {
                let location = ItemLocation {
                    levelid: *levelid,
                    racetype: RaceType::GemCup,
                };
                let item = zeroed_out_item_placement.get(&location).unwrap();
                inventory.add_item(*item);
                zeroed_out_item_placement
                    .remove(&location)
                    .expect("Should be in there");
                filled_locations.push(location);
            }
            LevelID::TurboTrack | LevelID::SlideColiseum => {
                let location = ItemLocation {
                    levelid: *levelid,
                    racetype: RaceType::RelicRaceSapphire,
                };
                let item = zeroed_out_item_placement.get(&location).unwrap();
                inventory.add_item(*item);
                zeroed_out_item_placement
                    .remove(&location)
                    .expect("Should be in there");
                filled_locations.push(location);

                let location = ItemLocation {
                    levelid: *levelid,
                    racetype: RaceType::RelicRaceGold,
                };
                let item = zeroed_out_item_placement.get(&location).unwrap();
                inventory.add_item(*item);
                zeroed_out_item_placement
                    .remove(&location)
                    .expect("Should be in there");
                filled_locations.push(location);

                let location = ItemLocation {
                    levelid: *levelid,
                    racetype: RaceType::RelicRacePlatinum,
                };
                if let Some(reward_shuffle) = opt_reward_shuffle {
                    if reward_shuffle.include_platinum_relics {
                        let item = zeroed_out_item_placement.get(&location).unwrap();
                        inventory.add_item(*item);
                    }
                }
                zeroed_out_item_placement
                    .remove(&location)
                    .expect("Should be in there");
                filled_locations.push(location);
            }
            LevelID::SkullRock
            | LevelID::RampageRuins
            | LevelID::RockyRoad
            | LevelID::NitroCourt => {
                let location = ItemLocation {
                    levelid: *levelid,
                    racetype: RaceType::CtrOrCrystalChallenge,
                };
                let item = zeroed_out_item_placement.get(&location).unwrap();
                inventory.add_item(*item);
                zeroed_out_item_placement
                    .remove(&location)
                    .expect("Should be in there");
                filled_locations.push(location);
            }
            _ => {
                let location = ItemLocation {
                    levelid: *levelid,
                    racetype: RaceType::TrophyRace,
                };
                let item = zeroed_out_item_placement.get(&location).unwrap();
                inventory.add_item(*item);
                zeroed_out_item_placement
                    .remove(&location)
                    .expect("Should be in there");
                filled_locations.push(location);

                // Add second Unlock Stage to checkable locations
                for (location, reqs) in &second_stage_unlocks {
                    if location.levelid == *levelid {
                        location_list.insert(*location, reqs.clone());
                    }
                }
            }
        };
    }

    //
    while !zeroed_out_item_placement.is_empty() {
        let mut reachable_empty_locations: Vec<ItemLocation> = Vec::new();

        // Find all currently reachable locations that don't have their
        // requirements set yet
        let mut continue_checking: bool = true;
        while continue_checking {
            continue_checking = false;
            for (location, reqs) in &location_list {
                if !reachable_empty_locations.contains(location)
                    && !filled_locations.contains(location)
                    && inventory.does_pass_requirements(reqs)
                {
                    reachable_empty_locations.push(*location);
                    continue_checking = true;
                }
            }
        }

        // Pick random, reachable location
        reachable_empty_locations.sort();
        let chosen_location = reachable_empty_locations.choose(seed).unwrap();

        // Choose item requirement to place here and assign it, unless when
        // it's a boss race as those have very different requirements.
        // We want an unequal weighting between the different item types,
        // otherwise tokens and relics are vastly overrepresented
        // We also tone down the chance for gems and keys
        let req_chances: HashMap<RequiredItem, u16> = HashMap::from([
            (RequiredItem::Trophy, 100),
            (RequiredItem::RedCtrToken, 15),
            (RequiredItem::GreenCtrToken, 15),
            (RequiredItem::BlueCtrToken, 15),
            (RequiredItem::YellowCtrToken, 15),
            (RequiredItem::PurpleCtrToken, 10),
            (RequiredItem::SapphireRelic, 20),
            (RequiredItem::GoldRelic, 20),
            (RequiredItem::PlatinumRelic, 20),
            (RequiredItem::Key, 25),
            (RequiredItem::RedGem, 2),
            (RequiredItem::GreenGem, 2),
            (RequiredItem::BlueGem, 2),
            (RequiredItem::YellowGem, 2),
            (RequiredItem::PurpleGem, 2),
        ]);
        if !matches!(chosen_location.racetype, RaceType::BossRace) {
            let mut possible_reqs: Vec<(RequiredItem, u8)> = Vec::new();
            let current_items = inventory.get_items();
            for (item, count) in &current_items {
                if (!matches!(item, RaceReward::PlatinumRelic)
                    || (opt_reward_shuffle.is_some()
                        && opt_reward_shuffle.unwrap().include_platinum_relics))
                    && (!matches!(
                        item,
                        RaceReward::RedGem
                            | RaceReward::GreenGem
                            | RaceReward::BlueGem
                            | RaceReward::YellowGem
                            | RaceReward::PurpleGem
                    ) || (opt_reward_shuffle.is_some()
                        && opt_reward_shuffle.unwrap().include_gems))
                    && count > &0u8
                {
                    possible_reqs.push((RequiredItem::try_from(*item).unwrap(), *count));
                }
            }
            possible_reqs.sort();
            println!("{:?}", possible_reqs);
            let chosen_reward =
                possible_reqs.choose_weighted(seed, |x| req_chances.get(&x.0).unwrap());
            println!("{:?}", chosen_reward);
            let chosen_reward = chosen_reward.unwrap();
            let mut required_item = chosen_reward.0;
            let mut required_amount = chosen_reward.1;

            if matches!(
                required_item,
                RequiredItem::RedCtrToken
                    | RequiredItem::GreenCtrToken
                    | RequiredItem::BlueCtrToken
                    | RequiredItem::YellowCtrToken
                    | RequiredItem::PurpleCtrToken
            ) {
                if seed.random_range(0..100) < 33 {
                    required_item = RequiredItem::AnyCtrToken;
                    required_amount = 0;
                    for (item, count) in &current_items {
                        if matches!(
                            item,
                            RaceReward::RedCtrToken
                                | RaceReward::GreenCtrToken
                                | RaceReward::BlueCtrToken
                                | RaceReward::YellowCtrToken
                                | RaceReward::PurpleCtrToken
                        ) {
                            required_amount += count;
                        }
                    }
                    required_amount = (((required_amount as f32) * 0.6).ceil()) as u8;
                }
            } else if matches!(
                required_item,
                RequiredItem::SapphireRelic | RequiredItem::GoldRelic | RequiredItem::PlatinumRelic
            ) {
                if seed.random_range(0..100) < 20 {
                    required_item = RequiredItem::AnyRelic;
                    required_amount = 0;
                    for (item, count) in current_items {
                        if matches!(item, RaceReward::SapphireRelic | RaceReward::GoldRelic)
                            || (matches!(item, RaceReward::PlatinumRelic)
                                && opt_reward_shuffle.is_some()
                                && opt_reward_shuffle.unwrap().include_platinum_relics)
                        {
                            required_amount += count;
                        }
                    }
                    required_amount = (((required_amount as f32) * 0.3).ceil()) as u8;
                }
            } else if matches!(
                required_item,
                RequiredItem::RedGem
                    | RequiredItem::GreenGem
                    | RequiredItem::BlueGem
                    | RequiredItem::YellowGem
                    | RequiredItem::PurpleGem
            ) && seed.random_range(0..100) < 80
            {
                required_item = RequiredItem::AnyGem;
                required_amount = 0;
                for (item, count) in current_items {
                    if matches!(
                        item,
                        RaceReward::RedGem
                            | RaceReward::GreenGem
                            | RaceReward::BlueGem
                            | RaceReward::YellowGem
                            | RaceReward::PurpleGem
                    ) {
                        required_amount += count;
                    }
                    if required_amount > 1 {
                        required_amount -= 1;
                    }
                }
            }

            random_unlocks.insert(
                (chosen_location.levelid, get_unlock_stage(*chosen_location)),
                Some(UnlockRequirementItem {
                    item_type: required_item,
                    count: required_amount,
                }),
            );
        }

        // Add location's item(s) to inventory
        // This is either one item, or 3 in case of Relic Races
        inventory.add_track(chosen_location.levelid);
        inventory.add_item(*zeroed_out_item_placement.get(chosen_location).unwrap());
        let _ = zeroed_out_item_placement.remove(chosen_location);

        if [
            RaceType::RelicRaceSapphire,
            RaceType::RelicRaceGold,
            RaceType::RelicRacePlatinum,
        ]
        .contains(&chosen_location.racetype)
        {
            //todo platinum not shuffled
            let mut relicraces_to_check: HashSet<RaceType> = HashSet::from([
                RaceType::RelicRaceSapphire,
                RaceType::RelicRaceGold,
                RaceType::RelicRacePlatinum,
            ]);
            let _ = relicraces_to_check.remove(&chosen_location.racetype);
            for racetype in relicraces_to_check {
                let inferred_location = ItemLocation {
                    levelid: chosen_location.levelid,
                    racetype,
                };
                inventory.add_item(*zeroed_out_item_placement.get(&inferred_location).unwrap());
                let _ = zeroed_out_item_placement.remove(&inferred_location);
                filled_locations.push(inferred_location);
            }
        }

        // Add second Unlock Stage to checkable locations, if there is one or
        // more corresponding one(s)
        if !matches!(chosen_location.racetype, RaceType::BossRace)
            && matches!(get_unlock_stage(*chosen_location), UnlockStage::One)
        {
            for (location, reqs) in &second_stage_unlocks {
                if location.levelid == chosen_location.levelid {
                    location_list.insert(*location, reqs.clone());
                }
            }
        }

        filled_locations.push(*chosen_location);
    }

    //println!("{:?}", random_unlocks);

    // Requirements post processing:
    // Lower some requirement counts by multiplying it by 0.6 and rounding up
    // Set "4 keys" requirements to "3 keys" if needed

    // First pull random_unlocks into Vec, so values don't get visited randomly
    /*
    let mut random_unlocks_vec: Vec<((LevelID, UnlockStage), Option<UnlockRequirementItem>)> =
        Vec::new();
    for (k, v) in &random_unlocks {
        random_unlocks_vec.push((k.clone(), *v));
    }
    random_unlocks_vec.sort();

    let mut unlock_modifications: HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>> =
        HashMap::new();
    for (k, opt_req) in &random_unlocks_vec { if matches!(
        //    req.item_type,
        //    RequiredItem::RedGem
        //        | RequiredItem::GreenGem
        //        | RequiredItem::BlueGem
        //        | RequiredItem::YellowGem
        //        | RequiredItem::PurpleGem
        //) {
        //    if seed.random_range(0..100) < 33 {
        //        unlock_modifications.insert(
        //            k.clone(),
        //            Some(UnlockRequirementItem {
        //                item_type: RequiredItem::AnyGem,
        //                count: req.count,
        //            }),
        //        );
        //    }
        //}
    }
    for (k, v) in unlock_modifications {
        let _ = random_unlocks.insert(k, v);
    }
    */

    // First pull random_unlocks into Vec, so values don't get visited randomly
    let mut random_unlocks_vec: Vec<((LevelID, UnlockStage), Option<UnlockRequirementItem>)> =
        Vec::new();
    for (k, v) in &random_unlocks {
        random_unlocks_vec.push((k.clone(), *v));
    }
    random_unlocks_vec.sort();
    let mut unlock_modifications: HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>> =
        HashMap::new();
    for (k, opt_req) in &random_unlocks_vec {
        let req = opt_req.unwrap();

        if seed.random_range(0..100) < 66 {
            if req.count != 0 {
                println!(
                    "Lowering {:?} {}",
                    req,
                    ((req.count as f32) * 0.6).ceil() as u8
                );
                unlock_modifications.insert(
                    k.clone(),
                    Some(UnlockRequirementItem {
                        item_type: req.item_type,
                        count: ((req.count as f32) * 0.6).ceil() as u8,
                    }),
                );
            }
        } else if matches!(req.item_type, RequiredItem::Key)
            && req.count == 4
            && matches!(
                requirement_setting,
                WarppadUnlockRequirements::RandomWithout4Keys
            )
        {
            println!("Setting '4 keys' requirement to '3 keys' {:?}", req,);
            unlock_modifications.insert(
                k.clone(),
                Some(UnlockRequirementItem {
                    item_type: req.item_type,
                    count: 3,
                }),
            );
        }
    }
    for (k, v) in unlock_modifications {
        let _ = random_unlocks.insert(k, v);
    }

    // Unlock requirements expects the warp pad level id to set the requirement
    // for, not the actual level.
    // So we have to get the warp pad level from the warppad links and adjust
    // the return data accordingly
    let mut random_unlocks_fixed: HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>> =
        HashMap::new();
    let inverted_warppad_links: HashMap<LevelID, LevelID> =
        warppad_links.iter().map(|(k, v)| (*v, *k)).collect();
    for (k, v) in random_unlocks {
        random_unlocks_fixed.insert((*inverted_warppad_links.get(&k.0).unwrap(), k.1), v);
    }

    Ok(random_unlocks_fixed)
}

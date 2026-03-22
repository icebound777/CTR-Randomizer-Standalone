use std::collections::HashMap;

use rand::seq::{IndexedRandom, SliceRandom};
use rand_chacha::ChaCha8Rng;

use crate::seed_generation::{
    game_world::{BossCharacter, Hubs},
    item_randomization::player_inventory::PlayerInventory,
    randomization_datastructures::{
        LevelID, ItemLocation, RaceReward, RaceType, RequiredItem, UnlockRequirement, UnlockRequirementItem, UnlockStage
    },
    seed_settings::RewardShuffle,
};

/// Generate item pool, create a location list to get logical requirements,
/// then run up to 1000 item placement attempts.
/// Returns placement result or Err.
pub fn get_shuffled_rewards(
    seed: &mut ChaCha8Rng,
    reward_shuffle: &RewardShuffle,
    force_vanilla_turbotrack: bool,
    location_list: &HashMap<ItemLocation, Vec<UnlockRequirement>>,
    shuffled_warppad_requirements: bool,
) -> Result<HashMap<ItemLocation, RaceReward>, String> {
    // generate item pool, based on
    // * include_keys
    // * include_gems
    // * include_platinum_relics
    let item_pool = build_item_pool(
        reward_shuffle,
        force_vanilla_turbotrack,
    );

    // generate logical requirements from warppad links, warppad_unlocks, hub requirements, and garage unlocks
    // The warppad_links here are VanillaTrackLocation: ActualTrack
    // The warppad_unlocks here are (ActualTrack, UnlockStage, Option<UnlockRequirement>)
    //let location_list = get_location_list(
    //    warppad_links,
    //    warppad_unlocks,
    //    bossgarage_requirements,
    //    hub_requirements,
    //);

    // run and return item placement
    let num_max_attempts = 1000;
    for attempts in 1..num_max_attempts+1 {
        let placement_result = get_item_placement(
            seed,
            item_pool.clone(),
            reward_shuffle,
            force_vanilla_turbotrack,
            location_list.clone(),
            shuffled_warppad_requirements,
        );
        if let Ok(x) = placement_result {
            println!("Item placement needed {attempts} attempts.");
            return Ok(x);
        }
    }
    let err_text = format!("Item placement failed after {num_max_attempts} attempts.");
    println!("{err_text}");
    Err(err_text)
}

pub fn build_item_pool(
    reward_shuffle: &RewardShuffle,
    force_vanilla_turbotrack: bool,
) -> Vec<RaceReward> {
    let mut item_pool: Vec<RaceReward> = vec![
        // 16 Trophies
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        RaceReward::Trophy,
        // 5 x 4 CTR Tokens
        RaceReward::RedCtrToken,
        RaceReward::RedCtrToken,
        RaceReward::RedCtrToken,
        RaceReward::RedCtrToken,
        RaceReward::GreenCtrToken,
        RaceReward::GreenCtrToken,
        RaceReward::GreenCtrToken,
        RaceReward::GreenCtrToken,
        RaceReward::BlueCtrToken,
        RaceReward::BlueCtrToken,
        RaceReward::BlueCtrToken,
        RaceReward::BlueCtrToken,
        RaceReward::YellowCtrToken,
        RaceReward::YellowCtrToken,
        RaceReward::YellowCtrToken,
        RaceReward::YellowCtrToken,
        RaceReward::PurpleCtrToken,
        RaceReward::PurpleCtrToken,
        RaceReward::PurpleCtrToken,
        RaceReward::PurpleCtrToken,
        // 17 Sapphire & 17 Gold Relics (1 of each are Turbo Track special-cased)
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::SapphireRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
        RaceReward::GoldRelic,
    ];

    if reward_shuffle.include_keys {
        for _ in 0..4 {
            item_pool.push(RaceReward::Key);
        }
    }

    if reward_shuffle.include_gems {
        item_pool.push(RaceReward::RedGem);
        item_pool.push(RaceReward::GreenGem);
        item_pool.push(RaceReward::BlueGem);
        item_pool.push(RaceReward::YellowGem);
        item_pool.push(RaceReward::PurpleGem);
    }

    if !force_vanilla_turbotrack {
        item_pool.push(RaceReward::SapphireRelic);
        item_pool.push(RaceReward::GoldRelic);
    }

    if reward_shuffle.include_platinum_relics {
        for _ in 0..17 {
            item_pool.push(RaceReward::PlatinumRelic);
        }
        if !force_vanilla_turbotrack {
            item_pool.push(RaceReward::PlatinumRelic);
        }
    }

    item_pool
}

fn get_item_placement(
    seed: &mut ChaCha8Rng,
    mut item_pool: Vec<RaceReward>,
    reward_shuffle: &RewardShuffle,
    force_vanilla_turbotrack: bool,
    location_list: HashMap<ItemLocation, Vec<UnlockRequirement>>,
    shuffled_warppad_requirements: bool,
) -> Result<HashMap<ItemLocation, RaceReward>, String> {
    let mut item_placement: HashMap<
        ItemLocation,
        (Vec<UnlockRequirement>, Option<RaceReward>),
    > = HashMap::new();

    // Enrich location data with an empty slot for items
    for (k, v) in location_list {
        item_placement.insert(k, (v, None));
    }

    // Pre-place items that cannot be shuffled
    item_placement
        .get_mut(&ItemLocation{levelid: LevelID::OxideStation, racetype: RaceType::BossRace})
        .unwrap()
        .1 = Some(RaceReward::BeatTheGame);
    // Pre-place items that the player does not want shuffled
    if !reward_shuffle.include_platinum_relics {
        for level_id in [
            // N.Sanity Beach
            LevelID::CrashCove,
            LevelID::RoosTubes,
            LevelID::MysteryCaves,
            LevelID::SewerSpeedway,
            // The Lost Ruins
            LevelID::TigerTemple,
            LevelID::CocoPark,
            LevelID::PapusPyramid,
            LevelID::DingoCanyon,
            // Glacier Park
            LevelID::BlizzardBluff,
            LevelID::DragonMines,
            LevelID::PolarPass,
            LevelID::TinyArena,
            // Citadel City
            LevelID::NGinLabs,
            LevelID::CortexCastle,
            LevelID::HotAirSkyway,
            LevelID::OxideStation,
            // Gem Stone Valley
            LevelID::TurboTrack,
            LevelID::SlideColiseum,
        ] {
            item_placement
                .get_mut(&ItemLocation {
                    levelid: level_id,
                    racetype: RaceType::RelicRacePlatinum,
                })
                .unwrap()
                .1 = Some(RaceReward::PlatinumRelic);
        }
    } else if force_vanilla_turbotrack {
        item_placement
            .get_mut(&ItemLocation {
                levelid: LevelID::TurboTrack,
                racetype: RaceType::RelicRacePlatinum,
            })
            .unwrap()
            .1 = Some(RaceReward::PlatinumRelic);
    }

    if force_vanilla_turbotrack {
        item_placement
            .get_mut(&ItemLocation {
                levelid: LevelID::TurboTrack,
                racetype: RaceType::RelicRaceSapphire,
            })
            .unwrap()
            .1 = Some(RaceReward::SapphireRelic);
        item_placement
            .get_mut(&ItemLocation {
                levelid: LevelID::TurboTrack,
                racetype: RaceType::RelicRaceGold,
            })
            .unwrap()
            .1 = Some(RaceReward::GoldRelic);
    }

    if !reward_shuffle.include_gems {
        item_placement
            .get_mut(&ItemLocation{levelid: LevelID::CupRed, racetype: RaceType::GemCup})
            .unwrap()
            .1 = Some(RaceReward::RedGem);
        item_placement
            .get_mut(&ItemLocation{levelid: LevelID::CupGreen, racetype: RaceType::GemCup})
            .unwrap()
            .1 = Some(RaceReward::GreenGem);
        item_placement
            .get_mut(&ItemLocation{levelid: LevelID::CupBlue, racetype: RaceType::GemCup})
            .unwrap()
            .1 = Some(RaceReward::BlueGem);
        item_placement
            .get_mut(&ItemLocation{levelid: LevelID::CupYellow, racetype: RaceType::GemCup})
            .unwrap()
            .1 = Some(RaceReward::YellowGem);
        item_placement
            .get_mut(&ItemLocation{levelid: LevelID::CupPurple, racetype: RaceType::GemCup})
            .unwrap()
            .1 = Some(RaceReward::PurpleGem);
    }

    if !reward_shuffle.include_keys {
        for level_id in [
            LevelID::RoosTubes,
            LevelID::PapusPyramid,
            LevelID::DragonMines,
            LevelID::HotAirSkyway,
        ] {
            item_placement
                .get_mut(&ItemLocation{levelid: level_id, racetype: RaceType::BossRace})
                .unwrap()
                .1 = Some(RaceReward::Key);
        }
    }

    item_pool.shuffle(seed);
    // Guarantee keys are placed first, and trophies if vanilla warppad reqs
    if !shuffled_warppad_requirements {
        item_pool.sort_by_key(|k| matches!(k, RaceReward::Trophy));
    }
    item_pool.sort_by_key(|k| matches!(k, RaceReward::Key));

    let num_items_to_place = item_pool.len();
    let mut num_placed_items = 0;
    let mut item_placement_success = true;

    // While there are any unfilled item locations left, attempt placing items
    while item_placement.iter().any(|x| x.1 .1.is_none()) && !item_pool.is_empty() {
        let item_to_place = item_pool.pop().expect("checked by while");
        //println!("{item_to_place:?}");

        // Initialize player inventory with all items yet to be placed, except
        // for the one item we want to place right now
        let mut inventory = PlayerInventory::new();
        for item in &item_pool {
            inventory.add_item(*item);
        }

        // Find all locations that are reachable with the player's current
        // inventory combined with all currently reachable items
        let mut reachable_locations: HashMap<ItemLocation, Option<RaceReward>> =
            HashMap::new();
        let mut reachable_empty_locations: Vec<ItemLocation> = Vec::new();
        let mut found_placed_item = true;

        while found_placed_item {
            found_placed_item = false;
            for (location, (requirements, placed_item)) in &item_placement {
                if inventory.does_pass_requirements(requirements)
                    && !reachable_locations
                        .keys()
                        .collect::<Vec<_>>()
                        .contains(&location)
                {
                    if let Some(x) = placed_item {
                        if !matches!(*x, RaceReward::PlatinumRelic)
                            || reward_shuffle.include_platinum_relics
                        {
                            // Found a plat relic, but we didn't shuffle them:
                            // Assume the player does not want to do plat relics
                            inventory.add_item(*x);
                        }
                        found_placed_item = true;
                    } else {
                        reachable_empty_locations.push(*location);
                    }
                    // If first possible race of this warp pad:
                    // Mark level as cleared for the purpose of boss garages
                    if [RaceType::TrophyRace, RaceType::GemCup].contains(&location.racetype)
                        || ([LevelID::TurboTrack, LevelID::SlideColiseum, LevelID::SkullRock, LevelID::RampageRuins, LevelID::RockyRoad, LevelID::NitroCourt].contains(&location.levelid)
                            && [RaceType::RelicRaceSapphire, RaceType::CtrOrCrystalChallenge].contains(&location.racetype))
                    {
                        inventory.add_track(location.levelid);
                        found_placed_item = true;
                    }
                    reachable_locations.insert(*location, *placed_item);
                }
            }
        }
        if reachable_empty_locations.is_empty() {
            print!("{num_placed_items} of {num_items_to_place} placed before abort - ");
            println!("reachable_empty_locations.is_empty()");
            //println!("{inventory:?}");
            //println!("{item_pool:?}");
            //println!("{item_placement:?}");
            item_placement_success = false;
            break;
        }

        // Pick one random empty location, and place our new item there
        let mut reachable_empty_locations_clone = reachable_empty_locations.clone();
        reachable_empty_locations_clone.sort();
        let chosen_location = reachable_empty_locations_clone.choose(seed).unwrap();
        println!("{chosen_location:?}: {item_to_place}");
        item_placement.get_mut(chosen_location).unwrap().1 = Some(item_to_place);

        num_placed_items += 1;
    }

    if item_placement_success {
        if !item_pool.is_empty() {
            print!("{num_placed_items} of {num_items_to_place} placed before abort - ");
            println!("!item_pool.is_empty()");
            //println!("{item_pool:?}");
            //println!("{item_placement:?}");
            panic!()
        }
        if item_placement.iter().any(|x| x.1 .1.is_none()) {
            print!("{num_placed_items} placed before abort - ");
            println!("item_placement still has empty item locations");
            //println!("{item_pool:?}");
            //println!("{item_placement:?}");
            panic!()
        }

        // Throw out the requirements; we no longer need them
        let mut filtered_item_placement = HashMap::new();

        for (k, (_, reward)) in item_placement {
            filtered_item_placement.insert(k, reward.expect("checked by if above"));
        }

        Ok(filtered_item_placement)
    } else {
        Err("Item placement failed.".to_string())
    }
}

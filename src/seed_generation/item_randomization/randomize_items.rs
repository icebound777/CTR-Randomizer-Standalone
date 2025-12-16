use std::collections::HashMap;

use rand::seq::{IndexedRandom, SliceRandom};
use rand_chacha::ChaCha8Rng;

use crate::seed_generation::{
    game_world::{BossCharacter, Hubs},
    item_randomization::player_inventory::PlayerInventory,
    randomization_datastructures::{
        LevelID, RaceReward, RaceType, RequiredItem, UnlockRequirement, UnlockRequirementItem, UnlockStage
    }, seed_settings::RewardShuffle,
};

pub fn get_shuffled_rewards(
    seed: &mut ChaCha8Rng,
    reward_shuffle: &RewardShuffle,
    warppad_links: HashMap<LevelID, LevelID>,
    warppad_unlocks: HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>>,
    bossgarage_requirements: HashMap<BossCharacter, UnlockRequirement>,
    hub_requirements: HashMap<Hubs, Option<UnlockRequirementItem>>,
) -> HashMap<(LevelID, RaceType), RaceReward> {
    // generate item pool, based on
    // * include_keys
    // * include_gems
    // * include_platinum_relics
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
        // 18 Sapphire & 18 Gold Relics
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

    if reward_shuffle.include_platinum_relics {
        for _ in 0..18 {
            item_pool.push(RaceReward::PlatinumRelic);
        }
    }

    // generate logical requirements from warppad links, warppad_unlocks, hub requirements, and garage unlocks
    // The warppad_links here are VanillaTrackLocation: ActualTrack
    // The warppad_unlocks here are (ActualTrack, UnlockStage, Option<UnlockRequirement>)
    let location_list = get_location_list(
        warppad_links,
        warppad_unlocks,
        bossgarage_requirements,
        hub_requirements,
    );

    // run and return item placement
    for attempts in 0..11 {
        let placement_result = get_item_placement(seed, item_pool.clone(), reward_shuffle, location_list.clone());
        if let Ok(x) = placement_result {
            println!("Item placement needed {attempts} attempts.");
            return x;
        }
    }
    println!("Item placement failed after 10 attempts.");
    panic!()
}

fn get_location_list(
    warppad_links: HashMap<LevelID, LevelID>,
    warppad_unlocks: HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>>,
    bossgarage_requirements: HashMap<BossCharacter, UnlockRequirement>,
    hub_requirements: HashMap<Hubs, Option<UnlockRequirementItem>>,
) -> HashMap<(LevelID, RaceType), Vec<UnlockRequirement>> {
    let mut location_list = HashMap::new();

    fn insert_trophy_warppad(
        location_list: &mut HashMap<(LevelID, RaceType), Vec<UnlockRequirement>>,
        warppad_unlocks: &HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>>,
        hub_requirements: Vec<UnlockRequirement>,
        level_id: LevelID
    ) {
        let mut unlocks = hub_requirements;
        unlocks.push(
            UnlockRequirement::Item(
                warppad_unlocks
                .get(&(level_id, UnlockStage::One))
                .unwrap()
                .unwrap()
            )
        );
        location_list.insert(
            (level_id, RaceType::TrophyRace),
            unlocks.clone()
        );

        unlocks.push(
            UnlockRequirement::Item(
                warppad_unlocks
                .get(&(level_id, UnlockStage::Two))
                .unwrap()
                .unwrap()
            )
        );
        location_list.insert(
            (level_id, RaceType::CtrOrCrystalChallenge),
            unlocks.clone()
        );
        location_list.insert(
            (level_id, RaceType::RelicRaceSapphire),
            unlocks.clone()
        );
        location_list.insert(
            (level_id, RaceType::RelicRaceGold),
            unlocks.clone()
        );
        location_list.insert(
            (level_id, RaceType::RelicRacePlatinum),
            unlocks
        );
    }

    fn insert_arena_warppad(
        location_list: &mut HashMap<(LevelID, RaceType), Vec<UnlockRequirement>>,
        warppad_unlocks: &HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>>,
        hub_requirements: Vec<UnlockRequirement>,
        level_id: LevelID
    ) {
        let mut unlocks = hub_requirements;

        unlocks.push(
            UnlockRequirement::Item(
                warppad_unlocks
                .get(&(level_id, UnlockStage::One))
                .unwrap()
                .unwrap()
            )
        );

        location_list.insert(
            (level_id, RaceType::CtrOrCrystalChallenge),
            unlocks
        );
    }

    fn insert_gemcup_warppad(
        location_list: &mut HashMap<(LevelID, RaceType), Vec<UnlockRequirement>>,
        warppad_unlocks: &HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>>,
        hub_requirements: Vec<UnlockRequirement>,
        level_id: LevelID
    ) {
        let mut unlocks = hub_requirements;

        unlocks.push(
            UnlockRequirement::Item(UnlockRequirementItem{item_type: RequiredItem::Key, count: 2})
        );
        unlocks.push(
            UnlockRequirement::Item(
                warppad_unlocks
                .get(&(level_id, UnlockStage::One))
                .unwrap()
                .unwrap()
            )
        );

        location_list.insert(
            (level_id, RaceType::GemCup),
            unlocks
        );
    }

    fn insert_reliconly_warppad(
        location_list: &mut HashMap<(LevelID, RaceType), Vec<UnlockRequirement>>,
        warppad_unlocks: &HashMap<(LevelID, UnlockStage), Option<UnlockRequirementItem>>,
        mut hub_requirements: Vec<UnlockRequirement>,
        level_id: LevelID
    ) {
        let mut unlocks: Vec<UnlockRequirement> = vec![
            UnlockRequirement::Item(
                warppad_unlocks
                .get(&(level_id, UnlockStage::One))
                .unwrap()
                .unwrap()
            ),
            UnlockRequirement::Item(
                UnlockRequirementItem {
                    item_type: RequiredItem::Key, count: 1
                }
            )
        ];

        unlocks.append(&mut hub_requirements);

        location_list.insert(
            (level_id, RaceType::RelicRaceSapphire),
            unlocks.clone()
        );
        location_list.insert(
            (level_id, RaceType::RelicRaceGold),
            unlocks.clone()
        );
        location_list.insert(
            (level_id, RaceType::RelicRacePlatinum),
            unlocks
        );
    }

    fn insert_boss_garage(
        location_list: &mut HashMap<(LevelID, RaceType), Vec<UnlockRequirement>>,
        bossgarage_requirements: &HashMap<BossCharacter, UnlockRequirement>,
        hub_requirements: &HashMap<Hubs, Option<UnlockRequirementItem>>,
        level_id: LevelID
    ) {
        let mut req_list = vec![
            bossgarage_requirements
            .get(match level_id {
                LevelID::RoosTubes => &BossCharacter::RipperRoo,
                LevelID::PapusPyramid => &BossCharacter::PapuPapu,
                LevelID::DragonMines => &BossCharacter::KomodoJoe,
                LevelID::HotAirSkyway => &BossCharacter::Pinstripe,
                LevelID::OxideStation => &BossCharacter::NOxide,
                _ => panic!("should never happen")
            })
            .unwrap()
            .clone()
        ];

        let hub_req = hub_requirements.get(
            match level_id {
                LevelID::RoosTubes => &Hubs::NSanityBeach,
                LevelID::PapusPyramid => &Hubs::TheLostRuins,
                LevelID::DragonMines => &Hubs::GlacierPark,
                LevelID::HotAirSkyway => &Hubs::CitadelCity,
                LevelID::OxideStation => &Hubs::GemStoneValley,
                _ => panic!("should never happen")
            }
        ).unwrap();

        if let Some(x) = hub_req {
            req_list.push(
                UnlockRequirement::Item(*x)
            );
        }

        location_list.insert(
            (level_id, RaceType::BossRace),
            req_list
        );
    }

    for (original_level, current_level) in warppad_links {
        // get hub requirements for original warppad location
        let current_hub_requirements: Vec<UnlockRequirement> = match original_level {
            LevelID::CrashCove | LevelID::RoosTubes | LevelID::MysteryCaves | LevelID::SewerSpeedway | LevelID::SkullRock => {
                let hubreq = hub_requirements.get(&Hubs::NSanityBeach).expect("has to exist");
                if hubreq.is_some() {
                     vec![UnlockRequirement::Item(hubreq.expect("checked by if"))]
                } else {
                    Vec::new()
                }
            },
            LevelID::TigerTemple | LevelID::CocoPark | LevelID::PapusPyramid | LevelID::DingoCanyon | LevelID::RampageRuins
            | LevelID::TurboTrack | LevelID::SlideColiseum => {
                let hubreq = hub_requirements.get(&Hubs::TheLostRuins).expect("has to exist");
                if hubreq.is_some() {
                    vec![UnlockRequirement::Item(hubreq.expect("checked by if"))]
                } else {
                    Vec::new()
                }
            },
            LevelID::CupRed | LevelID::CupGreen | LevelID::CupBlue | LevelID::CupYellow | LevelID::CupPurple => {
                let hubreq = hub_requirements.get(&Hubs::GemStoneValley).expect("has to exist");
                if hubreq.is_some() {
                    vec![
                        UnlockRequirement::Item(hubreq.expect("checked by if")),
                        UnlockRequirement::Item(UnlockRequirementItem{item_type: RequiredItem::Key, count: 2})
                    ]
                } else {
                    vec![UnlockRequirement::Item(UnlockRequirementItem{item_type: RequiredItem::Key, count: 2})]
                }
            },
            LevelID::BlizzardBluff | LevelID::DragonMines | LevelID::PolarPass | LevelID::TinyArena | LevelID::RockyRoad => {
                let hubreq = hub_requirements.get(&Hubs::GlacierPark).expect("has to exist");
                if hubreq.is_some() {
                    vec![UnlockRequirement::Item(hubreq.expect("checked by if"))]
                } else {
                    Vec::new()
                }
            },
            LevelID::NGinLabs | LevelID::CortexCastle | LevelID::HotAirSkyway | LevelID::OxideStation | LevelID::NitroCourt => {
                let hubreq = hub_requirements.get(&Hubs::CitadelCity).expect("has to exist");
                if hubreq.is_some() {
                    vec![UnlockRequirement::Item(hubreq.expect("checked by if"))]
                } else {
                    Vec::new()
                }
            }
        };

        // get race type and unlock stages based on current level
        match current_level {
            x @ (LevelID::CrashCove | LevelID::RoosTubes | LevelID::MysteryCaves | LevelID::SewerSpeedway | LevelID::TigerTemple
            | LevelID::CocoPark | LevelID::PapusPyramid | LevelID::DingoCanyon | LevelID::BlizzardBluff | LevelID::DragonMines | LevelID::PolarPass
            | LevelID::TinyArena | LevelID::NGinLabs | LevelID::CortexCastle | LevelID::HotAirSkyway | LevelID::OxideStation) => {
                insert_trophy_warppad(&mut location_list, &warppad_unlocks, current_hub_requirements, x);
            },
            x @ (LevelID::SkullRock | LevelID::RampageRuins | LevelID::RockyRoad | LevelID::NitroCourt) => {
                insert_arena_warppad(&mut location_list, &warppad_unlocks, current_hub_requirements, x);
            },
            x @ (LevelID::CupRed | LevelID::CupGreen | LevelID::CupBlue | LevelID::CupYellow | LevelID::CupPurple) => {
                insert_gemcup_warppad(&mut location_list, &warppad_unlocks, current_hub_requirements, x);
            },
            x @ (LevelID::TurboTrack | LevelID::SlideColiseum) => {
                insert_reliconly_warppad(&mut location_list, &warppad_unlocks, current_hub_requirements, x);
            }
        }
    }

    // Boss garages
    insert_boss_garage(&mut location_list, &bossgarage_requirements, &hub_requirements, LevelID::RoosTubes);
    insert_boss_garage(&mut location_list, &bossgarage_requirements, &hub_requirements, LevelID::PapusPyramid);
    insert_boss_garage(&mut location_list, &bossgarage_requirements, &hub_requirements, LevelID::DragonMines);
    insert_boss_garage(&mut location_list, &bossgarage_requirements, &hub_requirements, LevelID::HotAirSkyway);
    insert_boss_garage(&mut location_list, &bossgarage_requirements, &hub_requirements, LevelID::OxideStation);

    location_list
}

fn get_item_placement(
    seed: &mut ChaCha8Rng,
    mut item_pool: Vec<RaceReward>,
    reward_shuffle: &RewardShuffle,
    location_list: HashMap<(LevelID, RaceType), Vec<UnlockRequirement>>,
) -> Result<(HashMap<(LevelID, RaceType), RaceReward>), String> {
    let mut item_placement: HashMap<(LevelID, RaceType), (Vec<UnlockRequirement>, Option<RaceReward>)> = HashMap::new();

    // Enrich location data with an empty slot for items
    for (k, v) in location_list {
        item_placement.insert(k, (v, None));
    }

    // Pre-place items that cannot be shuffled
    item_placement.get_mut(&(LevelID::OxideStation, RaceType::BossRace)).unwrap().1 = Some(RaceReward::BeatTheGame);
    // Pre-place items that the player does not want shuffled
    if !reward_shuffle.include_platinum_relics {
        item_placement.get_mut(&(LevelID::CrashCove, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::RoosTubes, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::MysteryCaves, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::SewerSpeedway, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::TigerTemple, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::CocoPark, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::PapusPyramid, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::DingoCanyon, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::BlizzardBluff, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::DragonMines, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::PolarPass, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::TinyArena, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::NGinLabs, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::CortexCastle, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::HotAirSkyway, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::OxideStation, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::TurboTrack, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
        item_placement.get_mut(&(LevelID::SlideColiseum, RaceType::RelicRacePlatinum)).unwrap().1 = Some(RaceReward::PlatinumRelic);
    }

    if !reward_shuffle.include_gems {
        item_placement.get_mut(&(LevelID::CupRed, RaceType::GemCup)).unwrap().1 = Some(RaceReward::RedGem);
        item_placement.get_mut(&(LevelID::CupGreen, RaceType::GemCup)).unwrap().1 = Some(RaceReward::GreenGem);
        item_placement.get_mut(&(LevelID::CupBlue, RaceType::GemCup)).unwrap().1 = Some(RaceReward::BlueGem);
        item_placement.get_mut(&(LevelID::CupYellow, RaceType::GemCup)).unwrap().1 = Some(RaceReward::YellowGem);
        item_placement.get_mut(&(LevelID::CupPurple, RaceType::GemCup)).unwrap().1 = Some(RaceReward::PurpleGem);
    }

    if !reward_shuffle.include_keys {
        item_placement.get_mut(&(LevelID::RoosTubes, RaceType::BossRace)).unwrap().1 = Some(RaceReward::Key);
        item_placement.get_mut(&(LevelID::PapusPyramid, RaceType::BossRace)).unwrap().1 = Some(RaceReward::Key);
        item_placement.get_mut(&(LevelID::DragonMines, RaceType::BossRace)).unwrap().1 = Some(RaceReward::Key);
        item_placement.get_mut(&(LevelID::HotAirSkyway, RaceType::BossRace)).unwrap().1 = Some(RaceReward::Key);
    }

    item_pool.shuffle(seed);
    // Guarantee keys and trophies are placed first
    item_pool.sort_by_key(|k| matches!(k, RaceReward::Trophy));
    item_pool.sort_by_key(|k| matches!(k, RaceReward::Key));

    let mut num_placed_items = 0;
    let mut item_placement_success = true;

    // While there are any unfilled item locations left, attempt placing items
    while item_placement.iter().any(|x| x.1.1.is_none()) && !item_pool.is_empty() {
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
        let mut reachable_locations: HashMap<(LevelID, RaceType), Option<RaceReward>> = HashMap::new();
        let mut reachable_empty_locations: Vec<(LevelID, RaceType)> = Vec::new();
        let mut found_placed_item = true;

        while found_placed_item {
            found_placed_item = false;
            for (location, (requirements, placed_item)) in &item_placement {
                if inventory.does_pass_requirements(requirements) && !reachable_locations.keys().collect::<Vec<_>>().contains(&location) {
                    if let Some(x) = placed_item {
                        inventory.add_item(*x);
                        found_placed_item = true;
                    }
                    else {
                        reachable_empty_locations.push(*location);
                    }
                    if location.1 == RaceType::TrophyRace {
                        inventory.add_track(location.0);
                        found_placed_item = true;
                    }
                    reachable_locations.insert(*location, *placed_item);
                }
            }
        }
        if reachable_empty_locations.is_empty() {
            print!("{num_placed_items} placed before abort - ");
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
            print!("{num_placed_items} placed before abort - ");
            println!("!item_pool.is_empty()");
            //println!("{item_pool:?}");
            //println!("{item_placement:?}");
            panic!()
        }
        if item_placement.iter().any(|x| x.1.1.is_none()) {
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

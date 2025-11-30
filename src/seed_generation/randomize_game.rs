use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

use crate::seed_generation::{
    game_world::{BossCharacter, get_vanilla_gameworld}, randomization_datastructures::{
        GameSetup, LevelID, RequiredItem, SettingID, SettingValue, UnlockRequirement, UnlockRequirementItem
    }, seed_settings::{BossGarageRequirements, FinalOxideUnlock, RelicTime, SeedSettings, WarppadUnlockRequirements}
};

fn get_vanilla_game() -> GameSetup {
    GameSetup {
        game_world: get_vanilla_gameworld(),
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

pub fn get_randomized_game(seed: ChaCha8Rng, seed_as_number: u32, chosen_settings: &SeedSettings) -> GameSetup {
    let vanilla_gameworld = get_vanilla_game().game_world;

    let mut new_game_world = vanilla_gameworld.clone();
    let mut new_race_rewards = &new_game_world.get_race_rewards();

    let overwrite_seed_hash_1;
    let overwrite_seed_hash_2;

    if chosen_settings.randomization.shuffle_adventure {
        overwrite_seed_hash_1 = (seed_as_number >> 16) as u16;
        overwrite_seed_hash_2 = (seed_as_number & 0xFFFF) as u16;

        // Warppads
        if let Some(warppad_shuffle) = &chosen_settings.randomization.warppad_shuffle {
            let new_warppads = get_shuffled_warppads(
                seed,
                vanilla_gameworld.get_warppad_links(),
                warppad_shuffle.include_battle_arenas,
                warppad_shuffle.include_gem_cups,
                matches!(chosen_settings.randomization.warppad_unlock_requirements, WarppadUnlockRequirements::Vanilla)
            );

            new_game_world.set_warppad_links(new_warppads);
        }

        // Warppad Unlocks
        let new_warppad_unlocks = match chosen_settings.randomization.warppad_unlock_requirements {
            WarppadUnlockRequirements::Vanilla => {
                vanilla_gameworld.get_warppad_unlocks()
            },
            _ => todo!()
        };

        new_game_world.set_warppad_unlocks(new_warppad_unlocks);

        // Boss Garage requirements
        // Don't modify if Original4Tracks, as we expect that to be set by default
        if !matches!(chosen_settings.randomization.bossgarage_unlock_requirements, BossGarageRequirements::Original4Tracks) {
            let new_garage_unlocks = get_modified_garage_unlocks(
                chosen_settings.randomization.bossgarage_unlock_requirements,
                new_game_world.get_warppad_links(),
            );

            new_game_world.set_garage_unlocks(new_garage_unlocks);
        }

        // Race Rewards
    } else {
        overwrite_seed_hash_1 = 0u16;
        overwrite_seed_hash_2 = 0u16;
    }

    GameSetup {
        game_world: new_game_world,
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
            (
                SettingID::HelperTiziano,
                SettingValue::Boolean(chosen_settings.tricks.helper_tiziano),
            ),
            (
                SettingID::HelperTA,
                SettingValue::Boolean(chosen_settings.tricks.helper_ta),
            ),
        ],
    }
}

fn get_shuffled_warppads(
    mut seed: ChaCha8Rng,
    original_warppads: HashMap<LevelID, LevelID>,
    include_battle_arenas: bool,
    include_gem_cups: bool,
    vanilla_unlock_requirements: bool,
) -> HashMap<LevelID, LevelID> {
    let mut randomized_levels: HashMap<LevelID, LevelID> = original_warppads.clone();
    let mut untouched_levels: HashMap<LevelID, LevelID> = HashMap::new();
    let mut pre_randomized_levels: HashMap<LevelID, LevelID> = HashMap::new();

    if !include_battle_arenas || vanilla_unlock_requirements {
        let level_map = if !include_battle_arenas {
            &mut untouched_levels
        } else {
            // if vanilla warppad unlocks but warppad shuffle, then
            // dont put non-trophypads into trophy pads
            &mut pre_randomized_levels
        };

        for level_key in original_warppads.keys() {
            if *level_key >= LevelID::NitroCourt && *level_key <= LevelID::RockyRoad {
                level_map.insert(*level_key, *level_key);
            }
        }
    }

    if !include_gem_cups || vanilla_unlock_requirements {
        let level_map = if !include_gem_cups {
            &mut untouched_levels
        } else {
            // if vanilla warppad unlocks but warppad shuffle, then
            // dont put non-trophypads into trophy pads
            &mut pre_randomized_levels
        };

        for level_key in original_warppads.keys() {
            if *level_key >= LevelID::CupRed && *level_key <= LevelID::CupPurple {
                level_map.insert(*level_key, *level_key);
            }
        }
    }

    if vanilla_unlock_requirements {
        // if vanilla warppad unlocks but warppad shuffle, then
        // dont put non-trophypads into trophy pads
        for level_key in original_warppads.keys() {
            if *level_key == LevelID::TurboTrack || *level_key == LevelID::SlideColiseum {
                pre_randomized_levels.insert(*level_key, *level_key);
            }
        }
    }

    for level_key in untouched_levels.keys() {
        let _ = randomized_levels.remove(level_key);
    }
    for level_key in pre_randomized_levels.keys() {
        let _ = randomized_levels.remove(level_key);
    }

    // Shuffle pre_randomized level values, but make sure a gem cup does not end
    // up in the turbo track warp pad (requiring 5 gems to access)
    if pre_randomized_levels.len() > 0 {
        loop {
            let pre_randomized_levels_clone = pre_randomized_levels.clone();
            let mut level_values: Vec<&LevelID> = pre_randomized_levels_clone.values().collect::<Vec<_>>();
            level_values.sort();
            level_values.shuffle(&mut seed);

            let mut level_keys: Vec<&LevelID> = pre_randomized_levels_clone.keys().collect();
            level_keys.sort();
            for level_key in level_keys {
                pre_randomized_levels.insert(*level_key, *level_values.pop().expect("Same size as target vec."));
            }

            if !vec![LevelID::CupRed, LevelID::CupGreen, LevelID::CupBlue, LevelID::CupYellow, LevelID::CupPurple].contains(pre_randomized_levels.get(&LevelID::TurboTrack).expect("checked by first part of if")) {
                break
            }
            // if we get here: Gem Cup in TT location, reshuffle!
        };
    }

    // Shuffle the randomized_levels values
    let randomized_levels_clone = randomized_levels.clone();
    let mut level_values: Vec<&LevelID> = randomized_levels_clone.values().collect::<Vec<_>>();
    level_values.sort();
    level_values.shuffle(&mut seed);

    let mut level_keys: Vec<&LevelID> = randomized_levels_clone.keys().collect();
    level_keys.sort();
    for level_key in level_keys {
        randomized_levels.insert(*level_key, *level_values.pop().expect("Same size as target vec."));
    }

    // Add removed levels back into the full hashmap
    randomized_levels.extend(untouched_levels);
    randomized_levels.extend(pre_randomized_levels);

    randomized_levels
}

fn get_modified_garage_unlocks(
    garage_unlock: BossGarageRequirements,
    level_links: HashMap<LevelID, LevelID>,
) -> HashMap<BossCharacter, UnlockRequirement> {
    let mut new_garage_unlocks;

    match garage_unlock {
        BossGarageRequirements::Original4Tracks => panic!("this function is not supposed to get called with this value"),
        BossGarageRequirements::SameHubTracks => {
            new_garage_unlocks = HashMap::new();
            new_garage_unlocks.insert(
                BossCharacter::RipperRoo,
                UnlockRequirement::LevelList(
                    vec![
                        *level_links.get(&LevelID::CrashCove).expect("every level needs to be present in the level links"),
                        *level_links.get(&LevelID::RoosTubes).expect("every level needs to be present in the level links"),
                        *level_links.get(&LevelID::MysteryCaves).expect("every level needs to be present in the level links"),
                        *level_links.get(&LevelID::SewerSpeedway).expect("every level needs to be present in the level links"),
                    ]
                )
            );
            new_garage_unlocks.insert(
                BossCharacter::PapuPapu,
                UnlockRequirement::LevelList(
                    vec![
                        *level_links.get(&LevelID::TigerTemple).expect("every level needs to be present in the level links"),
                        *level_links.get(&LevelID::CocoPark).expect("every level needs to be present in the level links"),
                        *level_links.get(&LevelID::PapusPyramid).expect("every level needs to be present in the level links"),
                        *level_links.get(&LevelID::DingoCanyon).expect("every level needs to be present in the level links"),
                    ]
                )
            );
            new_garage_unlocks.insert(
                BossCharacter::KomodoJoe,
                UnlockRequirement::LevelList(
                    vec![
                        *level_links.get(&LevelID::BlizzardBluff).expect("every level needs to be present in the level links"),
                        *level_links.get(&LevelID::DragonMines).expect("every level needs to be present in the level links"),
                        *level_links.get(&LevelID::PolarPass).expect("every level needs to be present in the level links"),
                        *level_links.get(&LevelID::TinyArena).expect("every level needs to be present in the level links"),
                    ]
                )
            );
            new_garage_unlocks.insert(
                BossCharacter::Pinstripe,
                UnlockRequirement::LevelList(
                    vec![
                        *level_links.get(&LevelID::NGinLabs).expect("every level needs to be present in the level links"),
                        *level_links.get(&LevelID::CortexCastle).expect("every level needs to be present in the level links"),
                        *level_links.get(&LevelID::HotAirSkyway).expect("every level needs to be present in the level links"),
                        *level_links.get(&LevelID::OxideStation).expect("every level needs to be present in the level links"),
                    ]
                )
            );
        },
        BossGarageRequirements::Trophies => {
            new_garage_unlocks = HashMap::from([
                (
                    BossCharacter::RipperRoo,
                    UnlockRequirement::Item(UnlockRequirementItem{
                        item_type: RequiredItem::Trophy,
                        count: 4
                    })
                ),
                (
                    BossCharacter::PapuPapu,
                    UnlockRequirement::Item(UnlockRequirementItem{
                        item_type: RequiredItem::Trophy,
                        count: 8
                    })
                ),
                (
                    BossCharacter::KomodoJoe,
                    UnlockRequirement::Item(UnlockRequirementItem{
                        item_type: RequiredItem::Trophy,
                        count: 12
                    })
                ),
                (
                    BossCharacter::Pinstripe,
                    UnlockRequirement::Item(UnlockRequirementItem{
                        item_type: RequiredItem::Trophy,
                        count: 16
                    })
                )
            ]);
        },
    };

    new_garage_unlocks
}

use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

use crate::seed_generation::{
    game_world::get_vanilla_gameworld, randomization_datastructures::{
        GameSetup, LevelID, SettingID, SettingValue
    }, seed_settings::{BossGarageRequirements, FinalOxideUnlock, RelicTime, SeedSettings}
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
    let mut new_warppad_unlocks = &new_game_world.get_warppad_unlocks();
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
            );

            new_game_world.set_warppad_links(new_warppads);
        }

        // Warppad Unlocks
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

use std::{io, path::PathBuf};

use serde_json::{json, to_string_pretty};

use crate::seed_generation::seed_settings::{RewardShuffle, WarppadShuffle};
use crate::seed_generation::{
    game_world::{
        BattleArenaRewards, BossGarage, BossRaceRewards, GemCupRewards, RelicRaceOnlyRewards,
        Rewards, TokensAndRelicRewards, TrophyRaceRewards, WarpPad,
    },
    randomization_datastructures::{GameSetup, UnlockRequirement, UnlockRequirementItem},
    seed_settings::SeedSettings,
};

pub fn write_spoilerlog(
    new_rom_path: PathBuf,
    game_setup: GameSetup,
    seed: u32,
    chosen_settings: &SeedSettings,
) -> Result<(), io::Error> {
    let game_world = game_setup.game_world;
    let spoilerlog = json!({
        "seed_hash": get_seed_hash(seed),
        "adventure": {
            "hub_1": {
                "warppad_1_crashcove": get_formatted_warppad(game_world.hub_1.warppad_1),
                "warppad_2_roostubes": get_formatted_warppad(game_world.hub_1.warppad_2),
                "warppad_3_mysterycaves": get_formatted_warppad(game_world.hub_1.warppad_3),
                "warppad_4_sewerspeedway": get_formatted_warppad(game_world.hub_1.warppad_4),
                "warppad_5_skullrock": get_formatted_warppad(game_world.hub_1.warppad_arena),
                "boss_garage": get_formatted_bossgarage(game_world.hub_1.boss_garage)
            },
            "hub_2": {
                "warppad_1_cocopark": get_formatted_warppad(game_world.hub_2.warppad_1),
                "warppad_2_tigertemple": get_formatted_warppad(game_world.hub_2.warppad_2),
                "warppad_3_papuspyramid": get_formatted_warppad(game_world.hub_2.warppad_3),
                "warppad_4_dingocanyon": get_formatted_warppad(game_world.hub_2.warppad_4),
                "warppad_5_rampageruins": get_formatted_warppad(game_world.hub_2.warppad_arena),
                "boss_garage": get_formatted_bossgarage(game_world.hub_2.boss_garage)
            },
            "hub_3": {
                "warppad_1_blizzardbluff": get_formatted_warppad(game_world.hub_3.warppad_1),
                "warppad_2_dragonmines": get_formatted_warppad(game_world.hub_3.warppad_2),
                "warppad_3_polarpass": get_formatted_warppad(game_world.hub_3.warppad_3),
                "warppad_4_tinyarena": get_formatted_warppad(game_world.hub_3.warppad_4),
                "warppad_5_rockyroad": get_formatted_warppad(game_world.hub_3.warppad_arena),
                "boss_garage": get_formatted_bossgarage(game_world.hub_3.boss_garage)
            },
            "hub_4": {
                "warppad_1_nginlabs": get_formatted_warppad(game_world.hub_4.warppad_1),
                "warppad_2_cortexcastle": get_formatted_warppad(game_world.hub_4.warppad_2),
                "warppad_3_hotairskyway": get_formatted_warppad(game_world.hub_4.warppad_3),
                "warppad_4_oxidestation": get_formatted_warppad(game_world.hub_4.warppad_4),
                "warppad_5_nitrocourt": get_formatted_warppad(game_world.hub_4.warppad_arena),
                "boss_garage": get_formatted_bossgarage(game_world.hub_4.boss_garage)
            },
            "gemstonevalley": {
                "warppad_1_turbotrack": get_formatted_warppad(game_world.gemstone_valley.warppad_1),
                "warppad_2_slidecoliseum": get_formatted_warppad(game_world.gemstone_valley.warppad_2),
                "warppad_3_redgemcup": get_formatted_warppad(game_world.gemstone_valley.cup_warppad_1),
                "warppad_4_greengemcup": get_formatted_warppad(game_world.gemstone_valley.cup_warppad_2),
                "warppad_5_bluegemcup": get_formatted_warppad(game_world.gemstone_valley.cup_warppad_3),
                "warppad_6_yellowgemcup": get_formatted_warppad(game_world.gemstone_valley.cup_warppad_4),
                "warppad_7_purplegemcup": get_formatted_warppad(game_world.gemstone_valley.cup_warppad_5),
                "boss_garage": get_formatted_bossgarage(game_world.gemstone_valley.boss_garage)
            },
        },
        "settings": {
            "randomization": {
                "shuffle_adventure": chosen_settings.randomization.shuffle_adventure.to_string(),
                "shuffle_race_rewards": get_serialized_optional_setting(OptionalSettings::RewardShuffle(chosen_settings.randomization.shuffle_race_rewards)),
                "warppad_shuffle": get_serialized_optional_setting(OptionalSettings::WarppadShuffle(chosen_settings.randomization.warppad_shuffle)),
                "warppad_unlock_requirements": chosen_settings.randomization.warppad_unlock_requirements.to_string(),
                "bossgarage_unlock_requirements": chosen_settings.randomization.bossgarage_unlock_requirements.to_string(),
                "autounlock_ctrchallenge_relicrace": chosen_settings.randomization.autounlock_ctrchallenge_relicrace.to_string(),
            },
            "general": {
                "relicrace_required_minimum_time": chosen_settings.general.rr_required_minimum_time.to_string(),
                "relicrace_require_perfects": chosen_settings.general.rr_require_perfects.to_string(),
                "oxide_final_challenge_unlock": chosen_settings.general.oxide_final_challenge_unlock.to_string(),
            },
            "qol": {
                "skip_mask_hints": chosen_settings.qol.skip_mask_hints.to_string(),
                "skip_mask_congrats": chosen_settings.qol.skip_mask_congrats.to_string(),
                "autoskip_podium_cutscenes": chosen_settings.qol.autoskip_podium_cutscenes.to_string(),
            },
            "tricks": {
                "helper_tiziano": chosen_settings.tricks.helper_tiziano.to_string(),
                "helper_ta": chosen_settings.tricks.helper_ta.to_string(),
            }
        }
    });

    let mut spoilerlog_path = new_rom_path;
    let file_stem = spoilerlog_path.clone();
    let file_stem = file_stem.file_stem().unwrap();
    spoilerlog_path.pop();
    spoilerlog_path.push(format!(
        "{}{}",
        file_stem.to_str().unwrap(),
        "_spoilers.json"
    ));

    std::fs::write(&spoilerlog_path, to_string_pretty(&spoilerlog).unwrap())
}

fn get_seed_hash(seed: u32) -> String {
    fn get_character(seed_short: u8) -> String {
        let character_id: u8 = seed_short % 16;
        match character_id {
            0 => "Crash",
            1 => "Cortex",
            2 => "Tiny",
            3 => "Coco",
            4 => "NGin",
            5 => "Dingodile",
            6 => "Polar",
            7 => "Pura",
            8 => "Pinstripe",
            9 => "PapuPapu",
            10 => "RipperRoo",
            11 => "KomodoJoe",
            12 => "N.Tropy",
            13 => "Penta",
            14 => "FakeCrash",
            15 => "N.Oxide",
            _ => panic!(),
        }
        .to_string()
    }

    let character_icon_1 = get_character(((seed >> 24) & 0xFF) as u8);
    let character_icon_2 = get_character(((seed >> 16) & 0xFF) as u8);
    let character_icon_3 = get_character(((seed >> 8) & 0xFF) as u8);
    let character_icon_4 = get_character((seed & 0xFF) as u8);

    format!("{character_icon_1}, {character_icon_2}, {character_icon_3}, {character_icon_4}")
}

fn get_formatted_warppad(warppad: WarpPad) -> serde_json::Value {
    if let Some(unlock2) = warppad.get_unlock_2() {
        json!({
            "level": warppad.get_levelid().to_string(),
            "unlock_1": if let Some(x) = warppad.get_unlock_1().requirement {
                format!("{} (x{})", x.item_type, x.count)
            } else {
                "Open".to_string()
            },
            "reward_1": get_formatted_reward(warppad.get_unlock_1().reward),
            "unlock_2": if let Some(x) = unlock2.requirement {
                format!("{} (x{})", x.item_type, x.count)
            } else {
                "Open".to_string()
            },
            "reward_2": get_formatted_reward(unlock2.reward),
        })
    } else {
        json!({
            "level": warppad.get_levelid().to_string(),
            "unlock_1": if let Some(x) = warppad.get_unlock_1().requirement {
                format!("{} (x{})", x.item_type, x.count)
            } else {
                "Open".to_string()
            },
            "reward_1": get_formatted_reward(warppad.get_unlock_1().reward)
        })
    }
}

fn get_formatted_bossgarage(bossgarage: BossGarage) -> serde_json::Value {
    match bossgarage.requirement {
        UnlockRequirement::Item(UnlockRequirementItem { item_type, count }) => {
            json!({
                "unlock": format!("{} (x{})", item_type, count),
                "reward": get_formatted_reward(bossgarage.reward)
            })
        }
        UnlockRequirement::LevelList(x) => {
            let level_list: String = x
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            json!({
                "unlock": level_list,
                "reward": get_formatted_reward(bossgarage.reward)
            })
        }
    }
}

fn get_formatted_reward(reward: Rewards) -> serde_json::Value {
    match reward {
        Rewards::TrophyRaceRewards(TrophyRaceRewards { trophy_reward: rew }) => {
            json!({
                "Trophy Race": rew.to_string()
            })
        }
        Rewards::TokensAndRelicRewards(TokensAndRelicRewards {
            token_reward,
            relic_sapphire_reward,
            relic_gold_reward,
            relic_platinum_reward,
        }) => {
            json!({
                "CTR Challenge": token_reward.to_string(),
                "Sapphire Time": relic_sapphire_reward.to_string(),
                "Gold Time": relic_gold_reward.to_string(),
                "Platinum Time": relic_platinum_reward.to_string(),
            })
        }
        Rewards::BossRaceRewards(BossRaceRewards { single_reward }) => {
            json!({
                "Boss Reward": single_reward.to_string()
            })
        }
        Rewards::BattleArenaRewards(BattleArenaRewards { single_reward }) => {
            json!({
                "Crystal Challenge": single_reward.to_string()
            })
        }
        Rewards::RelicRaceOnlyRewards(RelicRaceOnlyRewards {
            relic_sapphire_reward,
            relic_gold_reward,
            relic_platinum_reward,
        }) => {
            json!({
                "Sapphire Time": relic_sapphire_reward.to_string(),
                "Gold Time": relic_gold_reward.to_string(),
                "Platinum Time": relic_platinum_reward.to_string(),
            })
        }
        Rewards::GemCupRewards(GemCupRewards { single_reward }) => {
            json!({
                "Cup Reward": single_reward.to_string()
            })
        }
    }
}

fn get_serialized_optional_setting(opt_setting: OptionalSettings) -> serde_json::Value {
    match opt_setting {
        OptionalSettings::RewardShuffle(x) => {
            if let Some(y) = x {
                let mut json_map: serde_json::Map<String, serde_json::Value> =
                    serde_json::Map::new();
                json_map.insert(
                    "include_keys".to_owned(),
                    serde_json::Value::Bool(y.include_keys),
                );
                json_map.insert(
                    "include_gems".to_owned(),
                    serde_json::Value::Bool(y.include_gems),
                );
                json_map.insert(
                    "include_platinum_relics".to_owned(),
                    serde_json::Value::Bool(y.include_platinum_relics),
                );
                serde_json::Value::Object(json_map)
            } else {
                serde_json::Value::String("false".to_owned())
            }
        }
        OptionalSettings::WarppadShuffle(x) => {
            if let Some(y) = x {
                let mut json_map: serde_json::Map<String, serde_json::Value> =
                    serde_json::Map::new();
                json_map.insert(
                    "include_battle_arenas".to_owned(),
                    serde_json::Value::Bool(y.include_battle_arenas),
                );
                json_map.insert(
                    "include_gem_cups".to_owned(),
                    serde_json::Value::Bool(y.include_gem_cups),
                );
                serde_json::Value::Object(json_map)
            } else {
                serde_json::Value::String("false".to_owned())
            }
        }
    }
}

enum OptionalSettings {
    RewardShuffle(Option<RewardShuffle>),
    WarppadShuffle(Option<WarppadShuffle>),
}

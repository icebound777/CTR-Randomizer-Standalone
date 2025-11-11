use std::{io, path::PathBuf};

use serde_json::{json, to_string_pretty};

use crate::seed_generation::{game_world::{BattleArenaRewards, BossGarage, BossRaceRewards, BossRequirement, GemCupRewards, RelicRaceOnlyRewards, Rewards, TokensAndRelicRewards, TrophyRaceRewards, WarpPad}, randomization_datastructures::{GameSetup, UnlockRequirement}};

pub fn write_spoilerlog(new_rom_path: PathBuf, game_setup: GameSetup) -> Result<(), io::Error> {
    let game_world = game_setup.game_world;
    let spoilerlog = json!({
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
    });

    let mut spoilerlog_path = new_rom_path;
    let file_stem = spoilerlog_path.clone();
    let file_stem = file_stem.file_stem().unwrap();
    spoilerlog_path.pop();
    spoilerlog_path.push(format!("{}{}", file_stem.to_str().unwrap(), "_spoilers.json"));

    std::fs::write(&spoilerlog_path, to_string_pretty(&spoilerlog).unwrap())
}

fn get_formatted_warppad(warppad: WarpPad) -> serde_json::Value {
    if let Some(unlock2) = warppad.unlock_2 {
        json!({
            "level": warppad.level_id.to_string(),
            "unlock_1": if let Some(x) = warppad.unlock_1.requirement {
                format!("{} (x{})", x.item_type.to_string(), x.count)
            } else {
                "Open".to_string()
            },
            "reward_1": get_formatted_reward(warppad.unlock_1.reward),
            "unlock_2": if let Some(x) = unlock2.requirement {
                format!("{} (x{})", x.item_type.to_string(), x.count)
            } else {
                "Open".to_string()
            },
            "reward_2": get_formatted_reward(unlock2.reward),
        })
    } else {
        json!({
            "level": warppad.level_id.to_string(),
            "unlock_1": if let Some(x) = warppad.unlock_1.requirement {
                format!("{} (x{})", x.item_type.to_string(), x.count)
            } else {
                "Open".to_string()
            },
            "reward_1": get_formatted_reward(warppad.unlock_1.reward)
        })
    }
}

fn get_formatted_bossgarage(bossgarage: BossGarage) -> serde_json::Value {
    match bossgarage.requirement {
        BossRequirement::UnlockRequirement(UnlockRequirement{item_type, count}) => {
            json!({
                "unlock": format!("{} (x{})", item_type.to_string(), count),
                "reward": get_formatted_reward(bossgarage.reward)
            })
        },
        BossRequirement::BossRequirement(x) => {
            let level_list: String = x.iter().map(|r|  r.to_string()).collect::<Vec<_>>().join(", ");
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
        },
        Rewards::TokensAndRelicRewards(TokensAndRelicRewards{token_reward,relic_sapphire_reward,relic_gold_reward,relic_platinum_reward}) => {
            json!({
                "CTR Challenge": token_reward.to_string(),
                "Sapphire Time": relic_sapphire_reward.to_string(),
                "Gold Time": relic_gold_reward.to_string(),
                "Platinum Time": relic_sapphire_reward.to_string(),
            })
        },
        Rewards::BossRaceRewards(BossRaceRewards{single_reward}) => {
            json!({
                "Boss Reward": single_reward.to_string()
            })
        },
        Rewards::BattleArenaRewards(BattleArenaRewards{single_reward}) => {
            json!({
                "Crystal Challenge": single_reward.to_string()
            })
        },
        Rewards::RelicRaceOnlyRewards(RelicRaceOnlyRewards{ relic_sapphire_reward, relic_gold_reward, relic_platinum_reward }) => {
            json!({
                "Sapphire Time": relic_sapphire_reward.to_string(),
                "Gold Time": relic_gold_reward.to_string(),
                "Platinum Time": relic_sapphire_reward.to_string(),
            })
        },
        Rewards::GemCupRewards(GemCupRewards{single_reward}) => {
            json!({
                "Cup Reward": single_reward.to_string()
            })
        }
    }
}

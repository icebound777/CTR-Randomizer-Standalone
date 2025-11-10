use std::{io, path::PathBuf};

use serde_json::{json, to_string_pretty};

use crate::seed_generation::{randomization_datastructures::GameSetup};

pub fn write_spoilerlog(new_rom_path: PathBuf, game_setup: GameSetup) -> Result<(), io::Error> {
    let game_world = game_setup.game_world;
    let spoilerlog = json!({
        "hub_1": {
            "warppad_1_crashcove": {
                "level": game_world.hub_1.warppad_1.level_id.to_string()
            }
        }
    });

    let mut spoilerlog_path = new_rom_path;
    let file_stem = spoilerlog_path.clone();
    let file_stem = file_stem.file_stem().unwrap();
    spoilerlog_path.pop();
    spoilerlog_path.push(format!("{}{}", file_stem.to_str().unwrap(), "_spoilers.json"));

    std::fs::write(&spoilerlog_path, to_string_pretty(&spoilerlog).unwrap())
}

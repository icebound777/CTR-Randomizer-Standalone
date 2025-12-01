use std::{collections::HashMap, os::unix::fs::FileExt, path::PathBuf};

use crate::seed_generation::randomization_datastructures::{GameSetup, RequiredItem, SettingValue, UnlockStage};

pub fn write_db_to_rom<'a>(rom_filepath: &PathBuf, randomized_game: &GameSetup) -> Result<(), &'a str> {
    // Transform the randomized game into bytes to write
    let write_location = 0xF1E8;

    let database = get_database_vec(randomized_game);

    // Write bytes
    let filehandle = std::fs::File::options().write(true).open(rom_filepath);

    match filehandle {
        Ok(x) => {
            let write_result = x.write_at(&database, write_location);
            if write_result.is_err() {
                return Err("Could not write randomization to patched ROM!");
            }
            Ok(())
        },
        _ => Err("Could not open patched ROM for writing randomization data!")
    }
}

fn get_database_vec(randomized_game: &GameSetup) -> Vec<u8> {
    // To reference what the resulting vec is supposed to look like, see
    // mod repository, src/CTRRandomizer_database.c file

    // Turn randomizer game into database hashmap
    let mut key_value_db: HashMap<u32, u16> = HashMap::new();
    let mut db_as_vec: Vec<u8> = Vec::new();

    // Warppad Links
    let db_prefix_levelids: u32 = 0xA000;
    for (key, value) in randomized_game.game_world.get_warppad_links() {
        key_value_db.insert((db_prefix_levelids | key as u32) << 16, value as u16);
    }

    // Race Rewards
    let db_prefix_rewards: u32 = 0xA100;
    for (level_id, racetype, reward) in randomized_game.game_world.get_race_rewards() {
        key_value_db.insert(
            ((db_prefix_rewards | level_id as u32) << 16) | racetype as u32,
            reward as u16
        );
    }

    // Warppad Unlocks
    let db_prefix_unlock_1: u32 = 0xA200;
    let db_prefix_unlock_2: u32 = 0xA300;
    for ((level_id, unlock_stage), requirement) in randomized_game.game_world.get_warppad_unlocks() {
        let cur_db_prefix = if unlock_stage == UnlockStage::Two {
            db_prefix_unlock_2
        } else {
            db_prefix_unlock_1
        };

        key_value_db.insert(
            (cur_db_prefix | level_id as u32) << 16,
            match requirement {
                Some(req) => {req.item_type as u16 | ((u16::from(req.count) & 0x1F) << 11)},
                None => {RequiredItem::Trophy as u16} // implicit -> count: 0
            }
        );
    }

    // Settings
    let db_prefix_settings: u32 = 0xAF00;
    for (setting_id, value) in &randomized_game.settings {
        let db_value = match value {
            SettingValue::Boolean(x) => u16::from(*x),
            SettingValue::RelicDifficulty(x) => *x as u16,
            SettingValue::BossGarageRequirements(x) => *x as u16,
            SettingValue::OxideRequiredRelics(x) => *x as u16,
            SettingValue::SeedHashPart(x) => *x,
        };

        key_value_db.insert((db_prefix_settings | *setting_id as u32) << 16, db_value);
    }

    // Begin u8 vec with RAM marker for client
    db_as_vec.push(0xDB);
    db_as_vec.push(0xDA);
    db_as_vec.push(0x00);
    db_as_vec.push(0x0D);
    db_as_vec.push(0xDB);
    db_as_vec.push(0xDA);

    // Use ordered hashmap keys to get an ordered vec later
    let mut db_keys: Vec<&u32> = key_value_db.keys().collect();
    db_keys.sort();

    // Transform database hashmap into u8 vec for simplified writing to ROM
    for key in db_keys {
        let value = key_value_db.get(key).unwrap();
        // Here we have to compensate for the big-endian / little-endian
        // swap of the CTR ISO, so for each u16 we have to write the lower
        // value u8 first
        db_as_vec.push(((key >> 16) & 0xFF) as u8);
        db_as_vec.push(((key >> 24) & 0xFF) as u8);
        db_as_vec.push((key & 0xFF) as u8);
        db_as_vec.push(((key >> 8) & 0xFF) as u8);
        db_as_vec.push((value & 0xFF) as u8);
        db_as_vec.push(((value >> 8) & 0xFF) as u8);
    }

    // Finish u8 vec with RAM marker for client and end-of-list marker
    db_as_vec.push(0xDB);
    db_as_vec.push(0xDA);
    db_as_vec.push(0xAA);
    db_as_vec.push(0x0D);
    db_as_vec.push(0xDB);
    db_as_vec.push(0xDA);
    db_as_vec.push(0xFF);
    db_as_vec.push(0xFF);

    db_as_vec
}

use std::ffi::OsStr;
use rand

use crate::seed_generation::seed_settings::SeedSettings;

pub fn generate_seed(rom_filepath: &str, chosen_settings: SeedSettings) {
    let rom_path = OsStr::new(rom_filepath);

    let mut seed: u32 = 0;
    loop {
        seed = rand::random::<u32>();

        if seed != 0u32 {
            break;
        }
    }

    // randomize game
    randomize_game(seed, chosen_settings);

    // apply base mod patch to rom
    apply_base_mod_patch(rom_filepath);

    // write randomization to rom
    write_to_rom();

    // if needed, write patch file
    if chosen_settings.write_patchfile {
        todo!();
    }

    // if needed, write spoiler log
    if chosen_settings.write_spoilerlog {
        todo!();
    }
}

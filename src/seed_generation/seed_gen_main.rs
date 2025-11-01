use std::ffi::OsStr;

use crate::seed_generation::seed_settings::SeedSettings;
use crate::seed_generation::rom_patching::xdelta_patching::apply_patchfile;

pub fn generate_seed(rom_filepath: &str, chosen_settings: SeedSettings) {
    let rom_path = OsStr::new(rom_filepath);

    let mut seed: u32 = 0;
    loop {
        seed = rand::random::<u32>();

        if seed != 0u32 {
            break;
        }
    }
    let seed: u32 = seed;

    // randomize game
    //randomize_game(seed, chosen_settings);

    // apply base mod patch to rom
    apply_patchfile(rom_filepath, seed);

    // write randomization to rom
    //write_to_rom();

    // if needed, write patch file
    if chosen_settings.write_patchfile {
        todo!();
    }

    // if needed, write spoiler log
    if chosen_settings.write_spoilerlog {
        todo!();
    }
}

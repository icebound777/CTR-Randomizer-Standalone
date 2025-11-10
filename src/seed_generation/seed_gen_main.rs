use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::seed_generation::randomize_game::get_randomized_game;
use crate::seed_generation::rom_patching::bsdiff_patching::{apply_patchfile, create_patchfile};
use crate::seed_generation::seed_settings::SeedSettings;
use crate::seed_generation::write_rando_db::write_db_to_rom;

pub fn generate_seed<'a>(rom_filepath: &'a str, chosen_settings: &'a SeedSettings) -> Result<(), &'a str> {
    let mut seed: u32;
    loop {
        seed = rand::random::<u32>();

        if seed != 0u32 {
            break;
        }
    }
    let seed: u32 = seed;

    // randomize game
    let rng = ChaCha8Rng::seed_from_u64(seed as u64);
    let randomized_game = get_randomized_game(rng, seed, chosen_settings);

    // apply base mod patch to rom
    let filepath_new_rom = apply_patchfile(rom_filepath, seed);

    match filepath_new_rom {
        Ok(new_rom) => {
            // write randomization to rom
            let write_result = write_db_to_rom(&new_rom, randomized_game);
            if write_result.is_err() {
                return Err(write_result.expect_err("str type error"));
            }

            // if needed, write patch file
            if chosen_settings.write_patchfile {
                let filepath_new_patch = create_patchfile(rom_filepath, new_rom);

                if filepath_new_patch.is_err() {
                    return Err("Could not create patch file!");
                }
            }

            // if needed, write spoiler log
            if chosen_settings.write_spoilerlog {
                let log_success = create_spoilerlog(new_rom, randomized_game);

                if log_success.is_err() {
                    return Err("Could not create spoiler log file!");
                }
            }
        },
        _ => { return Err("Could not apply base patch to vanilla ROM!");}
    }

    Ok(())
}

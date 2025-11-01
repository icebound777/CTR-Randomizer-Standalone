use crate::seed_generation::seed_settings::SeedSettings;
use crate::seed_generation::rom_patching::xdelta_patching::{apply_patchfile, create_patchfile};

pub fn generate_seed(rom_filepath: &str, chosen_settings: SeedSettings) {
    let mut seed: u32;
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
    let filepath_new_rom = apply_patchfile(rom_filepath, seed);

    match filepath_new_rom {
        Ok(x) => {
            // write randomization to rom
            //write_to_rom();

            // if needed, write patch file
            if chosen_settings.write_patchfile {
                let filepath_new_patch = create_patchfile(rom_filepath, x);

                match filepath_new_patch {
                    Ok(_) => (),
                    _ => (),
                }
            }

            // if needed, write spoiler log
            if chosen_settings.write_spoilerlog {
                todo!();
            }
        },
        _ => {}
    }
}

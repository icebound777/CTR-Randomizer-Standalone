use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::seed_generation::randomize_game::get_randomized_game;
use crate::seed_generation::rom_patching::bsdiff_patching::{apply_base_patchfile, create_patchfile, get_new_rom_file_path};
use crate::seed_generation::rom_patching::eccedc_fixing::full_recalc;
use crate::seed_generation::seed_settings::SeedSettings;
use crate::seed_generation::spoilerlog::{get_seed_hash, write_spoilerlog};
use crate::seed_generation::write_rando_db::write_db_to_rom;

use std::time::Instant;


pub struct SeedMetadata {
    pub seed_filename: String,
    pub seed_hash: String,
}


pub fn generate_seeds<'a>(
    rom_filepath: &'a str,
    chosen_settings: &'a SeedSettings,
    seed_count: u32,
    spoilerlog_only: bool,
) -> Result<SeedMetadata, String> {
    let now = Instant::now();

    let mut result = SeedMetadata {
        seed_filename: "placeholder".to_owned(), seed_hash: "placeholder".to_owned()
    };

    for _ in 0..seed_count {
        let one_seed_gen = generate_seed(
            rom_filepath,
            chosen_settings,
            spoilerlog_only
        );

        match one_seed_gen {
            Ok(x) => {result = x;},
            Err(_) => {},
        }
    }

    if seed_count > 1 {
        let elapsed = now.elapsed();
        println!("Generating {} seeds took: {:.2?}", seed_count, elapsed);
    }

    Ok(result)
}


fn generate_seed<'a>(
    rom_filepath: &'a str,
    chosen_settings: &'a SeedSettings,
    spoilerlog_only: bool,
) -> Result<SeedMetadata, String> {
    let now = Instant::now();

    let mut seed: u32;
    loop {
        seed = rand::random::<u32>();

        if seed != 0u32 {
            break;
        }
    }
    let seed: u32 = seed;
    println!("seed: {seed}");

    // randomize game
    let rng = ChaCha8Rng::seed_from_u64(u64::from(seed));
    let randomized_game = get_randomized_game(rng, seed, chosen_settings);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    if let Ok(randomized_game) = randomized_game {
        let new_rom_file_path = get_new_rom_file_path(rom_filepath, seed);

        if spoilerlog_only {
            let log_success = write_spoilerlog(&new_rom_file_path, randomized_game, seed, chosen_settings);

            if log_success.is_err() {
                return Err("Could not create spoiler log file!".to_owned());
            }

            return Ok(
                SeedMetadata {
                    seed_filename: new_rom_file_path.file_name().unwrap().to_string_lossy().to_string(),
                    seed_hash: get_seed_hash(seed),
                }
            );
        } else {
            // apply base mod patch to rom
            let filepath_new_rom = apply_base_patchfile(rom_filepath, seed);

            match filepath_new_rom {
                Ok(new_rom) => {
                    // write randomization to rom
                    let write_result = write_db_to_rom(&new_rom, &randomized_game);
                    if write_result.is_err() {
                        return Err(write_result.expect_err("str type error").to_owned());
                    }

                    // recalculate error detection / error correction code
                    let readwrite_result = full_recalc(&new_rom);
                    if readwrite_result.is_err() {
                        return Err("Could not fix EDC / ECC data!".to_owned());
                    }

                    // if needed, write patch file
                    if chosen_settings.write_patchfile {
                        let filepath_new_patch = create_patchfile(rom_filepath, &new_rom);

                        if filepath_new_patch.is_err() {
                            return Err("Could not create patch file!".to_owned());
                        }
                    }

                    // if needed, write spoiler log
                    if chosen_settings.write_spoilerlog {
                        let log_success = write_spoilerlog(&new_rom, randomized_game, seed, chosen_settings);

                        if log_success.is_err() {
                            return Err("Could not create spoiler log file!".to_owned());
                        }
                    }

                    return Ok(
                        SeedMetadata {
                            seed_filename: new_rom.file_name().unwrap().to_string_lossy().to_string(),
                            seed_hash: get_seed_hash(seed),
                        }
                    );
                },
                _ => { return Err("Could not apply base patch to vanilla ROM!".to_owned());}
            }
        }
    }

    Err(format!(
        "Failed to generate a randomized game!\nThis can rarely happen, just retry it.\n\
            If this continues happening, screenshot the following info\n\
            and send it to Icebound777 via GitHub or Discord:\n\n\
            Seed: {}\n\
            Version: {}\n\
            Settings:\n{}",
        seed,
        "1.0.0-beta5",
        chosen_settings
    ))
}

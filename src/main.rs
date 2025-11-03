// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use md5;
use open;
use rfd::FileDialog;
use slint::SharedString;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::seed_generation::seed_gen_main::generate_seed;
use crate::seed_generation::seed_settings::{
    BossGarageRequirements, FinalOxideUnlock, GeneralSettings, QualityOfLifeSettings,
    RandomizationSettings, RelicTime, SeedSettings, WarppadShuffle, WarppadUnlockRequirements,
};

slint::include_modules!();

pub mod seed_generation;

enum RomValidState {
    NoRom = 0,
    //Validating = 1, // not actually used, just for reference
    Invalid = 2,
    Valid = 3,
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = MainWindow::new()?;

    let main_ui_weak = ui.as_weak();
    let main_window = main_ui_weak.unwrap();

    ui.on_sources_generator(move || {
        let _ = open::that("https://github.com/icebound777/CTR-Randomizer-Standalone");
    });

    ui.on_sources_mod(move || {
        let _ = open::that("https://github.com/icebound777/CTR-Randomizer/");
    });

    ui.on_gen_seed(move || {
        // Collect settings chosen via UI
        let chosen_rando_settings = RandomizationSettings {
            shuffle_adventure: main_window.get_shuffle_adventure() == 1,
            shuffle_race_rewards: main_window.get_shuffle_race_rewards(),
            warppad_shuffle: if main_window.get_shuffle_warppads() {
                Some(WarppadShuffle {
                    include_battle_arenas: main_window.get_shuffle_warppads_battlearenas(),
                    include_gem_cups: main_window.get_shuffle_warppads_gemcups(),
                })
            } else {
                None
            },
            warppad_unlock_requirements: WarppadUnlockRequirements::try_from(
                main_window.get_warppad_unlock_requirements(),
            )
            .unwrap(),
            bossgarage_unlock_requirements: BossGarageRequirements::try_from(main_window.get_bossgarage_unlock_requirements()).unwrap(),
            autounlock_ctrchallenge_relicrace: main_window.get_autounlock_ctrchallenge_relicrace(),
        };
        let chosen_qol_settings = QualityOfLifeSettings {
            skip_mask_hints: main_window.get_qol_skip_mask_hints(),
            autoskip_podium_cutscenes: main_window.get_qol_skip_podium(),
            skip_mask_congrats: main_window.get_qol_skip_mask_congrats(),
        };
        let chosen_general_settings = GeneralSettings {
            rr_required_minimum_time: RelicTime::try_from(
                main_window.get_rr_required_minimum_time(),
            )
            .unwrap(),
            rr_require_perfects: main_window.get_rr_require_perfects(),
            oxide_final_challenge_unlock: FinalOxideUnlock::try_from(
                main_window.get_oxide_final_challenge_unlock(),
            )
            .unwrap(),
        };
        let chosen_settings = SeedSettings {
            randomization: chosen_rando_settings,
            general: chosen_general_settings,
            qol: chosen_qol_settings,
            write_spoilerlog: main_window.get_write_spoilerlog(),
            write_patchfile: false,
        };

        //todo Generate seed
        let rom_path = main_window.get_rom_path();
        let rom_path = rom_path.as_str();
        generate_seed(rom_path, chosen_settings);
    });

    let main_ui_weak = ui.as_weak();
    let main_window = main_ui_weak.unwrap();
    ui.on_pick_rom(move || {
        // Open File Picker Dialog
        let files = FileDialog::new().add_filter(".bin", &["bin"]).pick_file();
        if files.is_some() {
            let something = files.unwrap();
            let rom_path_str = something.to_str().unwrap();

            main_window.set_rom_valid_state(RomValidState::NoRom as i32);

            // Validate ROM
            let f = File::open(rom_path_str).unwrap();
            let f_len = f.metadata().unwrap().len();
            let buf_len = f_len.min(1_000_000) as usize;
            let mut f_buf = BufReader::with_capacity(buf_len, f);
            let mut context = md5::Context::new();
            loop {
                let part = f_buf.fill_buf().unwrap();
                if part.is_empty() {
                    break;
                }
                context.consume(part);
                let part_len = part.len();
                f_buf.consume(part_len);
            }
            let digest = context.finalize();
            if format!("{:x}", digest) == "ab95bfca8a4bb3d90daa6519acf6e944" {
                main_window.set_can_generate(true);
                main_window.set_rom_valid_state(RomValidState::Valid as i32);
            } else {
                main_window.set_can_generate(false);
                main_window.set_rom_valid_state(RomValidState::Invalid as i32);
            }

            main_window.set_rom_path(SharedString::from(rom_path_str));
        }
    });

    let _ = ui.run();

    Ok(())
}

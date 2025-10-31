// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, stdout};
use rfd::FileDialog;
use slint::SharedString;
use md5;
use open;

use crate::seed_settings::QualityOfLifeSettings;

slint::include_modules!();

pub mod seed_settings;

enum RomValidState {
    NoRom = 0,
    //Validating = 1, // not actually used, just for reference
    Invalid = 2,
    Valid = 3
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = MainWindow::new()?;

    let main_ui_weak = ui.as_weak();
    let main_window = main_ui_weak.unwrap();

    ui.on_sources_generator(move || {
        let _ = open::that("https://docs.rs/slint/latest/slint/struct.SharedString.html");
    });

    ui.on_sources_mod(move || {
        let _ = open::that("https://github.com/icebound777/CTR-Randomizer/");
    });

    ui.on_gen_seed(move || {
        // Collect settings chosen via UI
        let chosen_qol_settings = QualityOfLifeSettings {
            skip_mask_hints: main_window.get_qol_skip_mask_hints(),
            autoskip_podium_cutscenes: main_window.get_qol_skip_podium(),
            skip_mask_congrats: main_window.get_qol_skip_mask_congrats()
        };

        // Generate seed
        let mut lock = stdout().lock();
        write!(lock, "hello world").unwrap();
    });

    let main_ui_weak = ui.as_weak();
    let main_window = main_ui_weak.unwrap();
    ui.on_pick_rom(move || {
        // Open File Picker Dialog
        let files = FileDialog::new()
            .add_filter(".bin", &["bin"])
            .pick_file();
        if files.is_some() {
            let tmp_files = files.unwrap();
            let rom_path = tmp_files.to_str().unwrap();

            main_window.set_rom_valid_state(RomValidState::NoRom as i32);

            // Validate ROM
            let f = File::open(rom_path).unwrap();
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

            main_window.set_rom_path(SharedString::from(rom_path));
        }
    });

    let _ = ui.run();

    Ok(())
}

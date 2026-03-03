use std::{io, path::PathBuf};

use qbsdiff::{Bsdiff, Bspatch};

pub fn apply_base_patchfile(old_rom_path: &str, seed: u32) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let patchdata = include_bytes!("../../../res/base_patch.bsdiff4");

    apply_patch(old_rom_path, SeedOrFilename::Seed(seed), patchdata)
}
enum SeedOrFilename {
    Seed(u32),
    Filename(String),
}

fn apply_patch(old_rom_path: &str, seed_or_filename: SeedOrFilename, patchdata: &[u8]) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let old_rom = std::fs::read(old_rom_path)?;

    let mut new_rom = Vec::new();
    let patcher = Bspatch::new(patchdata)?;
    patcher.apply(&old_rom, io::Cursor::new(&mut new_rom))?;

    let new_rom_name = match seed_or_filename {
        SeedOrFilename::Seed(seed) => format!("{}{}{}", "CTR-Randomizer_", seed, ".bin"),
        SeedOrFilename::Filename(filename) => filename,
    };
    let mut new_rom_path = PathBuf::from(old_rom_path);
    new_rom_path.pop();
    new_rom_path.push(new_rom_name);
    std::fs::write(&new_rom_path, &new_rom)?;

    Ok(new_rom_path)
}

pub fn create_patchfile(old_rom_path: &str, new_rom_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut patch = Vec::new();
    let old_rom = std::fs::read(old_rom_path)?;
    let new_rom = std::fs::read(new_rom_path)?;

    let mut patchfile_path = new_rom_path.clone();
    let file_stem = patchfile_path.clone();
    let file_stem = file_stem.file_stem().unwrap();
    patchfile_path.pop();
    patchfile_path.push(format!("{}{}", file_stem.to_str().unwrap(), ".bsdiff4"));

    Bsdiff::new(&old_rom, &new_rom).compare(io::Cursor::new(&mut patch))?;

    std::fs::write(&patchfile_path, patch)?;
    Ok(patchfile_path)
}

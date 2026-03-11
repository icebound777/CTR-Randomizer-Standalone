use std::{io, path::PathBuf};

use qbsdiff::{Bsdiff, Bspatch};

pub fn apply_base_patchfile(old_rom_path: &str, seed: u32) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let patchdata = include_bytes!("../../../res/base_patch.bsdiff4");

    apply_patch(old_rom_path, SeedOrFilename::Seed(seed), Vec::from(patchdata))
}
enum SeedOrFilename {
    Seed(u32),
    Filename(String),
}

pub fn apply_patchfile(old_rom_path: &str, patch_file_path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let patch_file_path = PathBuf::from(patch_file_path);

    let file_stem = patch_file_path.clone();
    let file_stem = format!("{}{}", file_stem.file_stem().unwrap().to_str().unwrap(), ".bin") ;

    let patchdata = std::fs::read(patch_file_path).unwrap();

    apply_patch(old_rom_path, SeedOrFilename::Filename(file_stem), patchdata)
}

fn apply_patch(old_rom_path: &str, seed_or_filename: SeedOrFilename, patchdata: Vec<u8>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let old_rom = std::fs::read(old_rom_path)?;
    let mut new_rom = Vec::new();

    let mut patchdata_array: [u8; 1000000] = [0u8; 1000000]; // jank bs
    for (i, byte) in patchdata.iter().enumerate() {
        patchdata_array[i] = *byte;
    }
    let patcher = Bspatch::new(&patchdata_array)?;
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

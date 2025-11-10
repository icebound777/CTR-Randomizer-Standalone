use std::{env::current_exe, io, path::PathBuf};

use qbsdiff::{Bsdiff, Bspatch};

pub fn apply_patchfile(old_rom_path: &str, seed: u32) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut patchfile_path = current_exe().unwrap();
    patchfile_path.pop();
    patchfile_path.push("res");
    patchfile_path.push("base_patch.bsdiff4");

    let old_rom = std::fs::read(old_rom_path)?;
    //let patch = std::fs::read(patchfile_path)?;
    let patch = std::fs::read("/home/icebound/work/Git/CTRRandomizer-Standalone/res/base_patch.bsdiff4")?;

    let mut new_rom = Vec::new();
    let patcher = Bspatch::new(&patch)?;
    patcher.apply(&old_rom, io::Cursor::new(&mut new_rom))?;

    let mut new_rom_path = PathBuf::from(old_rom_path);
    new_rom_path.pop();
    new_rom_path.push(format!("{}{}{}", "CTR-Randomizer_", seed, ".bin"));
    std::fs::write(&new_rom_path, &new_rom)?;

    Ok(new_rom_path)
}

pub fn create_patchfile(old_rom_path: &str, new_rom_path: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut patch = Vec::new();
    let old_rom = std::fs::read(old_rom_path)?;
    let new_rom = std::fs::read(&new_rom_path)?;

    let mut patchfile_path = new_rom_path;
    let file_stem = patchfile_path.clone();
    let file_stem = file_stem.file_stem().unwrap();
    patchfile_path.pop();
    patchfile_path.push(format!("{}{}", file_stem.to_str().unwrap(), ".bsdiff4"));

    Bsdiff::new(&old_rom, &new_rom).compare(io::Cursor::new(&mut patch))?;

    std::fs::write(&patchfile_path, patch)?;
    Ok(patchfile_path)
}

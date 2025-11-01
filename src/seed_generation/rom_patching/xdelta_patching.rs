use std::{env::current_exe, io, path::PathBuf};

use qbsdiff::Bspatch;

pub fn apply_patchfile(old_rom_path: &str, seed: u32) -> Result<(), Box<dyn std::error::Error>> {
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
    new_rom_path.push(format!("{}{}{}", "CTR-Rando_", seed, ".bin"));
    std::fs::write(new_rom_path, &new_rom)?;

    Ok(())
}

pub fn create_patchfile(patchile_path: &str, rom_path: &str) {

}

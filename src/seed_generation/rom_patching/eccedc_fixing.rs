use std::{
    io::{Read, Seek, Write},
    path::PathBuf,
};

const SECTOR_SIZE: usize = 2352;

type Sector = [u8; SECTOR_SIZE];
type EDC = u32;

struct LookupTables {
    ecc_f: [u8; 256],
    ecc_b: [u8; 256],
    edc: [EDC; 256],
}

enum EccCodeSwitch {
    PCode,
    QCode,
}

enum GeneratedEccCode {
    PCode([u8; 86 * 2]),
    QCode([u8; 52 * 2]),
}

fn get_lookup_tables() -> LookupTables {
    let mut lookup_tables: LookupTables = LookupTables {
        ecc_f: [0u8; 256],
        ecc_b: [0u8; 256],
        edc: [0u32; 256],
    };

    let mut j: u8;
    let mut edc: u32;
    for i in 0..=255 {
        j = (((i as u16) << 1) ^ if i & 0x80 != 0 { 0x11Du16 } else { 0 }) as u8;
        lookup_tables.ecc_f[i as usize] = j;
        lookup_tables.ecc_b[(i ^ j) as usize] = i;
        edc = i as u32;

        for _ in 0..8 {
            edc = (edc >> 1) ^ if edc & 1 != 0 { 0xD8018001 } else { 0 };
        }

        lookup_tables.edc[i as usize] = edc;
    }

    lookup_tables
}

fn get_computed_edc_block(lookup_table: &LookupTables, sector_slice: &[u8]) -> EDC {
    let mut edc = 0;

    for b in sector_slice {
        edc = (edc >> 8) ^ lookup_table.edc[((edc ^ (*b as u32)) & 0xFF) as usize];
    }

    edc
}

fn get_computed_ecc_block(
    luts: &LookupTables,
    sector_slice: &[u8],
    code_type: EccCodeSwitch,
) -> GeneratedEccCode {
    let (major_count, minor_count, major_mult, minor_inc) = match code_type {
        EccCodeSwitch::PCode => (86u16, 24u16, 2u16, 86u16),
        EccCodeSwitch::QCode => (52u16, 43u16, 86u16, 88u16),
    };

    let size = major_count * minor_count;
    let mut block = if matches!(code_type, EccCodeSwitch::PCode) {
        GeneratedEccCode::PCode([0u8; 86 * 2])
    } else {
        GeneratedEccCode::QCode([0u8; 52 * 2])
    };

    for major in 0..major_count {
        let mut index = (major >> 1) * major_mult + (major & 1);
        let mut ecc_a = 0;
        let mut ecc_b = 0;

        for _ in 0..minor_count {
            let temp = sector_slice[index as usize];
            index += minor_inc;
            if index >= size {
                index -= size;
            }

            ecc_a ^= temp;
            ecc_b ^= temp;
            ecc_a = luts.ecc_f[ecc_a as usize];
        }

        ecc_a = luts.ecc_b[(luts.ecc_f[ecc_a as usize] ^ ecc_b) as usize];
        match &mut block {
            GeneratedEccCode::PCode(x) => {
                x[major as usize] = ecc_a;
                x[(major + major_count) as usize] = ecc_a ^ ecc_b;
            }
            GeneratedEccCode::QCode(x) => {
                x[major as usize] = ecc_a;
                x[(major + major_count) as usize] = ecc_a ^ ecc_b;
            }
        };
    }

    block
}

fn generate_ecc(lut: &LookupTables, sector: &mut Sector, zeroaddress: bool) {
    let mut address: [u8; 4] = [0u8; 4];

    if zeroaddress {
        address.copy_from_slice(&sector[12..12 + 4]);
        sector[12..12 + 4].copy_from_slice(&[0u8; 4]);
    }

    // Generate P Code
    let p_code = get_computed_ecc_block(lut, &sector[0xC..sector.len()], EccCodeSwitch::PCode);
    match p_code {
        GeneratedEccCode::PCode(x) => {
            sector[0x81C..0x81C + x.len()].copy_from_slice(&x);
        }
        _ => panic!(),
    }

    // Generate Q Code
    let q_code = get_computed_ecc_block(lut, &sector[0xC..sector.len()], EccCodeSwitch::QCode);
    match q_code {
        GeneratedEccCode::QCode(x) => {
            sector[0x8C8..0x8C8 + x.len()].copy_from_slice(&x);
        }
        _ => panic!(),
    }

    if zeroaddress {
        sector[12..12 + 4].copy_from_slice(&address);
    }
}

fn ecc_edc_generate(lut: &LookupTables, sector: &mut Sector) {
    const SYNCHEADER: [u8; 12] = [0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0];
    sector[0..12].copy_from_slice(&SYNCHEADER);

    let mode = sector[0xF];

    if mode == 0 {
        // sector[0x10:0x10+0x920] = [0] * 0x920
        sector[0x10..0x10 + 0x920].copy_from_slice(&[0u8; 0x920]);
    } else if mode == 1 {
        // let edc = get_computed_edc_block(sector[:0x810]);
        let edc = get_computed_edc_block(lut, &sector[0x0..0x810]);
        // sector[0x810:0x810+4] = edc.to_bytes(4, byteorder='little');
        sector[0x810..0x810 + 4].copy_from_slice(&(edc.to_le_bytes()));
        // sector[0x814:0x814+8] = [0] * 8;
        sector[0x814..0x814 + 8].copy_from_slice(&[0u8; 8]);

        generate_ecc(lut, sector, false);
    } else if mode == 2 {
        let form = sector[0x12] & 0x20;

        if form == 0 {
            // Form 1
            let edc = get_computed_edc_block(lut, &sector[0x10..0x10 + 0x808]);
            // sector[0x818:0x818+4] = edc.to_bytes(4, byteorder='little')
            sector[0x818..0x818 + 4].copy_from_slice(&(edc.to_le_bytes()));
            generate_ecc(lut, sector, true);
        } else {
            // Form 2
            let edc = get_computed_edc_block(lut, &sector[0x10..0x10 + 0x91C]);
            sector[0x92C..0x92C + 4].copy_from_slice(&(edc.to_le_bytes()));
        }
    }
}

pub fn full_recalc<'a>(target_file: &PathBuf) -> Result<(), &'a str> {
    let filehandle = std::fs::File::options()
        .read(true)
        .write(true)
        .open(target_file);

    match filehandle {
        Ok(mut f) => {
            let file_size = f.metadata().unwrap().len();
            if file_size % (SECTOR_SIZE as u64) != 0 {
                return Err("nay");
            }
            let sector_count = file_size / (SECTOR_SIZE as u64);
            let lut = get_lookup_tables();
            let mut sector = [0u8; SECTOR_SIZE];

            for sector_no in 16..sector_count {
                let start = sector_no * (SECTOR_SIZE as u64);
                let _ = f.seek(std::io::SeekFrom::Start(start));
                let _ = f.read(&mut sector).unwrap();
                ecc_edc_generate(&lut, &mut sector);

                let _ = f.seek(std::io::SeekFrom::Start(start));
                let _ = f.write(&sector);
                //stats.recalc_sectors += 1
            }
            Ok(())
        }
        _ => Err("Could not open patched ROM for writing randomization data!"),
    }
}

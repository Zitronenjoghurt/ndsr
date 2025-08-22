use binrw::{BinRead, Endian};
use std::io::{Read, Seek};

#[derive(Debug, BinRead)]
#[br(little)]
pub struct FNT {
    pub root_main_entry: RootMainTableEntry,
    #[br(count = root_main_entry.total_dirs - 1)]
    pub main_table: Vec<MainTableEntry>,
    #[br(parse_with = parse_subtables, args(&root_main_entry, &main_table))]
    pub sub_tables: Vec<Vec<SubTableEntry>>,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct RootMainTableEntry {
    pub subtable_offset: u32,
    pub first_file_id: u16,
    pub total_dirs: u16,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MainTableEntry {
    pub subtable_offset: u32,
    pub first_file_id: u16,
    pub parent_dir_id: u16,
}

#[derive(Debug)]
pub struct SubTableEntry {
    pub type_length: u8,
    pub name: String,
    pub sub_dir_id: Option<u16>,
}

fn parse_subtables<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
    args: (&RootMainTableEntry, &Vec<MainTableEntry>),
) -> binrw::BinResult<Vec<Vec<SubTableEntry>>> {
    let (root, main_entries) = args;
    let mut entries = Vec::new();

    reader.seek(std::io::SeekFrom::Start(root.subtable_offset as u64))?;
    entries.push(parse_subtable(reader, endian)?);

    for entry in main_entries {
        reader.seek(std::io::SeekFrom::Start(entry.subtable_offset as u64))?;
        entries.push(parse_subtable(reader, endian)?);
    }

    Ok(entries)
}

fn parse_subtable<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
) -> binrw::BinResult<Vec<SubTableEntry>> {
    let mut entries = Vec::new();

    loop {
        let type_length = u8::read_options(reader, endian, ())?;
        if type_length == 0 {
            break;
        }

        let is_sub_dir = type_length > 0x80;
        let name_length = type_length & 0x7F;

        let mut name_bytes = vec![0; name_length as usize];
        reader.read_exact(&mut name_bytes)?;
        let name = String::from_utf8_lossy(&name_bytes).to_string();

        let sub_dir_id = if is_sub_dir {
            Some(u16::read_options(reader, endian, ())?)
        } else {
            None
        };

        entries.push(SubTableEntry {
            type_length,
            name,
            sub_dir_id,
        });
    }

    Ok(entries)
}

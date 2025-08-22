use binrw::{BinRead, BinWrite};

#[derive(Debug, BinRead, BinWrite)]
#[br(little, import(fat_size: u32))]
#[bw(little)]
pub struct FAT {
    #[br(count = fat_size / 8)]
    pub entries: Vec<FATEntry>,
}

#[derive(Debug, BinRead, BinWrite)]
#[br(little)]
#[bw(little)]
pub struct FATEntry {
    pub start_address: u32,
    pub end_address: u32,
}

impl FATEntry {
    pub fn size(&self) -> u32 {
        self.end_address.saturating_sub(self.start_address)
    }
}

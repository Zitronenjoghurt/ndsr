use crate::codec::data::filesystem::Filesystem;
use crate::codec::data::icon_title::{RawIconTitle, ICON_TITLE_SIZE};
use crate::codec::raw::header::HEADER_SIZE;
use crate::codec::utils::read_to_end;
use crate::error::NDSRResult;
use binrw::{BinRead, BinWrite};
use filesystem::fat::FAT;
use filesystem::fnt::FNT;
use std::io::Cursor;

pub mod filesystem;
pub mod icon_bitmap;
pub mod icon_palette;
pub mod icon_title;

#[derive(Debug, Clone, Hash, BinRead, BinWrite)]
#[br(little)]
#[bw(little)]
pub struct ROMData {
    #[br(parse_with = read_to_end)]
    pub data: Vec<u8>,
}

impl ROMData {
    pub fn get_data_slice(&self, absolute_rom_address: usize, size: usize) -> &[u8] {
        // Since the header is already decoded as RawHeader and not included in data
        let data_address = absolute_rom_address - HEADER_SIZE;
        &self.data[data_address..data_address + size]
    }

    pub fn extract_file(&self, file_offset: u32, file_size: u32) -> NDSRResult<Vec<u8>> {
        Ok(self
            .get_data_slice(file_offset as usize, file_size as usize)
            .to_vec())
    }

    pub fn extract_icon_title(&self, icon_title_offset: u32) -> NDSRResult<RawIconTitle> {
        Ok(RawIconTitle::read(&mut Cursor::new(self.get_data_slice(
            icon_title_offset as usize,
            ICON_TITLE_SIZE,
        )))?)
    }

    pub fn extract_fat(&self, fat_offset: u32, fat_size: u32) -> NDSRResult<FAT> {
        let data = self.get_data_slice(fat_offset as usize, fat_size as usize);
        Ok(FAT::read_args(&mut Cursor::new(data), (fat_size,))?)
    }

    pub fn extract_fnt(&self, fnt_offset: u32, fnt_size: u32) -> NDSRResult<FNT> {
        let data = self.get_data_slice(fnt_offset as usize, fnt_size as usize);
        Ok(FNT::read(&mut Cursor::new(data))?)
    }

    pub fn extract_filesystem(
        &self,
        fat_offset: u32,
        fat_size: u32,
        fnt_offset: u32,
        fnt_size: u32,
    ) -> NDSRResult<Filesystem> {
        Filesystem::build(
            &self.extract_fat(fat_offset, fat_size)?,
            &self.extract_fnt(fnt_offset, fnt_size)?,
        )
    }
}

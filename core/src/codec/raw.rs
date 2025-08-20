use crate::codec::raw::header::HEADER_SIZE;
use crate::codec::raw::icon_title::{RawIconTitle, ICON_TITLE_SIZE};
use crate::codec::rom::NDSRom;
use crate::error::{NDSRError, NDSRResult};
use binrw::{BinRead, BinWrite};
use header::RawHeader;
use std::io::Cursor;

pub mod bgr555;
pub mod header;
pub mod icon_bitmap;
pub mod icon_palette;
pub mod icon_title;

#[derive(Debug, BinRead, BinWrite)]
#[br(little)]
#[bw(little)]
pub struct RawNDSRom {
    pub header: RawHeader,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub data: Vec<u8>,
}

impl RawNDSRom {
    pub fn get_data_slice(&self, absolute_rom_address: usize, size: usize) -> &[u8] {
        // Since the header is already decoded as RawHeader and not included in data
        let data_address = absolute_rom_address - HEADER_SIZE;
        &self.data[data_address..data_address + size]
    }

    pub fn get_icon_title(&self) -> NDSRResult<RawIconTitle> {
        Ok(RawIconTitle::read(&mut Cursor::new(self.get_data_slice(
            self.header.icon_title_offset as usize,
            ICON_TITLE_SIZE,
        )))?)
    }

    pub fn from_bytes(bytes: &[u8]) -> NDSRResult<Self> {
        Ok(Self::read(&mut Cursor::new(bytes))?)
    }

    pub fn to_bytes(&self) -> NDSRResult<Vec<u8>> {
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        self.write(&mut cursor)?;
        Ok(buffer)
    }
}

impl TryFrom<&NDSRom> for RawNDSRom {
    type Error = NDSRError;

    fn try_from(rom: &NDSRom) -> NDSRResult<Self> {
        let header = RawHeader::try_from(rom)?;
        Ok(Self {
            header,
            data: rom.data.clone(),
        })
    }
}

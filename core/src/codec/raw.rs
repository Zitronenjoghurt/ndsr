use crate::codec::rom::NDSRom;
use crate::error::{NDSRError, NDSRResult};
use binrw::{BinRead, BinWrite};
use header::RawNDSHeader;
use std::io::Cursor;

pub mod header;

#[derive(Debug, BinRead, BinWrite)]
#[br(little)]
pub struct RawNDSRom {
    pub header: RawNDSHeader,
}

impl RawNDSRom {
    pub fn from_bytes(bytes: &[u8]) -> NDSRResult<Self> {
        Ok(Self::read(&mut Cursor::new(bytes))?)
    }
}

impl TryFrom<NDSRom> for RawNDSRom {
    type Error = NDSRError;

    fn try_from(rom: NDSRom) -> NDSRResult<Self> {
        let header = RawNDSHeader::try_from(rom)?;
        Ok(Self { header })
    }
}

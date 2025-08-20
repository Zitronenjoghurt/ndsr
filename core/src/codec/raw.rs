use crate::codec::rom::NDSRom;
use crate::error::{NDSRError, NDSRResult};
use binrw::{BinRead, BinWrite};
use header::RawNDSHeader;
use std::io::Cursor;

pub mod header;

#[derive(Debug, BinRead, BinWrite)]
#[br(little)]
#[bw(little)]
pub struct RawNDSRom {
    pub header: RawNDSHeader,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub data: Vec<u8>,
}

impl RawNDSRom {
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
        let header = RawNDSHeader::try_from(rom)?;
        Ok(Self {
            header,
            data: rom.data.clone(),
        })
    }
}

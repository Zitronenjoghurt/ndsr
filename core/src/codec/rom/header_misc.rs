use crate::codec::raw::header::RawNDSHeader;
use crate::error::{NDSRError, NDSRResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderMisc {
    pub encryption_seed_select: u8,
    pub _reserved1: [u8; 7],
}

impl TryFrom<&RawNDSHeader> for HeaderMisc {
    type Error = NDSRError;

    fn try_from(header: &RawNDSHeader) -> NDSRResult<Self> {
        let misc = Self {
            encryption_seed_select: header.encryption_seed_select,
            _reserved1: header._reserved1,
        };

        Ok(misc)
    }
}

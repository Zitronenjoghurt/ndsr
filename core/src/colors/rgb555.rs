use binrw::{BinRead, BinWrite};
use modular_bitfield::bitfield;
use modular_bitfield::prelude::*;
use serde::{Deserialize, Serialize};

#[bitfield]
#[derive(Debug, Default, Clone, Copy, BinRead, BinWrite, Serialize, Deserialize)]
#[br(map = |x: u16| Self::from_bytes(x.to_le_bytes()))]
#[bw(map = |x: &Self| u16::from_le_bytes(x.into_bytes()))]
pub struct RGB555 {
    pub red: B5,
    pub green: B5,
    pub blue: B5,
    pub unused: B1,
}

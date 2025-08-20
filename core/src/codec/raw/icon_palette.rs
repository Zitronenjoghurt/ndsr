use crate::codec::raw::bgr555::BGR555;
use binrw::{BinRead, BinWrite};

#[derive(Debug, BinRead, BinWrite)]
pub struct RawIconPalette {
    /// Color 0 is always ignored => it's transparent
    pub colors: [BGR555; 16],
}

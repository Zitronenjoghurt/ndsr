use crate::colors::rgb555::RGB555;
use binrw::{BinRead, BinWrite};

#[derive(Debug, Clone, Copy, BinRead, BinWrite)]
pub struct RGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB8 {
    pub fn as_vec(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<RGB555> for RGB8 {
    fn from(value: RGB555) -> Self {
        Self {
            r: (value.red() << 3) | (value.red() >> 2),
            g: (value.green() << 3) | (value.green() >> 2),
            b: (value.blue() << 3) | (value.blue() >> 2),
        }
    }
}

use crate::colors::rgb555::RGB555;
use crate::colors::rgb8::RGB8;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, BinRead, BinWrite, Serialize, Deserialize)]
pub struct RawIconPalette {
    /// Color 0 is always ignored => it's transparent
    pub colors: [RGB555; 16],
}

impl RawIconPalette {
    pub fn get_palette_bytes(&self) -> Vec<u8> {
        self.get_rgba_colors()
            .iter()
            .fold(Vec::new(), |mut acc, color| {
                acc.extend(color.as_vec());
                acc
            })
    }

    pub fn get_rgba_colors(&self) -> [RGB8; 16] {
        [
            self.colors[0].into(),
            self.colors[1].into(),
            self.colors[2].into(),
            self.colors[3].into(),
            self.colors[4].into(),
            self.colors[5].into(),
            self.colors[6].into(),
            self.colors[7].into(),
            self.colors[8].into(),
            self.colors[9].into(),
            self.colors[10].into(),
            self.colors[11].into(),
            self.colors[12].into(),
            self.colors[13].into(),
            self.colors[14].into(),
            self.colors[15].into(),
        ]
    }
}

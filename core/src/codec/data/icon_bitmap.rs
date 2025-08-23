use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, BinRead, BinWrite, Serialize, Deserialize)]
pub struct RawIconBitmap {
    /// 4x4 grid of 8x8 pixels
    pub tiles: [RawIconBitmapTile; 16],
}

impl RawIconBitmap {
    pub fn get_indexed_image_data(&self) -> [u8; 512] {
        let mut image_data = [0u8; 512];

        for y in 0..32 {
            for x in 0..16 {
                let pixel_index = x + 16 * y;
                let tile_index = (x / 4) + 4 * (y / 8);
                let tile_x = x % 4;
                let tile_y = y % 8;
                let tile_pixel_index = tile_x + 4 * tile_y;

                let byte = self.tiles[tile_index].pixels[tile_pixel_index];
                image_data[pixel_index] = ((byte & 0x0F) << 4) | ((byte >> 4) & 0x0F);
            }
        }

        image_data
    }
}

#[derive(Debug, Default, Clone, BinRead, BinWrite, Serialize, Deserialize)]
pub struct RawIconBitmapTile {
    /// 8x8 Grid, 4 bit per pixel
    pub pixels: [u8; 32],
}

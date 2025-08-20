use binrw::{BinRead, BinWrite};

#[derive(Debug, BinRead, BinWrite)]
pub struct RawIconBitmap {
    /// 4x4 grid of 8x8 pixels
    pub tiles: [RawIconBitmapTile; 16],
}

#[derive(Debug, BinRead, BinWrite)]
pub struct RawIconBitmapTile {
    /// 8x8 Grid, 4 bit per pixel
    pub pixels: [u8; 32],
}

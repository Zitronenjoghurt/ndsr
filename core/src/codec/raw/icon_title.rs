use crate::codec::raw::icon_bitmap::RawIconBitmap;
use crate::codec::raw::icon_palette::RawIconPalette;
use binrw::{BinRead, BinWrite};
use png::{BitDepth, ColorType};

pub const ICON_TITLE_SIZE: usize = 0x1240;

#[derive(Debug, BinRead, BinWrite)]
#[br(little)]
#[bw(little)]
pub struct RawIconTitle {
    pub version: u16,
    pub crc1: u16,
    pub crc2: u16,
    pub crc3: u16,
    pub crc4: u16,
    pub _reserved1: [u8; 22],
    pub icon_bitmap: RawIconBitmap,
    pub icon_palette: RawIconPalette,
    pub title_japanese: [u16; 128],
    pub title_english: [u16; 128],
    pub title_french: [u16; 128],
    pub title_german: [u16; 128],
    pub title_italian: [u16; 128],
    pub title_spanish: [u16; 128],
    pub title_chinese: [u16; 128],
    pub title_korean: [u16; 128],
    pub _reserved2: [u8; 2048],
}

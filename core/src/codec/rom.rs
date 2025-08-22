use crate::codec::data::filesystem::Filesystem;
use crate::codec::data::ROMData;
use crate::codec::raw::RawNDSRom;
use crate::codec::utils::{decode_string, trim_zeros_u8};
use crate::error::{NDSRError, NDSRResult};

mod cartridge_size;
mod destination_language;
mod header_misc;
mod icon;
mod nds_region;
mod titles;
mod unique_code_category;
mod unit_code;

#[derive(Debug)]
pub struct NDSRom {
    pub titles: titles::Titles,
    pub short_title: String,
    pub title_code: String,
    pub destination_language: destination_language::DestinationLanguage,
    pub unique_code_category: unique_code_category::UniqueCodeCategory,
    pub maker_code: String,
    pub unit_code: unit_code::UnitCode,
    pub cartridge_size: cartridge_size::CartridgeSize,
    pub rom_version: u8,
    pub rom_size: u32,
    pub header_misc: header_misc::HeaderMisc,
    pub icon: icon::RomIcon,
    pub data: ROMData,
}

impl NDSRom {
    pub fn from_bytes(bytes: &[u8]) -> NDSRResult<Self> {
        Self::try_from(RawNDSRom::from_bytes(bytes)?)
    }

    pub fn into_bytes(self) -> NDSRResult<Vec<u8>> {
        RawNDSRom::try_from(self).map(|raw| raw.to_bytes())?
    }

    pub fn get_filesystem(&self) -> NDSRResult<Filesystem> {
        self.data.extract_filesystem(
            self.header_misc.fat_offset,
            self.header_misc.fat_size,
            self.header_misc.fnt_offset,
            self.header_misc.fnt_size,
        )
    }
}

impl TryFrom<RawNDSRom> for NDSRom {
    type Error = NDSRError;

    fn try_from(raw: RawNDSRom) -> NDSRResult<Self> {
        let raw_icon_title = raw.data.extract_icon_title(raw.header.icon_title_offset)?;
        let titles = titles::Titles::try_from(&raw_icon_title)?;

        let rom = Self {
            titles,
            short_title: decode_string(&trim_zeros_u8(&raw.header.game_title)),
            title_code: decode_string(&raw.header.game_code[1..=2]),
            destination_language: destination_language::DestinationLanguage::from(
                raw.header.game_code[3],
            ),
            unique_code_category: unique_code_category::UniqueCodeCategory::from(
                raw.header.game_code[0],
            ),
            maker_code: decode_string(&raw.header.maker_code),
            unit_code: unit_code::UnitCode::from(raw.header.unit_code),
            cartridge_size: cartridge_size::CartridgeSize::from(raw.header.device_capacity),
            rom_version: raw.header.rom_version,
            rom_size: raw.header.total_used_rom_size,
            header_misc: header_misc::HeaderMisc::try_from(&raw.header)?,
            icon: icon::RomIcon::from(raw_icon_title),
            data: raw.data,
        };

        Ok(rom)
    }
}

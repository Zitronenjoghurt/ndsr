use crate::codec::raw::RawNDSRom;
use crate::codec::utils::{decode_string, trim_zeros};
use crate::error::{NDSRError, NDSRResult};
use serde::{Deserialize, Serialize};

mod cartridge_size;
mod destination_language;
mod header_misc;
mod unique_code_category;
mod unit_code;

#[derive(Debug, Serialize, Deserialize)]
pub struct NDSRom {
    pub game_title: String,
    pub short_title: String,
    pub destination_language: destination_language::DestinationLanguage,
    pub unique_code_category: unique_code_category::UniqueCodeCategory,
    pub maker_code: String,
    pub unit_code: unit_code::UnitCode,
    pub cartridge_size: cartridge_size::CartridgeSize,
    pub header_misc: header_misc::HeaderMisc,
}

impl NDSRom {
    pub fn from_bytes(bytes: &[u8]) -> NDSRResult<Self> {
        Self::try_from(RawNDSRom::from_bytes(bytes)?)
    }
}

impl TryFrom<RawNDSRom> for NDSRom {
    type Error = NDSRError;

    fn try_from(raw: RawNDSRom) -> NDSRResult<Self> {
        let rom = Self {
            game_title: decode_string(&trim_zeros(&raw.header.game_title)),
            short_title: decode_string(&raw.header.game_code[1..=2]),
            destination_language: destination_language::DestinationLanguage::from(
                raw.header.game_code[3],
            ),
            unique_code_category: unique_code_category::UniqueCodeCategory::from(
                raw.header.game_code[0],
            ),
            maker_code: decode_string(&raw.header.maker_code),
            unit_code: unit_code::UnitCode::from(raw.header.unit_code),
            cartridge_size: cartridge_size::CartridgeSize::from(raw.header.device_capacity),
            header_misc: header_misc::HeaderMisc::try_from(&raw.header)?,
        };

        Ok(rom)
    }
}

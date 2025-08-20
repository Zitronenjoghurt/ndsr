use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum UniqueCodeCategory {
    Unknown(u8),
    BuiltInInfraredPort,
    NDSCommonGames(u8),
    NDSManyGames(u8),
    NDSNintendoChannelDemos,
    NDSUtilitiesEducationalOrUncommonExtraHardware,
    DSiEnhanced,
    DSiExclusives,
    DSiWareGames,
    DSiWareSystemUtilities,
}

impl From<u8> for UniqueCodeCategory {
    fn from(value: u8) -> Self {
        match value {
            b'A' => Self::NDSCommonGames(b'A'),
            b'B' => Self::NDSCommonGames(b'B'),
            b'C' => Self::NDSCommonGames(b'C'),
            b'D' => Self::DSiExclusives,
            b'H' => Self::DSiWareSystemUtilities,
            b'I' => Self::BuiltInInfraredPort,
            b'K' => Self::DSiWareGames,
            b'N' => Self::NDSNintendoChannelDemos,
            b'T' => Self::NDSManyGames(b'T'),
            b'U' => Self::NDSUtilitiesEducationalOrUncommonExtraHardware,
            b'V' => Self::DSiEnhanced,
            b'Y' => Self::NDSManyGames(b'Y'),
            _ => Self::Unknown(value),
        }
    }
}

impl From<UniqueCodeCategory> for u8 {
    fn from(value: UniqueCodeCategory) -> Self {
        match value {
            UniqueCodeCategory::Unknown(v) => v,
            UniqueCodeCategory::BuiltInInfraredPort => b'I',
            UniqueCodeCategory::NDSCommonGames(v) => v,
            UniqueCodeCategory::NDSManyGames(v) => v,
            UniqueCodeCategory::NDSNintendoChannelDemos => b'N',
            UniqueCodeCategory::NDSUtilitiesEducationalOrUncommonExtraHardware => b'U',
            UniqueCodeCategory::DSiEnhanced => b'V',
            UniqueCodeCategory::DSiExclusives => b'D',
            UniqueCodeCategory::DSiWareGames => b'K',
            UniqueCodeCategory::DSiWareSystemUtilities => b'H',
        }
    }
}

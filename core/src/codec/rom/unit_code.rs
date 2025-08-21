#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone)]
pub enum UnitCode {
    Unknown(u8),
    NDS,
    NDSandDSi,
    DSi,
}

impl From<u8> for UnitCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NDS,
            0x02 => Self::NDSandDSi,
            0x03 => Self::DSi,
            _ => Self::Unknown(value),
        }
    }
}

impl From<UnitCode> for u8 {
    fn from(value: UnitCode) -> Self {
        match value {
            UnitCode::Unknown(v) => v,
            UnitCode::NDS => 0x00,
            UnitCode::NDSandDSi => 0x02,
            UnitCode::DSi => 0x03,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash)]
pub enum NDSRegion {
    Unknown(u8),
    Normal,
    China,
    Korea,
}

impl From<u8> for NDSRegion {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Normal,
            0x80 => Self::China,
            0x40 => Self::Korea,
            _ => Self::Unknown(value),
        }
    }
}

impl From<NDSRegion> for u8 {
    fn from(value: NDSRegion) -> Self {
        match value {
            NDSRegion::Normal => 0x00,
            NDSRegion::China => 0x80,
            NDSRegion::Korea => 0x40,
            NDSRegion::Unknown(v) => v,
        }
    }
}

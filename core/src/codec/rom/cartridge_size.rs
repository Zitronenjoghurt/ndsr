/// Manufactured cartridge sizes only ranged from 8MB to 512MB
/// Source: https://en.wikipedia.org/wiki/Nintendo_Game_Card
#[derive(Debug, Copy, Clone)]
pub enum CartridgeSize {
    Unknown(u8),
    KB128,
    KB256,
    KB512,
    MB1,
    MB2,
    MB4,
    MB8,
    MB16,
    MB32,
    MB64,
    MB128,
    MB256,
    MB512,
}

impl From<u8> for CartridgeSize {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::KB128,
            1 => Self::KB256,
            2 => Self::KB512,
            3 => Self::MB1,
            4 => Self::MB2,
            5 => Self::MB4,
            6 => Self::MB8,
            7 => Self::MB16,
            8 => Self::MB32,
            9 => Self::MB64,
            10 => Self::MB128,
            11 => Self::MB256,
            12 => Self::MB512,
            _ => Self::Unknown(value),
        }
    }
}

impl From<CartridgeSize> for u8 {
    fn from(value: CartridgeSize) -> Self {
        match value {
            CartridgeSize::KB128 => 0,
            CartridgeSize::KB256 => 1,
            CartridgeSize::KB512 => 2,
            CartridgeSize::MB1 => 3,
            CartridgeSize::MB2 => 4,
            CartridgeSize::MB4 => 5,
            CartridgeSize::MB8 => 6,
            CartridgeSize::MB16 => 7,
            CartridgeSize::MB32 => 8,
            CartridgeSize::MB64 => 9,
            CartridgeSize::MB128 => 10,
            CartridgeSize::MB256 => 11,
            CartridgeSize::MB512 => 12,
            CartridgeSize::Unknown(value) => value,
        }
    }
}

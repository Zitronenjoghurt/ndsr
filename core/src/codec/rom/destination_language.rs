#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone)]
pub enum DestinationLanguage {
    Unknown(u8),
    Asian,
    Australian,
    Chinese,
    Danish,
    Dutch,
    EnglishUSA,
    Europe(u8),
    EuropeAndAustralia,
    French,
    German,
    Int,
    Italian,
    Japanese,
    Korean,
    Nor,
    Russian,
    Spanish,
    Swedish,
    USA,
    USAandAustralia,
}

impl From<u8> for DestinationLanguage {
    fn from(value: u8) -> Self {
        match value {
            b'A' => Self::Asian,
            b'C' => Self::Chinese,
            b'D' => Self::German,
            b'E' => Self::EnglishUSA,
            b'F' => Self::French,
            b'H' => Self::Dutch,
            b'I' => Self::Italian,
            b'J' => Self::Japanese,
            b'K' => Self::Korean,
            b'L' => Self::USA,
            b'M' => Self::Swedish,
            b'N' => Self::Nor,
            b'O' => Self::Int,
            b'P' => Self::Europe(b'P'),
            b'Q' => Self::Danish,
            b'R' => Self::Russian,
            b'S' => Self::Spanish,
            b'T' => Self::USAandAustralia,
            b'U' => Self::Australian,
            b'V' => Self::EuropeAndAustralia,
            b'W' => Self::Europe(b'W'),
            b'X' => Self::Europe(b'X'),
            b'Y' => Self::Europe(b'Y'),
            b'Z' => Self::Europe(b'Z'),
            _ => Self::Unknown(value),
        }
    }
}

impl From<DestinationLanguage> for u8 {
    fn from(value: DestinationLanguage) -> Self {
        match value {
            DestinationLanguage::Unknown(v) => v,
            DestinationLanguage::Asian => b'A',
            DestinationLanguage::Australian => b'U',
            DestinationLanguage::Chinese => b'C',
            DestinationLanguage::Danish => b'Q',
            DestinationLanguage::Dutch => b'H',
            DestinationLanguage::EnglishUSA => b'E',
            DestinationLanguage::Europe(v) => v,
            DestinationLanguage::EuropeAndAustralia => b'V',
            DestinationLanguage::French => b'F',
            DestinationLanguage::German => b'D',
            DestinationLanguage::Int => b'O',
            DestinationLanguage::Italian => b'I',
            DestinationLanguage::Japanese => b'J',
            DestinationLanguage::Korean => b'K',
            DestinationLanguage::Nor => b'N',
            DestinationLanguage::Russian => b'R',
            DestinationLanguage::Spanish => b'S',
            DestinationLanguage::Swedish => b'M',
            DestinationLanguage::USA => b'L',
            DestinationLanguage::USAandAustralia => b'T',
        }
    }
}

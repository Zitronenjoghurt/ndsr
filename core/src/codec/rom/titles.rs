use crate::codec::data::icon_title::RawIconTitle;
use crate::codec::utils::trim_zeros_u16;
use crate::error::{NDSRError, NDSRResult};

#[derive(Debug)]
pub struct Titles {
    pub japanese: TitleEntry,
    pub english: TitleEntry,
    pub french: TitleEntry,
    pub german: TitleEntry,
    pub italian: TitleEntry,
    pub spanish: TitleEntry,
    pub chinese: Option<TitleEntry>,
    pub korean: Option<TitleEntry>,
}

#[derive(Debug)]
pub struct TitleEntry {
    pub title: String,
    pub sub_title: Option<String>,
    pub manufacturer: String,
}

impl TryFrom<&[u16]> for TitleEntry {
    type Error = NDSRError;

    fn try_from(value: &[u16]) -> NDSRResult<Self> {
        let fields: Vec<String> = value
            .split(|&c| c == 0xA)
            .take(3)
            .map(|field| String::from_utf16_lossy(&trim_zeros_u16(field)))
            .filter(|s| !s.is_empty())
            .collect();

        match fields.len() {
            2 => Ok(Self {
                title: fields[0].clone(),
                sub_title: None,
                manufacturer: fields[1].clone(),
            }),
            3 => Ok(Self {
                title: fields[0].clone(),
                sub_title: Some(fields[1].clone()),
                manufacturer: fields[2].clone(),
            }),
            _ => Ok(Self {
                title: fields.first().cloned().unwrap_or_default(),
                sub_title: None,
                manufacturer: fields.get(1).cloned().unwrap_or_default(),
            }),
        }
    }
}

impl TryFrom<&RawIconTitle> for Titles {
    type Error = NDSRError;

    fn try_from(raw: &RawIconTitle) -> NDSRResult<Self> {
        let chinese = if raw.version >= 2 {
            Some(TitleEntry::try_from(raw.title_chinese.as_slice())?)
        } else {
            None
        };

        let korean = if raw.version >= 3 {
            Some(TitleEntry::try_from(raw.title_korean.as_slice())?)
        } else {
            None
        };

        Ok(Self {
            japanese: TitleEntry::try_from(raw.title_japanese.as_slice())?,
            english: TitleEntry::try_from(raw.title_english.as_slice())?,
            french: TitleEntry::try_from(raw.title_french.as_slice())?,
            german: TitleEntry::try_from(raw.title_german.as_slice())?,
            italian: TitleEntry::try_from(raw.title_italian.as_slice())?,
            spanish: TitleEntry::try_from(raw.title_spanish.as_slice())?,
            chinese,
            korean,
        })
    }
}

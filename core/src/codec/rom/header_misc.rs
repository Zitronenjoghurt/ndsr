use crate::codec::raw::header::RawHeader;
use crate::codec::rom::nds_region::NDSRegion;
use crate::codec::utils::serde_bytes;
use crate::error::{NDSRError, NDSRResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderMisc {
    pub encryption_seed_select: u8,
    #[serde(skip_serializing)]
    pub _reserved1: [u8; 7],
    pub dsi_flags: u8,
    pub nds_region: NDSRegion,
    pub autostart: u8,
    pub arm9_offset: u32,
    pub arm9_entry_address: u32,
    pub arm9_ram_address: u32,
    pub arm9_size: u32,
    pub arm7_rom_offset: u32,
    pub arm7_entry_address: u32,
    pub arm7_ram_address: u32,
    pub arm7_size: u32,
    pub fnt_offset: u32,
    pub fnt_size: u32,
    pub fat_offset: u32,
    pub fat_size: u32,
    pub file_arm9_overlay_offset: u32,
    pub file_arm9_overlay_size: u32,
    pub file_arm7_overlay_offset: u32,
    pub file_arm7_overlay_size: u32,
    pub gamecard_bus_normal_command_setting: u32,
    pub gamecard_bus_key1_command_setting: u32,
    pub icon_title_offset: u32,
    pub secure_area_checksum: u16,
    pub secure_area_delay: u16,
    pub arm9_autoload_list_hook_ram_address: u32,
    pub arm7_autoload_list_hook_ram_address: u32,
    #[serde(skip_serializing)]
    pub secure_area_disable: [u8; 8],
    pub header_size: u32,
    #[serde(skip_serializing)]
    #[serde(with = "serde_bytes")]
    pub _reserved2: [u8; 56],
    #[serde(skip_serializing)]
    #[serde(with = "serde_bytes")]
    pub nintendo_logo: [u8; 156],
    pub nintendo_logo_checksum: u16,
    pub header_checksum: u16,
    pub debug_rom_offset: u32,
    pub debug_rom_size: u32,
    pub debug_ram_address: u32,
    #[serde(skip_serializing)]
    #[serde(with = "serde_bytes")]
    pub _reserved3: [u8; 148],
}

impl TryFrom<&RawHeader> for HeaderMisc {
    type Error = NDSRError;

    fn try_from(header: &RawHeader) -> NDSRResult<Self> {
        let misc = Self {
            encryption_seed_select: header.encryption_seed_select,
            _reserved1: header._reserved1,
            dsi_flags: header.dsi_flags,
            nds_region: header.nds_region.into(),
            autostart: header.autostart,
            arm9_offset: header.arm9_offset,
            arm9_entry_address: header.arm9_entry_address,
            arm9_ram_address: header.arm9_ram_address,
            arm9_size: header.arm9_size,
            arm7_rom_offset: header.arm7_rom_offset,
            arm7_entry_address: header.arm7_entry_address,
            arm7_ram_address: header.arm7_ram_address,
            arm7_size: header.arm7_size,
            fnt_offset: header.fnt_offset,
            fnt_size: header.fnt_size,
            fat_offset: header.fat_offset,
            fat_size: header.fat_size,
            file_arm9_overlay_offset: header.file_arm9_overlay_offset,
            file_arm9_overlay_size: header.file_arm9_overlay_size,
            file_arm7_overlay_offset: header.file_arm7_overlay_offset,
            file_arm7_overlay_size: header.file_arm7_overlay_size,
            gamecard_bus_normal_command_setting: header.gamecard_bus_normal_command_setting,
            gamecard_bus_key1_command_setting: header.gamecard_bus_key1_command_setting,
            icon_title_offset: header.icon_title_offset,
            secure_area_checksum: header.secure_area_checksum,
            secure_area_delay: header.secure_area_delay,
            arm9_autoload_list_hook_ram_address: header.arm9_autoload_list_hook_ram_address,
            arm7_autoload_list_hook_ram_address: header.arm7_autoload_list_hook_ram_address,
            secure_area_disable: header.secure_area_disable,
            header_size: header.rom_header_size,
            _reserved2: header._reserved2,
            nintendo_logo: header.nintendo_logo,
            nintendo_logo_checksum: header.nintendo_logo_checksum,
            header_checksum: header.header_checksum,
            debug_rom_offset: header.debug_rom_offset,
            debug_rom_size: header.debug_rom_size,
            debug_ram_address: header.debug_ram_address,
            _reserved3: header._reserved3,
        };

        Ok(misc)
    }
}

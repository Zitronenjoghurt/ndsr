use crate::codec::rom::NDSRom;
use crate::error::{NDSRError, NDSRResult};
use binrw::{BinRead, BinWrite};

/// Spec: https://mgba-emu.github.io/gbatek/#dscartridgeheader
#[derive(Debug, BinRead, BinWrite)]
pub struct RawNDSHeader {
    pub game_title: [u8; 12],
    pub game_code: [u8; 4],
    pub maker_code: [u8; 2],
    pub unit_code: u8,
    pub encryption_seed_select: u8,
    pub device_capacity: u8,
    pub _reserved1: [u8; 7],
    pub dsi_flags: u8,
    pub nds_region: u8,
    pub rom_version: u8,
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
    pub secure_area_disable: [u8; 8],
    pub total_used_rom_size: u32,
    pub rom_header_size: u32,
    pub _reserved2: [u8; 56],
    pub nintendo_logo: [u8; 156],
    pub nintendo_logo_checksum: u16,
    pub debug_rom_offset: u32,
    pub debug_rom_size: u32,
    pub debug_ram_address: u32,
    pub _reserved3: [u8; 148],
}

impl TryFrom<NDSRom> for RawNDSHeader {
    type Error = NDSRError;

    fn try_from(rom: NDSRom) -> NDSRResult<Self> {
        let game_title_bytes = rom.game_title.bytes().collect::<Vec<_>>();
        let game_title = [
            game_title_bytes.get(0).copied().unwrap_or(0),
            game_title_bytes.get(1).copied().unwrap_or(0),
            game_title_bytes.get(2).copied().unwrap_or(0),
            game_title_bytes.get(3).copied().unwrap_or(0),
            game_title_bytes.get(4).copied().unwrap_or(0),
            game_title_bytes.get(5).copied().unwrap_or(0),
            game_title_bytes.get(6).copied().unwrap_or(0),
            game_title_bytes.get(7).copied().unwrap_or(0),
            game_title_bytes.get(8).copied().unwrap_or(0),
            game_title_bytes.get(9).copied().unwrap_or(0),
            game_title_bytes.get(10).copied().unwrap_or(0),
            game_title_bytes.get(11).copied().unwrap_or(0),
        ];

        let unique_code = rom.unique_code_category.into();
        let short_title_bytes = rom.short_title.bytes().collect::<Vec<_>>();
        let destination_language = rom.destination_language.into();
        let game_code = [
            unique_code,
            short_title_bytes.get(0).copied().unwrap_or(0),
            short_title_bytes.get(1).copied().unwrap_or(0),
            destination_language,
        ];

        let maker_code_bytes = rom.maker_code.bytes().collect::<Vec<_>>();
        let maker_code = [
            maker_code_bytes.get(0).copied().unwrap_or(0),
            maker_code_bytes.get(1).copied().unwrap_or(0),
        ];

        let header = Self {
            game_title,
            game_code,
            maker_code,
            unit_code: rom.unit_code.into(),
            encryption_seed_select: rom.header_misc.encryption_seed_select,
            device_capacity: rom.cartridge_size.into(),
            _reserved1: rom.header_misc._reserved1,
            dsi_flags: 0,
            nds_region: 0,
            rom_version: 0,
            autostart: 0,
            arm9_offset: 0,
            arm9_entry_address: 0,
            arm9_ram_address: 0,
            arm9_size: 0,
            arm7_rom_offset: 0,
            arm7_entry_address: 0,
            arm7_ram_address: 0,
            arm7_size: 0,
            fnt_offset: 0,
            fnt_size: 0,
            fat_offset: 0,
            fat_size: 0,
            file_arm9_overlay_offset: 0,
            file_arm9_overlay_size: 0,
            file_arm7_overlay_offset: 0,
            file_arm7_overlay_size: 0,
            gamecard_bus_normal_command_setting: 0,
            gamecard_bus_key1_command_setting: 0,
            icon_title_offset: 0,
            secure_area_checksum: 0,
            secure_area_delay: 0,
            arm9_autoload_list_hook_ram_address: 0,
            arm7_autoload_list_hook_ram_address: 0,
            secure_area_disable: [0; 8],
            total_used_rom_size: 0,
            rom_header_size: 0,
            _reserved2: [0; 56],
            nintendo_logo: [0; 156],
            nintendo_logo_checksum: 0,
            debug_rom_offset: 0,
            debug_rom_size: 0,
            debug_ram_address: 0,
            _reserved3: [0; 148],
        };

        Ok(header)
    }
}

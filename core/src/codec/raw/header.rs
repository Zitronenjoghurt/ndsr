use crate::codec::rom::NDSRom;
use crate::error::{NDSRError, NDSRResult};
use binrw::{BinRead, BinWrite};

pub const HEADER_SIZE: usize = 0x200;

/// Spec: https://mgba-emu.github.io/gbatek/#dscartridgeheader
#[derive(Debug, BinRead, BinWrite)]
pub struct RawHeader {
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
    pub header_checksum: u16,
    pub debug_rom_offset: u32,
    pub debug_rom_size: u32,
    pub debug_ram_address: u32,
    pub _reserved3: [u8; 148],
}

impl TryFrom<&NDSRom> for RawHeader {
    type Error = NDSRError;

    fn try_from(rom: &NDSRom) -> NDSRResult<Self> {
        let game_title_bytes = rom.short_title.bytes().collect::<Vec<_>>();
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
        let short_title_bytes = rom.title_code.bytes().collect::<Vec<_>>();
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
            encryption_seed_select: rom.header.encryption_seed_select,
            device_capacity: rom.cartridge_size.into(),
            _reserved1: rom.header._reserved1,
            dsi_flags: rom.header.dsi_flags,
            nds_region: rom.header.nds_region.into(),
            rom_version: rom.rom_version,
            autostart: rom.header.autostart,
            arm9_offset: rom.header.arm9_offset,
            arm9_entry_address: rom.header.arm9_entry_address,
            arm9_ram_address: rom.header.arm9_ram_address,
            arm9_size: rom.header.arm9_size,
            arm7_rom_offset: rom.header.arm7_rom_offset,
            arm7_entry_address: rom.header.arm7_entry_address,
            arm7_ram_address: rom.header.arm7_ram_address,
            arm7_size: rom.header.arm7_size,
            fnt_offset: rom.header.fnt_offset,
            fnt_size: rom.header.fnt_size,
            fat_offset: rom.header.fat_offset,
            fat_size: rom.header.fat_size,
            file_arm9_overlay_offset: rom.header.file_arm9_overlay_offset,
            file_arm9_overlay_size: rom.header.file_arm9_overlay_size,
            file_arm7_overlay_offset: rom.header.file_arm7_overlay_offset,
            file_arm7_overlay_size: rom.header.file_arm7_overlay_size,
            gamecard_bus_normal_command_setting: rom.header.gamecard_bus_normal_command_setting,
            gamecard_bus_key1_command_setting: rom.header.gamecard_bus_key1_command_setting,
            icon_title_offset: rom.header.icon_title_offset,
            secure_area_checksum: rom.header.secure_area_checksum,
            secure_area_delay: rom.header.secure_area_delay,
            arm9_autoload_list_hook_ram_address: rom.header.arm9_autoload_list_hook_ram_address,
            arm7_autoload_list_hook_ram_address: rom.header.arm7_autoload_list_hook_ram_address,
            secure_area_disable: rom.header.secure_area_disable,
            total_used_rom_size: rom.rom_size,
            rom_header_size: rom.header.header_size,
            _reserved2: rom.header._reserved2,
            nintendo_logo: rom.header.nintendo_logo,
            nintendo_logo_checksum: rom.header.nintendo_logo_checksum,
            header_checksum: rom.header.header_checksum,
            debug_rom_offset: rom.header.debug_rom_offset,
            debug_rom_size: rom.header.debug_rom_size,
            debug_ram_address: rom.header.debug_ram_address,
            _reserved3: rom.header._reserved3,
        };

        Ok(header)
    }
}

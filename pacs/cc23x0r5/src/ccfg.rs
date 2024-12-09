use bitfield_struct::bitfield;


#[bitfield(u32)]
pub struct SerialRomBldrParam {
    pub bldr_enabled: bool,
    pub pin_trigger_enabled: bool,
    pub pin_trigger_level: bool,
    #[bits(13)]
    pub res0: u16,
    #[bits(6)]
    pub pin_trigger_dio: u8,
    #[bits(2)]
    pub res1: u8,
    #[bits(3)]
    pub serial_io_cfg_index: u8,
    #[bits(5)]
    pub res2: u8,
}

#[repr(C)]
pub struct BootCfg {
    // Pointer to user bootloader vector table
    pub p_bldr_vtor: u32, // TODO: *const u8,
    // Parameter passed to bootloader
    pub bldr_param: SerialRomBldrParam,
    // Pointer to application VTOR table
    pub p_app_vtor: u32, // TODO: *const fn(),
    pub crc32: u32,
}

#[bitfield(u32)]
pub struct Permissions {
    #[bits(4)]
    pub allow_return_to_factory: u8,
    #[bits(4)]
    pub allow_fake_stby: u8,
    #[bits(4)]
    pub allow_tools_client_mode: u8,
    #[bits(4)]
    pub allow_chip_erase: u8,
    #[bits(4)]
    pub allow_flash_program: u8,
    #[bits(4)]
    pub allow_flash_verify: u8,
    #[bits(4)]
    pub allow_energy_trace: u8,
    #[bits(4)]
    pub allow_debug_port: u8,
}

#[bitfield(u32)]
pub struct Misc {
    #[bits(3)]
    pub saci_timeout_exp: u8,
    pub saci_timeout_override: bool,
    #[bits(28)]
    pub res0: u32,
}

#[repr(C)]
pub struct FlashProt {
    pub write_erase_prot: WriteEraseProt,
    pub res: u32,
    pub chip_erase_retain: ChipEraseRetain,
    pub res0: [u32; 2],
}

#[repr(C)]
pub struct WriteEraseProt {
    pub main_sectors_0_31: u32,
    pub main_sectors_32_255: u32,
    pub aux_sectors: u32,
}

#[repr(C)]
pub struct ChipEraseRetain {
    pub main_sectors_0_31: u32,
    pub main_sectors_32_255: u32,
}

#[repr(C)]
pub struct UserRecord {
    pub val32: [u32; 31],
    pub crc32: u32,
}

#[repr(C)]
pub struct DebugCfg {
    pub authorization: u8,
    pub allow_bldr: u8,
    pub res0: [u8; 2],
    pub pwd_id: [u8; 8],
    pub pwd_hash: [u8; 32],
    pub crc32: u32,
}

#[repr(C)]
pub struct Ccfg {
    // Bootloader/application configuration
    pub boot_cfg: BootCfg,
    // Paperspin options
    pub hw_opts: [u32; 2],
    // Device permissions
    pub permissions: Permissions,
    // Miscellaneous fields
    pub misc: Misc,
    // Flash protection
    pub flash_prot: FlashProt,
    // Optional HW initialization copy-list
    pub hw_init_copy_list: [u32; (2048/4) - 61],
    // CRC across hwOpts through hwInitCopyList
    pub crc32: u32,
    // User record
    pub user_record: UserRecord,
    // Debug configuration and password
    pub debug_cfg: DebugCfg,
}

impl Ccfg {
    pub const fn new() -> Self {
        Ccfg {
            boot_cfg: BootCfg {
                p_bldr_vtor: 0xFFFFFFFF,
                bldr_param: SerialRomBldrParam::new()
                    .with_bldr_enabled (false)
                    .with_pin_trigger_enabled(false)
                    .with_pin_trigger_level(false)
                    .with_res0(0)
                    .with_pin_trigger_dio(0)
                    .with_res1(0)
                    .with_serial_io_cfg_index(0)
                    .with_res2(0),
                p_app_vtor: 0x00000000,
                crc32: 0x0BAD0BAD,
            },
            hw_opts: [0xFFFFFFFF; 2],
            permissions: Permissions::new()
                .with_allow_return_to_factory(0xA)
                .with_allow_fake_stby(0xA)
                .with_allow_tools_client_mode(0xA)
                .with_allow_chip_erase(0xA)
                .with_allow_flash_program(0xA)
                .with_allow_flash_verify(0xA)
                .with_allow_energy_trace(0xA)
                .with_allow_debug_port(0xA),
            misc: Misc::new()
                .with_saci_timeout_exp(7)
                .with_saci_timeout_override(false)
                .with_res0(0)
,
            flash_prot: FlashProt {
                write_erase_prot: WriteEraseProt {
                    main_sectors_0_31: 0xFFFFFFFF,
                    main_sectors_32_255: 0xFFFFFFFF,
                    aux_sectors: 0xFFFFFFFF,
                },
                res: 0xFFFFFFFF,
                chip_erase_retain: ChipEraseRetain {
                    main_sectors_0_31: 0,
                    main_sectors_32_255: 0,
                },
                res0: [0; 2],
            },
            hw_init_copy_list: [0; (2048/4) - 61],
            crc32: 0x0BAD0BAD,
            user_record: UserRecord {
                val32: [0; 31],
                crc32: 0x0BAD0BAD,
            },
            debug_cfg: DebugCfg {
                authorization: 0x5A,
                allow_bldr: 0xA5,
                res0: [0; 2],
                pwd_id: [1, 1, 2, 3, 5, 8, 13, 21],
                pwd_hash: [
                    0x6D, 0xD7, 0xE4, 0x36, 0xEB, 0xF4, 0x31, 0xDF,
                    0x95, 0xAE, 0x15, 0xEE, 0x03, 0xBA, 0x8E, 0xE4,
                    0xC4, 0xC6, 0x3F, 0xD8, 0x45, 0x3F, 0x67, 0x5E,
                    0x74, 0xD7, 0xC2, 0x01, 0x2C, 0x90, 0x58, 0xE5,
                ],
                crc32: 0x0BAD0BAD,
            },
        }
    }
}

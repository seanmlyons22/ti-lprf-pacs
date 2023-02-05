/// A struct that represents the CCFG flash region
/// This struct is used to initialize the CCFG region at application compile time
/// Details about the CCFG region can be found in the \
/// [TRM](https://www.ti.com/lit/ug/swcu193/swcu193.pdf), in the Device Configuration chapter.
///
/// # Initialization
///
/// Since this struct must go in flash, it should be initialized as `pub static`.
/// Its values should be constant initialized.
///
/// The used (`#[used]`) attribute is used to ensure that the CCFG struct is not optimized out by the linker.
/// Since the CCFG struct is read by the boot ROM and other critical parts of the
/// chip, it has a specific address, at the end of flash. To control placement of the struct in flash,
/// the `#[link_section = ".ccfg"]` attribute must be used.
///
/// Finally, name mangling should be avoided with the `#[no_mangle]` attribute.
///
/// # Example
///
/// The following example will initialize the CCFG with default values.
/// Note that the `Default` trait cannot be used because the CCFG must be
/// constant initialized.
/// ```
/// #[used]
/// #[no_mangle]
/// #[link_section = ".ccfg"]
/// pub static CCFG: Ccfg = Ccfg::new().update_crcs();
/// ```
///
/// The linker script must also be updated, add this at the end of the file:
///
/// ```ld
/// SECTIONS
/// {
///     .ccfg :
///     {
///         KEEP(*(.ccfg));
///     } > FLASH_CCFG
/// }
/// ```
///
/// # CRCs
///
/// There are CRCs which are computed over certain fields of the CCFG.
/// These CRCs are used by the boot ROM to validate the CCFG is correct and valid.
/// If the CRCs are not correct, the device may not boot properly.
///
/// Thus, the CCFG struct's initialization must be always followed by a call to
/// `update_crcs()`, which updates the CRCs with the current values in the struct.
///

use bitfield_struct::bitfield;

/// Special values for bootloader vector table pointer
pub const CCFG_BC_PBLDR_USE_FCFG: *const () = 0xFFFFFFF0 as *const ();
pub const XCFG_BC_PBLDR_FORBID: *const () = 0xFFFFFFFC as *const ();
pub const XCFG_BC_PBLDR_UNDEF: *const () = 0xFFFFFFFF as *const ();

/// Special values for application vector table pointer
pub const CCFG_BC_PAPP_NONE: *const () = 0xFFFFFFFF as *const ();

#[bitfield(u32)]
pub struct SerialRomBldrParam {
    /// Enable bootloader
    pub bldr_enabled: bool,
    /// Enable pin trigger
    pub pin_trigger_enabled: bool,
    /// Pin trigger level configuration
    pub pin_trigger_level: bool,
    /// Reserved
    #[bits(13)]
    pub res0: u16,
    /// DIO pin number for pin trigger
    #[bits(6)]
    pub pin_trigger_dio: u8,
    /// Reserved
    #[bits(2)]
    pub res1: u8,
    /// Serial I/O configuration index
    #[bits(3)]
    pub serial_io_cfg_index: u8,
    /// Reserved
    #[bits(5)]
    pub res2: u8,
}

/// Bootloader/application configuration [0]: length 16B
#[repr(C)]
pub struct BootCfg {
    /// Pointer to user bootloader vector table
    pub p_bldr_vtor: *const (),
    /// Parameter passed to bootloader
    pub bldr_param: SerialRomBldrParam,
    /// Pointer to application VTOR table
    pub p_app_vtor: *const (),
    /// CRC32 checksum
    pub crc32: u32,
}

// Safety: BootCfg is a compile-time constant and can be safely shared between threads
unsafe impl Send for BootCfg {}
unsafe impl Sync for BootCfg {}

/// Device permissions [24]: length 4 B
/// This is maximally-restrictive combined with similar field in FCFG
#[bitfield(u32)]
pub struct Permissions {
    /// Allow return to factory mode
    #[bits(4)]
    pub allow_return_to_factory: u8,
    /// Allow fake standby mode
    #[bits(4)]
    pub allow_fake_stby: u8,
    /// Allow tools client mode
    #[bits(4)]
    pub allow_tools_client_mode: u8,
    /// Allow chip erase
    #[bits(4)]
    pub allow_chip_erase: u8,
    /// Allow flash programming
    #[bits(4)]
    pub allow_flash_program: u8,
    /// Allow flash verification
    #[bits(4)]
    pub allow_flash_verify: u8,
    /// Allow energy trace
    #[bits(4)]
    pub allow_energy_trace: u8,
    /// Allow debug port access
    #[bits(4)]
    pub allow_debug_port: u8,
}

/// Miscellaneous fields [28]: length 4B
#[bitfield(u32)]
pub struct Misc {
    /// SACI timeout exponent (0=infinite, else (2^saciTimeoutExp)*64 ms)
    /// Fcfg timeout applied instead if CCfg.saciTimeoutOverride==0
    #[bits(3)]
    pub saci_timeout_exp: u8,
    /// Override SACI timeout from FCFG
    pub saci_timeout_override: bool,
    /// Reserved
    #[bits(28)]
    pub res0: u32,
}

/// Flash protection [32]: length 32 B
/// This is maximally-restrictive combined with similar field in FCFG
#[repr(C)]
pub struct FlashProt {
    /// Write/erase protection configuration
    pub write_erase_prot: WriteEraseProt,
    /// Reserved
    pub res: u32,
    /// Chip erase retain configuration
    pub chip_erase_retain: ChipEraseRetain,
    /// Reserved for future flash increases
    pub res0: [u32; 2],
}

/// Write/erase protection configuration
#[repr(C)]
pub struct WriteEraseProt {
    /// Sticky-0 bits written to VIMS.WEPRA (sectors 0-31, 1/bit)
    pub main_sectors_0_31: u32,
    /// Sticky-0 bits written to VIMS.WEPRB(0) (sectors 32-255, 8/bit)
    pub main_sectors_32_255: u32,
    /// Sticky-0 bit written to VIMS.WEPRAUX
    pub aux_sectors: u32,
}

/// Chip erase retain configuration
#[repr(C)]
pub struct ChipEraseRetain {
    /// Set bits (sectors 0-31, 1/bit) define what a chip erase command can optionally retain
    pub main_sectors_0_31: u32,
    /// Set bits (sectors 32-255, 8/bit) define what a chip erase command can optionally retain
    pub main_sectors_32_255: u32,
}

/// User record (programmable also through separate SACI command), no dependencies in boot code
/// User record size is fixed at 128 B. Last word assumed to be CRC over first 124 B (optional)
#[repr(C)]
pub struct UserRecord {
    /// Generic 32b record layout (31 words)
    pub val32: [u32; 31],
    /// CRC field across first 124B of userRecord (supported by SACI verifyCcfg command)
    pub crc32: u32,
}

/// Debug configuration and password [End-48]: length 48B
#[repr(C)]
pub struct DebugCfg {
    /// Debug authorization requirements
    /// - 0xA5 = Require password
    /// - 0x5A = Debug open
    /// - 0x00 = Debug forbidden
    pub authorization: u8,
    /// Allow debugging of bootloader
    pub allow_bldr: u8,
    /// Reserved
    pub res0: [u8; 2],
    /// Password ID
    pub pwd_id: [u8; 8],
    /// Password hash
    pub pwd_hash: [u8; 32],
    /// Reserved
    pub crc32: u32,
}

/// CCFG sector data structure definition
/// This structure should be allocated at the base of CCFG section defined in
/// the linker file.
#[repr(C)]
pub struct Ccfg {
    /// Bootloader/application configuration [0]: length 16B
    pub boot_cfg: BootCfg,
    /// HW options [16]: length 8 B
    pub hw_opts: [u32; 2],
    /// Device permissions [24]: length 4 B
    /// This is maximally-restrictive combined with similar field in FCFG
    pub permissions: Permissions,
    /// Miscellaneous fields [28]: length 4B
    pub misc: Misc,
    /// Flash protection [32]: length 32 B
    /// This is maximally-restrictive combined with similar field in FCFG
    pub flash_prot: FlashProt,
    /// Optional HW initialization copy-list [64]: length x B
    /// Copy list applied before user application is entered. May be used by customer/SYSCFG to
    /// initialize hardware right before application is entered.
    /// Also used to pad out CCFG to correct size
    pub hw_init_copy_list: [u32; (2048/4) - 61],
    /// CRC across hwOpts through hwInitCopyList [End-180]: length 4B
    pub crc32: u32,
    /// User record (programmable also through separate SACI command), no dependencies in boot code
    /// User record size is fixed at 128 B. Last word assumed to be CRC over first 124 B (optional)
    pub user_record: UserRecord,
    /// Debug configuration and password [End-48]: length 48B
    pub debug_cfg: DebugCfg,
}

impl Ccfg {
    pub const fn new() -> Self {
        Ccfg {
            boot_cfg: BootCfg {
                p_bldr_vtor: XCFG_BC_PBLDR_UNDEF,
                bldr_param: SerialRomBldrParam::new()
                    .with_bldr_enabled (false)
                    .with_pin_trigger_enabled(false)
                    .with_pin_trigger_level(false)
                    .with_res0(0)
                    .with_pin_trigger_dio(0)
                    .with_res1(0)
                    .with_serial_io_cfg_index(0)
                    .with_res2(0),
                p_app_vtor: 0x00000000 as *const (),
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
                .with_res0(0),
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
    const fn calc_crc32(data: &[u8]) -> u32 {
        const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        CRC.checksum(data)
    }
    pub const fn update_crcs(mut self) -> Self {
        // Caculate CRC32 for boot_cfg
        let boot_cfg_bytes = unsafe {
            core::slice::from_raw_parts(
                &self.boot_cfg as *const _ as *const u8,
                core::mem::size_of::<BootCfg>() - 4, // Exclude CRC field
            )
        };
        self.boot_cfg.crc32 = Self::calc_crc32(boot_cfg_bytes);
        // Calculate CRC32 over hw_opts, permissions, misc, flash_prot, and hw_init_copy_list
        let hw_bytes = unsafe {
            core::slice::from_raw_parts(
                &self.hw_opts as *const _ as *const u8,
                core::mem::size_of::<[u32; 2]>() +
                core::mem::size_of::<Permissions>() +
                core::mem::size_of::<Misc>() +
                core::mem::size_of::<FlashProt>() +
                core::mem::size_of::<[u32; (2048/4) - 61]>()
            )
        };
        self.crc32 = Self::calc_crc32(hw_bytes);
        // Calculate CRC32 for user_record
        let user_bytes = unsafe {
            core::slice::from_raw_parts(
                &self.user_record as *const _ as *const u8,
                core::mem::size_of::<UserRecord>() - 4, // Exclude CRC field
            )
        };
        self.user_record.crc32 = Self::calc_crc32(user_bytes);
        // Calculate CRC32 for debug_cfg
        let debug_bytes = unsafe {
            core::slice::from_raw_parts(
                &self.debug_cfg as *const _ as *const u8,
                core::mem::size_of::<DebugCfg>() - 4, // Exclude CRC field
            )
        };
        self.debug_cfg.crc32 = Self::calc_crc32(debug_bytes);
        self
    }
}

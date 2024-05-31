/// A struct that represents the CCFG flash region
/// This struct is used to initialize the CCFG region at application compile time
/// Details about the CCFG region can be found in the \
/// [TRM](https://www.ti.com/lit/ug/swcu194/swcu194.pdf)
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
///
///
/// #[used]
/// #[no_mangle]
/// #[link_section = ".ccfg"]
/// pub static CCFG: Ccfg = Ccfg::new();
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
#[repr(C)]
pub struct Ccfg {
    /// Size and disable flags
    pub size_and_dis_flags: u32,
    /// Mode configuration 0
    pub mode_conf: u32,
    /// Mode configuration 1
    pub mode_conf_1: u32,
    /// Not to be touched
    pub volt_load_0: u32,
    /// Not to be touched
    pub volt_load_1: u32,
    /// External LF clock configuration
    pub ext_lf_clk: u32,
    /// Lower 32 bits of IEEE MAC address,
    /// if different than 0xFFFFFFFF, this will override the value in FCFG
    pub ieee_mac_0: u32,
    /// Upper 32 bits of IEEE MAC address,
    /// if different than 0xFFFFFFFF, this will override the value in FCFG
    pub ieee_mac_1: u32,
    /// Lower 32 bits of BLE address,
    /// if different than 0xFFFFFFFF, this will override the value in FCFG
    pub ieee_ble_0: u32,
    /// Upper 32 bits of BLE address,
    /// if different than 0xFFFFFFFF, this will override the value in FCFG
    pub ieee_ble_1: u32,
    /// Configures the functionality of the ROM boot loader.
    pub bl_config: u32,
    /// Flash erase configuration
    pub erase_conf: u32,
    /// CCFG erase configuration
    pub erase_conf_1: u32,
    /// TI Failure Analysis options
    pub ccfg_ti_options: u32,
    /// TAP and DAP configuration 0
    pub ccfg_tap_dap_0: u32,
    /// TAP and DAP configuration 1
    pub ccfg_tap_dap_1: u32,
    /// The address value of the start of the application flash vector table
    pub image_valid_conf: u32,
    /// Flash Protect Sectors 0-31
    pub ccfg_weprot_31_0_by2k: u32,
    /// Reserved field
    pub ccfg_weprot_spare_1: u32,
    /// Reserved field
    pub ccfg_weprot_spare_2: u32,
    /// Reserved field
    pub ccfg_weprot_spare_3: u32,
    /// Paritioning of the flash into secure vs non-secure regions vs non-secure callable regions
    pub trustzone_flash_cfg: u32,
    /// Paritioning of the SRAM into secure vs non-secure regions vs non-secure callable regions
    pub trustzone_sram_cfg: u32,
    /// Configuration register for the SRAM
    pub sram_cfg: u32,
    /// Reserved field
    pub reserved0: u32,
    /// Configuration register for MCU CPU lock options
    pub cpu_lock_cfg: u32,
    /// Configuration register for debug authentication
    pub dbg_auth_cfg: u32,
    /// Customer key 0
    pub ckey0: u32,
    /// Customer key 1
    pub ckey1: u32,
    /// Customer key 2
    pub ckey2: u32,
    /// Customer key 3
    pub ckey3: u32,
}

impl Ccfg {
    /// Create a new CCFG struct with default values
    ///
    /// Since we cannot use the Default trait as described above, the new
    /// function is opinionated about the default values and is intended remove
    /// the boilerplate of setting the default values.
    ///
    /// # Example
    /// However, you can still override the default values if you want to.
    /// ```
    /// #[used]
    /// #[no_mangle]
    /// #[link_section = ".ccfg"]
    /// let ccfg = Ccfg {
    ///    ieee_mac_0: 0xBBBBBBBB,
    ///   ..Ccfg::new()
    /// };
    /// ```
    pub const fn new() -> Self {
        Self {
            size_and_dis_flags: size_and_dis_flags(true, false, true),
            mode_conf: mode_conf(
                CCFG_DEFAULT_MODE_CONF_VDDR_TRIM_SLEEP_DELTA,
                CCFG_DEFAULT_MODE_CONF_DCDC_RECHARGE_DISABLE,
                false,
                CCFG_DEFAULT_MODE_CONF_VDDS_BOD_LEVEL,
                CCFG_DEFAULT_MODE_CONF_SCLK_LF,
                true,
                CCFG_DEFAULT_MODE_CONF_XOSC_HF,
                true,
                0xFF,
            ),
            mode_conf_1: mode_conf_1(
                CCFG_DEFAULT_MODE_CONF_1_TCXO_TYPE,
                CCFG_DEFAULT_MODE_CONF_1_TCXO_MAX_START,
                CCFG_DEFAULT_MODE_CONF_1_ALT_DCDC_VMIN,
                CCFG_DEFAULT_MODE_CONF_1_ALT_DCDC_DITHER_EN,
                CCFG_DEFAULT_MODE_CONF_1_ALT_DCDC_IPEAK,
                CCFG_DEFAULT_MODE_CONF_1_ALT_IBIAS_INIT,
                CCFG_DEFAULT_MODE_CONF_1_ALT_IBIAS_OFFSET,
                CCFG_DEFAULT_MODE_CONF_1_ALT_XOSC_MAX_START,
            ),
            volt_load_0: 0xffffffff, // DON'T TOUCH
            volt_load_1: 0xffffffff, // DON'T TOUCH
            ext_lf_clk: ext_lf_clk(
                CCFG_DEFAULT_EXT_LF_CLK_RTC_INCREMENT,
                CCFG_DEFAULT_EXT_LF_CLK_DIO,
            ),
            ieee_mac_0: 0xffffffff,
            ieee_mac_1: 0xffffffff,
            ieee_ble_0: 0xffffffff,
            ieee_ble_1: 0xffffffff,
            bl_config: bl_config(
                CCFG_BL_CONFIG_ENABLE::DISABLED,
                CCFG_BL_CONFIG_BACKDOOR_ENABLE::DISABLED,
                true,
                CCFG_DEFAULT_BL_CONFIG_BL_BACKDOOR_PIN,
            ),
            erase_conf: erase_conf(true, true),
            erase_conf_1: erase_conf_1(true),
            ccfg_ti_options: ccfg_ti_options(
                CCFG_TI_OPTIONS_TI_FA::DISABLED,
                CCFG_TI_OPTIONS_IDAU_CFG::ENABLED,
                CCFG_TI_OPTIONS_C_KEY_XOR::DISABLED,
            ),
            ccfg_tap_dap_0: ccfg_tap_dap_0(
                CCFG_DEFAULT_TAP_DAP_0_CPU_DAP,
                CCFG_DEFAULT_TAP_DAP_0_PWRPROF,
                CCFG_DEFAULT_TAP_DAP_0_TEST_TAP,
            ),
            ccfg_tap_dap_1: ccfg_tap_dap_1(
                CCFG_DEFAULT_TAP_DAP_1_AON_TAP,
                CCFG_DEFAULT_TAP_DAP_1_PBIST1_TAP,
                CCFG_DEFAULT_TAP_DAP_1_PBIST2_TAP,
            ),
            image_valid_conf: 0x00000000,
            ccfg_weprot_31_0_by2k: 0xFFFFFFFF,
            ccfg_weprot_spare_1: 0xFFFFFFFF,
            ccfg_weprot_spare_2: 0xFFFFFFFF,
            ccfg_weprot_spare_3: 0xFFFFFFFF,
            trustzone_flash_cfg: trustzone_flash_cfg(0x1, 0x0),
            trustzone_sram_cfg: trustzone_sram_cfg(0x0, 0x0),
            sram_cfg: sram_cfg(0xFFFFFF, false),
            reserved0: 0xFFFFFFFF,
            cpu_lock_cfg: cpu_lock_cfg(true, true, true, true, true),
            dbg_auth_cfg: dbg_auth_cfg(true, true, true, true),
            ckey0: 0xFFFFFFFF,
            ckey1: 0xFFFFFFFF,
            ckey2: 0xFFFFFFFF,
            ckey3: 0xFFFFFFFF,
        }
    }
}

pub enum CCFG_MODE_CONF_1_TCXO_TYPE {
    CMOS_TYPE = 0,
    CLIPPED_SINE_TYPE = 1,
}
pub enum CCFG_MODE_CONF_VDDS_BOD_LEVEL {
    /// 2.0V: (necessary for external load mode, or for maximum PA output power on CC13xx).
    VDDS_BOD_2_0V = 0,
    /// 1.8V (or 1.65V for external regulator mode), this is default
    VDDS_BOD_1_8V = 1,
}
pub enum CCFG_TAP_DAP_ENABLE {
    /// TAP or DAP is enabled
    ENABLED = 0xC5,
    /// TAP or DAP is disabled
    DISABLED = 0x00,
}

pub enum CCFG_MODE_CONF_SCLK_LF {
    /// F clock derived from HF clock. Note: using this configuration will block the device from entering Standby mode.
    LF_DERIVED_FROM_HF = 0x0,
    /// External LF Clock
    LF_EXT = 0x1,
    /// LF XOSC
    LF_XTAL = 0x2,
    /// LF RCOSC
    LF_RCOSC = 0x3,
}

pub enum CCFG_MODE_CONF_XOSC_HF {
    /// HF source is 48 MHz TCXO
    TCXO = 0x0,
    /// HF source is HPOSC (BAW) (only valid for CC2652RB)
    BAW = 0x1,
    /// HF source is a 48 MHz xtal
    XTAL = 0x2,
}

pub enum CCFG_BL_CONFIG_ENABLE {
    /// BL is enabled
    ENABLED = 0xC5,
    /// BL is disabled
    DISABLED = 0x00,
}

pub enum CCFG_BL_CONFIG_BACKDOOR_ENABLE {
    /// backdoor is enabled
    ENABLED = 0xC5,
    /// backdoor is disabled
    DISABLED = 0xFF,
}

pub enum CCFG_TI_OPTIONS_TI_FA {
    /// FA is enabled
    ENABLED = 0xC5,
    /// FA is disabled
    DISABLED = 0x00,
}

pub enum CCFG_TI_OPTIONS_C_KEY_XOR {
    /// XOR of CKEY with TI FA option unlock key is enabled
    ENABLED = 0x00,
    /// XOR of CKEY with TI FA option unlock key is disabled
    DISABLED = 0xC5,
}

pub enum CCFG_TI_OPTIONS_IDAU_CFG {
    /// IDAU configuration is enabled
    ENABLED = 0x00,
    /// IDAU configuration is disabled
    DISABLED = 0xC5,
}

pub const SIZE_OF_CCFG: u8 = 0x7C;
pub const CCFG_DEFAULT_EXT_LF_CLK_RTC_INCREMENT: u32 = 0x800000;
pub const CCFG_DEFAULT_EXT_LF_CLK_DIO: u8 = 0x01;
pub const CCFG_DEFAULT_MODE_CONF_1_TCXO_TYPE: CCFG_MODE_CONF_1_TCXO_TYPE =
    CCFG_MODE_CONF_1_TCXO_TYPE::CLIPPED_SINE_TYPE;
pub const CCFG_DEFAULT_MODE_CONF_1_TCXO_MAX_START: u8 = 0x7F; // Maximum TCXO startup time in units of 100us
pub const CCFG_DEFAULT_MODE_CONF_1_ALT_DCDC_VMIN: u8 = 0x8;
pub const CCFG_DEFAULT_MODE_CONF_1_ALT_DCDC_DITHER_EN: bool = false;
pub const CCFG_DEFAULT_MODE_CONF_1_ALT_DCDC_IPEAK: u8 = 0x0;
pub const CCFG_DEFAULT_MODE_CONF_1_ALT_IBIAS_INIT: u8 = 0x0;
pub const CCFG_DEFAULT_MODE_CONF_1_ALT_IBIAS_OFFSET: u8 = 0x0;
pub const CCFG_DEFAULT_MODE_CONF_1_ALT_XOSC_MAX_START: u8 = 0x10;
pub const CCFG_DEFAULT_MODE_CONF_VDDR_TRIM_SLEEP_DELTA: u8 = 0xF;
pub const CCFG_DEFAULT_MODE_CONF_DCDC_RECHARGE_DISABLE: bool = false;
pub const CCFG_DEFAULT_MODE_CONF_VDDS_BOD_LEVEL: CCFG_MODE_CONF_VDDS_BOD_LEVEL =
    CCFG_MODE_CONF_VDDS_BOD_LEVEL::VDDS_BOD_1_8V;
pub const CCFG_DEFAULT_MODE_CONF_SCLK_LF: CCFG_MODE_CONF_SCLK_LF = CCFG_MODE_CONF_SCLK_LF::LF_XTAL;
pub const CCFG_DEFAULT_MODE_CONF_XOSC_HF: CCFG_MODE_CONF_XOSC_HF = CCFG_MODE_CONF_XOSC_HF::XTAL; // HF source is a 48 MHz xtal
pub const CCFG_DEFAULT_BL_CONFIG_BL_BACKDOOR_PIN: u8 = 0xFF; // Default ROM bootloader backdoor PIN
pub const CCFG_DEFAULT_TAP_DAP_0_CPU_DAP: CCFG_TAP_DAP_ENABLE = CCFG_TAP_DAP_ENABLE::ENABLED; // Access enabled if also enabled in FCFG
pub const CCFG_DEFAULT_TAP_DAP_0_PWRPROF: CCFG_TAP_DAP_ENABLE = CCFG_TAP_DAP_ENABLE::ENABLED; // Access enabled if also enabled in FCFG
pub const CCFG_DEFAULT_TAP_DAP_0_TEST_TAP: CCFG_TAP_DAP_ENABLE = CCFG_TAP_DAP_ENABLE::DISABLED; // Access disabled
pub const CCFG_DEFAULT_TAP_DAP_1_AON_TAP: CCFG_TAP_DAP_ENABLE = CCFG_TAP_DAP_ENABLE::DISABLED; // Access disabled
pub const CCFG_DEFAULT_TAP_DAP_1_PBIST1_TAP: CCFG_TAP_DAP_ENABLE = CCFG_TAP_DAP_ENABLE::DISABLED; // Access disabled
pub const CCFG_DEFAULT_TAP_DAP_1_PBIST2_TAP: CCFG_TAP_DAP_ENABLE = CCFG_TAP_DAP_ENABLE::DISABLED; // Access disabled

/// Initializer for [`Ccfg`::ext_lf_clk`] field
///
/// # Arguments
///
/// * `rtc_increment` - Unsigned integer, defining the input frequency of the external clock
///                     and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows:
///
///   `EXT_LF_CLK.RTC_INCREMENT = 2^38/ InputClockFrequency` in Hertz
///   (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)
///
/// * `dio` -  Unsigned integer, defining the DIO number to be used for the external LF clock
///         (e.g.: DIO=0x01 for DIO1)
pub const fn ext_lf_clk(rtc_increment: u32, dio: u8) -> u32 {
    (rtc_increment & 0x00ff_ffff) | ((dio as u32 & 0xff) << 24)
}
/// Initializer for [`Ccfg`::mode_conf_1`] field
///
/// # Arguments
/// * `tcxo_type` -  Defining the TCXO type. Bit field value is only valid if `mode_conf::xosc_source=0`.
/// * `tcxo_max_start` -  Unsigned integer, defining the maximum TCXO startup time in units of 100us
///                 Bit field value is only valid if `mode_conf::xosc_source=0`.
/// * `dcdc_min` - Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled
///                (size_and_dis_flags.disable_alt_dcdc_setting=0).
///
///   Voltage is set by the following equation:
///
///   `Voltage = (28 + ALT_DCDC_VMIN) / 16`
///
///   Example:
///    * 0: 1.75V
///    * 1: 1.8125V
///    * ...
///    * 14: 2.625V
///    * 15: 2.6875V
/// * `dither_enabled` - Boolean, defining if the dithering is enabled. Only valid if (size_and_dis_flags.disable_alt_dcdc_setting=0).
/// * `dcdc_ipeak` - Inductor peak current if alternate DC/DC setting is enabled (size_and_dis_flags.disable_alt_dcdc_setting=0).
///   Assuming 10uH external inductor!
///
///   Example:
///    * 0: 46mA (min)
///    * 4: 70mA
///    * 7: 87mA (max)
/// * `ibas_init` - Delta value for ibias init. Only valid if (size_and_dis_flags.disable_xosc_override=0).
/// * `ibas_offset` - Delta value for ibias offset. Only valid if (size_and_dis_flags.disable_xosc_override=0).
/// * `xosc_max_start` - Maximum value of XOSC startup time in units of 100us. Only valid if (size_and_dis_flags.disable_xosc_override=0).
pub const fn mode_conf_1(
    tcxo_type: CCFG_MODE_CONF_1_TCXO_TYPE,
    tcxo_max_start: u8,
    dcdc_min: u8,
    dither_enabled: bool,
    dcdc_ipeak: u8,
    ibas_init: u8,
    ibas_offset: u8,
    xosc_max_start: u8,
) -> u32 {
    ((tcxo_type as u32) << 31)
        | ((tcxo_max_start as u32) << 24)
        | ((dcdc_min as u32) << 20)
        | ((dither_enabled as u32) << 19)
        | ((dcdc_ipeak as u32) << 16)
        | ((ibas_init as u32) << 8 << 12)
        | (ibas_offset as u32)
        | (xosc_max_start as u32)
}

/// Initializer for [`Ccfg`::size_and_dis_flags`] field
///
/// # Arguments
/// * `disable_gpram` - Boolean, defining if the GPRAM is disabled (or use the 8K VIMS RAM as CACHE RAM). When GPRAM is disabled cache is used.
/// * `disable_alt_dcdc_setting` - Boolean, defining if the alternate DC/DC setting is disabled
/// * `disable_xosc_override` - Boolean, defining if the XOSC override is disabled
pub const fn size_and_dis_flags(
    disable_gpram: bool,
    disable_alt_dcdc_setting: bool,
    disable_xosc_override: bool,
) -> u32 {
    const DISABLE_FLAGS_MASK: u32 = 0x0000FFF0;

    ((SIZE_OF_CCFG as u32) << 16)
    | (DISABLE_FLAGS_MASK)
    | (0x01 << 4)
    | (0x1 << 3) // Disable TCXO no longer used always set to one
    | ((disable_gpram as u32) << 2)
    | ((disable_alt_dcdc_setting as u32) << 1)
    | ((disable_xosc_override as u32) << 0)
}

/// Initializer for [`Ccfg`::mode_conf`] field
///
/// # Arguments
/// * `trim_sleep_delta` - Unsigned integer, Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one.
///
///   Example:
///   * 0x8: Delta -7
///   * 0xF: Delta 0
///   * 0x0: Delta +1
///   * 0x07: Delta +8

/// * `disable_dcdc_recharge` - Boolean, defining if the DC/DC recharge is disabled during powerdown
/// * `disable_dcdc_active` - Boolean, defining if the DC/DC active is disabled in active mode
/// * `vdds_bod_level` - Unsigned integer, defining the VDDS BOD level.
/// * `lf_clk_option` - defining the LF clock option
/// * `disable_vddr_sleep_trim_temp_comp` - Boolean, defining if the VDDR sleep trim temperature compensation is disabled
/// * `xosc_source` - defining the XOSC source
/// * `disable_xosc_cap_array_delta` - Boolean, defining if the XOSC cap array delta is disabled
/// * `cap_array_delta` - Unsigned integer, defining the cap array delta
pub const fn mode_conf(
    trim_sleep_delta: u8,
    disable_dcdc_recharge: bool,
    disable_dcdc_active: bool,
    vdds_bod_level: CCFG_MODE_CONF_VDDS_BOD_LEVEL,
    lf_clk_option: CCFG_MODE_CONF_SCLK_LF,
    disable_vddr_sleep_trim_temp_comp: bool,
    xosc_source: CCFG_MODE_CONF_XOSC_HF,
    disable_xosc_cap_array_delta: bool,
    cap_array_delta: u8,
) -> u32 {
    ((trim_sleep_delta as u32 ) <<  28)
    | ((disable_dcdc_recharge as u32) << 27)
    | ((disable_dcdc_active as u32) << 26)
    | ((0x01 as u32) << 25) // VDDR external load
    | ((vdds_bod_level as u32) << 24) // VDDR internal load, don't touch!
    | ((lf_clk_option as u32) << 22)
    | ((disable_vddr_sleep_trim_temp_comp as u32) << 21)
    | ((0x1 as u32) << 20) // RTC comp, don't touch!
    | ((xosc_source as u32) << 18)
    | ((disable_xosc_cap_array_delta as u32) << 17)
    | ((0x1 as u32) << 16) // HF COMP, don't touch!
    | ((cap_array_delta as u32) << 8)
    | (0xFF as u32)
}

/// Initializer for [`Ccfg`::bl_config`] field
///
/// # Arguments
/// * `bl_enable` - Boolean, defining if the ROM bootloader is enabled
/// * `backdoor_enable` - Boolean, defining if the ROM bootloader backdoor (pin triggered BL access) is enabled
/// * `bl_backdoor_active_high` - Boolean, defining if the backdoor is active high
/// * `bl_backdoor_pin` - Unsigned integer, defining the backdoor pin
pub const fn bl_config(
    bl_enable: CCFG_BL_CONFIG_ENABLE,
    backdoor_enable: CCFG_BL_CONFIG_BACKDOOR_ENABLE,
    bl_backdoor_active_high: bool,
    bl_backdoor_pin: u8,
) -> u32 {
    ((bl_enable as u32) << 24)
        | ((bl_backdoor_active_high as u32 | 0xFE) << 16)
        | ((bl_backdoor_pin as u32) << 8)
        | (backdoor_enable as u32)
}

/// Initializer for [`Ccfg`::erase_conf`] field
///
/// # Arguments
/// * `enable_bl_chip_erase` - Boolean, defining if chip erase functionality is enabled
/// * `enable_bl_bank_erase` - Boolean, defining if bank erase functionality is enabled
pub const fn erase_conf(enable_bl_chip_erase: bool, enable_bl_bank_erase: bool) -> u32 {
    (0xFFFF << 16)
        | ((enable_bl_chip_erase as u32 | 0xFE) << 8)
        | (enable_bl_bank_erase as u32 | 0xFE)
}

/// Initializer for [`Ccfg`::erase_conf_1`] field
///
/// # Arguments
/// * `enable_ccfg_erase` - Boolean, defining if ccfg erase functionality is enabled
pub const fn erase_conf_1(enable_ccfg_erase: bool) -> u32 {
    (0xFFFFFFFE) | (enable_ccfg_erase as u32)
}

/// Initializer for [`Ccfg`::ccfg_ti_options`] field
///
/// # Arguments
/// * `enable_ti_fa` - Boolean, defining if the TI Failure Analysis options are enabled
/// * `enable_idau_cfg` - Boolean, defining if the IDAU is allowed to be configured
/// * `enable_ckey_xor` - Boolean, defining if the customer key (CKEY) is to be XOR'd with TI FA option unlock key
pub const fn ccfg_ti_options(
    enable_ti_fa: CCFG_TI_OPTIONS_TI_FA,
    enable_idau_cfg: CCFG_TI_OPTIONS_IDAU_CFG,
    enable_ckey_xor: CCFG_TI_OPTIONS_C_KEY_XOR,
) -> u32 {
    enable_ti_fa as u32
        | (enable_idau_cfg as u32) << 8
        | (enable_ckey_xor as u32) << 16
        | 0xFF000000
}

/// Initializer for [`Ccfg`::ccfg_tap_dap_0`] field
///
/// # Arguments
/// * `cpu_dap` - defining if the CPU DAP is enabled
/// * `pwrprof_tap` - defining if the power profile TAP is enabled
pub const fn ccfg_tap_dap_0(
    cpu_dap: CCFG_TAP_DAP_ENABLE,
    pwrprof_tap: CCFG_TAP_DAP_ENABLE,
    test_tap: CCFG_TAP_DAP_ENABLE,
) -> u32 {
    (0xFF << 24) | (cpu_dap as u32) << 16 | (pwrprof_tap as u32) << 8 | (test_tap as u32)
}

/// Initializer for [`Ccfg`::ccfg_tap_dap_1`] field
///
/// # Arguments
/// * `aon_tap` - defining if the CPU DAP is enabled
/// * `pbist1_tap` - defining if the self-test tap 1 is enabled
/// * `pbist2_tap` - defining if the self-test tap 2 is enabled
pub const fn ccfg_tap_dap_1(
    aon_tap: CCFG_TAP_DAP_ENABLE,
    pbist1_tap: CCFG_TAP_DAP_ENABLE,
    pbist2_tap: CCFG_TAP_DAP_ENABLE,
) -> u32 {
    (0xFF << 24) | (pbist2_tap as u32) << 16 | (pbist1_tap as u32) << 8 | (aon_tap as u32)
}

/// Initializer for [`Ccfg`::trustzone_flash_cfg`] field
///
/// # Arguments
/// * `ns_addr_boundary` - Boundary address where secure flash ends and non-secure flash starts
/// * `nsc_addr_boundary` - Boundary address where non-secure callable flash starts
pub const fn trustzone_flash_cfg(ns_addr_boundary: u32, nsc_addr_boundary: u32) -> u32 {
    ((ns_addr_boundary << 10) | !0x0001FC00) & (nsc_addr_boundary | !0x000003FF)
}

/// Initializer for [`Ccfg`::trustzone_sram_cfg`] field
///
/// # Arguments
/// * `ns_addr_boundary` - Boundary address where secure sram ends and non-secure sram starts
/// * `nsc_addr_boundary` - Boundary address where non-secure callable sram starts
pub const fn trustzone_sram_cfg(ns_addr_boundary: u32, nsc_addr_boundary: u32) -> u32 {
    ((ns_addr_boundary << 9) | !0x0003FE00) & (nsc_addr_boundary | !0x000001FF)
}

/// Initializer for [`Ccfg`::sram_cfg`] field
///
/// # Arguments
/// * `bank_enable` - Enable SRAM banks
/// * `parity_disable` - Boundary address where non-secure callable sram starts
pub const fn sram_cfg(bank_enable: u32, parity_disable: bool) -> u32 {
    ((bank_enable << 8) | !0xFFFFFF00) & (parity_disable as u32 | !0x00000001)
}

/// Initializer for [`Ccfg`::cpu_lock_cfg`] field
///
/// # Arguments
/// * `disable_ns_vtor_lock` - Disable lock on the non-secure vector table address
/// * `disable_s_aircr_lock` - Disable lock on the secure AIRCR register
/// * `disable_sau_lock` - Disable lock on SAU regions
/// * `disable_ns_mpu_lock` - Disable lock on non-secure MPU
/// * `disable_s_mpu_lock` - Disable lock on secure MPU
pub const fn cpu_lock_cfg(
    disable_ns_vtor_lock: bool,
    disable_s_aircr_lock: bool,
    disable_sau_lock: bool,
    disable_ns_mpu_lock: bool,
    disable_s_mpu_lock: bool,
) -> u32 {
    (disable_ns_vtor_lock as u32) << 4
        | (disable_s_aircr_lock as u32) << 3
        | (disable_sau_lock as u32) << 2
        | (disable_ns_mpu_lock as u32) << 1
        | (disable_s_mpu_lock as u32)
        | 0xFFFF_FFE0
}

/// Initializer for [`Ccfg`::dbg_auth_cfg`] field
///
/// # Arguments
/// * `s_non_invaisive_dbg_en` - Internal secure non-invasive debug enable
/// * `s_non_invaisive_dbg_sel` - Internal secure non-invasive debug select
/// * `s_invaisive_dbg_en` - Internal secure invasive debug enable
/// * `s_invaisive_dbg_sel` - Internal secure invasive debug select
pub const fn dbg_auth_cfg(
    s_non_invaisive_dbg_en: bool,
    s_non_invaisive_dbg_sel: bool,
    s_invaisive_dbg_en: bool,
    s_invaisive_dbg_sel: bool,
) -> u32 {
    (s_non_invaisive_dbg_en as u32) << 3
        | (s_non_invaisive_dbg_sel as u32) << 2
        | (s_invaisive_dbg_en as u32) << 1
        | (s_invaisive_dbg_sel as u32)
        | 0xFFFF_FFF0
}

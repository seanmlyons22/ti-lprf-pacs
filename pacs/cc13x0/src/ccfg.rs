#[repr(C)]
pub struct Ccfg {
    pub ext_lf_clk: u32,         // 0x50004FA8
    pub mode_conf_1: u32,        // 0x50004FAC
    pub size_and_dis_flags: u32, // 0x50004FB0
    pub mode_conf: u32,          // 0x50004FB4
    pub volt_load_0: u32,        // 0x50004FB8
    pub volt_load_1: u32,        // 0x50004FBC
    pub rtc_offset: u32,         // 0x50004FC0
    pub freq_offset: u32,        // 0x50004FC4
    pub ieee_mac_0: u32,         // 0x50004FC8
    pub ieee_mac_1: u32,         // 0x50004FCC
    pub ieee_ble_0: u32,         // 0x50004FD0
    pub ieee_ble_1: u32,         // 0x50004FD4
    pub bl_config: u32,          // 0x50004FD8
    pub erase_conf: u32,         // 0x5 0004FDC
    pub ccfg_ti_options: u32,    // 0x50004FE0
    pub ccfg_tap_dap_0: u32,     // 0x50004FE4
    pub ccfg_tap_dap_1: u32,     // 0x50004FE8
    pub image_valid_conf: u32,   // 0x50004FEC
    pub ccfg_prot_31_0: u32,     // 0x50004FF0
    pub ccfg_prot_63_32: u32,    // 0x50004FF4
    pub ccfg_prot_95_64: u32,    // 0x50004FF8
    pub ccfg_prot_127_96: u32,   // 0x50004FFC
}

pub const CCFG_EXT_LF_CLK_RTC_INCREMENT: u32 = 0x800000;
pub const CCFG_EXT_LF_CLK_DIO: u8 = 0x01;
pub const CCFG_MODE_CONF_1_TCXO_TYPE: u8 = 0x1;
pub const CCFG_MODE_CONF_1_TCXO_MAX_START: u8 = 0x7F; // Maximum TCXO startup time in units of 100us
pub const CCFG_MODE_CONF_1_ALT_DCDC_VMIN: u8 = 0x8;
pub const CCFG_MODE_CONF_1_ALT_DCDC_DITHER_EN: bool = false;
pub const CCFG_MODE_CONF_1_ALT_DCDC_IPEAK: u8 = 0x0;
pub const CCFG_MODE_CONF_1_ALT_IBIAS_INIT: u8 = 0x0;
pub const CCFG_MODE_CONF_1_ALT_IBIAS_OFFSET: u8 = 0x0;
pub const CCFG_MODE_CONF_1_ALT_IBIAS_MAX_START: u8 = 0x10;
pub const SIZE_OF_CCFG: u8 = 0x58;
pub const CCFG_MODE_CONF_VDDR_TRIM_SLEEP_DELTA: u8 = 0xF;
pub const CCFG_MODE_CONF_DCDC_RECHARGE_DISABLE: bool = false;
pub const CCFG_MODE_CONF_VDDS_BOD_LEVEL_1_8V: u8 = 0x01; // VDDS BOD level is 1.8V (or 1.65V for external regulator mode)
pub const CCFG_MODE_CONF_SCLK_LF_DERIVED_FROM_HF: u8 = 0x0; // LF clock derived from HF clock. Note: using this configuration will block the device from entering Standby mode.
pub const CCFG_MODE_CONF_SCLK_LF_EXT: u8 = 0x1; // External LF clock
pub const CCFG_MODE_CONF_SCLK_LF_XTAL: u8 = 0x2; // LF XOSC
pub const CCFG_MODE_CONF_SCLK_LF_RCOSC: u8 = 0x3; // LF RCOSC
pub const CCFG_MODE_CONF_XOSC_TCXO: u8 = 0x0; // HF source is 48 MHz TCXO
pub const CCFG_MODE_CONF_XOSC_BAW: u8 = 0x1; // HF source is HPOSC (BAW) (only valid for CC2652RB)
pub const CCFG_MODE_CONF_XOSC_XTAL: u8 = 0x2; // HF source is a 48 MHz xtal
pub const CCFG_BL_CONFIG_BL_DISABLE: u8 = 0x00; // Disable ROM bootloader
pub const CCFG_BL_CONFIG_BL_ENABLE: u8 = 0xC5; // Enable ROM bootloader
pub const CCFG_BL_CONFIG_BL_BACKDOOR_PIN: u8 = 0xFF; // Default ROM bootloader backdoor PIN
pub const CCFG_BL_CONFIG_BL_BACKDOOR_DISABLE: u8 = 0xFF; // Default ROM bootloader backdoor PIN
pub const CCFG_BL_CONFIG_BL_BACKDOOR_ENABLE: u8 = 0xC5; // Default ROM bootloader backdoor PIN\
pub const CCFG_CCFG_TI_OPTIONS_TI_FA_DISABLE: u8 = 0x00; // Disable unlocking of TI FA option
pub const CCFG_CCFG_TI_OPTIONS_TI_FA_ENABLE: u8 = 0xC5; // Enable unlocking of TI FA option with the unlock key
pub const CCFG_TAP_DAP_0_CPU_DAP_DISABLE: u8 = 0x00; // Access disabled
pub const CCFG_TAP_DAP_0_CPU_DAP_ENABLE: u8 = 0xC5; // Access enabled if also enabled in FCFG
pub const CCFG_TAP_DAP_0_PWRPROF_TAP_DISABLE: u8 = 0x00; // Access disabled
pub const CCFG_TAP_DAP_0_PWRPROF_TAP_ENABLE: u8 = 0xC5; // Access enabled if also enabled in FCFG
pub const CCFG_TAP_DAP_0_TEST_TAP_DISABLE: u8 = 0x00; // Access disabled
pub const CCFG_TAP_DAP_0_TEST_TAP_ENABLE: u8 = 0xC5; // Access enabled if also enabled in FCFG
pub const CCFG_TAP_DAP_1_AON_TAP_DISABLE: u8 = 0x00; // Access disabled
pub const CCFG_TAP_DAP_1_AON_TAP_ENABLE: u8 = 0xC5; // Access enabled if also enabled in FCFG
pub const CCFG_TAP_DAP_1_PBIST1_TAP_DISABLE: u8 = 0x00; // Access disabled
pub const CCFG_TAP_DAP_1_PBIST1_TAP_ENABLE: u8 = 0xC5; // Access enabled if also enabled in FCFG
pub const CCFG_TAP_DAP_1_PBIST2_TAP_DISABLE: u8 = 0x00; // Access disabled
pub const CCFG_TAP_DAP_1_PBIST2_TAP_ENABLE: u8 = 0xC5; // Access enabled if also enabled in FCFG

pub const fn ext_lf_clk(rtc_increment: u32, dio: u8) -> u32 {
    (rtc_increment & 0x00ff_ffff) | ((dio as u32 & 0xff) << 24)
}
// define an constant function that can be used to set the value of the DEFAULT_CCFG_MODE_CONF_1
pub const fn mode_conf_1(
    tcxo_type: u8,
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

pub const fn size_and_dis_flags(
    disable_linear_cap_array_delta: bool,
    disable_gpram: bool,
    disable_alt_dcdc_setting: bool,
    disable_xosc_override: bool,
) -> u32 {
    const DISABLE_FLAGS_MASK: u32 = 0x0000FFE0;

    ((SIZE_OF_CCFG as u32) << 16)
    | (DISABLE_FLAGS_MASK)
    | ((disable_linear_cap_array_delta as u32) << 4)
    | (0x1 << 3) // Disable TCXO no longer used always set to one
    | ((disable_gpram as u32) << 2)
    | ((disable_alt_dcdc_setting as u32) << 1)
    | ((disable_xosc_override as u32) << 0)
}

pub const fn mode_conf(
    trim_sleep_delta: u8,
    disable_dcdc_recharge: bool,
    disable_dcdc_active: bool,
    vdds_bod_level: u8,
    lf_clk_option: u8,
    disable_vddr_sleep_trim_temp_comp: bool,
    xosc_source: u8,
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
    | ((cap_array_delta as u32) << 8) // HF XOSC, don't touch!
    | (0xFF as u32)
}

pub const fn bl_config(
    bl_enable: bool,
    backdoor_enable: bool,
    bl_backdoor_active_high: bool,
    bl_backdoor_pin: u8,
) -> u32 {
    let enable = if bl_enable {
        CCFG_BL_CONFIG_BL_ENABLE
    } else {
        CCFG_BL_CONFIG_BL_DISABLE
    };
    let backdoor = if backdoor_enable {
        CCFG_BL_CONFIG_BL_BACKDOOR_ENABLE
    } else {
        CCFG_BL_CONFIG_BL_BACKDOOR_DISABLE
    };

    ((enable as u32) << 24)
        | ((bl_backdoor_active_high as u32 | 0xFE) << 16)
        | ((bl_backdoor_pin as u32) << 8)
        | (backdoor as u32)
}

pub const fn erase_conf(enable_bl_chip_erase: bool, enable_bl_bank_erase: bool) -> u32 {
    (0xFFFF << 16)
        | ((enable_bl_chip_erase as u32 | 0xFE) << 8)
        | (enable_bl_bank_erase as u32 | 0xFE)
}

pub const fn ccfg_ti_options(enable_ti_fa: u8) -> u32 {
    enable_ti_fa as u32 | 0xFFFFFF00
}

pub const fn ccfg_tap_dap_0(cpu_dap: u8, pwrprof_tap: u8, test_tap: u8) -> u32 {
    (0xFF << 24) | (cpu_dap as u32) << 16 | (pwrprof_tap as u32) << 8 | (test_tap as u32)
}

pub const fn ccfg_tap_dap_1(aon_tap: u8, pbist1_tap: u8, pbist2_tap: u8) -> u32 {
    (0xFF << 24) | (pbist2_tap as u32) << 16 | (pbist1_tap as u32) << 8 | (aon_tap as u32)
}

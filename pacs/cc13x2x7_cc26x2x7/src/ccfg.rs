#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved_0: RESERVED_0,
    _reserved1: [u8; 0x1fa4],
    #[doc = "0x1fa8 - Extern LF clock configuration"]
    pub ext_lf_clk: EXT_LF_CLK,
    #[doc = "0x1fac - Mode Configuration 1"]
    pub mode_conf_1: MODE_CONF_1,
    #[doc = "0x1fb0 - CCFG Size and Disable Flags"]
    pub size_and_dis_flags: SIZE_AND_DIS_FLAGS,
    #[doc = "0x1fb4 - Mode Configuration 0"]
    pub mode_conf: MODE_CONF,
    #[doc = "0x1fb8 - Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    pub volt_load_0: VOLT_LOAD_0,
    #[doc = "0x1fbc - Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    pub volt_load_1: VOLT_LOAD_1,
    #[doc = "0x1fc0 - Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP."]
    pub rtc_offset: RTC_OFFSET,
    #[doc = "0x1fc4 - Frequency Offset"]
    pub freq_offset: FREQ_OFFSET,
    #[doc = "0x1fc8 - IEEE MAC Address 0"]
    pub ieee_mac_0: IEEE_MAC_0,
    #[doc = "0x1fcc - IEEE MAC Address 1"]
    pub ieee_mac_1: IEEE_MAC_1,
    #[doc = "0x1fd0 - IEEE BLE Address 0"]
    pub ieee_ble_0: IEEE_BLE_0,
    #[doc = "0x1fd4 - IEEE BLE Address 1"]
    pub ieee_ble_1: IEEE_BLE_1,
    #[doc = "0x1fd8 - Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
    pub bl_config: BL_CONFIG,
    #[doc = "0x1fdc - Erase Configuration"]
    pub erase_conf: ERASE_CONF,
    #[doc = "0x1fe0 - TI Options"]
    pub ccfg_ti_options: CCFG_TI_OPTIONS,
    #[doc = "0x1fe4 - Test Access Points Enable 0"]
    pub ccfg_tap_dap_0: CCFG_TAP_DAP_0,
    #[doc = "0x1fe8 - Test Access Points Enable 1"]
    pub ccfg_tap_dap_1: CCFG_TAP_DAP_1,
    #[doc = "0x1fec - Image Valid"]
    pub image_valid_conf: IMAGE_VALID_CONF,
    #[doc = "0x1ff0 - Protect Sectors 0-31 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
    pub ccfg_prot_31_0: CCFG_PROT_31_0,
    #[doc = "0x1ff4 - Protect Sectors 32-63 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
    pub ccfg_prot_63_32: CCFG_PROT_63_32,
    #[doc = "0x1ff8 - Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
    pub ccfg_prot_95_64: CCFG_PROT_95_64,
    #[doc = "0x1ffc - Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
    pub ccfg_prot_127_96: CCFG_PROT_127_96,
}
#[doc = "RESERVED_0 (rw) register accessor: an alias for `Reg<RESERVED_0_SPEC>`"]
pub type RESERVED_0 = crate::Reg<reserved_0::RESERVED_0_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_0;
#[doc = "EXT_LF_CLK (rw) register accessor: an alias for `Reg<EXT_LF_CLK_SPEC>`"]
pub type EXT_LF_CLK = crate::Reg<ext_lf_clk::EXT_LF_CLK_SPEC>;
#[doc = "Extern LF clock configuration"]
pub mod ext_lf_clk;
#[doc = "MODE_CONF_1 (rw) register accessor: an alias for `Reg<MODE_CONF_1_SPEC>`"]
pub type MODE_CONF_1 = crate::Reg<mode_conf_1::MODE_CONF_1_SPEC>;
#[doc = "Mode Configuration 1"]
pub mod mode_conf_1;
#[doc = "SIZE_AND_DIS_FLAGS (rw) register accessor: an alias for `Reg<SIZE_AND_DIS_FLAGS_SPEC>`"]
pub type SIZE_AND_DIS_FLAGS = crate::Reg<size_and_dis_flags::SIZE_AND_DIS_FLAGS_SPEC>;
#[doc = "CCFG Size and Disable Flags"]
pub mod size_and_dis_flags;
#[doc = "MODE_CONF (rw) register accessor: an alias for `Reg<MODE_CONF_SPEC>`"]
pub type MODE_CONF = crate::Reg<mode_conf::MODE_CONF_SPEC>;
#[doc = "Mode Configuration 0"]
pub mod mode_conf;
#[doc = "VOLT_LOAD_0 (rw) register accessor: an alias for `Reg<VOLT_LOAD_0_SPEC>`"]
pub type VOLT_LOAD_0 = crate::Reg<volt_load_0::VOLT_LOAD_0_SPEC>;
#[doc = "Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub mod volt_load_0;
#[doc = "VOLT_LOAD_1 (rw) register accessor: an alias for `Reg<VOLT_LOAD_1_SPEC>`"]
pub type VOLT_LOAD_1 = crate::Reg<volt_load_1::VOLT_LOAD_1_SPEC>;
#[doc = "Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub mod volt_load_1;
#[doc = "RTC_OFFSET (rw) register accessor: an alias for `Reg<RTC_OFFSET_SPEC>`"]
pub type RTC_OFFSET = crate::Reg<rtc_offset::RTC_OFFSET_SPEC>;
#[doc = "Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP."]
pub mod rtc_offset;
#[doc = "FREQ_OFFSET (rw) register accessor: an alias for `Reg<FREQ_OFFSET_SPEC>`"]
pub type FREQ_OFFSET = crate::Reg<freq_offset::FREQ_OFFSET_SPEC>;
#[doc = "Frequency Offset"]
pub mod freq_offset;
#[doc = "IEEE_MAC_0 (rw) register accessor: an alias for `Reg<IEEE_MAC_0_SPEC>`"]
pub type IEEE_MAC_0 = crate::Reg<ieee_mac_0::IEEE_MAC_0_SPEC>;
#[doc = "IEEE MAC Address 0"]
pub mod ieee_mac_0;
#[doc = "IEEE_MAC_1 (rw) register accessor: an alias for `Reg<IEEE_MAC_1_SPEC>`"]
pub type IEEE_MAC_1 = crate::Reg<ieee_mac_1::IEEE_MAC_1_SPEC>;
#[doc = "IEEE MAC Address 1"]
pub mod ieee_mac_1;
#[doc = "IEEE_BLE_0 (rw) register accessor: an alias for `Reg<IEEE_BLE_0_SPEC>`"]
pub type IEEE_BLE_0 = crate::Reg<ieee_ble_0::IEEE_BLE_0_SPEC>;
#[doc = "IEEE BLE Address 0"]
pub mod ieee_ble_0;
#[doc = "IEEE_BLE_1 (rw) register accessor: an alias for `Reg<IEEE_BLE_1_SPEC>`"]
pub type IEEE_BLE_1 = crate::Reg<ieee_ble_1::IEEE_BLE_1_SPEC>;
#[doc = "IEEE BLE Address 1"]
pub mod ieee_ble_1;
#[doc = "BL_CONFIG (rw) register accessor: an alias for `Reg<BL_CONFIG_SPEC>`"]
pub type BL_CONFIG = crate::Reg<bl_config::BL_CONFIG_SPEC>;
#[doc = "Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
pub mod bl_config;
#[doc = "ERASE_CONF (rw) register accessor: an alias for `Reg<ERASE_CONF_SPEC>`"]
pub type ERASE_CONF = crate::Reg<erase_conf::ERASE_CONF_SPEC>;
#[doc = "Erase Configuration"]
pub mod erase_conf;
#[doc = "CCFG_TI_OPTIONS (rw) register accessor: an alias for `Reg<CCFG_TI_OPTIONS_SPEC>`"]
pub type CCFG_TI_OPTIONS = crate::Reg<ccfg_ti_options::CCFG_TI_OPTIONS_SPEC>;
#[doc = "TI Options"]
pub mod ccfg_ti_options;
#[doc = "CCFG_TAP_DAP_0 (rw) register accessor: an alias for `Reg<CCFG_TAP_DAP_0_SPEC>`"]
pub type CCFG_TAP_DAP_0 = crate::Reg<ccfg_tap_dap_0::CCFG_TAP_DAP_0_SPEC>;
#[doc = "Test Access Points Enable 0"]
pub mod ccfg_tap_dap_0;
#[doc = "CCFG_TAP_DAP_1 (rw) register accessor: an alias for `Reg<CCFG_TAP_DAP_1_SPEC>`"]
pub type CCFG_TAP_DAP_1 = crate::Reg<ccfg_tap_dap_1::CCFG_TAP_DAP_1_SPEC>;
#[doc = "Test Access Points Enable 1"]
pub mod ccfg_tap_dap_1;
#[doc = "IMAGE_VALID_CONF (rw) register accessor: an alias for `Reg<IMAGE_VALID_CONF_SPEC>`"]
pub type IMAGE_VALID_CONF = crate::Reg<image_valid_conf::IMAGE_VALID_CONF_SPEC>;
#[doc = "Image Valid"]
pub mod image_valid_conf;
#[doc = "CCFG_PROT_31_0 (rw) register accessor: an alias for `Reg<CCFG_PROT_31_0_SPEC>`"]
pub type CCFG_PROT_31_0 = crate::Reg<ccfg_prot_31_0::CCFG_PROT_31_0_SPEC>;
#[doc = "Protect Sectors 0-31 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
pub mod ccfg_prot_31_0;
#[doc = "CCFG_PROT_63_32 (rw) register accessor: an alias for `Reg<CCFG_PROT_63_32_SPEC>`"]
pub type CCFG_PROT_63_32 = crate::Reg<ccfg_prot_63_32::CCFG_PROT_63_32_SPEC>;
#[doc = "Protect Sectors 32-63 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
pub mod ccfg_prot_63_32;
#[doc = "CCFG_PROT_95_64 (rw) register accessor: an alias for `Reg<CCFG_PROT_95_64_SPEC>`"]
pub type CCFG_PROT_95_64 = crate::Reg<ccfg_prot_95_64::CCFG_PROT_95_64_SPEC>;
#[doc = "Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
pub mod ccfg_prot_95_64;
#[doc = "CCFG_PROT_127_96 (rw) register accessor: an alias for `Reg<CCFG_PROT_127_96_SPEC>`"]
pub type CCFG_PROT_127_96 = crate::Reg<ccfg_prot_127_96::CCFG_PROT_127_96_SPEC>;
#[doc = "Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
pub mod ccfg_prot_127_96;

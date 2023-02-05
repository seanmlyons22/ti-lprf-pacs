#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CCFG Size and Disable Flags"]
    pub size_and_dis_flags: SIZE_AND_DIS_FLAGS,
    #[doc = "0x04 - Mode Configuration 0"]
    pub mode_conf: MODE_CONF,
    #[doc = "0x08 - Mode Configuration 1"]
    pub mode_conf_1: MODE_CONF_1,
    #[doc = "0x0c - Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    pub volt_load_0: VOLT_LOAD_0,
    #[doc = "0x10 - Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    pub volt_load_1: VOLT_LOAD_1,
    #[doc = "0x14 - Extern LF clock configuration"]
    pub ext_lf_clk: EXT_LF_CLK,
    #[doc = "0x18 - IEEE MAC Address 0"]
    pub ieee_mac_0: IEEE_MAC_0,
    #[doc = "0x1c - IEEE MAC Address 1"]
    pub ieee_mac_1: IEEE_MAC_1,
    #[doc = "0x20 - IEEE BLE Address 0"]
    pub ieee_ble_0: IEEE_BLE_0,
    #[doc = "0x24 - IEEE BLE Address 1"]
    pub ieee_ble_1: IEEE_BLE_1,
    #[doc = "0x28 - Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
    pub bl_config: BL_CONFIG,
    #[doc = "0x2c - Erase Configuration"]
    pub erase_conf: ERASE_CONF,
    #[doc = "0x30 - Erase Configuration 1"]
    pub erase_conf_1: ERASE_CONF_1,
    #[doc = "0x34 - TI Options"]
    pub ccfg_ti_options: CCFG_TI_OPTIONS,
    #[doc = "0x38 - Test Access Points Enable 0"]
    pub ccfg_tap_dap_0: CCFG_TAP_DAP_0,
    #[doc = "0x3c - Test Access Points Enable 1"]
    pub ccfg_tap_dap_1: CCFG_TAP_DAP_1,
    #[doc = "0x40 - Image Valid"]
    pub image_valid_conf: IMAGE_VALID_CONF,
    #[doc = "0x44 - Protect Sectors 0-31 Each bit write protects one 2KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector WriteErase protect."]
    pub ccfg_weprot_31_0_by2k: CCFG_WEPROT_31_0_BY2K,
    #[doc = "0x48 - Spare register for WriteErase configuration"]
    pub ccfg_weprot_spare_1: CCFG_WEPROT_SPARE_1,
    #[doc = "0x4c - Spare register for WriteErase configuration"]
    pub ccfg_weprot_spare_2: CCFG_WEPROT_SPARE_2,
    #[doc = "0x50 - Spare register for WriteErase configuration"]
    pub ccfg_weprot_spare_3: CCFG_WEPROT_SPARE_3,
    #[doc = "0x54 - Trustzone configuration register for flash"]
    pub trustzone_flash_cfg: TRUSTZONE_FLASH_CFG,
    #[doc = "0x58 - Trustzone configuration register for MCU SRAM"]
    pub trustzone_sram_cfg: TRUSTZONE_SRAM_CFG,
    #[doc = "0x5c - Configuration register for MCU SRAM"]
    pub sram_cfg: SRAM_CFG,
    _reserved24: [u8; 0x04],
    #[doc = "0x64 - Configuration register for MCU CPU lock options"]
    pub cpu_lock_cfg: CPU_LOCK_CFG,
    #[doc = "0x68 - Configuration register for debug authentication"]
    pub deb_auth_cfg: DEB_AUTH_CFG,
    #[doc = "0x6c - Customer key"]
    pub ckey0: CKEY0,
    #[doc = "0x70 - Customer key"]
    pub ckey1: CKEY1,
    #[doc = "0x74 - Customer key"]
    pub ckey2: CKEY2,
    #[doc = "0x78 - Customer key"]
    pub ckey3: CKEY3,
}
#[doc = "SIZE_AND_DIS_FLAGS (rw) register accessor: an alias for `Reg<SIZE_AND_DIS_FLAGS_SPEC>`"]
pub type SIZE_AND_DIS_FLAGS = crate::Reg<size_and_dis_flags::SIZE_AND_DIS_FLAGS_SPEC>;
#[doc = "CCFG Size and Disable Flags"]
pub mod size_and_dis_flags;
#[doc = "MODE_CONF (rw) register accessor: an alias for `Reg<MODE_CONF_SPEC>`"]
pub type MODE_CONF = crate::Reg<mode_conf::MODE_CONF_SPEC>;
#[doc = "Mode Configuration 0"]
pub mod mode_conf;
#[doc = "MODE_CONF_1 (rw) register accessor: an alias for `Reg<MODE_CONF_1_SPEC>`"]
pub type MODE_CONF_1 = crate::Reg<mode_conf_1::MODE_CONF_1_SPEC>;
#[doc = "Mode Configuration 1"]
pub mod mode_conf_1;
#[doc = "VOLT_LOAD_0 (rw) register accessor: an alias for `Reg<VOLT_LOAD_0_SPEC>`"]
pub type VOLT_LOAD_0 = crate::Reg<volt_load_0::VOLT_LOAD_0_SPEC>;
#[doc = "Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub mod volt_load_0;
#[doc = "VOLT_LOAD_1 (rw) register accessor: an alias for `Reg<VOLT_LOAD_1_SPEC>`"]
pub type VOLT_LOAD_1 = crate::Reg<volt_load_1::VOLT_LOAD_1_SPEC>;
#[doc = "Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub mod volt_load_1;
#[doc = "EXT_LF_CLK (rw) register accessor: an alias for `Reg<EXT_LF_CLK_SPEC>`"]
pub type EXT_LF_CLK = crate::Reg<ext_lf_clk::EXT_LF_CLK_SPEC>;
#[doc = "Extern LF clock configuration"]
pub mod ext_lf_clk;
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
#[doc = "ERASE_CONF_1 (rw) register accessor: an alias for `Reg<ERASE_CONF_1_SPEC>`"]
pub type ERASE_CONF_1 = crate::Reg<erase_conf_1::ERASE_CONF_1_SPEC>;
#[doc = "Erase Configuration 1"]
pub mod erase_conf_1;
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
#[doc = "CCFG_WEPROT_31_0_BY2K (rw) register accessor: an alias for `Reg<CCFG_WEPROT_31_0_BY2K_SPEC>`"]
pub type CCFG_WEPROT_31_0_BY2K = crate::Reg<ccfg_weprot_31_0_by2k::CCFG_WEPROT_31_0_BY2K_SPEC>;
#[doc = "Protect Sectors 0-31 Each bit write protects one 2KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector WriteErase protect."]
pub mod ccfg_weprot_31_0_by2k;
#[doc = "CCFG_WEPROT_SPARE_1 (rw) register accessor: an alias for `Reg<CCFG_WEPROT_SPARE_1_SPEC>`"]
pub type CCFG_WEPROT_SPARE_1 = crate::Reg<ccfg_weprot_spare_1::CCFG_WEPROT_SPARE_1_SPEC>;
#[doc = "Spare register for WriteErase configuration"]
pub mod ccfg_weprot_spare_1;
#[doc = "CCFG_WEPROT_SPARE_2 (rw) register accessor: an alias for `Reg<CCFG_WEPROT_SPARE_2_SPEC>`"]
pub type CCFG_WEPROT_SPARE_2 = crate::Reg<ccfg_weprot_spare_2::CCFG_WEPROT_SPARE_2_SPEC>;
#[doc = "Spare register for WriteErase configuration"]
pub mod ccfg_weprot_spare_2;
#[doc = "CCFG_WEPROT_SPARE_3 (rw) register accessor: an alias for `Reg<CCFG_WEPROT_SPARE_3_SPEC>`"]
pub type CCFG_WEPROT_SPARE_3 = crate::Reg<ccfg_weprot_spare_3::CCFG_WEPROT_SPARE_3_SPEC>;
#[doc = "Spare register for WriteErase configuration"]
pub mod ccfg_weprot_spare_3;
#[doc = "TRUSTZONE_FLASH_CFG (rw) register accessor: an alias for `Reg<TRUSTZONE_FLASH_CFG_SPEC>`"]
pub type TRUSTZONE_FLASH_CFG = crate::Reg<trustzone_flash_cfg::TRUSTZONE_FLASH_CFG_SPEC>;
#[doc = "Trustzone configuration register for flash"]
pub mod trustzone_flash_cfg;
#[doc = "TRUSTZONE_SRAM_CFG (rw) register accessor: an alias for `Reg<TRUSTZONE_SRAM_CFG_SPEC>`"]
pub type TRUSTZONE_SRAM_CFG = crate::Reg<trustzone_sram_cfg::TRUSTZONE_SRAM_CFG_SPEC>;
#[doc = "Trustzone configuration register for MCU SRAM"]
pub mod trustzone_sram_cfg;
#[doc = "SRAM_CFG (rw) register accessor: an alias for `Reg<SRAM_CFG_SPEC>`"]
pub type SRAM_CFG = crate::Reg<sram_cfg::SRAM_CFG_SPEC>;
#[doc = "Configuration register for MCU SRAM"]
pub mod sram_cfg;
#[doc = "CPU_LOCK_CFG (rw) register accessor: an alias for `Reg<CPU_LOCK_CFG_SPEC>`"]
pub type CPU_LOCK_CFG = crate::Reg<cpu_lock_cfg::CPU_LOCK_CFG_SPEC>;
#[doc = "Configuration register for MCU CPU lock options"]
pub mod cpu_lock_cfg;
#[doc = "DEB_AUTH_CFG (rw) register accessor: an alias for `Reg<DEB_AUTH_CFG_SPEC>`"]
pub type DEB_AUTH_CFG = crate::Reg<deb_auth_cfg::DEB_AUTH_CFG_SPEC>;
#[doc = "Configuration register for debug authentication"]
pub mod deb_auth_cfg;
#[doc = "CKEY0 (rw) register accessor: an alias for `Reg<CKEY0_SPEC>`"]
pub type CKEY0 = crate::Reg<ckey0::CKEY0_SPEC>;
#[doc = "Customer key"]
pub mod ckey0;
#[doc = "CKEY1 (rw) register accessor: an alias for `Reg<CKEY1_SPEC>`"]
pub type CKEY1 = crate::Reg<ckey1::CKEY1_SPEC>;
#[doc = "Customer key"]
pub mod ckey1;
#[doc = "CKEY2 (rw) register accessor: an alias for `Reg<CKEY2_SPEC>`"]
pub type CKEY2 = crate::Reg<ckey2::CKEY2_SPEC>;
#[doc = "Customer key"]
pub mod ckey2;
#[doc = "CKEY3 (rw) register accessor: an alias for `Reg<CKEY3_SPEC>`"]
pub type CKEY3 = crate::Reg<ckey3::CKEY3_SPEC>;
#[doc = "Customer key"]
pub mod ckey3;

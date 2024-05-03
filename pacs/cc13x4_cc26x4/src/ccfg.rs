#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    size_and_dis_flags: SizeAndDisFlags,
    mode_conf: ModeConf,
    mode_conf_1: ModeConf1,
    volt_load_0: VoltLoad0,
    volt_load_1: VoltLoad1,
    ext_lf_clk: ExtLfClk,
    ieee_mac_0: IeeeMac0,
    ieee_mac_1: IeeeMac1,
    ieee_ble_0: IeeeBle0,
    ieee_ble_1: IeeeBle1,
    bl_config: BlConfig,
    erase_conf: EraseConf,
    erase_conf_1: EraseConf1,
    ccfg_ti_options: CcfgTiOptions,
    ccfg_tap_dap_0: CcfgTapDap0,
    ccfg_tap_dap_1: CcfgTapDap1,
    image_valid_conf: ImageValidConf,
    ccfg_weprot_31_0_by2k: CcfgWeprot31_0By2k,
    ccfg_weprot_spare_1: CcfgWeprotSpare1,
    ccfg_weprot_spare_2: CcfgWeprotSpare2,
    ccfg_weprot_spare_3: CcfgWeprotSpare3,
    trustzone_flash_cfg: TrustzoneFlashCfg,
    trustzone_sram_cfg: TrustzoneSramCfg,
    sram_cfg: SramCfg,
    _reserved24: [u8; 0x04],
    cpu_lock_cfg: CpuLockCfg,
    deb_auth_cfg: DebAuthCfg,
    ckey0: Ckey0,
    ckey1: Ckey1,
    ckey2: Ckey2,
    ckey3: Ckey3,
}
impl RegisterBlock {
    #[doc = "0x00 - CCFG Size and Disable Flags"]
    #[inline(always)]
    pub const fn size_and_dis_flags(&self) -> &SizeAndDisFlags {
        &self.size_and_dis_flags
    }
    #[doc = "0x04 - Mode Configuration 0"]
    #[inline(always)]
    pub const fn mode_conf(&self) -> &ModeConf {
        &self.mode_conf
    }
    #[doc = "0x08 - Mode Configuration 1"]
    #[inline(always)]
    pub const fn mode_conf_1(&self) -> &ModeConf1 {
        &self.mode_conf_1
    }
    #[doc = "0x0c - Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    #[inline(always)]
    pub const fn volt_load_0(&self) -> &VoltLoad0 {
        &self.volt_load_0
    }
    #[doc = "0x10 - Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    #[inline(always)]
    pub const fn volt_load_1(&self) -> &VoltLoad1 {
        &self.volt_load_1
    }
    #[doc = "0x14 - Extern LF clock configuration"]
    #[inline(always)]
    pub const fn ext_lf_clk(&self) -> &ExtLfClk {
        &self.ext_lf_clk
    }
    #[doc = "0x18 - IEEE MAC Address 0"]
    #[inline(always)]
    pub const fn ieee_mac_0(&self) -> &IeeeMac0 {
        &self.ieee_mac_0
    }
    #[doc = "0x1c - IEEE MAC Address 1"]
    #[inline(always)]
    pub const fn ieee_mac_1(&self) -> &IeeeMac1 {
        &self.ieee_mac_1
    }
    #[doc = "0x20 - IEEE BLE Address 0"]
    #[inline(always)]
    pub const fn ieee_ble_0(&self) -> &IeeeBle0 {
        &self.ieee_ble_0
    }
    #[doc = "0x24 - IEEE BLE Address 1"]
    #[inline(always)]
    pub const fn ieee_ble_1(&self) -> &IeeeBle1 {
        &self.ieee_ble_1
    }
    #[doc = "0x28 - Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
    #[inline(always)]
    pub const fn bl_config(&self) -> &BlConfig {
        &self.bl_config
    }
    #[doc = "0x2c - Erase Configuration"]
    #[inline(always)]
    pub const fn erase_conf(&self) -> &EraseConf {
        &self.erase_conf
    }
    #[doc = "0x30 - Erase Configuration 1"]
    #[inline(always)]
    pub const fn erase_conf_1(&self) -> &EraseConf1 {
        &self.erase_conf_1
    }
    #[doc = "0x34 - TI Options"]
    #[inline(always)]
    pub const fn ccfg_ti_options(&self) -> &CcfgTiOptions {
        &self.ccfg_ti_options
    }
    #[doc = "0x38 - Test Access Points Enable 0"]
    #[inline(always)]
    pub const fn ccfg_tap_dap_0(&self) -> &CcfgTapDap0 {
        &self.ccfg_tap_dap_0
    }
    #[doc = "0x3c - Test Access Points Enable 1"]
    #[inline(always)]
    pub const fn ccfg_tap_dap_1(&self) -> &CcfgTapDap1 {
        &self.ccfg_tap_dap_1
    }
    #[doc = "0x40 - Image Valid"]
    #[inline(always)]
    pub const fn image_valid_conf(&self) -> &ImageValidConf {
        &self.image_valid_conf
    }
    #[doc = "0x44 - Protect Sectors 0-31 Each bit write protects one 2KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector WriteErase protect."]
    #[inline(always)]
    pub const fn ccfg_weprot_31_0_by2k(&self) -> &CcfgWeprot31_0By2k {
        &self.ccfg_weprot_31_0_by2k
    }
    #[doc = "0x48 - Spare register for WriteErase configuration"]
    #[inline(always)]
    pub const fn ccfg_weprot_spare_1(&self) -> &CcfgWeprotSpare1 {
        &self.ccfg_weprot_spare_1
    }
    #[doc = "0x4c - Spare register for WriteErase configuration"]
    #[inline(always)]
    pub const fn ccfg_weprot_spare_2(&self) -> &CcfgWeprotSpare2 {
        &self.ccfg_weprot_spare_2
    }
    #[doc = "0x50 - Spare register for WriteErase configuration"]
    #[inline(always)]
    pub const fn ccfg_weprot_spare_3(&self) -> &CcfgWeprotSpare3 {
        &self.ccfg_weprot_spare_3
    }
    #[doc = "0x54 - Trustzone configuration register for flash"]
    #[inline(always)]
    pub const fn trustzone_flash_cfg(&self) -> &TrustzoneFlashCfg {
        &self.trustzone_flash_cfg
    }
    #[doc = "0x58 - Trustzone configuration register for MCU SRAM"]
    #[inline(always)]
    pub const fn trustzone_sram_cfg(&self) -> &TrustzoneSramCfg {
        &self.trustzone_sram_cfg
    }
    #[doc = "0x5c - Configuration register for MCU SRAM"]
    #[inline(always)]
    pub const fn sram_cfg(&self) -> &SramCfg {
        &self.sram_cfg
    }
    #[doc = "0x64 - Configuration register for MCU CPU lock options"]
    #[inline(always)]
    pub const fn cpu_lock_cfg(&self) -> &CpuLockCfg {
        &self.cpu_lock_cfg
    }
    #[doc = "0x68 - Configuration register for debug authentication"]
    #[inline(always)]
    pub const fn deb_auth_cfg(&self) -> &DebAuthCfg {
        &self.deb_auth_cfg
    }
    #[doc = "0x6c - Customer key"]
    #[inline(always)]
    pub const fn ckey0(&self) -> &Ckey0 {
        &self.ckey0
    }
    #[doc = "0x70 - Customer key"]
    #[inline(always)]
    pub const fn ckey1(&self) -> &Ckey1 {
        &self.ckey1
    }
    #[doc = "0x74 - Customer key"]
    #[inline(always)]
    pub const fn ckey2(&self) -> &Ckey2 {
        &self.ckey2
    }
    #[doc = "0x78 - Customer key"]
    #[inline(always)]
    pub const fn ckey3(&self) -> &Ckey3 {
        &self.ckey3
    }
}
#[doc = "SIZE_AND_DIS_FLAGS (rw) register accessor: CCFG Size and Disable Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`size_and_dis_flags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`size_and_dis_flags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@size_and_dis_flags`]
module"]
#[doc(alias = "SIZE_AND_DIS_FLAGS")]
pub type SizeAndDisFlags = crate::Reg<size_and_dis_flags::SizeAndDisFlagsSpec>;
#[doc = "CCFG Size and Disable Flags"]
pub mod size_and_dis_flags;
#[doc = "MODE_CONF (rw) register accessor: Mode Configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode_conf`]
module"]
#[doc(alias = "MODE_CONF")]
pub type ModeConf = crate::Reg<mode_conf::ModeConfSpec>;
#[doc = "Mode Configuration 0"]
pub mod mode_conf;
#[doc = "MODE_CONF_1 (rw) register accessor: Mode Configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_conf_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_conf_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode_conf_1`]
module"]
#[doc(alias = "MODE_CONF_1")]
pub type ModeConf1 = crate::Reg<mode_conf_1::ModeConf1Spec>;
#[doc = "Mode Configuration 1"]
pub mod mode_conf_1;
#[doc = "VOLT_LOAD_0 (rw) register accessor: Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`volt_load_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`volt_load_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@volt_load_0`]
module"]
#[doc(alias = "VOLT_LOAD_0")]
pub type VoltLoad0 = crate::Reg<volt_load_0::VoltLoad0Spec>;
#[doc = "Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub mod volt_load_0;
#[doc = "VOLT_LOAD_1 (rw) register accessor: Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`volt_load_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`volt_load_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@volt_load_1`]
module"]
#[doc(alias = "VOLT_LOAD_1")]
pub type VoltLoad1 = crate::Reg<volt_load_1::VoltLoad1Spec>;
#[doc = "Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub mod volt_load_1;
#[doc = "EXT_LF_CLK (rw) register accessor: Extern LF clock configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_lf_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_lf_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_lf_clk`]
module"]
#[doc(alias = "EXT_LF_CLK")]
pub type ExtLfClk = crate::Reg<ext_lf_clk::ExtLfClkSpec>;
#[doc = "Extern LF clock configuration"]
pub mod ext_lf_clk;
#[doc = "IEEE_MAC_0 (rw) register accessor: IEEE MAC Address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_mac_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_mac_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieee_mac_0`]
module"]
#[doc(alias = "IEEE_MAC_0")]
pub type IeeeMac0 = crate::Reg<ieee_mac_0::IeeeMac0Spec>;
#[doc = "IEEE MAC Address 0"]
pub mod ieee_mac_0;
#[doc = "IEEE_MAC_1 (rw) register accessor: IEEE MAC Address 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_mac_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_mac_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieee_mac_1`]
module"]
#[doc(alias = "IEEE_MAC_1")]
pub type IeeeMac1 = crate::Reg<ieee_mac_1::IeeeMac1Spec>;
#[doc = "IEEE MAC Address 1"]
pub mod ieee_mac_1;
#[doc = "IEEE_BLE_0 (rw) register accessor: IEEE BLE Address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_ble_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_ble_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieee_ble_0`]
module"]
#[doc(alias = "IEEE_BLE_0")]
pub type IeeeBle0 = crate::Reg<ieee_ble_0::IeeeBle0Spec>;
#[doc = "IEEE BLE Address 0"]
pub mod ieee_ble_0;
#[doc = "IEEE_BLE_1 (rw) register accessor: IEEE BLE Address 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_ble_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_ble_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieee_ble_1`]
module"]
#[doc(alias = "IEEE_BLE_1")]
pub type IeeeBle1 = crate::Reg<ieee_ble_1::IeeeBle1Spec>;
#[doc = "IEEE BLE Address 1"]
pub mod ieee_ble_1;
#[doc = "BL_CONFIG (rw) register accessor: Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bl_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bl_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bl_config`]
module"]
#[doc(alias = "BL_CONFIG")]
pub type BlConfig = crate::Reg<bl_config::BlConfigSpec>;
#[doc = "Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
pub mod bl_config;
#[doc = "ERASE_CONF (rw) register accessor: Erase Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`erase_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`erase_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erase_conf`]
module"]
#[doc(alias = "ERASE_CONF")]
pub type EraseConf = crate::Reg<erase_conf::EraseConfSpec>;
#[doc = "Erase Configuration"]
pub mod erase_conf;
#[doc = "ERASE_CONF_1 (rw) register accessor: Erase Configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`erase_conf_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`erase_conf_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erase_conf_1`]
module"]
#[doc(alias = "ERASE_CONF_1")]
pub type EraseConf1 = crate::Reg<erase_conf_1::EraseConf1Spec>;
#[doc = "Erase Configuration 1"]
pub mod erase_conf_1;
#[doc = "CCFG_TI_OPTIONS (rw) register accessor: TI Options\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_ti_options::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_ti_options::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_ti_options`]
module"]
#[doc(alias = "CCFG_TI_OPTIONS")]
pub type CcfgTiOptions = crate::Reg<ccfg_ti_options::CcfgTiOptionsSpec>;
#[doc = "TI Options"]
pub mod ccfg_ti_options;
#[doc = "CCFG_TAP_DAP_0 (rw) register accessor: Test Access Points Enable 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_tap_dap_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_tap_dap_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_tap_dap_0`]
module"]
#[doc(alias = "CCFG_TAP_DAP_0")]
pub type CcfgTapDap0 = crate::Reg<ccfg_tap_dap_0::CcfgTapDap0Spec>;
#[doc = "Test Access Points Enable 0"]
pub mod ccfg_tap_dap_0;
#[doc = "CCFG_TAP_DAP_1 (rw) register accessor: Test Access Points Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_tap_dap_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_tap_dap_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_tap_dap_1`]
module"]
#[doc(alias = "CCFG_TAP_DAP_1")]
pub type CcfgTapDap1 = crate::Reg<ccfg_tap_dap_1::CcfgTapDap1Spec>;
#[doc = "Test Access Points Enable 1"]
pub mod ccfg_tap_dap_1;
#[doc = "IMAGE_VALID_CONF (rw) register accessor: Image Valid\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`image_valid_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`image_valid_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@image_valid_conf`]
module"]
#[doc(alias = "IMAGE_VALID_CONF")]
pub type ImageValidConf = crate::Reg<image_valid_conf::ImageValidConfSpec>;
#[doc = "Image Valid"]
pub mod image_valid_conf;
#[doc = "CCFG_WEPROT_31_0_BY2K (rw) register accessor: Protect Sectors 0-31 Each bit write protects one 2KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector WriteErase protect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_weprot_31_0_by2k::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_weprot_31_0_by2k::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_weprot_31_0_by2k`]
module"]
#[doc(alias = "CCFG_WEPROT_31_0_BY2K")]
pub type CcfgWeprot31_0By2k = crate::Reg<ccfg_weprot_31_0_by2k::CcfgWeprot31_0By2kSpec>;
#[doc = "Protect Sectors 0-31 Each bit write protects one 2KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector WriteErase protect."]
pub mod ccfg_weprot_31_0_by2k;
#[doc = "CCFG_WEPROT_SPARE_1 (rw) register accessor: Spare register for WriteErase configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_weprot_spare_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_weprot_spare_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_weprot_spare_1`]
module"]
#[doc(alias = "CCFG_WEPROT_SPARE_1")]
pub type CcfgWeprotSpare1 = crate::Reg<ccfg_weprot_spare_1::CcfgWeprotSpare1Spec>;
#[doc = "Spare register for WriteErase configuration"]
pub mod ccfg_weprot_spare_1;
#[doc = "CCFG_WEPROT_SPARE_2 (rw) register accessor: Spare register for WriteErase configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_weprot_spare_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_weprot_spare_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_weprot_spare_2`]
module"]
#[doc(alias = "CCFG_WEPROT_SPARE_2")]
pub type CcfgWeprotSpare2 = crate::Reg<ccfg_weprot_spare_2::CcfgWeprotSpare2Spec>;
#[doc = "Spare register for WriteErase configuration"]
pub mod ccfg_weprot_spare_2;
#[doc = "CCFG_WEPROT_SPARE_3 (rw) register accessor: Spare register for WriteErase configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_weprot_spare_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_weprot_spare_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_weprot_spare_3`]
module"]
#[doc(alias = "CCFG_WEPROT_SPARE_3")]
pub type CcfgWeprotSpare3 = crate::Reg<ccfg_weprot_spare_3::CcfgWeprotSpare3Spec>;
#[doc = "Spare register for WriteErase configuration"]
pub mod ccfg_weprot_spare_3;
#[doc = "TRUSTZONE_FLASH_CFG (rw) register accessor: Trustzone configuration register for flash\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trustzone_flash_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trustzone_flash_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trustzone_flash_cfg`]
module"]
#[doc(alias = "TRUSTZONE_FLASH_CFG")]
pub type TrustzoneFlashCfg = crate::Reg<trustzone_flash_cfg::TrustzoneFlashCfgSpec>;
#[doc = "Trustzone configuration register for flash"]
pub mod trustzone_flash_cfg;
#[doc = "TRUSTZONE_SRAM_CFG (rw) register accessor: Trustzone configuration register for MCU SRAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trustzone_sram_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trustzone_sram_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trustzone_sram_cfg`]
module"]
#[doc(alias = "TRUSTZONE_SRAM_CFG")]
pub type TrustzoneSramCfg = crate::Reg<trustzone_sram_cfg::TrustzoneSramCfgSpec>;
#[doc = "Trustzone configuration register for MCU SRAM"]
pub mod trustzone_sram_cfg;
#[doc = "SRAM_CFG (rw) register accessor: Configuration register for MCU SRAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_cfg`]
module"]
#[doc(alias = "SRAM_CFG")]
pub type SramCfg = crate::Reg<sram_cfg::SramCfgSpec>;
#[doc = "Configuration register for MCU SRAM"]
pub mod sram_cfg;
#[doc = "CPU_LOCK_CFG (rw) register accessor: Configuration register for MCU CPU lock options\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_lock_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_lock_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_lock_cfg`]
module"]
#[doc(alias = "CPU_LOCK_CFG")]
pub type CpuLockCfg = crate::Reg<cpu_lock_cfg::CpuLockCfgSpec>;
#[doc = "Configuration register for MCU CPU lock options"]
pub mod cpu_lock_cfg;
#[doc = "DEB_AUTH_CFG (rw) register accessor: Configuration register for debug authentication\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deb_auth_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deb_auth_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deb_auth_cfg`]
module"]
#[doc(alias = "DEB_AUTH_CFG")]
pub type DebAuthCfg = crate::Reg<deb_auth_cfg::DebAuthCfgSpec>;
#[doc = "Configuration register for debug authentication"]
pub mod deb_auth_cfg;
#[doc = "CKEY0 (rw) register accessor: Customer key\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckey0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckey0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckey0`]
module"]
#[doc(alias = "CKEY0")]
pub type Ckey0 = crate::Reg<ckey0::Ckey0Spec>;
#[doc = "Customer key"]
pub mod ckey0;
#[doc = "CKEY1 (rw) register accessor: Customer key\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckey1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckey1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckey1`]
module"]
#[doc(alias = "CKEY1")]
pub type Ckey1 = crate::Reg<ckey1::Ckey1Spec>;
#[doc = "Customer key"]
pub mod ckey1;
#[doc = "CKEY2 (rw) register accessor: Customer key\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckey2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckey2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckey2`]
module"]
#[doc(alias = "CKEY2")]
pub type Ckey2 = crate::Reg<ckey2::Ckey2Spec>;
#[doc = "Customer key"]
pub mod ckey2;
#[doc = "CKEY3 (rw) register accessor: Customer key\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckey3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckey3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckey3`]
module"]
#[doc(alias = "CKEY3")]
pub type Ckey3 = crate::Reg<ckey3::Ckey3Spec>;
#[doc = "Customer key"]
pub mod ckey3;

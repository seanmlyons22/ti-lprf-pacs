#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reserved_0: Reserved0,
    _reserved1: [u8; 0x1fa4],
    ext_lf_clk: ExtLfClk,
    mode_conf_1: ModeConf1,
    size_and_dis_flags: SizeAndDisFlags,
    mode_conf: ModeConf,
    volt_load_0: VoltLoad0,
    volt_load_1: VoltLoad1,
    rtc_offset: RtcOffset,
    freq_offset: FreqOffset,
    ieee_mac_0: IeeeMac0,
    ieee_mac_1: IeeeMac1,
    ieee_ble_0: IeeeBle0,
    ieee_ble_1: IeeeBle1,
    bl_config: BlConfig,
    erase_conf: EraseConf,
    ccfg_ti_options: CcfgTiOptions,
    ccfg_tap_dap_0: CcfgTapDap0,
    ccfg_tap_dap_1: CcfgTapDap1,
    image_valid_conf: ImageValidConf,
    ccfg_prot_31_0: CcfgProt31_0,
    ccfg_prot_63_32: CcfgProt63_32,
    ccfg_prot_95_64: CcfgProt95_64,
    ccfg_prot_127_96: CcfgProt127_96,
}
impl RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn reserved_0(&self) -> &Reserved0 {
        &self.reserved_0
    }
    #[doc = "0x1fa8 - Extern LF clock configuration"]
    #[inline(always)]
    pub const fn ext_lf_clk(&self) -> &ExtLfClk {
        &self.ext_lf_clk
    }
    #[doc = "0x1fac - Mode Configuration 1"]
    #[inline(always)]
    pub const fn mode_conf_1(&self) -> &ModeConf1 {
        &self.mode_conf_1
    }
    #[doc = "0x1fb0 - CCFG Size and Disable Flags"]
    #[inline(always)]
    pub const fn size_and_dis_flags(&self) -> &SizeAndDisFlags {
        &self.size_and_dis_flags
    }
    #[doc = "0x1fb4 - Mode Configuration 0"]
    #[inline(always)]
    pub const fn mode_conf(&self) -> &ModeConf {
        &self.mode_conf
    }
    #[doc = "0x1fb8 - Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    #[inline(always)]
    pub const fn volt_load_0(&self) -> &VoltLoad0 {
        &self.volt_load_0
    }
    #[doc = "0x1fbc - Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    #[inline(always)]
    pub const fn volt_load_1(&self) -> &VoltLoad1 {
        &self.volt_load_1
    }
    #[doc = "0x1fc0 - Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP."]
    #[inline(always)]
    pub const fn rtc_offset(&self) -> &RtcOffset {
        &self.rtc_offset
    }
    #[doc = "0x1fc4 - Frequency Offset"]
    #[inline(always)]
    pub const fn freq_offset(&self) -> &FreqOffset {
        &self.freq_offset
    }
    #[doc = "0x1fc8 - IEEE MAC Address 0"]
    #[inline(always)]
    pub const fn ieee_mac_0(&self) -> &IeeeMac0 {
        &self.ieee_mac_0
    }
    #[doc = "0x1fcc - IEEE MAC Address 1"]
    #[inline(always)]
    pub const fn ieee_mac_1(&self) -> &IeeeMac1 {
        &self.ieee_mac_1
    }
    #[doc = "0x1fd0 - IEEE BLE Address 0"]
    #[inline(always)]
    pub const fn ieee_ble_0(&self) -> &IeeeBle0 {
        &self.ieee_ble_0
    }
    #[doc = "0x1fd4 - IEEE BLE Address 1"]
    #[inline(always)]
    pub const fn ieee_ble_1(&self) -> &IeeeBle1 {
        &self.ieee_ble_1
    }
    #[doc = "0x1fd8 - Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
    #[inline(always)]
    pub const fn bl_config(&self) -> &BlConfig {
        &self.bl_config
    }
    #[doc = "0x1fdc - Erase Configuration"]
    #[inline(always)]
    pub const fn erase_conf(&self) -> &EraseConf {
        &self.erase_conf
    }
    #[doc = "0x1fe0 - TI Options"]
    #[inline(always)]
    pub const fn ccfg_ti_options(&self) -> &CcfgTiOptions {
        &self.ccfg_ti_options
    }
    #[doc = "0x1fe4 - Test Access Points Enable 0"]
    #[inline(always)]
    pub const fn ccfg_tap_dap_0(&self) -> &CcfgTapDap0 {
        &self.ccfg_tap_dap_0
    }
    #[doc = "0x1fe8 - Test Access Points Enable 1"]
    #[inline(always)]
    pub const fn ccfg_tap_dap_1(&self) -> &CcfgTapDap1 {
        &self.ccfg_tap_dap_1
    }
    #[doc = "0x1fec - Image Valid"]
    #[inline(always)]
    pub const fn image_valid_conf(&self) -> &ImageValidConf {
        &self.image_valid_conf
    }
    #[doc = "0x1ff0 - Protect Sectors 0-31 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
    #[inline(always)]
    pub const fn ccfg_prot_31_0(&self) -> &CcfgProt31_0 {
        &self.ccfg_prot_31_0
    }
    #[doc = "0x1ff4 - Protect Sectors 32-63 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
    #[inline(always)]
    pub const fn ccfg_prot_63_32(&self) -> &CcfgProt63_32 {
        &self.ccfg_prot_63_32
    }
    #[doc = "0x1ff8 - Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
    #[inline(always)]
    pub const fn ccfg_prot_95_64(&self) -> &CcfgProt95_64 {
        &self.ccfg_prot_95_64
    }
    #[doc = "0x1ffc - Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
    #[inline(always)]
    pub const fn ccfg_prot_127_96(&self) -> &CcfgProt127_96 {
        &self.ccfg_prot_127_96
    }
}
#[doc = "RESERVED_0 (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved_0`]
module"]
#[doc(alias = "RESERVED_0")]
pub type Reserved0 = crate::Reg<reserved_0::Reserved0Spec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_0;
#[doc = "EXT_LF_CLK (rw) register accessor: Extern LF clock configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_lf_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_lf_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_lf_clk`]
module"]
#[doc(alias = "EXT_LF_CLK")]
pub type ExtLfClk = crate::Reg<ext_lf_clk::ExtLfClkSpec>;
#[doc = "Extern LF clock configuration"]
pub mod ext_lf_clk;
#[doc = "MODE_CONF_1 (rw) register accessor: Mode Configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_conf_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_conf_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode_conf_1`]
module"]
#[doc(alias = "MODE_CONF_1")]
pub type ModeConf1 = crate::Reg<mode_conf_1::ModeConf1Spec>;
#[doc = "Mode Configuration 1"]
pub mod mode_conf_1;
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
#[doc = "RTC_OFFSET (rw) register accessor: Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_offset`]
module"]
#[doc(alias = "RTC_OFFSET")]
pub type RtcOffset = crate::Reg<rtc_offset::RtcOffsetSpec>;
#[doc = "Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP."]
pub mod rtc_offset;
#[doc = "FREQ_OFFSET (rw) register accessor: Frequency Offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freq_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freq_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freq_offset`]
module"]
#[doc(alias = "FREQ_OFFSET")]
pub type FreqOffset = crate::Reg<freq_offset::FreqOffsetSpec>;
#[doc = "Frequency Offset"]
pub mod freq_offset;
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
#[doc = "CCFG_PROT_31_0 (rw) register accessor: Protect Sectors 0-31 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_prot_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_prot_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_prot_31_0`]
module"]
#[doc(alias = "CCFG_PROT_31_0")]
pub type CcfgProt31_0 = crate::Reg<ccfg_prot_31_0::CcfgProt31_0Spec>;
#[doc = "Protect Sectors 0-31 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
pub mod ccfg_prot_31_0;
#[doc = "CCFG_PROT_63_32 (rw) register accessor: Protect Sectors 32-63 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_prot_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_prot_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_prot_63_32`]
module"]
#[doc(alias = "CCFG_PROT_63_32")]
pub type CcfgProt63_32 = crate::Reg<ccfg_prot_63_32::CcfgProt63_32Spec>;
#[doc = "Protect Sectors 32-63 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
pub mod ccfg_prot_63_32;
#[doc = "CCFG_PROT_95_64 (rw) register accessor: Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_prot_95_64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_prot_95_64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_prot_95_64`]
module"]
#[doc(alias = "CCFG_PROT_95_64")]
pub type CcfgProt95_64 = crate::Reg<ccfg_prot_95_64::CcfgProt95_64Spec>;
#[doc = "Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
pub mod ccfg_prot_95_64;
#[doc = "CCFG_PROT_127_96 (rw) register accessor: Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_prot_127_96::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_prot_127_96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_prot_127_96`]
module"]
#[doc(alias = "CCFG_PROT_127_96")]
pub type CcfgProt127_96 = crate::Reg<ccfg_prot_127_96::CcfgProt127_96Spec>;
#[doc = "Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
pub mod ccfg_prot_127_96;

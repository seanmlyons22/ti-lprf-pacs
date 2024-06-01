#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0xa0],
    misc_conf_1: MiscConf1,
    misc_conf_2: MiscConf2,
    _reserved2: [u8; 0x08],
    hposc_meas_5: HposcMeas5,
    hposc_meas_4: HposcMeas4,
    hposc_meas_3: HposcMeas3,
    hposc_meas_2: HposcMeas2,
    hposc_meas_1: HposcMeas1,
    config_cc26_fe: ConfigCc26Fe,
    config_cc13_fe: ConfigCc13Fe,
    config_rf_common: ConfigRfCommon,
    config_synth_div2_cc26_2g4: ConfigSynthDiv2Cc26_2g4,
    config_synth_div2_cc13_2g4: ConfigSynthDiv2Cc13_2g4,
    config_synth_div2_cc26_1g: ConfigSynthDiv2Cc26_1g,
    config_synth_div2_cc13_1g: ConfigSynthDiv2Cc13_1g,
    config_synth_div4_cc26: ConfigSynthDiv4Cc26,
    config_synth_div4_cc13: ConfigSynthDiv4Cc13,
    config_synth_div5: ConfigSynthDiv5,
    config_synth_div6_cc26: ConfigSynthDiv6Cc26,
    config_synth_div6_cc13: ConfigSynthDiv6Cc13,
    config_synth_div10: ConfigSynthDiv10,
    config_synth_div12_cc26: ConfigSynthDiv12Cc26,
    config_synth_div12_cc13: ConfigSynthDiv12Cc13,
    config_synth_div15: ConfigSynthDiv15,
    config_synth_div30: ConfigSynthDiv30,
    _reserved24: [u8; 0x5c],
    flash_number: FlashNumber,
    _reserved25: [u8; 0x04],
    flash_coordinate: FlashCoordinate,
    flash_e_p: FlashEP,
    flash_c_e_p_r: FlashCEPR,
    flash_p_r_pv: FlashPRPv,
    flash_eh_seq: FlashEhSeq,
    flash_vhv_e: FlashVhvE,
    flash_pp: FlashPp,
    flash_prog_ep: FlashProgEp,
    flash_era_pw: FlashEraPw,
    flash_vhv: FlashVhv,
    flash_vhv_pv: FlashVhvPv,
    flash_v: FlashV,
    _reserved37: [u8; 0xf8],
    user_id: UserId,
    _reserved38: [u8; 0x18],
    flash_otp_data3: FlashOtpData3,
    ana2_trim: Ana2Trim,
    ldo_trim: LdoTrim,
    _reserved41: [u8; 0x2c],
    mac_ble_0: MacBle0,
    mac_ble_1: MacBle1,
    mac_15_4_0: Mac15_4_0,
    mac_15_4_1: Mac15_4_1,
    _reserved45: [u8; 0x10],
    flash_otp_data4: FlashOtpData4,
    misc_trim: MiscTrim,
    rcosc_hf_tempcomp: RcoscHfTempcomp,
    _reserved48: [u8; 0x04],
    icepick_device_id: IcepickDeviceId,
    fcfg1_revision: Fcfg1Revision,
    misc_otp_data: MiscOtpData,
    _reserved51: [u8; 0x20],
    ioconf: Ioconf,
    _reserved52: [u8; 0x04],
    config_if_adc: ConfigIfAdc,
    config_osc_top: ConfigOscTop,
    _reserved54: [u8; 0x08],
    soc_adc_abs_gain: SocAdcAbsGain,
    soc_adc_rel_gain: SocAdcRelGain,
    _reserved56: [u8; 0x04],
    soc_adc_offset_int: SocAdcOffsetInt,
    soc_adc_ref_trim_and_offset_ext: SocAdcRefTrimAndOffsetExt,
    ampcomp_th1: AmpcompTh1,
    ampcomp_th2: AmpcompTh2,
    ampcomp_ctrl1: AmpcompCtrl1,
    anabypass_value2: AnabypassValue2,
    _reserved62: [u8; 0x08],
    volt_trim: VoltTrim,
    osc_conf: OscConf,
    freq_offset: FreqOffset,
    _reserved65: [u8; 0x04],
    misc_otp_data_1: MiscOtpData1,
    _reserved66: [u8; 0x34],
    shdw_die_id_0: ShdwDieId0,
    shdw_die_id_1: ShdwDieId1,
    shdw_die_id_2: ShdwDieId2,
    shdw_die_id_3: ShdwDieId3,
    _reserved70: [u8; 0x18],
    shdw_osc_bias_ldo_trim: ShdwOscBiasLdoTrim,
    shdw_ana_trim: ShdwAnaTrim,
    _reserved72: [u8; 0x08],
    osc_conf1: OscConf1,
    dac_bias_cnf: DacBiasCnf,
    _reserved74: [u8; 0x08],
    tfw_probe: TfwProbe,
    tfw_ft: TfwFt,
    dac_cal0: DacCal0,
    dac_cal1: DacCal1,
    dac_cal2: DacCal2,
    dac_cal3: DacCal3,
    _reserved80: [u8; 0x08],
    reserved_n: ReservedN,
}
impl RegisterBlock {
    #[doc = "0xa0 - Misc configurations"]
    #[inline(always)]
    pub const fn misc_conf_1(&self) -> &MiscConf1 {
        &self.misc_conf_1
    }
    #[doc = "0xa4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn misc_conf_2(&self) -> &MiscConf2 {
        &self.misc_conf_2
    }
    #[doc = "0xb0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn hposc_meas_5(&self) -> &HposcMeas5 {
        &self.hposc_meas_5
    }
    #[doc = "0xb4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn hposc_meas_4(&self) -> &HposcMeas4 {
        &self.hposc_meas_4
    }
    #[doc = "0xb8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn hposc_meas_3(&self) -> &HposcMeas3 {
        &self.hposc_meas_3
    }
    #[doc = "0xbc - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn hposc_meas_2(&self) -> &HposcMeas2 {
        &self.hposc_meas_2
    }
    #[doc = "0xc0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn hposc_meas_1(&self) -> &HposcMeas1 {
        &self.hposc_meas_1
    }
    #[doc = "0xc4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_cc26_fe(&self) -> &ConfigCc26Fe {
        &self.config_cc26_fe
    }
    #[doc = "0xc8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_cc13_fe(&self) -> &ConfigCc13Fe {
        &self.config_cc13_fe
    }
    #[doc = "0xcc - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_rf_common(&self) -> &ConfigRfCommon {
        &self.config_rf_common
    }
    #[doc = "0xd0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div2_cc26_2g4(&self) -> &ConfigSynthDiv2Cc26_2g4 {
        &self.config_synth_div2_cc26_2g4
    }
    #[doc = "0xd4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div2_cc13_2g4(&self) -> &ConfigSynthDiv2Cc13_2g4 {
        &self.config_synth_div2_cc13_2g4
    }
    #[doc = "0xd8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div2_cc26_1g(&self) -> &ConfigSynthDiv2Cc26_1g {
        &self.config_synth_div2_cc26_1g
    }
    #[doc = "0xdc - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div2_cc13_1g(&self) -> &ConfigSynthDiv2Cc13_1g {
        &self.config_synth_div2_cc13_1g
    }
    #[doc = "0xe0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div4_cc26(&self) -> &ConfigSynthDiv4Cc26 {
        &self.config_synth_div4_cc26
    }
    #[doc = "0xe4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div4_cc13(&self) -> &ConfigSynthDiv4Cc13 {
        &self.config_synth_div4_cc13
    }
    #[doc = "0xe8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div5(&self) -> &ConfigSynthDiv5 {
        &self.config_synth_div5
    }
    #[doc = "0xec - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div6_cc26(&self) -> &ConfigSynthDiv6Cc26 {
        &self.config_synth_div6_cc26
    }
    #[doc = "0xf0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div6_cc13(&self) -> &ConfigSynthDiv6Cc13 {
        &self.config_synth_div6_cc13
    }
    #[doc = "0xf4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div10(&self) -> &ConfigSynthDiv10 {
        &self.config_synth_div10
    }
    #[doc = "0xf8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div12_cc26(&self) -> &ConfigSynthDiv12Cc26 {
        &self.config_synth_div12_cc26
    }
    #[doc = "0xfc - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div12_cc13(&self) -> &ConfigSynthDiv12Cc13 {
        &self.config_synth_div12_cc13
    }
    #[doc = "0x100 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div15(&self) -> &ConfigSynthDiv15 {
        &self.config_synth_div15
    }
    #[doc = "0x104 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_synth_div30(&self) -> &ConfigSynthDiv30 {
        &self.config_synth_div30
    }
    #[doc = "0x164 - Flash information"]
    #[inline(always)]
    pub const fn flash_number(&self) -> &FlashNumber {
        &self.flash_number
    }
    #[doc = "0x16c - Flash information"]
    #[inline(always)]
    pub const fn flash_coordinate(&self) -> &FlashCoordinate {
        &self.flash_coordinate
    }
    #[doc = "0x170 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_e_p(&self) -> &FlashEP {
        &self.flash_e_p
    }
    #[doc = "0x174 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_c_e_p_r(&self) -> &FlashCEPR {
        &self.flash_c_e_p_r
    }
    #[doc = "0x178 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_p_r_pv(&self) -> &FlashPRPv {
        &self.flash_p_r_pv
    }
    #[doc = "0x17c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_eh_seq(&self) -> &FlashEhSeq {
        &self.flash_eh_seq
    }
    #[doc = "0x180 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_vhv_e(&self) -> &FlashVhvE {
        &self.flash_vhv_e
    }
    #[doc = "0x184 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_pp(&self) -> &FlashPp {
        &self.flash_pp
    }
    #[doc = "0x188 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_prog_ep(&self) -> &FlashProgEp {
        &self.flash_prog_ep
    }
    #[doc = "0x18c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_era_pw(&self) -> &FlashEraPw {
        &self.flash_era_pw
    }
    #[doc = "0x190 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_vhv(&self) -> &FlashVhv {
        &self.flash_vhv
    }
    #[doc = "0x194 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_vhv_pv(&self) -> &FlashVhvPv {
        &self.flash_vhv_pv
    }
    #[doc = "0x198 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_v(&self) -> &FlashV {
        &self.flash_v
    }
    #[doc = "0x294 - User Identification. Reading this register and the FCFG1:ICEPICK_DEVICE_ID register is the only supported way of identifying a device. The value of this register will be written to AON_PMCTL:JTAGUSERCODE by boot FW while in safezone."]
    #[inline(always)]
    pub const fn user_id(&self) -> &UserId {
        &self.user_id
    }
    #[doc = "0x2b0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_otp_data3(&self) -> &FlashOtpData3 {
        &self.flash_otp_data3
    }
    #[doc = "0x2b4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ana2_trim(&self) -> &Ana2Trim {
        &self.ana2_trim
    }
    #[doc = "0x2b8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ldo_trim(&self) -> &LdoTrim {
        &self.ldo_trim
    }
    #[doc = "0x2e8 - MAC BLE Address 0"]
    #[inline(always)]
    pub const fn mac_ble_0(&self) -> &MacBle0 {
        &self.mac_ble_0
    }
    #[doc = "0x2ec - MAC BLE Address 1"]
    #[inline(always)]
    pub const fn mac_ble_1(&self) -> &MacBle1 {
        &self.mac_ble_1
    }
    #[doc = "0x2f0 - MAC IEEE 802.15.4 Address 0"]
    #[inline(always)]
    pub const fn mac_15_4_0(&self) -> &Mac15_4_0 {
        &self.mac_15_4_0
    }
    #[doc = "0x2f4 - MAC IEEE 802.15.4 Address 1"]
    #[inline(always)]
    pub const fn mac_15_4_1(&self) -> &Mac15_4_1 {
        &self.mac_15_4_1
    }
    #[doc = "0x308 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_otp_data4(&self) -> &FlashOtpData4 {
        &self.flash_otp_data4
    }
    #[doc = "0x30c - Miscellaneous Trim Parameters"]
    #[inline(always)]
    pub const fn misc_trim(&self) -> &MiscTrim {
        &self.misc_trim
    }
    #[doc = "0x310 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn rcosc_hf_tempcomp(&self) -> &RcoscHfTempcomp {
        &self.rcosc_hf_tempcomp
    }
    #[doc = "0x318 - IcePick Device Identification Reading this register and the FCFG1:USER_ID register is the only supported way of identifying a device."]
    #[inline(always)]
    pub const fn icepick_device_id(&self) -> &IcepickDeviceId {
        &self.icepick_device_id
    }
    #[doc = "0x31c - Factory Configuration (FCFG1) Revision"]
    #[inline(always)]
    pub const fn fcfg1_revision(&self) -> &Fcfg1Revision {
        &self.fcfg1_revision
    }
    #[doc = "0x320 - Misc OTP Data"]
    #[inline(always)]
    pub const fn misc_otp_data(&self) -> &MiscOtpData {
        &self.misc_otp_data
    }
    #[doc = "0x344 - IO Configuration"]
    #[inline(always)]
    pub const fn ioconf(&self) -> &Ioconf {
        &self.ioconf
    }
    #[doc = "0x34c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_if_adc(&self) -> &ConfigIfAdc {
        &self.config_if_adc
    }
    #[doc = "0x350 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn config_osc_top(&self) -> &ConfigOscTop {
        &self.config_osc_top
    }
    #[doc = "0x35c - AUX_ADC Gain in Absolute Reference Mode"]
    #[inline(always)]
    pub const fn soc_adc_abs_gain(&self) -> &SocAdcAbsGain {
        &self.soc_adc_abs_gain
    }
    #[doc = "0x360 - AUX_ADC Gain in Relative Reference Mode"]
    #[inline(always)]
    pub const fn soc_adc_rel_gain(&self) -> &SocAdcRelGain {
        &self.soc_adc_rel_gain
    }
    #[doc = "0x368 - AUX_ADC Temperature Offsets in Absolute Reference Mode"]
    #[inline(always)]
    pub const fn soc_adc_offset_int(&self) -> &SocAdcOffsetInt {
        &self.soc_adc_offset_int
    }
    #[doc = "0x36c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn soc_adc_ref_trim_and_offset_ext(&self) -> &SocAdcRefTrimAndOffsetExt {
        &self.soc_adc_ref_trim_and_offset_ext
    }
    #[doc = "0x370 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ampcomp_th1(&self) -> &AmpcompTh1 {
        &self.ampcomp_th1
    }
    #[doc = "0x374 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ampcomp_th2(&self) -> &AmpcompTh2 {
        &self.ampcomp_th2
    }
    #[doc = "0x378 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ampcomp_ctrl1(&self) -> &AmpcompCtrl1 {
        &self.ampcomp_ctrl1
    }
    #[doc = "0x37c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn anabypass_value2(&self) -> &AnabypassValue2 {
        &self.anabypass_value2
    }
    #[doc = "0x388 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn volt_trim(&self) -> &VoltTrim {
        &self.volt_trim
    }
    #[doc = "0x38c - OSC Configuration"]
    #[inline(always)]
    pub const fn osc_conf(&self) -> &OscConf {
        &self.osc_conf
    }
    #[doc = "0x390 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn freq_offset(&self) -> &FreqOffset {
        &self.freq_offset
    }
    #[doc = "0x398 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn misc_otp_data_1(&self) -> &MiscOtpData1 {
        &self.misc_otp_data_1
    }
    #[doc = "0x3d0 - Shadow of DIE_ID_0 register in eFuse"]
    #[inline(always)]
    pub const fn shdw_die_id_0(&self) -> &ShdwDieId0 {
        &self.shdw_die_id_0
    }
    #[doc = "0x3d4 - Shadow of DIE_ID_1 register in eFuse"]
    #[inline(always)]
    pub const fn shdw_die_id_1(&self) -> &ShdwDieId1 {
        &self.shdw_die_id_1
    }
    #[doc = "0x3d8 - Shadow of DIE_ID_2 register in eFuse"]
    #[inline(always)]
    pub const fn shdw_die_id_2(&self) -> &ShdwDieId2 {
        &self.shdw_die_id_2
    }
    #[doc = "0x3dc - Shadow of DIE_ID_3 register in eFuse"]
    #[inline(always)]
    pub const fn shdw_die_id_3(&self) -> &ShdwDieId3 {
        &self.shdw_die_id_3
    }
    #[doc = "0x3f8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn shdw_osc_bias_ldo_trim(&self) -> &ShdwOscBiasLdoTrim {
        &self.shdw_osc_bias_ldo_trim
    }
    #[doc = "0x3fc - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn shdw_ana_trim(&self) -> &ShdwAnaTrim {
        &self.shdw_ana_trim
    }
    #[doc = "0x408 - Oscillator configuration"]
    #[inline(always)]
    pub const fn osc_conf1(&self) -> &OscConf1 {
        &self.osc_conf1
    }
    #[doc = "0x40c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn dac_bias_cnf(&self) -> &DacBiasCnf {
        &self.dac_bias_cnf
    }
    #[doc = "0x418 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn tfw_probe(&self) -> &TfwProbe {
        &self.tfw_probe
    }
    #[doc = "0x41c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn tfw_ft(&self) -> &TfwFt {
        &self.tfw_ft
    }
    #[doc = "0x420 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn dac_cal0(&self) -> &DacCal0 {
        &self.dac_cal0
    }
    #[doc = "0x424 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn dac_cal1(&self) -> &DacCal1 {
        &self.dac_cal1
    }
    #[doc = "0x428 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn dac_cal2(&self) -> &DacCal2 {
        &self.dac_cal2
    }
    #[doc = "0x42c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn dac_cal3(&self) -> &DacCal3 {
        &self.dac_cal3
    }
    #[doc = "0x438 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn reserved_n(&self) -> &ReservedN {
        &self.reserved_n
    }
}
#[doc = "MISC_CONF_1 (rw) register accessor: Misc configurations\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_conf_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf_1`]
module"]
#[doc(alias = "MISC_CONF_1")]
pub type MiscConf1 = crate::Reg<misc_conf_1::MiscConf1Spec>;
#[doc = "Misc configurations"]
pub mod misc_conf_1;
#[doc = "MISC_CONF_2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_conf_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf_2`]
module"]
#[doc(alias = "MISC_CONF_2")]
pub type MiscConf2 = crate::Reg<misc_conf_2::MiscConf2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod misc_conf_2;
#[doc = "HPOSC_MEAS_5 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hposc_meas_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hposc_meas_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hposc_meas_5`]
module"]
#[doc(alias = "HPOSC_MEAS_5")]
pub type HposcMeas5 = crate::Reg<hposc_meas_5::HposcMeas5Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod hposc_meas_5;
#[doc = "HPOSC_MEAS_4 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hposc_meas_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hposc_meas_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hposc_meas_4`]
module"]
#[doc(alias = "HPOSC_MEAS_4")]
pub type HposcMeas4 = crate::Reg<hposc_meas_4::HposcMeas4Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod hposc_meas_4;
#[doc = "HPOSC_MEAS_3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hposc_meas_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hposc_meas_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hposc_meas_3`]
module"]
#[doc(alias = "HPOSC_MEAS_3")]
pub type HposcMeas3 = crate::Reg<hposc_meas_3::HposcMeas3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod hposc_meas_3;
#[doc = "HPOSC_MEAS_2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hposc_meas_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hposc_meas_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hposc_meas_2`]
module"]
#[doc(alias = "HPOSC_MEAS_2")]
pub type HposcMeas2 = crate::Reg<hposc_meas_2::HposcMeas2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod hposc_meas_2;
#[doc = "HPOSC_MEAS_1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hposc_meas_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hposc_meas_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hposc_meas_1`]
module"]
#[doc(alias = "HPOSC_MEAS_1")]
pub type HposcMeas1 = crate::Reg<hposc_meas_1::HposcMeas1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod hposc_meas_1;
#[doc = "CONFIG_CC26_FE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_cc26_fe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_cc26_fe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_cc26_fe`]
module"]
#[doc(alias = "CONFIG_CC26_FE")]
pub type ConfigCc26Fe = crate::Reg<config_cc26_fe::ConfigCc26FeSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_cc26_fe;
#[doc = "CONFIG_CC13_FE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_cc13_fe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_cc13_fe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_cc13_fe`]
module"]
#[doc(alias = "CONFIG_CC13_FE")]
pub type ConfigCc13Fe = crate::Reg<config_cc13_fe::ConfigCc13FeSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_cc13_fe;
#[doc = "CONFIG_RF_COMMON (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_rf_common::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_rf_common::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_rf_common`]
module"]
#[doc(alias = "CONFIG_RF_COMMON")]
pub type ConfigRfCommon = crate::Reg<config_rf_common::ConfigRfCommonSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_common;
#[doc = "CONFIG_SYNTH_DIV2_CC26_2G4 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div2_cc26_2g4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div2_cc26_2g4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div2_cc26_2g4`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV2_CC26_2G4")]
pub type ConfigSynthDiv2Cc26_2g4 =
    crate::Reg<config_synth_div2_cc26_2g4::ConfigSynthDiv2Cc26_2g4Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc26_2g4;
#[doc = "CONFIG_SYNTH_DIV2_CC13_2G4 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div2_cc13_2g4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div2_cc13_2g4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div2_cc13_2g4`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV2_CC13_2G4")]
pub type ConfigSynthDiv2Cc13_2g4 =
    crate::Reg<config_synth_div2_cc13_2g4::ConfigSynthDiv2Cc13_2g4Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc13_2g4;
#[doc = "CONFIG_SYNTH_DIV2_CC26_1G (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div2_cc26_1g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div2_cc26_1g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div2_cc26_1g`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV2_CC26_1G")]
pub type ConfigSynthDiv2Cc26_1g = crate::Reg<config_synth_div2_cc26_1g::ConfigSynthDiv2Cc26_1gSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc26_1g;
#[doc = "CONFIG_SYNTH_DIV2_CC13_1G (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div2_cc13_1g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div2_cc13_1g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div2_cc13_1g`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV2_CC13_1G")]
pub type ConfigSynthDiv2Cc13_1g = crate::Reg<config_synth_div2_cc13_1g::ConfigSynthDiv2Cc13_1gSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc13_1g;
#[doc = "CONFIG_SYNTH_DIV4_CC26 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div4_cc26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div4_cc26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div4_cc26`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV4_CC26")]
pub type ConfigSynthDiv4Cc26 = crate::Reg<config_synth_div4_cc26::ConfigSynthDiv4Cc26Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div4_cc26;
#[doc = "CONFIG_SYNTH_DIV4_CC13 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div4_cc13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div4_cc13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div4_cc13`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV4_CC13")]
pub type ConfigSynthDiv4Cc13 = crate::Reg<config_synth_div4_cc13::ConfigSynthDiv4Cc13Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div4_cc13;
#[doc = "CONFIG_SYNTH_DIV5 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div5`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV5")]
pub type ConfigSynthDiv5 = crate::Reg<config_synth_div5::ConfigSynthDiv5Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div5;
#[doc = "CONFIG_SYNTH_DIV6_CC26 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div6_cc26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div6_cc26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div6_cc26`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV6_CC26")]
pub type ConfigSynthDiv6Cc26 = crate::Reg<config_synth_div6_cc26::ConfigSynthDiv6Cc26Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div6_cc26;
#[doc = "CONFIG_SYNTH_DIV6_CC13 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div6_cc13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div6_cc13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div6_cc13`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV6_CC13")]
pub type ConfigSynthDiv6Cc13 = crate::Reg<config_synth_div6_cc13::ConfigSynthDiv6Cc13Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div6_cc13;
#[doc = "CONFIG_SYNTH_DIV10 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div10`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV10")]
pub type ConfigSynthDiv10 = crate::Reg<config_synth_div10::ConfigSynthDiv10Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div10;
#[doc = "CONFIG_SYNTH_DIV12_CC26 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div12_cc26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div12_cc26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div12_cc26`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV12_CC26")]
pub type ConfigSynthDiv12Cc26 = crate::Reg<config_synth_div12_cc26::ConfigSynthDiv12Cc26Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div12_cc26;
#[doc = "CONFIG_SYNTH_DIV12_CC13 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div12_cc13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div12_cc13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div12_cc13`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV12_CC13")]
pub type ConfigSynthDiv12Cc13 = crate::Reg<config_synth_div12_cc13::ConfigSynthDiv12Cc13Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div12_cc13;
#[doc = "CONFIG_SYNTH_DIV15 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div15`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV15")]
pub type ConfigSynthDiv15 = crate::Reg<config_synth_div15::ConfigSynthDiv15Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div15;
#[doc = "CONFIG_SYNTH_DIV30 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_synth_div30`]
module"]
#[doc(alias = "CONFIG_SYNTH_DIV30")]
pub type ConfigSynthDiv30 = crate::Reg<config_synth_div30::ConfigSynthDiv30Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div30;
#[doc = "FLASH_NUMBER (rw) register accessor: Flash information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_number::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_number::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_number`]
module"]
#[doc(alias = "FLASH_NUMBER")]
pub type FlashNumber = crate::Reg<flash_number::FlashNumberSpec>;
#[doc = "Flash information"]
pub mod flash_number;
#[doc = "FLASH_COORDINATE (rw) register accessor: Flash information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_coordinate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_coordinate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_coordinate`]
module"]
#[doc(alias = "FLASH_COORDINATE")]
pub type FlashCoordinate = crate::Reg<flash_coordinate::FlashCoordinateSpec>;
#[doc = "Flash information"]
pub mod flash_coordinate;
#[doc = "FLASH_E_P (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_e_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_e_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_e_p`]
module"]
#[doc(alias = "FLASH_E_P")]
pub type FlashEP = crate::Reg<flash_e_p::FlashEPSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_e_p;
#[doc = "FLASH_C_E_P_R (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_c_e_p_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_c_e_p_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_c_e_p_r`]
module"]
#[doc(alias = "FLASH_C_E_P_R")]
pub type FlashCEPR = crate::Reg<flash_c_e_p_r::FlashCEPRSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_c_e_p_r;
#[doc = "FLASH_P_R_PV (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_p_r_pv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_p_r_pv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_p_r_pv`]
module"]
#[doc(alias = "FLASH_P_R_PV")]
pub type FlashPRPv = crate::Reg<flash_p_r_pv::FlashPRPvSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_p_r_pv;
#[doc = "FLASH_EH_SEQ (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_eh_seq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_eh_seq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_eh_seq`]
module"]
#[doc(alias = "FLASH_EH_SEQ")]
pub type FlashEhSeq = crate::Reg<flash_eh_seq::FlashEhSeqSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_eh_seq;
#[doc = "FLASH_VHV_E (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_vhv_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_vhv_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_vhv_e`]
module"]
#[doc(alias = "FLASH_VHV_E")]
pub type FlashVhvE = crate::Reg<flash_vhv_e::FlashVhvESpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv_e;
#[doc = "FLASH_PP (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_pp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_pp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_pp`]
module"]
#[doc(alias = "FLASH_PP")]
pub type FlashPp = crate::Reg<flash_pp::FlashPpSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_pp;
#[doc = "FLASH_PROG_EP (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_prog_ep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_prog_ep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prog_ep`]
module"]
#[doc(alias = "FLASH_PROG_EP")]
pub type FlashProgEp = crate::Reg<flash_prog_ep::FlashProgEpSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_prog_ep;
#[doc = "FLASH_ERA_PW (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_era_pw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_era_pw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_era_pw`]
module"]
#[doc(alias = "FLASH_ERA_PW")]
pub type FlashEraPw = crate::Reg<flash_era_pw::FlashEraPwSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_era_pw;
#[doc = "FLASH_VHV (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_vhv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_vhv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_vhv`]
module"]
#[doc(alias = "FLASH_VHV")]
pub type FlashVhv = crate::Reg<flash_vhv::FlashVhvSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv;
#[doc = "FLASH_VHV_PV (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_vhv_pv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_vhv_pv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_vhv_pv`]
module"]
#[doc(alias = "FLASH_VHV_PV")]
pub type FlashVhvPv = crate::Reg<flash_vhv_pv::FlashVhvPvSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv_pv;
#[doc = "FLASH_V (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_v`]
module"]
#[doc(alias = "FLASH_V")]
pub type FlashV = crate::Reg<flash_v::FlashVSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_v;
#[doc = "USER_ID (rw) register accessor: User Identification. Reading this register and the FCFG1:ICEPICK_DEVICE_ID register is the only supported way of identifying a device. The value of this register will be written to AON_PMCTL:JTAGUSERCODE by boot FW while in safezone.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_id`]
module"]
#[doc(alias = "USER_ID")]
pub type UserId = crate::Reg<user_id::UserIdSpec>;
#[doc = "User Identification. Reading this register and the FCFG1:ICEPICK_DEVICE_ID register is the only supported way of identifying a device. The value of this register will be written to AON_PMCTL:JTAGUSERCODE by boot FW while in safezone."]
pub mod user_id;
#[doc = "FLASH_OTP_DATA3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_otp_data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_otp_data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_otp_data3`]
module"]
#[doc(alias = "FLASH_OTP_DATA3")]
pub type FlashOtpData3 = crate::Reg<flash_otp_data3::FlashOtpData3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_otp_data3;
#[doc = "ANA2_TRIM (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana2_trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana2_trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana2_trim`]
module"]
#[doc(alias = "ANA2_TRIM")]
pub type Ana2Trim = crate::Reg<ana2_trim::Ana2TrimSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ana2_trim;
#[doc = "LDO_TRIM (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldo_trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldo_trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo_trim`]
module"]
#[doc(alias = "LDO_TRIM")]
pub type LdoTrim = crate::Reg<ldo_trim::LdoTrimSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ldo_trim;
#[doc = "MAC_BLE_0 (rw) register accessor: MAC BLE Address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_ble_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_ble_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_ble_0`]
module"]
#[doc(alias = "MAC_BLE_0")]
pub type MacBle0 = crate::Reg<mac_ble_0::MacBle0Spec>;
#[doc = "MAC BLE Address 0"]
pub mod mac_ble_0;
#[doc = "MAC_BLE_1 (rw) register accessor: MAC BLE Address 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_ble_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_ble_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_ble_1`]
module"]
#[doc(alias = "MAC_BLE_1")]
pub type MacBle1 = crate::Reg<mac_ble_1::MacBle1Spec>;
#[doc = "MAC BLE Address 1"]
pub mod mac_ble_1;
#[doc = "MAC_15_4_0 (rw) register accessor: MAC IEEE 802.15.4 Address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_15_4_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_15_4_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_15_4_0`]
module"]
#[doc(alias = "MAC_15_4_0")]
pub type Mac15_4_0 = crate::Reg<mac_15_4_0::Mac15_4_0Spec>;
#[doc = "MAC IEEE 802.15.4 Address 0"]
pub mod mac_15_4_0;
#[doc = "MAC_15_4_1 (rw) register accessor: MAC IEEE 802.15.4 Address 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_15_4_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_15_4_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_15_4_1`]
module"]
#[doc(alias = "MAC_15_4_1")]
pub type Mac15_4_1 = crate::Reg<mac_15_4_1::Mac15_4_1Spec>;
#[doc = "MAC IEEE 802.15.4 Address 1"]
pub mod mac_15_4_1;
#[doc = "FLASH_OTP_DATA4 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_otp_data4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_otp_data4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_otp_data4`]
module"]
#[doc(alias = "FLASH_OTP_DATA4")]
pub type FlashOtpData4 = crate::Reg<flash_otp_data4::FlashOtpData4Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_otp_data4;
#[doc = "MISC_TRIM (rw) register accessor: Miscellaneous Trim Parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_trim`]
module"]
#[doc(alias = "MISC_TRIM")]
pub type MiscTrim = crate::Reg<misc_trim::MiscTrimSpec>;
#[doc = "Miscellaneous Trim Parameters"]
pub mod misc_trim;
#[doc = "RCOSC_HF_TEMPCOMP (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcosc_hf_tempcomp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcosc_hf_tempcomp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcosc_hf_tempcomp`]
module"]
#[doc(alias = "RCOSC_HF_TEMPCOMP")]
pub type RcoscHfTempcomp = crate::Reg<rcosc_hf_tempcomp::RcoscHfTempcompSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod rcosc_hf_tempcomp;
#[doc = "ICEPICK_DEVICE_ID (rw) register accessor: IcePick Device Identification Reading this register and the FCFG1:USER_ID register is the only supported way of identifying a device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icepick_device_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icepick_device_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icepick_device_id`]
module"]
#[doc(alias = "ICEPICK_DEVICE_ID")]
pub type IcepickDeviceId = crate::Reg<icepick_device_id::IcepickDeviceIdSpec>;
#[doc = "IcePick Device Identification Reading this register and the FCFG1:USER_ID register is the only supported way of identifying a device."]
pub mod icepick_device_id;
#[doc = "FCFG1_REVISION (rw) register accessor: Factory Configuration (FCFG1) Revision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg1_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg1_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg1_revision`]
module"]
#[doc(alias = "FCFG1_REVISION")]
pub type Fcfg1Revision = crate::Reg<fcfg1_revision::Fcfg1RevisionSpec>;
#[doc = "Factory Configuration (FCFG1) Revision"]
pub mod fcfg1_revision;
#[doc = "MISC_OTP_DATA (rw) register accessor: Misc OTP Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_otp_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_otp_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_otp_data`]
module"]
#[doc(alias = "MISC_OTP_DATA")]
pub type MiscOtpData = crate::Reg<misc_otp_data::MiscOtpDataSpec>;
#[doc = "Misc OTP Data"]
pub mod misc_otp_data;
#[doc = "IOCONF (rw) register accessor: IO Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioconf`]
module"]
#[doc(alias = "IOCONF")]
pub type Ioconf = crate::Reg<ioconf::IoconfSpec>;
#[doc = "IO Configuration"]
pub mod ioconf;
#[doc = "CONFIG_IF_ADC (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_if_adc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_if_adc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_if_adc`]
module"]
#[doc(alias = "CONFIG_IF_ADC")]
pub type ConfigIfAdc = crate::Reg<config_if_adc::ConfigIfAdcSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_if_adc;
#[doc = "CONFIG_OSC_TOP (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_osc_top::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_osc_top::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_osc_top`]
module"]
#[doc(alias = "CONFIG_OSC_TOP")]
pub type ConfigOscTop = crate::Reg<config_osc_top::ConfigOscTopSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_osc_top;
#[doc = "SOC_ADC_ABS_GAIN (rw) register accessor: AUX_ADC Gain in Absolute Reference Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_adc_abs_gain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_adc_abs_gain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_adc_abs_gain`]
module"]
#[doc(alias = "SOC_ADC_ABS_GAIN")]
pub type SocAdcAbsGain = crate::Reg<soc_adc_abs_gain::SocAdcAbsGainSpec>;
#[doc = "AUX_ADC Gain in Absolute Reference Mode"]
pub mod soc_adc_abs_gain;
#[doc = "SOC_ADC_REL_GAIN (rw) register accessor: AUX_ADC Gain in Relative Reference Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_adc_rel_gain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_adc_rel_gain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_adc_rel_gain`]
module"]
#[doc(alias = "SOC_ADC_REL_GAIN")]
pub type SocAdcRelGain = crate::Reg<soc_adc_rel_gain::SocAdcRelGainSpec>;
#[doc = "AUX_ADC Gain in Relative Reference Mode"]
pub mod soc_adc_rel_gain;
#[doc = "SOC_ADC_OFFSET_INT (rw) register accessor: AUX_ADC Temperature Offsets in Absolute Reference Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_adc_offset_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_adc_offset_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_adc_offset_int`]
module"]
#[doc(alias = "SOC_ADC_OFFSET_INT")]
pub type SocAdcOffsetInt = crate::Reg<soc_adc_offset_int::SocAdcOffsetIntSpec>;
#[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode"]
pub mod soc_adc_offset_int;
#[doc = "SOC_ADC_REF_TRIM_AND_OFFSET_EXT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_adc_ref_trim_and_offset_ext::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_adc_ref_trim_and_offset_ext::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_adc_ref_trim_and_offset_ext`]
module"]
#[doc(alias = "SOC_ADC_REF_TRIM_AND_OFFSET_EXT")]
pub type SocAdcRefTrimAndOffsetExt =
    crate::Reg<soc_adc_ref_trim_and_offset_ext::SocAdcRefTrimAndOffsetExtSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod soc_adc_ref_trim_and_offset_ext;
#[doc = "AMPCOMP_TH1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcomp_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcomp_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampcomp_th1`]
module"]
#[doc(alias = "AMPCOMP_TH1")]
pub type AmpcompTh1 = crate::Reg<ampcomp_th1::AmpcompTh1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_th1;
#[doc = "AMPCOMP_TH2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcomp_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcomp_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampcomp_th2`]
module"]
#[doc(alias = "AMPCOMP_TH2")]
pub type AmpcompTh2 = crate::Reg<ampcomp_th2::AmpcompTh2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_th2;
#[doc = "AMPCOMP_CTRL1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcomp_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcomp_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampcomp_ctrl1`]
module"]
#[doc(alias = "AMPCOMP_CTRL1")]
pub type AmpcompCtrl1 = crate::Reg<ampcomp_ctrl1::AmpcompCtrl1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_ctrl1;
#[doc = "ANABYPASS_VALUE2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`anabypass_value2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`anabypass_value2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anabypass_value2`]
module"]
#[doc(alias = "ANABYPASS_VALUE2")]
pub type AnabypassValue2 = crate::Reg<anabypass_value2::AnabypassValue2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod anabypass_value2;
#[doc = "VOLT_TRIM (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`volt_trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`volt_trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@volt_trim`]
module"]
#[doc(alias = "VOLT_TRIM")]
pub type VoltTrim = crate::Reg<volt_trim::VoltTrimSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod volt_trim;
#[doc = "OSC_CONF (rw) register accessor: OSC Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc_conf`]
module"]
#[doc(alias = "OSC_CONF")]
pub type OscConf = crate::Reg<osc_conf::OscConfSpec>;
#[doc = "OSC Configuration"]
pub mod osc_conf;
#[doc = "FREQ_OFFSET (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freq_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freq_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freq_offset`]
module"]
#[doc(alias = "FREQ_OFFSET")]
pub type FreqOffset = crate::Reg<freq_offset::FreqOffsetSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod freq_offset;
#[doc = "MISC_OTP_DATA_1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_otp_data_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_otp_data_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_otp_data_1`]
module"]
#[doc(alias = "MISC_OTP_DATA_1")]
pub type MiscOtpData1 = crate::Reg<misc_otp_data_1::MiscOtpData1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod misc_otp_data_1;
#[doc = "SHDW_DIE_ID_0 (rw) register accessor: Shadow of DIE_ID_0 register in eFuse\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_die_id_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_die_id_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shdw_die_id_0`]
module"]
#[doc(alias = "SHDW_DIE_ID_0")]
pub type ShdwDieId0 = crate::Reg<shdw_die_id_0::ShdwDieId0Spec>;
#[doc = "Shadow of DIE_ID_0 register in eFuse"]
pub mod shdw_die_id_0;
#[doc = "SHDW_DIE_ID_1 (rw) register accessor: Shadow of DIE_ID_1 register in eFuse\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_die_id_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_die_id_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shdw_die_id_1`]
module"]
#[doc(alias = "SHDW_DIE_ID_1")]
pub type ShdwDieId1 = crate::Reg<shdw_die_id_1::ShdwDieId1Spec>;
#[doc = "Shadow of DIE_ID_1 register in eFuse"]
pub mod shdw_die_id_1;
#[doc = "SHDW_DIE_ID_2 (rw) register accessor: Shadow of DIE_ID_2 register in eFuse\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_die_id_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_die_id_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shdw_die_id_2`]
module"]
#[doc(alias = "SHDW_DIE_ID_2")]
pub type ShdwDieId2 = crate::Reg<shdw_die_id_2::ShdwDieId2Spec>;
#[doc = "Shadow of DIE_ID_2 register in eFuse"]
pub mod shdw_die_id_2;
#[doc = "SHDW_DIE_ID_3 (rw) register accessor: Shadow of DIE_ID_3 register in eFuse\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_die_id_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_die_id_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shdw_die_id_3`]
module"]
#[doc(alias = "SHDW_DIE_ID_3")]
pub type ShdwDieId3 = crate::Reg<shdw_die_id_3::ShdwDieId3Spec>;
#[doc = "Shadow of DIE_ID_3 register in eFuse"]
pub mod shdw_die_id_3;
#[doc = "SHDW_OSC_BIAS_LDO_TRIM (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_osc_bias_ldo_trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_osc_bias_ldo_trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shdw_osc_bias_ldo_trim`]
module"]
#[doc(alias = "SHDW_OSC_BIAS_LDO_TRIM")]
pub type ShdwOscBiasLdoTrim = crate::Reg<shdw_osc_bias_ldo_trim::ShdwOscBiasLdoTrimSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod shdw_osc_bias_ldo_trim;
#[doc = "SHDW_ANA_TRIM (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_ana_trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_ana_trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shdw_ana_trim`]
module"]
#[doc(alias = "SHDW_ANA_TRIM")]
pub type ShdwAnaTrim = crate::Reg<shdw_ana_trim::ShdwAnaTrimSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod shdw_ana_trim;
#[doc = "OSC_CONF1 (rw) register accessor: Oscillator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc_conf1`]
module"]
#[doc(alias = "OSC_CONF1")]
pub type OscConf1 = crate::Reg<osc_conf1::OscConf1Spec>;
#[doc = "Oscillator configuration"]
pub mod osc_conf1;
#[doc = "DAC_BIAS_CNF (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_bias_cnf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_bias_cnf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_bias_cnf`]
module"]
#[doc(alias = "DAC_BIAS_CNF")]
pub type DacBiasCnf = crate::Reg<dac_bias_cnf::DacBiasCnfSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_bias_cnf;
#[doc = "TFW_PROBE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfw_probe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfw_probe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfw_probe`]
module"]
#[doc(alias = "TFW_PROBE")]
pub type TfwProbe = crate::Reg<tfw_probe::TfwProbeSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tfw_probe;
#[doc = "TFW_FT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfw_ft::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfw_ft::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfw_ft`]
module"]
#[doc(alias = "TFW_FT")]
pub type TfwFt = crate::Reg<tfw_ft::TfwFtSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tfw_ft;
#[doc = "DAC_CAL0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_cal0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_cal0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_cal0`]
module"]
#[doc(alias = "DAC_CAL0")]
pub type DacCal0 = crate::Reg<dac_cal0::DacCal0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal0;
#[doc = "DAC_CAL1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_cal1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_cal1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_cal1`]
module"]
#[doc(alias = "DAC_CAL1")]
pub type DacCal1 = crate::Reg<dac_cal1::DacCal1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal1;
#[doc = "DAC_CAL2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_cal2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_cal2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_cal2`]
module"]
#[doc(alias = "DAC_CAL2")]
pub type DacCal2 = crate::Reg<dac_cal2::DacCal2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal2;
#[doc = "DAC_CAL3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_cal3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_cal3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_cal3`]
module"]
#[doc(alias = "DAC_CAL3")]
pub type DacCal3 = crate::Reg<dac_cal3::DacCal3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal3;
#[doc = "RESERVED_N (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved_n::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved_n::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved_n`]
module"]
#[doc(alias = "RESERVED_N")]
pub type ReservedN = crate::Reg<reserved_n::ReservedNSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_n;

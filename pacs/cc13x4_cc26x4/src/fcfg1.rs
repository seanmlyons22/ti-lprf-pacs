#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved_0: RESERVED_0,
    #[doc = "0x04 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved_4: RESERVED_4,
    _reserved2: [u8; 0x98],
    #[doc = "0xa0 - Misc configurations"]
    pub misc_conf_1: MISC_CONF_1,
    #[doc = "0xa4 - Internal. Only to be used through TI provided API."]
    pub misc_conf_2: MISC_CONF_2,
    _reserved4: [u8; 0x08],
    #[doc = "0xb0 - Internal. Only to be used through TI provided API."]
    pub hposc_meas_5: HPOSC_MEAS_5,
    #[doc = "0xb4 - Internal. Only to be used through TI provided API."]
    pub hposc_meas_4: HPOSC_MEAS_4,
    #[doc = "0xb8 - Internal. Only to be used through TI provided API."]
    pub hposc_meas_3: HPOSC_MEAS_3,
    #[doc = "0xbc - Internal. Only to be used through TI provided API."]
    pub hposc_meas_2: HPOSC_MEAS_2,
    #[doc = "0xc0 - Internal. Only to be used through TI provided API."]
    pub hposc_meas_1: HPOSC_MEAS_1,
    #[doc = "0xc4 - Internal. Only to be used through TI provided API."]
    pub config_cc26_fe: CONFIG_CC26_FE,
    #[doc = "0xc8 - Internal. Only to be used through TI provided API."]
    pub config_cc13_fe: CONFIG_CC13_FE,
    #[doc = "0xcc - Internal. Only to be used through TI provided API."]
    pub config_rf_common: CONFIG_RF_COMMON,
    #[doc = "0xd0 - Internal. Only to be used through TI provided API."]
    pub config_synth_div2_cc26_2g4: CONFIG_SYNTH_DIV2_CC26_2G4,
    #[doc = "0xd4 - Internal. Only to be used through TI provided API."]
    pub config_synth_div2_cc13_2g4: CONFIG_SYNTH_DIV2_CC13_2G4,
    #[doc = "0xd8 - Internal. Only to be used through TI provided API."]
    pub config_synth_div2_cc26_1g: CONFIG_SYNTH_DIV2_CC26_1G,
    #[doc = "0xdc - Internal. Only to be used through TI provided API."]
    pub config_synth_div2_cc13_1g: CONFIG_SYNTH_DIV2_CC13_1G,
    #[doc = "0xe0 - Internal. Only to be used through TI provided API."]
    pub config_synth_div4_cc26: CONFIG_SYNTH_DIV4_CC26,
    #[doc = "0xe4 - Internal. Only to be used through TI provided API."]
    pub config_synth_div4_cc13: CONFIG_SYNTH_DIV4_CC13,
    #[doc = "0xe8 - Internal. Only to be used through TI provided API."]
    pub config_synth_div5: CONFIG_SYNTH_DIV5,
    #[doc = "0xec - Internal. Only to be used through TI provided API."]
    pub config_synth_div6_cc26: CONFIG_SYNTH_DIV6_CC26,
    #[doc = "0xf0 - Internal. Only to be used through TI provided API."]
    pub config_synth_div6_cc13: CONFIG_SYNTH_DIV6_CC13,
    #[doc = "0xf4 - Internal. Only to be used through TI provided API."]
    pub config_synth_div10: CONFIG_SYNTH_DIV10,
    #[doc = "0xf8 - Internal. Only to be used through TI provided API."]
    pub config_synth_div12_cc26: CONFIG_SYNTH_DIV12_CC26,
    #[doc = "0xfc - Internal. Only to be used through TI provided API."]
    pub config_synth_div12_cc13: CONFIG_SYNTH_DIV12_CC13,
    #[doc = "0x100 - Internal. Only to be used through TI provided API."]
    pub config_synth_div15: CONFIG_SYNTH_DIV15,
    #[doc = "0x104 - Internal. Only to be used through TI provided API."]
    pub config_synth_div30: CONFIG_SYNTH_DIV30,
    _reserved26: [u8; 0x3c],
    #[doc = "0x144 - IO Configuration"]
    pub ioconf: IOCONF,
    _reserved27: [u8; 0x08],
    #[doc = "0x150 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved_140: RESERVED_140,
    _reserved28: [u8; 0x0140],
    #[doc = "0x294 - User Identification. Reading this register and the FCFG1:ICEPICK_DEVICE_ID register is the only supported way of identifying a device. The value of this register will be written to AON_PMCTL:JTAGUSERCODE by boot FW while in safezone."]
    pub user_id: USER_ID,
    _reserved29: [u8; 0x18],
    #[doc = "0x2b0 - Internal. Only to be used through TI provided API."]
    pub flash_otp_data3: FLASH_OTP_DATA3,
    #[doc = "0x2b4 - Internal. Only to be used through TI provided API."]
    pub ana2_trim: ANA2_TRIM,
    #[doc = "0x2b8 - Internal. Only to be used through TI provided API."]
    pub ldo_trim: LDO_TRIM,
    _reserved32: [u8; 0x2c],
    #[doc = "0x2e8 - MAC BLE Address 0"]
    pub mac_ble_0: MAC_BLE_0,
    #[doc = "0x2ec - MAC BLE Address 1"]
    pub mac_ble_1: MAC_BLE_1,
    #[doc = "0x2f0 - MAC IEEE 802.15.4 Address 0"]
    pub mac_15_4_0: MAC_15_4_0,
    #[doc = "0x2f4 - MAC IEEE 802.15.4 Address 1"]
    pub mac_15_4_1: MAC_15_4_1,
    _reserved36: [u8; 0x14],
    #[doc = "0x30c - Miscellaneous Trim Parameters"]
    pub misc_trim: MISC_TRIM,
    #[doc = "0x310 - Internal. Only to be used through TI provided API."]
    pub rcosc_hf_tempcomp: RCOSC_HF_TEMPCOMP,
    _reserved38: [u8; 0x04],
    #[doc = "0x318 - IcePick Device Identification Reading this register and the FCFG1:USER_ID register is the only supported way of identifying a device."]
    pub icepick_device_id: ICEPICK_DEVICE_ID,
    #[doc = "0x31c - Factory Configuration (FCFG1) Revision"]
    pub fcfg1_revision: FCFG1_REVISION,
    #[doc = "0x320 - Misc OTP Data"]
    pub misc_otp_data: MISC_OTP_DATA,
    #[doc = "0x324 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved_324: RESERVED_324,
    _reserved42: [u8; 0x24],
    #[doc = "0x34c - Internal. Only to be used through TI provided API."]
    pub config_if_adc: CONFIG_IF_ADC,
    #[doc = "0x350 - Internal. Only to be used through TI provided API."]
    pub config_osc_top: CONFIG_OSC_TOP,
    _reserved44: [u8; 0x08],
    #[doc = "0x35c - AUX_ADC Gain in Absolute Reference Mode"]
    pub soc_adc_abs_gain: SOC_ADC_ABS_GAIN,
    #[doc = "0x360 - AUX_ADC Gain in Relative Reference Mode"]
    pub soc_adc_rel_gain: SOC_ADC_REL_GAIN,
    _reserved46: [u8; 0x04],
    #[doc = "0x368 - AUX_ADC Temperature Offsets in Absolute Reference Mode"]
    pub soc_adc_offset_int: SOC_ADC_OFFSET_INT,
    #[doc = "0x36c - Internal. Only to be used through TI provided API."]
    pub soc_adc_ref_trim_and_offset_ext: SOC_ADC_REF_TRIM_AND_OFFSET_EXT,
    #[doc = "0x370 - Internal. Only to be used through TI provided API."]
    pub ampcomp_th1: AMPCOMP_TH1,
    #[doc = "0x374 - Internal. Only to be used through TI provided API."]
    pub ampcomp_th2: AMPCOMP_TH2,
    #[doc = "0x378 - Internal. Only to be used through TI provided API."]
    pub ampcomp_ctrl1: AMPCOMP_CTRL1,
    #[doc = "0x37c - Internal. Only to be used through TI provided API."]
    pub anabypass_value2: ANABYPASS_VALUE2,
    _reserved52: [u8; 0x08],
    #[doc = "0x388 - Internal. Only to be used through TI provided API."]
    pub volt_trim: VOLT_TRIM,
    #[doc = "0x38c - OSC Configuration"]
    pub osc_conf: OSC_CONF,
    #[doc = "0x390 - Internal. Only to be used through TI provided API."]
    pub freq_offset: FREQ_OFFSET,
    _reserved55: [u8; 0x04],
    #[doc = "0x398 - Internal. Only to be used through TI provided API."]
    pub misc_otp_data_1: MISC_OTP_DATA_1,
    _reserved56: [u8; 0x34],
    #[doc = "0x3d0 - Shadow of DIE_ID_0 register in eFuse"]
    pub shdw_die_id_0: SHDW_DIE_ID_0,
    #[doc = "0x3d4 - Shadow of DIE_ID_1 register in eFuse"]
    pub shdw_die_id_1: SHDW_DIE_ID_1,
    #[doc = "0x3d8 - Shadow of DIE_ID_2 register in eFuse"]
    pub shdw_die_id_2: SHDW_DIE_ID_2,
    #[doc = "0x3dc - Shadow of DIE_ID_3 register in eFuse"]
    pub shdw_die_id_3: SHDW_DIE_ID_3,
    _reserved60: [u8; 0x18],
    #[doc = "0x3f8 - Internal. Only to be used through TI provided API."]
    pub shdw_scan_mcu3_sec: SHDW_SCAN_MCU3_SEC,
    #[doc = "0x3fc - Internal. Only to be used through TI provided API."]
    pub shdw_scan_data1_crc: SHDW_SCAN_DATA1_CRC,
    _reserved62: [u8; 0x04],
    #[doc = "0x404 - Internal. Only to be used through TI provided API."]
    pub shdw_ana_trim: SHDW_ANA_TRIM,
    #[doc = "0x408 - Oscillator configuration"]
    pub osc_conf1: OSC_CONF1,
    #[doc = "0x40c - Internal. Only to be used through TI provided API."]
    pub dac_bias_cnf: DAC_BIAS_CNF,
    _reserved65: [u8; 0x08],
    #[doc = "0x418 - Internal. Only to be used through TI provided API."]
    pub tfw_probe: TFW_PROBE,
    #[doc = "0x41c - Internal. Only to be used through TI provided API."]
    pub tfw_ft: TFW_FT,
    #[doc = "0x420 - Internal. Only to be used through TI provided API."]
    pub dac_cal0: DAC_CAL0,
    #[doc = "0x424 - Internal. Only to be used through TI provided API."]
    pub dac_cal1: DAC_CAL1,
    #[doc = "0x428 - Internal. Only to be used through TI provided API."]
    pub dac_cal2: DAC_CAL2,
    #[doc = "0x42c - Internal. Only to be used through TI provided API."]
    pub dac_cal3: DAC_CAL3,
    _reserved71: [u8; 0x08],
    #[doc = "0x438 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved_n: RESERVED_N,
}
#[doc = "RESERVED_0 (rw) register accessor: an alias for `Reg<RESERVED_0_SPEC>`"]
pub type RESERVED_0 = crate::Reg<reserved_0::RESERVED_0_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_0;
#[doc = "RESERVED_4 (rw) register accessor: an alias for `Reg<RESERVED_4_SPEC>`"]
pub type RESERVED_4 = crate::Reg<reserved_4::RESERVED_4_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_4;
#[doc = "MISC_CONF_1 (rw) register accessor: an alias for `Reg<MISC_CONF_1_SPEC>`"]
pub type MISC_CONF_1 = crate::Reg<misc_conf_1::MISC_CONF_1_SPEC>;
#[doc = "Misc configurations"]
pub mod misc_conf_1;
#[doc = "MISC_CONF_2 (rw) register accessor: an alias for `Reg<MISC_CONF_2_SPEC>`"]
pub type MISC_CONF_2 = crate::Reg<misc_conf_2::MISC_CONF_2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod misc_conf_2;
#[doc = "HPOSC_MEAS_5 (rw) register accessor: an alias for `Reg<HPOSC_MEAS_5_SPEC>`"]
pub type HPOSC_MEAS_5 = crate::Reg<hposc_meas_5::HPOSC_MEAS_5_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod hposc_meas_5;
#[doc = "HPOSC_MEAS_4 (rw) register accessor: an alias for `Reg<HPOSC_MEAS_4_SPEC>`"]
pub type HPOSC_MEAS_4 = crate::Reg<hposc_meas_4::HPOSC_MEAS_4_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod hposc_meas_4;
#[doc = "HPOSC_MEAS_3 (rw) register accessor: an alias for `Reg<HPOSC_MEAS_3_SPEC>`"]
pub type HPOSC_MEAS_3 = crate::Reg<hposc_meas_3::HPOSC_MEAS_3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod hposc_meas_3;
#[doc = "HPOSC_MEAS_2 (rw) register accessor: an alias for `Reg<HPOSC_MEAS_2_SPEC>`"]
pub type HPOSC_MEAS_2 = crate::Reg<hposc_meas_2::HPOSC_MEAS_2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod hposc_meas_2;
#[doc = "HPOSC_MEAS_1 (rw) register accessor: an alias for `Reg<HPOSC_MEAS_1_SPEC>`"]
pub type HPOSC_MEAS_1 = crate::Reg<hposc_meas_1::HPOSC_MEAS_1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod hposc_meas_1;
#[doc = "CONFIG_CC26_FE (rw) register accessor: an alias for `Reg<CONFIG_CC26_FE_SPEC>`"]
pub type CONFIG_CC26_FE = crate::Reg<config_cc26_fe::CONFIG_CC26_FE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_cc26_fe;
#[doc = "CONFIG_CC13_FE (rw) register accessor: an alias for `Reg<CONFIG_CC13_FE_SPEC>`"]
pub type CONFIG_CC13_FE = crate::Reg<config_cc13_fe::CONFIG_CC13_FE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_cc13_fe;
#[doc = "CONFIG_RF_COMMON (rw) register accessor: an alias for `Reg<CONFIG_RF_COMMON_SPEC>`"]
pub type CONFIG_RF_COMMON = crate::Reg<config_rf_common::CONFIG_RF_COMMON_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_common;
#[doc = "CONFIG_SYNTH_DIV2_CC26_2G4 (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV2_CC26_2G4_SPEC>`"]
pub type CONFIG_SYNTH_DIV2_CC26_2G4 =
    crate::Reg<config_synth_div2_cc26_2g4::CONFIG_SYNTH_DIV2_CC26_2G4_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc26_2g4;
#[doc = "CONFIG_SYNTH_DIV2_CC13_2G4 (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV2_CC13_2G4_SPEC>`"]
pub type CONFIG_SYNTH_DIV2_CC13_2G4 =
    crate::Reg<config_synth_div2_cc13_2g4::CONFIG_SYNTH_DIV2_CC13_2G4_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc13_2g4;
#[doc = "CONFIG_SYNTH_DIV2_CC26_1G (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV2_CC26_1G_SPEC>`"]
pub type CONFIG_SYNTH_DIV2_CC26_1G =
    crate::Reg<config_synth_div2_cc26_1g::CONFIG_SYNTH_DIV2_CC26_1G_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc26_1g;
#[doc = "CONFIG_SYNTH_DIV2_CC13_1G (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV2_CC13_1G_SPEC>`"]
pub type CONFIG_SYNTH_DIV2_CC13_1G =
    crate::Reg<config_synth_div2_cc13_1g::CONFIG_SYNTH_DIV2_CC13_1G_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc13_1g;
#[doc = "CONFIG_SYNTH_DIV4_CC26 (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV4_CC26_SPEC>`"]
pub type CONFIG_SYNTH_DIV4_CC26 = crate::Reg<config_synth_div4_cc26::CONFIG_SYNTH_DIV4_CC26_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div4_cc26;
#[doc = "CONFIG_SYNTH_DIV4_CC13 (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV4_CC13_SPEC>`"]
pub type CONFIG_SYNTH_DIV4_CC13 = crate::Reg<config_synth_div4_cc13::CONFIG_SYNTH_DIV4_CC13_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div4_cc13;
#[doc = "CONFIG_SYNTH_DIV5 (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV5_SPEC>`"]
pub type CONFIG_SYNTH_DIV5 = crate::Reg<config_synth_div5::CONFIG_SYNTH_DIV5_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div5;
#[doc = "CONFIG_SYNTH_DIV6_CC26 (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV6_CC26_SPEC>`"]
pub type CONFIG_SYNTH_DIV6_CC26 = crate::Reg<config_synth_div6_cc26::CONFIG_SYNTH_DIV6_CC26_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div6_cc26;
#[doc = "CONFIG_SYNTH_DIV6_CC13 (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV6_CC13_SPEC>`"]
pub type CONFIG_SYNTH_DIV6_CC13 = crate::Reg<config_synth_div6_cc13::CONFIG_SYNTH_DIV6_CC13_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div6_cc13;
#[doc = "CONFIG_SYNTH_DIV10 (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV10_SPEC>`"]
pub type CONFIG_SYNTH_DIV10 = crate::Reg<config_synth_div10::CONFIG_SYNTH_DIV10_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div10;
#[doc = "CONFIG_SYNTH_DIV12_CC26 (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV12_CC26_SPEC>`"]
pub type CONFIG_SYNTH_DIV12_CC26 =
    crate::Reg<config_synth_div12_cc26::CONFIG_SYNTH_DIV12_CC26_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div12_cc26;
#[doc = "CONFIG_SYNTH_DIV12_CC13 (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV12_CC13_SPEC>`"]
pub type CONFIG_SYNTH_DIV12_CC13 =
    crate::Reg<config_synth_div12_cc13::CONFIG_SYNTH_DIV12_CC13_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div12_cc13;
#[doc = "CONFIG_SYNTH_DIV15 (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV15_SPEC>`"]
pub type CONFIG_SYNTH_DIV15 = crate::Reg<config_synth_div15::CONFIG_SYNTH_DIV15_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div15;
#[doc = "CONFIG_SYNTH_DIV30 (rw) register accessor: an alias for `Reg<CONFIG_SYNTH_DIV30_SPEC>`"]
pub type CONFIG_SYNTH_DIV30 = crate::Reg<config_synth_div30::CONFIG_SYNTH_DIV30_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div30;
#[doc = "IOCONF (rw) register accessor: an alias for `Reg<IOCONF_SPEC>`"]
pub type IOCONF = crate::Reg<ioconf::IOCONF_SPEC>;
#[doc = "IO Configuration"]
pub mod ioconf;
#[doc = "RESERVED_140 (rw) register accessor: an alias for `Reg<RESERVED_140_SPEC>`"]
pub type RESERVED_140 = crate::Reg<reserved_140::RESERVED_140_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_140;
#[doc = "USER_ID (rw) register accessor: an alias for `Reg<USER_ID_SPEC>`"]
pub type USER_ID = crate::Reg<user_id::USER_ID_SPEC>;
#[doc = "User Identification. Reading this register and the FCFG1:ICEPICK_DEVICE_ID register is the only supported way of identifying a device. The value of this register will be written to AON_PMCTL:JTAGUSERCODE by boot FW while in safezone."]
pub mod user_id;
#[doc = "FLASH_OTP_DATA3 (rw) register accessor: an alias for `Reg<FLASH_OTP_DATA3_SPEC>`"]
pub type FLASH_OTP_DATA3 = crate::Reg<flash_otp_data3::FLASH_OTP_DATA3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_otp_data3;
#[doc = "ANA2_TRIM (rw) register accessor: an alias for `Reg<ANA2_TRIM_SPEC>`"]
pub type ANA2_TRIM = crate::Reg<ana2_trim::ANA2_TRIM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ana2_trim;
#[doc = "LDO_TRIM (rw) register accessor: an alias for `Reg<LDO_TRIM_SPEC>`"]
pub type LDO_TRIM = crate::Reg<ldo_trim::LDO_TRIM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ldo_trim;
#[doc = "MAC_BLE_0 (rw) register accessor: an alias for `Reg<MAC_BLE_0_SPEC>`"]
pub type MAC_BLE_0 = crate::Reg<mac_ble_0::MAC_BLE_0_SPEC>;
#[doc = "MAC BLE Address 0"]
pub mod mac_ble_0;
#[doc = "MAC_BLE_1 (rw) register accessor: an alias for `Reg<MAC_BLE_1_SPEC>`"]
pub type MAC_BLE_1 = crate::Reg<mac_ble_1::MAC_BLE_1_SPEC>;
#[doc = "MAC BLE Address 1"]
pub mod mac_ble_1;
#[doc = "MAC_15_4_0 (rw) register accessor: an alias for `Reg<MAC_15_4_0_SPEC>`"]
pub type MAC_15_4_0 = crate::Reg<mac_15_4_0::MAC_15_4_0_SPEC>;
#[doc = "MAC IEEE 802.15.4 Address 0"]
pub mod mac_15_4_0;
#[doc = "MAC_15_4_1 (rw) register accessor: an alias for `Reg<MAC_15_4_1_SPEC>`"]
pub type MAC_15_4_1 = crate::Reg<mac_15_4_1::MAC_15_4_1_SPEC>;
#[doc = "MAC IEEE 802.15.4 Address 1"]
pub mod mac_15_4_1;
#[doc = "MISC_TRIM (rw) register accessor: an alias for `Reg<MISC_TRIM_SPEC>`"]
pub type MISC_TRIM = crate::Reg<misc_trim::MISC_TRIM_SPEC>;
#[doc = "Miscellaneous Trim Parameters"]
pub mod misc_trim;
#[doc = "RCOSC_HF_TEMPCOMP (rw) register accessor: an alias for `Reg<RCOSC_HF_TEMPCOMP_SPEC>`"]
pub type RCOSC_HF_TEMPCOMP = crate::Reg<rcosc_hf_tempcomp::RCOSC_HF_TEMPCOMP_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod rcosc_hf_tempcomp;
#[doc = "ICEPICK_DEVICE_ID (rw) register accessor: an alias for `Reg<ICEPICK_DEVICE_ID_SPEC>`"]
pub type ICEPICK_DEVICE_ID = crate::Reg<icepick_device_id::ICEPICK_DEVICE_ID_SPEC>;
#[doc = "IcePick Device Identification Reading this register and the FCFG1:USER_ID register is the only supported way of identifying a device."]
pub mod icepick_device_id;
#[doc = "FCFG1_REVISION (rw) register accessor: an alias for `Reg<FCFG1_REVISION_SPEC>`"]
pub type FCFG1_REVISION = crate::Reg<fcfg1_revision::FCFG1_REVISION_SPEC>;
#[doc = "Factory Configuration (FCFG1) Revision"]
pub mod fcfg1_revision;
#[doc = "MISC_OTP_DATA (rw) register accessor: an alias for `Reg<MISC_OTP_DATA_SPEC>`"]
pub type MISC_OTP_DATA = crate::Reg<misc_otp_data::MISC_OTP_DATA_SPEC>;
#[doc = "Misc OTP Data"]
pub mod misc_otp_data;
#[doc = "RESERVED_324 (rw) register accessor: an alias for `Reg<RESERVED_324_SPEC>`"]
pub type RESERVED_324 = crate::Reg<reserved_324::RESERVED_324_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_324;
#[doc = "CONFIG_IF_ADC (rw) register accessor: an alias for `Reg<CONFIG_IF_ADC_SPEC>`"]
pub type CONFIG_IF_ADC = crate::Reg<config_if_adc::CONFIG_IF_ADC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_if_adc;
#[doc = "CONFIG_OSC_TOP (rw) register accessor: an alias for `Reg<CONFIG_OSC_TOP_SPEC>`"]
pub type CONFIG_OSC_TOP = crate::Reg<config_osc_top::CONFIG_OSC_TOP_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_osc_top;
#[doc = "SOC_ADC_ABS_GAIN (rw) register accessor: an alias for `Reg<SOC_ADC_ABS_GAIN_SPEC>`"]
pub type SOC_ADC_ABS_GAIN = crate::Reg<soc_adc_abs_gain::SOC_ADC_ABS_GAIN_SPEC>;
#[doc = "AUX_ADC Gain in Absolute Reference Mode"]
pub mod soc_adc_abs_gain;
#[doc = "SOC_ADC_REL_GAIN (rw) register accessor: an alias for `Reg<SOC_ADC_REL_GAIN_SPEC>`"]
pub type SOC_ADC_REL_GAIN = crate::Reg<soc_adc_rel_gain::SOC_ADC_REL_GAIN_SPEC>;
#[doc = "AUX_ADC Gain in Relative Reference Mode"]
pub mod soc_adc_rel_gain;
#[doc = "SOC_ADC_OFFSET_INT (rw) register accessor: an alias for `Reg<SOC_ADC_OFFSET_INT_SPEC>`"]
pub type SOC_ADC_OFFSET_INT = crate::Reg<soc_adc_offset_int::SOC_ADC_OFFSET_INT_SPEC>;
#[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode"]
pub mod soc_adc_offset_int;
#[doc = "SOC_ADC_REF_TRIM_AND_OFFSET_EXT (rw) register accessor: an alias for `Reg<SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC>`"]
pub type SOC_ADC_REF_TRIM_AND_OFFSET_EXT =
    crate::Reg<soc_adc_ref_trim_and_offset_ext::SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod soc_adc_ref_trim_and_offset_ext;
#[doc = "AMPCOMP_TH1 (rw) register accessor: an alias for `Reg<AMPCOMP_TH1_SPEC>`"]
pub type AMPCOMP_TH1 = crate::Reg<ampcomp_th1::AMPCOMP_TH1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_th1;
#[doc = "AMPCOMP_TH2 (rw) register accessor: an alias for `Reg<AMPCOMP_TH2_SPEC>`"]
pub type AMPCOMP_TH2 = crate::Reg<ampcomp_th2::AMPCOMP_TH2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_th2;
#[doc = "AMPCOMP_CTRL1 (rw) register accessor: an alias for `Reg<AMPCOMP_CTRL1_SPEC>`"]
pub type AMPCOMP_CTRL1 = crate::Reg<ampcomp_ctrl1::AMPCOMP_CTRL1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_ctrl1;
#[doc = "ANABYPASS_VALUE2 (rw) register accessor: an alias for `Reg<ANABYPASS_VALUE2_SPEC>`"]
pub type ANABYPASS_VALUE2 = crate::Reg<anabypass_value2::ANABYPASS_VALUE2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod anabypass_value2;
#[doc = "VOLT_TRIM (rw) register accessor: an alias for `Reg<VOLT_TRIM_SPEC>`"]
pub type VOLT_TRIM = crate::Reg<volt_trim::VOLT_TRIM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod volt_trim;
#[doc = "OSC_CONF (rw) register accessor: an alias for `Reg<OSC_CONF_SPEC>`"]
pub type OSC_CONF = crate::Reg<osc_conf::OSC_CONF_SPEC>;
#[doc = "OSC Configuration"]
pub mod osc_conf;
#[doc = "FREQ_OFFSET (rw) register accessor: an alias for `Reg<FREQ_OFFSET_SPEC>`"]
pub type FREQ_OFFSET = crate::Reg<freq_offset::FREQ_OFFSET_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod freq_offset;
#[doc = "MISC_OTP_DATA_1 (rw) register accessor: an alias for `Reg<MISC_OTP_DATA_1_SPEC>`"]
pub type MISC_OTP_DATA_1 = crate::Reg<misc_otp_data_1::MISC_OTP_DATA_1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod misc_otp_data_1;
#[doc = "SHDW_DIE_ID_0 (rw) register accessor: an alias for `Reg<SHDW_DIE_ID_0_SPEC>`"]
pub type SHDW_DIE_ID_0 = crate::Reg<shdw_die_id_0::SHDW_DIE_ID_0_SPEC>;
#[doc = "Shadow of DIE_ID_0 register in eFuse"]
pub mod shdw_die_id_0;
#[doc = "SHDW_DIE_ID_1 (rw) register accessor: an alias for `Reg<SHDW_DIE_ID_1_SPEC>`"]
pub type SHDW_DIE_ID_1 = crate::Reg<shdw_die_id_1::SHDW_DIE_ID_1_SPEC>;
#[doc = "Shadow of DIE_ID_1 register in eFuse"]
pub mod shdw_die_id_1;
#[doc = "SHDW_DIE_ID_2 (rw) register accessor: an alias for `Reg<SHDW_DIE_ID_2_SPEC>`"]
pub type SHDW_DIE_ID_2 = crate::Reg<shdw_die_id_2::SHDW_DIE_ID_2_SPEC>;
#[doc = "Shadow of DIE_ID_2 register in eFuse"]
pub mod shdw_die_id_2;
#[doc = "SHDW_DIE_ID_3 (rw) register accessor: an alias for `Reg<SHDW_DIE_ID_3_SPEC>`"]
pub type SHDW_DIE_ID_3 = crate::Reg<shdw_die_id_3::SHDW_DIE_ID_3_SPEC>;
#[doc = "Shadow of DIE_ID_3 register in eFuse"]
pub mod shdw_die_id_3;
#[doc = "SHDW_SCAN_MCU3_SEC (rw) register accessor: an alias for `Reg<SHDW_SCAN_MCU3_SEC_SPEC>`"]
pub type SHDW_SCAN_MCU3_SEC = crate::Reg<shdw_scan_mcu3_sec::SHDW_SCAN_MCU3_SEC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod shdw_scan_mcu3_sec;
#[doc = "SHDW_SCAN_DATA1_CRC (rw) register accessor: an alias for `Reg<SHDW_SCAN_DATA1_CRC_SPEC>`"]
pub type SHDW_SCAN_DATA1_CRC = crate::Reg<shdw_scan_data1_crc::SHDW_SCAN_DATA1_CRC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod shdw_scan_data1_crc;
#[doc = "SHDW_ANA_TRIM (rw) register accessor: an alias for `Reg<SHDW_ANA_TRIM_SPEC>`"]
pub type SHDW_ANA_TRIM = crate::Reg<shdw_ana_trim::SHDW_ANA_TRIM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod shdw_ana_trim;
#[doc = "OSC_CONF1 (rw) register accessor: an alias for `Reg<OSC_CONF1_SPEC>`"]
pub type OSC_CONF1 = crate::Reg<osc_conf1::OSC_CONF1_SPEC>;
#[doc = "Oscillator configuration"]
pub mod osc_conf1;
#[doc = "DAC_BIAS_CNF (rw) register accessor: an alias for `Reg<DAC_BIAS_CNF_SPEC>`"]
pub type DAC_BIAS_CNF = crate::Reg<dac_bias_cnf::DAC_BIAS_CNF_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_bias_cnf;
#[doc = "TFW_PROBE (rw) register accessor: an alias for `Reg<TFW_PROBE_SPEC>`"]
pub type TFW_PROBE = crate::Reg<tfw_probe::TFW_PROBE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tfw_probe;
#[doc = "TFW_FT (rw) register accessor: an alias for `Reg<TFW_FT_SPEC>`"]
pub type TFW_FT = crate::Reg<tfw_ft::TFW_FT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tfw_ft;
#[doc = "DAC_CAL0 (rw) register accessor: an alias for `Reg<DAC_CAL0_SPEC>`"]
pub type DAC_CAL0 = crate::Reg<dac_cal0::DAC_CAL0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal0;
#[doc = "DAC_CAL1 (rw) register accessor: an alias for `Reg<DAC_CAL1_SPEC>`"]
pub type DAC_CAL1 = crate::Reg<dac_cal1::DAC_CAL1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal1;
#[doc = "DAC_CAL2 (rw) register accessor: an alias for `Reg<DAC_CAL2_SPEC>`"]
pub type DAC_CAL2 = crate::Reg<dac_cal2::DAC_CAL2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal2;
#[doc = "DAC_CAL3 (rw) register accessor: an alias for `Reg<DAC_CAL3_SPEC>`"]
pub type DAC_CAL3 = crate::Reg<dac_cal3::DAC_CAL3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal3;
#[doc = "RESERVED_N (rw) register accessor: an alias for `Reg<RESERVED_N_SPEC>`"]
pub type RESERVED_N = crate::Reg<reserved_n::RESERVED_N_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_n;

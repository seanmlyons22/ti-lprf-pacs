#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub weprot_b0_31_0_by1: WEPROT_B0_31_0_BY1,
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    pub weprot_aux_by1: WEPROT_AUX_BY1,
    _reserved2: [u8; 0x14],
    #[doc = "0x1c - NW and Efuse Status"]
    pub stat: STAT,
    _reserved3: [u8; 0x04],
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    pub cfg: CFG,
    _reserved4: [u8; 0x04],
    #[doc = "0x2c - Internal. Only to be used through TI provided API."]
    pub flash_size: FLASH_SIZE,
    _reserved5: [u8; 0x0c],
    #[doc = "0x3c - Internal. Only to be used through TI provided API."]
    pub fwlock: FWLOCK,
    #[doc = "0x40 - Internal. Only to be used through TI provided API."]
    pub fwflag: FWFLAG,
    _reserved7: [u8; 0x0c],
    #[doc = "0x50 - Internal. Only to be used through TI provided API."]
    pub bank0_trim_cfg_3: BANK0_TRIM_CFG_3,
    #[doc = "0x54 - Internal. Only to be used through TI provided API."]
    pub bank0_trim_cfg_2: BANK0_TRIM_CFG_2,
    #[doc = "0x58 - Internal. Only to be used through TI provided API."]
    pub bank0_trim_cfg_1: BANK0_TRIM_CFG_1,
    #[doc = "0x5c - Internal. Only to be used through TI provided API."]
    pub bank0_trim_cfg_0: BANK0_TRIM_CFG_0,
    #[doc = "0x60 - Internal. Only to be used through TI provided API."]
    pub bank1_trim_cfg_3: BANK1_TRIM_CFG_3,
    #[doc = "0x64 - Internal. Only to be used through TI provided API."]
    pub bank1_trim_cfg_2: BANK1_TRIM_CFG_2,
    #[doc = "0x68 - Internal. Only to be used through TI provided API."]
    pub bank1_trim_cfg_1: BANK1_TRIM_CFG_1,
    #[doc = "0x6c - Internal. Only to be used through TI provided API."]
    pub bank1_trim_cfg_0: BANK1_TRIM_CFG_0,
    #[doc = "0x70 - Internal. Only to be used through TI provided API."]
    pub pump_trim_cfg_2: PUMP_TRIM_CFG_2,
    #[doc = "0x74 - Internal. Only to be used through TI provided API."]
    pub pump_trim_cfg_1: PUMP_TRIM_CFG_1,
    #[doc = "0x78 - Internal. Only to be used through TI provided API."]
    pub pump_trim_cfg_0: PUMP_TRIM_CFG_0,
    _reserved18: [u8; 0x0f84],
    #[doc = "0x1000 - Internal. Only to be used through TI provided API."]
    pub efuse: EFUSE,
    #[doc = "0x1004 - Internal. Only to be used through TI provided API."]
    pub efuseaddr: EFUSEADDR,
    #[doc = "0x1008 - Internal. Only to be used through TI provided API."]
    pub dataupper: DATAUPPER,
    #[doc = "0x100c - Internal. Only to be used through TI provided API."]
    pub datalower: DATALOWER,
    #[doc = "0x1010 - Internal. Only to be used through TI provided API."]
    pub efusecfg: EFUSECFG,
    #[doc = "0x1014 - Internal. Only to be used through TI provided API."]
    pub efusestat: EFUSESTAT,
    #[doc = "0x1018 - Internal. Only to be used through TI provided API."]
    pub acc: ACC,
    #[doc = "0x101c - Internal. Only to be used through TI provided API."]
    pub boundary: BOUNDARY,
    #[doc = "0x1020 - Internal. Only to be used through TI provided API."]
    pub efuseflag: EFUSEFLAG,
    #[doc = "0x1024 - Internal. Only to be used through TI provided API."]
    pub efusekey: EFUSEKEY,
    #[doc = "0x1028 - Internal. Only to be used through TI provided API."]
    pub efuserelease: EFUSERELEASE,
    #[doc = "0x102c - Internal. Only to be used through TI provided API."]
    pub efusepins: EFUSEPINS,
    #[doc = "0x1030 - Internal. Only to be used through TI provided API."]
    pub efusecra: EFUSECRA,
    #[doc = "0x1034 - Internal. Only to be used through TI provided API."]
    pub efuseread: EFUSEREAD,
    #[doc = "0x1038 - Internal. Only to be used through TI provided API."]
    pub efuseprogram: EFUSEPROGRAM,
    #[doc = "0x103c - Internal. Only to be used through TI provided API."]
    pub efuseerror: EFUSEERROR,
    #[doc = "0x1040 - Internal. Only to be used through TI provided API."]
    pub singlebit: SINGLEBIT,
    #[doc = "0x1044 - Internal. Only to be used through TI provided API."]
    pub twobit: TWOBIT,
    #[doc = "0x1048 - Internal. Only to be used through TI provided API."]
    pub selftestcyc: SELFTESTCYC,
    #[doc = "0x104c - Internal. Only to be used through TI provided API."]
    pub selftestsign: SELFTESTSIGN,
}
#[doc = "WEPROT_B0_31_0_BY1 (rw) register accessor: an alias for `Reg<WEPROT_B0_31_0_BY1_SPEC>`"]
pub type WEPROT_B0_31_0_BY1 = crate::Reg<weprot_b0_31_0_by1::WEPROT_B0_31_0_BY1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod weprot_b0_31_0_by1;
#[doc = "WEPROT_AUX_BY1 (rw) register accessor: an alias for `Reg<WEPROT_AUX_BY1_SPEC>`"]
pub type WEPROT_AUX_BY1 = crate::Reg<weprot_aux_by1::WEPROT_AUX_BY1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod weprot_aux_by1;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "NW and Efuse Status"]
pub mod stat;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cfg;
#[doc = "FLASH_SIZE (rw) register accessor: an alias for `Reg<FLASH_SIZE_SPEC>`"]
pub type FLASH_SIZE = crate::Reg<flash_size::FLASH_SIZE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_size;
#[doc = "FWLOCK (rw) register accessor: an alias for `Reg<FWLOCK_SPEC>`"]
pub type FWLOCK = crate::Reg<fwlock::FWLOCK_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwlock;
#[doc = "FWFLAG (rw) register accessor: an alias for `Reg<FWFLAG_SPEC>`"]
pub type FWFLAG = crate::Reg<fwflag::FWFLAG_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwflag;
#[doc = "BANK0_TRIM_CFG_3 (rw) register accessor: an alias for `Reg<BANK0_TRIM_CFG_3_SPEC>`"]
pub type BANK0_TRIM_CFG_3 = crate::Reg<bank0_trim_cfg_3::BANK0_TRIM_CFG_3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank0_trim_cfg_3;
#[doc = "BANK0_TRIM_CFG_2 (rw) register accessor: an alias for `Reg<BANK0_TRIM_CFG_2_SPEC>`"]
pub type BANK0_TRIM_CFG_2 = crate::Reg<bank0_trim_cfg_2::BANK0_TRIM_CFG_2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank0_trim_cfg_2;
#[doc = "BANK0_TRIM_CFG_1 (rw) register accessor: an alias for `Reg<BANK0_TRIM_CFG_1_SPEC>`"]
pub type BANK0_TRIM_CFG_1 = crate::Reg<bank0_trim_cfg_1::BANK0_TRIM_CFG_1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank0_trim_cfg_1;
#[doc = "BANK0_TRIM_CFG_0 (rw) register accessor: an alias for `Reg<BANK0_TRIM_CFG_0_SPEC>`"]
pub type BANK0_TRIM_CFG_0 = crate::Reg<bank0_trim_cfg_0::BANK0_TRIM_CFG_0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank0_trim_cfg_0;
#[doc = "BANK1_TRIM_CFG_3 (rw) register accessor: an alias for `Reg<BANK1_TRIM_CFG_3_SPEC>`"]
pub type BANK1_TRIM_CFG_3 = crate::Reg<bank1_trim_cfg_3::BANK1_TRIM_CFG_3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank1_trim_cfg_3;
#[doc = "BANK1_TRIM_CFG_2 (rw) register accessor: an alias for `Reg<BANK1_TRIM_CFG_2_SPEC>`"]
pub type BANK1_TRIM_CFG_2 = crate::Reg<bank1_trim_cfg_2::BANK1_TRIM_CFG_2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank1_trim_cfg_2;
#[doc = "BANK1_TRIM_CFG_1 (rw) register accessor: an alias for `Reg<BANK1_TRIM_CFG_1_SPEC>`"]
pub type BANK1_TRIM_CFG_1 = crate::Reg<bank1_trim_cfg_1::BANK1_TRIM_CFG_1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank1_trim_cfg_1;
#[doc = "BANK1_TRIM_CFG_0 (rw) register accessor: an alias for `Reg<BANK1_TRIM_CFG_0_SPEC>`"]
pub type BANK1_TRIM_CFG_0 = crate::Reg<bank1_trim_cfg_0::BANK1_TRIM_CFG_0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank1_trim_cfg_0;
#[doc = "PUMP_TRIM_CFG_2 (rw) register accessor: an alias for `Reg<PUMP_TRIM_CFG_2_SPEC>`"]
pub type PUMP_TRIM_CFG_2 = crate::Reg<pump_trim_cfg_2::PUMP_TRIM_CFG_2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod pump_trim_cfg_2;
#[doc = "PUMP_TRIM_CFG_1 (rw) register accessor: an alias for `Reg<PUMP_TRIM_CFG_1_SPEC>`"]
pub type PUMP_TRIM_CFG_1 = crate::Reg<pump_trim_cfg_1::PUMP_TRIM_CFG_1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod pump_trim_cfg_1;
#[doc = "PUMP_TRIM_CFG_0 (rw) register accessor: an alias for `Reg<PUMP_TRIM_CFG_0_SPEC>`"]
pub type PUMP_TRIM_CFG_0 = crate::Reg<pump_trim_cfg_0::PUMP_TRIM_CFG_0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod pump_trim_cfg_0;
#[doc = "EFUSE (rw) register accessor: an alias for `Reg<EFUSE_SPEC>`"]
pub type EFUSE = crate::Reg<efuse::EFUSE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuse;
#[doc = "EFUSEADDR (rw) register accessor: an alias for `Reg<EFUSEADDR_SPEC>`"]
pub type EFUSEADDR = crate::Reg<efuseaddr::EFUSEADDR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseaddr;
#[doc = "DATAUPPER (rw) register accessor: an alias for `Reg<DATAUPPER_SPEC>`"]
pub type DATAUPPER = crate::Reg<dataupper::DATAUPPER_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dataupper;
#[doc = "DATALOWER (rw) register accessor: an alias for `Reg<DATALOWER_SPEC>`"]
pub type DATALOWER = crate::Reg<datalower::DATALOWER_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod datalower;
#[doc = "EFUSECFG (rw) register accessor: an alias for `Reg<EFUSECFG_SPEC>`"]
pub type EFUSECFG = crate::Reg<efusecfg::EFUSECFG_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusecfg;
#[doc = "EFUSESTAT (rw) register accessor: an alias for `Reg<EFUSESTAT_SPEC>`"]
pub type EFUSESTAT = crate::Reg<efusestat::EFUSESTAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusestat;
#[doc = "ACC (rw) register accessor: an alias for `Reg<ACC_SPEC>`"]
pub type ACC = crate::Reg<acc::ACC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod acc;
#[doc = "BOUNDARY (rw) register accessor: an alias for `Reg<BOUNDARY_SPEC>`"]
pub type BOUNDARY = crate::Reg<boundary::BOUNDARY_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod boundary;
#[doc = "EFUSEFLAG (rw) register accessor: an alias for `Reg<EFUSEFLAG_SPEC>`"]
pub type EFUSEFLAG = crate::Reg<efuseflag::EFUSEFLAG_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseflag;
#[doc = "EFUSEKEY (rw) register accessor: an alias for `Reg<EFUSEKEY_SPEC>`"]
pub type EFUSEKEY = crate::Reg<efusekey::EFUSEKEY_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusekey;
#[doc = "EFUSERELEASE (rw) register accessor: an alias for `Reg<EFUSERELEASE_SPEC>`"]
pub type EFUSERELEASE = crate::Reg<efuserelease::EFUSERELEASE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuserelease;
#[doc = "EFUSEPINS (rw) register accessor: an alias for `Reg<EFUSEPINS_SPEC>`"]
pub type EFUSEPINS = crate::Reg<efusepins::EFUSEPINS_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusepins;
#[doc = "EFUSECRA (rw) register accessor: an alias for `Reg<EFUSECRA_SPEC>`"]
pub type EFUSECRA = crate::Reg<efusecra::EFUSECRA_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusecra;
#[doc = "EFUSEREAD (rw) register accessor: an alias for `Reg<EFUSEREAD_SPEC>`"]
pub type EFUSEREAD = crate::Reg<efuseread::EFUSEREAD_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseread;
#[doc = "EFUSEPROGRAM (rw) register accessor: an alias for `Reg<EFUSEPROGRAM_SPEC>`"]
pub type EFUSEPROGRAM = crate::Reg<efuseprogram::EFUSEPROGRAM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseprogram;
#[doc = "EFUSEERROR (rw) register accessor: an alias for `Reg<EFUSEERROR_SPEC>`"]
pub type EFUSEERROR = crate::Reg<efuseerror::EFUSEERROR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseerror;
#[doc = "SINGLEBIT (rw) register accessor: an alias for `Reg<SINGLEBIT_SPEC>`"]
pub type SINGLEBIT = crate::Reg<singlebit::SINGLEBIT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod singlebit;
#[doc = "TWOBIT (rw) register accessor: an alias for `Reg<TWOBIT_SPEC>`"]
pub type TWOBIT = crate::Reg<twobit::TWOBIT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod twobit;
#[doc = "SELFTESTCYC (rw) register accessor: an alias for `Reg<SELFTESTCYC_SPEC>`"]
pub type SELFTESTCYC = crate::Reg<selftestcyc::SELFTESTCYC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod selftestcyc;
#[doc = "SELFTESTSIGN (rw) register accessor: an alias for `Reg<SELFTESTSIGN_SPEC>`"]
pub type SELFTESTSIGN = crate::Reg<selftestsign::SELFTESTSIGN_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod selftestsign;

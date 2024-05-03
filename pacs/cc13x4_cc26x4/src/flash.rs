#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    weprot_b0_31_0_by1: WeprotB0_31_0By1,
    weprot_aux_by1: WeprotAuxBy1,
    _reserved2: [u8; 0x14],
    stat: Stat,
    _reserved3: [u8; 0x04],
    cfg: Cfg,
    _reserved4: [u8; 0x04],
    flash_size: FlashSize,
    _reserved5: [u8; 0x0c],
    fwlock: Fwlock,
    fwflag: Fwflag,
    _reserved7: [u8; 0x0c],
    bank0_trim_cfg_3: Bank0TrimCfg3,
    bank0_trim_cfg_2: Bank0TrimCfg2,
    bank0_trim_cfg_1: Bank0TrimCfg1,
    bank0_trim_cfg_0: Bank0TrimCfg0,
    bank1_trim_cfg_3: Bank1TrimCfg3,
    bank1_trim_cfg_2: Bank1TrimCfg2,
    bank1_trim_cfg_1: Bank1TrimCfg1,
    bank1_trim_cfg_0: Bank1TrimCfg0,
    pump_trim_cfg_2: PumpTrimCfg2,
    pump_trim_cfg_1: PumpTrimCfg1,
    pump_trim_cfg_0: PumpTrimCfg0,
    _reserved18: [u8; 0x0f84],
    efuse: Efuse,
    efuseaddr: Efuseaddr,
    dataupper: Dataupper,
    datalower: Datalower,
    efusecfg: Efusecfg,
    efusestat: Efusestat,
    acc: Acc,
    boundary: Boundary,
    efuseflag: Efuseflag,
    efusekey: Efusekey,
    efuserelease: Efuserelease,
    efusepins: Efusepins,
    efusecra: Efusecra,
    efuseread: Efuseread,
    efuseprogram: Efuseprogram,
    efuseerror: Efuseerror,
    singlebit: Singlebit,
    twobit: Twobit,
    selftestcyc: Selftestcyc,
    selftestsign: Selftestsign,
}
impl RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn weprot_b0_31_0_by1(&self) -> &WeprotB0_31_0By1 {
        &self.weprot_b0_31_0_by1
    }
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn weprot_aux_by1(&self) -> &WeprotAuxBy1 {
        &self.weprot_aux_by1
    }
    #[doc = "0x1c - NW and Efuse Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x2c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_size(&self) -> &FlashSize {
        &self.flash_size
    }
    #[doc = "0x3c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fwlock(&self) -> &Fwlock {
        &self.fwlock
    }
    #[doc = "0x40 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fwflag(&self) -> &Fwflag {
        &self.fwflag
    }
    #[doc = "0x50 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn bank0_trim_cfg_3(&self) -> &Bank0TrimCfg3 {
        &self.bank0_trim_cfg_3
    }
    #[doc = "0x54 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn bank0_trim_cfg_2(&self) -> &Bank0TrimCfg2 {
        &self.bank0_trim_cfg_2
    }
    #[doc = "0x58 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn bank0_trim_cfg_1(&self) -> &Bank0TrimCfg1 {
        &self.bank0_trim_cfg_1
    }
    #[doc = "0x5c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn bank0_trim_cfg_0(&self) -> &Bank0TrimCfg0 {
        &self.bank0_trim_cfg_0
    }
    #[doc = "0x60 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn bank1_trim_cfg_3(&self) -> &Bank1TrimCfg3 {
        &self.bank1_trim_cfg_3
    }
    #[doc = "0x64 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn bank1_trim_cfg_2(&self) -> &Bank1TrimCfg2 {
        &self.bank1_trim_cfg_2
    }
    #[doc = "0x68 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn bank1_trim_cfg_1(&self) -> &Bank1TrimCfg1 {
        &self.bank1_trim_cfg_1
    }
    #[doc = "0x6c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn bank1_trim_cfg_0(&self) -> &Bank1TrimCfg0 {
        &self.bank1_trim_cfg_0
    }
    #[doc = "0x70 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn pump_trim_cfg_2(&self) -> &PumpTrimCfg2 {
        &self.pump_trim_cfg_2
    }
    #[doc = "0x74 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn pump_trim_cfg_1(&self) -> &PumpTrimCfg1 {
        &self.pump_trim_cfg_1
    }
    #[doc = "0x78 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn pump_trim_cfg_0(&self) -> &PumpTrimCfg0 {
        &self.pump_trim_cfg_0
    }
    #[doc = "0x1000 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuse(&self) -> &Efuse {
        &self.efuse
    }
    #[doc = "0x1004 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuseaddr(&self) -> &Efuseaddr {
        &self.efuseaddr
    }
    #[doc = "0x1008 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn dataupper(&self) -> &Dataupper {
        &self.dataupper
    }
    #[doc = "0x100c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn datalower(&self) -> &Datalower {
        &self.datalower
    }
    #[doc = "0x1010 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efusecfg(&self) -> &Efusecfg {
        &self.efusecfg
    }
    #[doc = "0x1014 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efusestat(&self) -> &Efusestat {
        &self.efusestat
    }
    #[doc = "0x1018 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn acc(&self) -> &Acc {
        &self.acc
    }
    #[doc = "0x101c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn boundary(&self) -> &Boundary {
        &self.boundary
    }
    #[doc = "0x1020 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuseflag(&self) -> &Efuseflag {
        &self.efuseflag
    }
    #[doc = "0x1024 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efusekey(&self) -> &Efusekey {
        &self.efusekey
    }
    #[doc = "0x1028 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuserelease(&self) -> &Efuserelease {
        &self.efuserelease
    }
    #[doc = "0x102c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efusepins(&self) -> &Efusepins {
        &self.efusepins
    }
    #[doc = "0x1030 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efusecra(&self) -> &Efusecra {
        &self.efusecra
    }
    #[doc = "0x1034 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuseread(&self) -> &Efuseread {
        &self.efuseread
    }
    #[doc = "0x1038 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuseprogram(&self) -> &Efuseprogram {
        &self.efuseprogram
    }
    #[doc = "0x103c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuseerror(&self) -> &Efuseerror {
        &self.efuseerror
    }
    #[doc = "0x1040 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn singlebit(&self) -> &Singlebit {
        &self.singlebit
    }
    #[doc = "0x1044 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn twobit(&self) -> &Twobit {
        &self.twobit
    }
    #[doc = "0x1048 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn selftestcyc(&self) -> &Selftestcyc {
        &self.selftestcyc
    }
    #[doc = "0x104c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn selftestsign(&self) -> &Selftestsign {
        &self.selftestsign
    }
}
#[doc = "WEPROT_B0_31_0_BY1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`weprot_b0_31_0_by1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`weprot_b0_31_0_by1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weprot_b0_31_0_by1`]
module"]
#[doc(alias = "WEPROT_B0_31_0_BY1")]
pub type WeprotB0_31_0By1 = crate::Reg<weprot_b0_31_0_by1::WeprotB0_31_0By1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod weprot_b0_31_0_by1;
#[doc = "WEPROT_AUX_BY1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`weprot_aux_by1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`weprot_aux_by1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weprot_aux_by1`]
module"]
#[doc(alias = "WEPROT_AUX_BY1")]
pub type WeprotAuxBy1 = crate::Reg<weprot_aux_by1::WeprotAuxBy1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod weprot_aux_by1;
#[doc = "STAT (rw) register accessor: NW and Efuse Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "NW and Efuse Status"]
pub mod stat;
#[doc = "CFG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cfg;
#[doc = "FLASH_SIZE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_size`]
module"]
#[doc(alias = "FLASH_SIZE")]
pub type FlashSize = crate::Reg<flash_size::FlashSizeSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_size;
#[doc = "FWLOCK (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwlock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwlock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwlock`]
module"]
#[doc(alias = "FWLOCK")]
pub type Fwlock = crate::Reg<fwlock::FwlockSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwlock;
#[doc = "FWFLAG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwflag`]
module"]
#[doc(alias = "FWFLAG")]
pub type Fwflag = crate::Reg<fwflag::FwflagSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwflag;
#[doc = "BANK0_TRIM_CFG_3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank0_trim_cfg_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank0_trim_cfg_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank0_trim_cfg_3`]
module"]
#[doc(alias = "BANK0_TRIM_CFG_3")]
pub type Bank0TrimCfg3 = crate::Reg<bank0_trim_cfg_3::Bank0TrimCfg3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank0_trim_cfg_3;
#[doc = "BANK0_TRIM_CFG_2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank0_trim_cfg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank0_trim_cfg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank0_trim_cfg_2`]
module"]
#[doc(alias = "BANK0_TRIM_CFG_2")]
pub type Bank0TrimCfg2 = crate::Reg<bank0_trim_cfg_2::Bank0TrimCfg2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank0_trim_cfg_2;
#[doc = "BANK0_TRIM_CFG_1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank0_trim_cfg_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank0_trim_cfg_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank0_trim_cfg_1`]
module"]
#[doc(alias = "BANK0_TRIM_CFG_1")]
pub type Bank0TrimCfg1 = crate::Reg<bank0_trim_cfg_1::Bank0TrimCfg1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank0_trim_cfg_1;
#[doc = "BANK0_TRIM_CFG_0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank0_trim_cfg_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank0_trim_cfg_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank0_trim_cfg_0`]
module"]
#[doc(alias = "BANK0_TRIM_CFG_0")]
pub type Bank0TrimCfg0 = crate::Reg<bank0_trim_cfg_0::Bank0TrimCfg0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank0_trim_cfg_0;
#[doc = "BANK1_TRIM_CFG_3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank1_trim_cfg_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank1_trim_cfg_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank1_trim_cfg_3`]
module"]
#[doc(alias = "BANK1_TRIM_CFG_3")]
pub type Bank1TrimCfg3 = crate::Reg<bank1_trim_cfg_3::Bank1TrimCfg3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank1_trim_cfg_3;
#[doc = "BANK1_TRIM_CFG_2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank1_trim_cfg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank1_trim_cfg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank1_trim_cfg_2`]
module"]
#[doc(alias = "BANK1_TRIM_CFG_2")]
pub type Bank1TrimCfg2 = crate::Reg<bank1_trim_cfg_2::Bank1TrimCfg2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank1_trim_cfg_2;
#[doc = "BANK1_TRIM_CFG_1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank1_trim_cfg_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank1_trim_cfg_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank1_trim_cfg_1`]
module"]
#[doc(alias = "BANK1_TRIM_CFG_1")]
pub type Bank1TrimCfg1 = crate::Reg<bank1_trim_cfg_1::Bank1TrimCfg1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank1_trim_cfg_1;
#[doc = "BANK1_TRIM_CFG_0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank1_trim_cfg_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank1_trim_cfg_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank1_trim_cfg_0`]
module"]
#[doc(alias = "BANK1_TRIM_CFG_0")]
pub type Bank1TrimCfg0 = crate::Reg<bank1_trim_cfg_0::Bank1TrimCfg0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bank1_trim_cfg_0;
#[doc = "PUMP_TRIM_CFG_2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pump_trim_cfg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pump_trim_cfg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pump_trim_cfg_2`]
module"]
#[doc(alias = "PUMP_TRIM_CFG_2")]
pub type PumpTrimCfg2 = crate::Reg<pump_trim_cfg_2::PumpTrimCfg2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod pump_trim_cfg_2;
#[doc = "PUMP_TRIM_CFG_1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pump_trim_cfg_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pump_trim_cfg_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pump_trim_cfg_1`]
module"]
#[doc(alias = "PUMP_TRIM_CFG_1")]
pub type PumpTrimCfg1 = crate::Reg<pump_trim_cfg_1::PumpTrimCfg1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod pump_trim_cfg_1;
#[doc = "PUMP_TRIM_CFG_0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pump_trim_cfg_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pump_trim_cfg_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pump_trim_cfg_0`]
module"]
#[doc(alias = "PUMP_TRIM_CFG_0")]
pub type PumpTrimCfg0 = crate::Reg<pump_trim_cfg_0::PumpTrimCfg0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod pump_trim_cfg_0;
#[doc = "EFUSE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse`]
module"]
#[doc(alias = "EFUSE")]
pub type Efuse = crate::Reg<efuse::EfuseSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuse;
#[doc = "EFUSEADDR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuseaddr`]
module"]
#[doc(alias = "EFUSEADDR")]
pub type Efuseaddr = crate::Reg<efuseaddr::EfuseaddrSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseaddr;
#[doc = "DATAUPPER (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dataupper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dataupper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataupper`]
module"]
#[doc(alias = "DATAUPPER")]
pub type Dataupper = crate::Reg<dataupper::DataupperSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dataupper;
#[doc = "DATALOWER (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datalower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datalower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datalower`]
module"]
#[doc(alias = "DATALOWER")]
pub type Datalower = crate::Reg<datalower::DatalowerSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod datalower;
#[doc = "EFUSECFG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusecfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusecfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efusecfg`]
module"]
#[doc(alias = "EFUSECFG")]
pub type Efusecfg = crate::Reg<efusecfg::EfusecfgSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusecfg;
#[doc = "EFUSESTAT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusestat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusestat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efusestat`]
module"]
#[doc(alias = "EFUSESTAT")]
pub type Efusestat = crate::Reg<efusestat::EfusestatSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusestat;
#[doc = "ACC (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc`]
module"]
#[doc(alias = "ACC")]
pub type Acc = crate::Reg<acc::AccSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod acc;
#[doc = "BOUNDARY (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boundary::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boundary::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boundary`]
module"]
#[doc(alias = "BOUNDARY")]
pub type Boundary = crate::Reg<boundary::BoundarySpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod boundary;
#[doc = "EFUSEFLAG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuseflag`]
module"]
#[doc(alias = "EFUSEFLAG")]
pub type Efuseflag = crate::Reg<efuseflag::EfuseflagSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseflag;
#[doc = "EFUSEKEY (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusekey::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusekey::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efusekey`]
module"]
#[doc(alias = "EFUSEKEY")]
pub type Efusekey = crate::Reg<efusekey::EfusekeySpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusekey;
#[doc = "EFUSERELEASE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuserelease::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuserelease::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuserelease`]
module"]
#[doc(alias = "EFUSERELEASE")]
pub type Efuserelease = crate::Reg<efuserelease::EfusereleaseSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuserelease;
#[doc = "EFUSEPINS (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusepins::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusepins::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efusepins`]
module"]
#[doc(alias = "EFUSEPINS")]
pub type Efusepins = crate::Reg<efusepins::EfusepinsSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusepins;
#[doc = "EFUSECRA (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusecra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusecra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efusecra`]
module"]
#[doc(alias = "EFUSECRA")]
pub type Efusecra = crate::Reg<efusecra::EfusecraSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusecra;
#[doc = "EFUSEREAD (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseread::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseread::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuseread`]
module"]
#[doc(alias = "EFUSEREAD")]
pub type Efuseread = crate::Reg<efuseread::EfusereadSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseread;
#[doc = "EFUSEPROGRAM (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseprogram::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseprogram::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuseprogram`]
module"]
#[doc(alias = "EFUSEPROGRAM")]
pub type Efuseprogram = crate::Reg<efuseprogram::EfuseprogramSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseprogram;
#[doc = "EFUSEERROR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseerror::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseerror::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuseerror`]
module"]
#[doc(alias = "EFUSEERROR")]
pub type Efuseerror = crate::Reg<efuseerror::EfuseerrorSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseerror;
#[doc = "SINGLEBIT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singlebit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlebit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlebit`]
module"]
#[doc(alias = "SINGLEBIT")]
pub type Singlebit = crate::Reg<singlebit::SinglebitSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod singlebit;
#[doc = "TWOBIT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twobit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twobit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twobit`]
module"]
#[doc(alias = "TWOBIT")]
pub type Twobit = crate::Reg<twobit::TwobitSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod twobit;
#[doc = "SELFTESTCYC (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`selftestcyc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`selftestcyc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@selftestcyc`]
module"]
#[doc(alias = "SELFTESTCYC")]
pub type Selftestcyc = crate::Reg<selftestcyc::SelftestcycSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod selftestcyc;
#[doc = "SELFTESTSIGN (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`selftestsign::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`selftestsign::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@selftestsign`]
module"]
#[doc(alias = "SELFTESTSIGN")]
pub type Selftestsign = crate::Reg<selftestsign::SelftestsignSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod selftestsign;

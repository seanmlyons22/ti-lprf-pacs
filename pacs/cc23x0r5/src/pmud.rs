#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    meascfg: Meascfg,
    comptest: Comptest,
    _reserved3: [u8; 0x1c],
    bat: Bat,
    batupd: Batupd,
    temp: Temp,
    tempupd: Tempupd,
    comptestout: Comptestout,
    _reserved8: [u8; 0x0c],
    eventmask: Eventmask,
    event: Event,
    battul: Battul,
    battll: Battll,
    tempul: Tempul,
    templl: Templl,
    _reserved14: [u8; 0x20],
    prefsys: Prefsys,
    _reserved15: [u8; 0x0c],
    preg0: Preg0,
    preg1: Preg1,
    preg2: Preg2,
    dcdccfg: Dcdccfg,
    dcdcstat: Dcdcstat,
    _reserved20: [u8; 0x54],
    dtbbatmon: Dtbbatmon,
    dtbdcdc: Dtbdcdc,
}
impl RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn meascfg(&self) -> &Meascfg {
        &self.meascfg
    }
    #[doc = "0x08 - Enable BATMON comparator test mode. Note: This register is write-protected based on global lock signal from SYS0 on production devices."]
    #[inline(always)]
    pub const fn comptest(&self) -> &Comptest {
        &self.comptest
    }
    #[doc = "0x28 - Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1"]
    #[inline(always)]
    pub const fn bat(&self) -> &Bat {
        &self.bat
    }
    #[doc = "0x2c - Battery Update Indicates BAT Updates"]
    #[inline(always)]
    pub const fn batupd(&self) -> &Batupd {
        &self.batupd
    }
    #[doc = "0x30 - Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1."]
    #[inline(always)]
    pub const fn temp(&self) -> &Temp {
        &self.temp
    }
    #[doc = "0x34 - Temperature Update Indicates TEMP Updates"]
    #[inline(always)]
    pub const fn tempupd(&self) -> &Tempupd {
        &self.tempupd
    }
    #[doc = "0x38 - BATMON comparator test mode output."]
    #[inline(always)]
    pub const fn comptestout(&self) -> &Comptestout {
        &self.comptestout
    }
    #[doc = "0x48 - Event Mask"]
    #[inline(always)]
    pub const fn eventmask(&self) -> &Eventmask {
        &self.eventmask
    }
    #[doc = "0x4c - Event"]
    #[inline(always)]
    pub const fn event(&self) -> &Event {
        &self.event
    }
    #[doc = "0x50 - Battery Upper Limit"]
    #[inline(always)]
    pub const fn battul(&self) -> &Battul {
        &self.battul
    }
    #[doc = "0x54 - Battery Lower Limit"]
    #[inline(always)]
    pub const fn battll(&self) -> &Battll {
        &self.battll
    }
    #[doc = "0x58 - Temperature Upper Limit"]
    #[inline(always)]
    pub const fn tempul(&self) -> &Tempul {
        &self.tempul
    }
    #[doc = "0x5c - Temperature Lower Limit"]
    #[inline(always)]
    pub const fn templl(&self) -> &Templl {
        &self.templl
    }
    #[doc = "0x80 - PMU REFSYS test register. These test bits connect to PMU REFSYS analog module directly. Note: This register is write-protected except for bits \\[3:1\\]
based on global lock signal from SYS0 on production devices."]
    #[inline(always)]
    pub const fn prefsys(&self) -> &Prefsys {
        &self.prefsys
    }
    #[doc = "0x90 - PMU REG 0 register. Note: All bits in this register except UDIG_LDO_EN are write-protected based on global lock signal from SYS0 on production devices."]
    #[inline(always)]
    pub const fn preg0(&self) -> &Preg0 {
        &self.preg0
    }
    #[doc = "0x94 - PMU REG 1 register. Note: All bits in this register except DITHER_EN are write-protected based on global lock signal from SYS0 on production devices."]
    #[inline(always)]
    pub const fn preg1(&self) -> &Preg1 {
        &self.preg1
    }
    #[doc = "0x98 - PMU REG 2 register. Note: This register is write-protected based on global lock signal from SYS0 on production devices."]
    #[inline(always)]
    pub const fn preg2(&self) -> &Preg2 {
        &self.preg2
    }
    #[doc = "0x9c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn dcdccfg(&self) -> &Dcdccfg {
        &self.dcdccfg
    }
    #[doc = "0xa0 - DCDC status register"]
    #[inline(always)]
    pub const fn dcdcstat(&self) -> &Dcdcstat {
        &self.dcdcstat
    }
    #[doc = "0xf8 - BATMON DTB MUX selection signal"]
    #[inline(always)]
    pub const fn dtbbatmon(&self) -> &Dtbbatmon {
        &self.dtbbatmon
    }
    #[doc = "0xfc - DCDC DTB MUX selection signal"]
    #[inline(always)]
    pub const fn dtbdcdc(&self) -> &Dtbdcdc {
        &self.dtbdcdc
    }
}
#[doc = "CTL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ctl;
#[doc = "MEASCFG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`meascfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`meascfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meascfg`]
module"]
#[doc(alias = "MEASCFG")]
pub type Meascfg = crate::Reg<meascfg::MeascfgSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod meascfg;
#[doc = "COMPTEST (rw) register accessor: Enable BATMON comparator test mode. Note: This register is write-protected based on global lock signal from SYS0 on production devices.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comptest::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comptest::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comptest`]
module"]
#[doc(alias = "COMPTEST")]
pub type Comptest = crate::Reg<comptest::ComptestSpec>;
#[doc = "Enable BATMON comparator test mode. Note: This register is write-protected based on global lock signal from SYS0 on production devices."]
pub mod comptest;
#[doc = "BAT (rw) register accessor: Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bat`]
module"]
#[doc(alias = "BAT")]
pub type Bat = crate::Reg<bat::BatSpec>;
#[doc = "Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1"]
pub mod bat;
#[doc = "BATUPD (rw) register accessor: Battery Update Indicates BAT Updates\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batupd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batupd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batupd`]
module"]
#[doc(alias = "BATUPD")]
pub type Batupd = crate::Reg<batupd::BatupdSpec>;
#[doc = "Battery Update Indicates BAT Updates"]
pub mod batupd;
#[doc = "TEMP (rw) register accessor: Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`temp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`temp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp`]
module"]
#[doc(alias = "TEMP")]
pub type Temp = crate::Reg<temp::TempSpec>;
#[doc = "Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1."]
pub mod temp;
#[doc = "TEMPUPD (rw) register accessor: Temperature Update Indicates TEMP Updates\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempupd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tempupd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempupd`]
module"]
#[doc(alias = "TEMPUPD")]
pub type Tempupd = crate::Reg<tempupd::TempupdSpec>;
#[doc = "Temperature Update Indicates TEMP Updates"]
pub mod tempupd;
#[doc = "COMPTESTOUT (rw) register accessor: BATMON comparator test mode output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comptestout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comptestout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comptestout`]
module"]
#[doc(alias = "COMPTESTOUT")]
pub type Comptestout = crate::Reg<comptestout::ComptestoutSpec>;
#[doc = "BATMON comparator test mode output."]
pub mod comptestout;
#[doc = "EVENTMASK (rw) register accessor: Event Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eventmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eventmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventmask`]
module"]
#[doc(alias = "EVENTMASK")]
pub type Eventmask = crate::Reg<eventmask::EventmaskSpec>;
#[doc = "Event Mask"]
pub mod eventmask;
#[doc = "EVENT (rw) register accessor: Event\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event`]
module"]
#[doc(alias = "EVENT")]
pub type Event = crate::Reg<event::EventSpec>;
#[doc = "Event"]
pub mod event;
#[doc = "BATTUL (rw) register accessor: Battery Upper Limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`battul::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`battul::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@battul`]
module"]
#[doc(alias = "BATTUL")]
pub type Battul = crate::Reg<battul::BattulSpec>;
#[doc = "Battery Upper Limit"]
pub mod battul;
#[doc = "BATTLL (rw) register accessor: Battery Lower Limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`battll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`battll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@battll`]
module"]
#[doc(alias = "BATTLL")]
pub type Battll = crate::Reg<battll::BattllSpec>;
#[doc = "Battery Lower Limit"]
pub mod battll;
#[doc = "TEMPUL (rw) register accessor: Temperature Upper Limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempul::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tempul::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempul`]
module"]
#[doc(alias = "TEMPUL")]
pub type Tempul = crate::Reg<tempul::TempulSpec>;
#[doc = "Temperature Upper Limit"]
pub mod tempul;
#[doc = "TEMPLL (rw) register accessor: Temperature Lower Limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`templl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`templl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@templl`]
module"]
#[doc(alias = "TEMPLL")]
pub type Templl = crate::Reg<templl::TempllSpec>;
#[doc = "Temperature Lower Limit"]
pub mod templl;
#[doc = "PREFSYS (rw) register accessor: PMU REFSYS test register. These test bits connect to PMU REFSYS analog module directly. Note: This register is write-protected except for bits \\[3:1\\]
based on global lock signal from SYS0 on production devices.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prefsys::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prefsys::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prefsys`]
module"]
#[doc(alias = "PREFSYS")]
pub type Prefsys = crate::Reg<prefsys::PrefsysSpec>;
#[doc = "PMU REFSYS test register. These test bits connect to PMU REFSYS analog module directly. Note: This register is write-protected except for bits \\[3:1\\]
based on global lock signal from SYS0 on production devices."]
pub mod prefsys;
#[doc = "PREG0 (rw) register accessor: PMU REG 0 register. Note: All bits in this register except UDIG_LDO_EN are write-protected based on global lock signal from SYS0 on production devices.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`preg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`preg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preg0`]
module"]
#[doc(alias = "PREG0")]
pub type Preg0 = crate::Reg<preg0::Preg0Spec>;
#[doc = "PMU REG 0 register. Note: All bits in this register except UDIG_LDO_EN are write-protected based on global lock signal from SYS0 on production devices."]
pub mod preg0;
#[doc = "PREG1 (rw) register accessor: PMU REG 1 register. Note: All bits in this register except DITHER_EN are write-protected based on global lock signal from SYS0 on production devices.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`preg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`preg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preg1`]
module"]
#[doc(alias = "PREG1")]
pub type Preg1 = crate::Reg<preg1::Preg1Spec>;
#[doc = "PMU REG 1 register. Note: All bits in this register except DITHER_EN are write-protected based on global lock signal from SYS0 on production devices."]
pub mod preg1;
#[doc = "PREG2 (rw) register accessor: PMU REG 2 register. Note: This register is write-protected based on global lock signal from SYS0 on production devices.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`preg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`preg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preg2`]
module"]
#[doc(alias = "PREG2")]
pub type Preg2 = crate::Reg<preg2::Preg2Spec>;
#[doc = "PMU REG 2 register. Note: This register is write-protected based on global lock signal from SYS0 on production devices."]
pub mod preg2;
#[doc = "DCDCCFG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdccfg`]
module"]
#[doc(alias = "DCDCCFG")]
pub type Dcdccfg = crate::Reg<dcdccfg::DcdccfgSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dcdccfg;
#[doc = "DCDCSTAT (rw) register accessor: DCDC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdcstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdcstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcstat`]
module"]
#[doc(alias = "DCDCSTAT")]
pub type Dcdcstat = crate::Reg<dcdcstat::DcdcstatSpec>;
#[doc = "DCDC status register"]
pub mod dcdcstat;
#[doc = "DTBBATMON (rw) register accessor: BATMON DTB MUX selection signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtbbatmon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtbbatmon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtbbatmon`]
module"]
#[doc(alias = "DTBBATMON")]
pub type Dtbbatmon = crate::Reg<dtbbatmon::DtbbatmonSpec>;
#[doc = "BATMON DTB MUX selection signal"]
pub mod dtbbatmon;
#[doc = "DTBDCDC (rw) register accessor: DCDC DTB MUX selection signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtbdcdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtbdcdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtbdcdc`]
module"]
#[doc(alias = "DTBDCDC")]
pub type Dtbdcdc = crate::Reg<dtbdcdc::DtbdcdcSpec>;
#[doc = "DCDC DTB MUX selection signal"]
pub mod dtbdcdc;

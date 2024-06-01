#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    out0: Out0,
    out1: Out1,
    irqflagstat: Irqflagstat,
    irqflagmask: Irqflagmask,
    irqflagclr: Irqflagclr,
    ctl: Ctl,
    cfg0: Cfg0,
    alarmcnt: Alarmcnt,
    froen: Froen,
    frodetune: Frodetune,
    alarmmask: Alarmmask,
    alarmstop: Alarmstop,
    lfsr0: Lfsr0,
    lfsr1: Lfsr1,
    lfsr2: Lfsr2,
    _reserved15: [u8; 0x3c],
    hwopt: Hwopt,
    hwver0: Hwver0,
    _reserved17: [u8; 0x1f58],
    irqstatmask: Irqstatmask,
    _reserved18: [u8; 0x04],
    hwver1: Hwver1,
    _reserved19: [u8; 0x08],
    irqset: Irqset,
    swreset: Swreset,
    _reserved21: [u8; 0x04],
    irqstat: Irqstat,
}
impl RegisterBlock {
    #[doc = "0x00 - Random Number Lower Word Readout Value"]
    #[inline(always)]
    pub const fn out0(&self) -> &Out0 {
        &self.out0
    }
    #[doc = "0x04 - Random Number Upper Word Readout Value"]
    #[inline(always)]
    pub const fn out1(&self) -> &Out1 {
        &self.out1
    }
    #[doc = "0x08 - Interrupt Status"]
    #[inline(always)]
    pub const fn irqflagstat(&self) -> &Irqflagstat {
        &self.irqflagstat
    }
    #[doc = "0x0c - Interrupt Mask"]
    #[inline(always)]
    pub const fn irqflagmask(&self) -> &Irqflagmask {
        &self.irqflagmask
    }
    #[doc = "0x10 - Interrupt Flag Clear"]
    #[inline(always)]
    pub const fn irqflagclr(&self) -> &Irqflagclr {
        &self.irqflagclr
    }
    #[doc = "0x14 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x18 - Configuration 0"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x1c - Alarm Control"]
    #[inline(always)]
    pub const fn alarmcnt(&self) -> &Alarmcnt {
        &self.alarmcnt
    }
    #[doc = "0x20 - FRO Enable"]
    #[inline(always)]
    pub const fn froen(&self) -> &Froen {
        &self.froen
    }
    #[doc = "0x24 - FRO De-tune Bit"]
    #[inline(always)]
    pub const fn frodetune(&self) -> &Frodetune {
        &self.frodetune
    }
    #[doc = "0x28 - Alarm Event"]
    #[inline(always)]
    pub const fn alarmmask(&self) -> &Alarmmask {
        &self.alarmmask
    }
    #[doc = "0x2c - Alarm Shutdown"]
    #[inline(always)]
    pub const fn alarmstop(&self) -> &Alarmstop {
        &self.alarmstop
    }
    #[doc = "0x30 - LFSR Readout Value"]
    #[inline(always)]
    pub const fn lfsr0(&self) -> &Lfsr0 {
        &self.lfsr0
    }
    #[doc = "0x34 - LFSR Readout Value"]
    #[inline(always)]
    pub const fn lfsr1(&self) -> &Lfsr1 {
        &self.lfsr1
    }
    #[doc = "0x38 - LFSR Readout Value"]
    #[inline(always)]
    pub const fn lfsr2(&self) -> &Lfsr2 {
        &self.lfsr2
    }
    #[doc = "0x78 - TRNG Engine Options Information"]
    #[inline(always)]
    pub const fn hwopt(&self) -> &Hwopt {
        &self.hwopt
    }
    #[doc = "0x7c - HW Version 0 EIP Number And Core Revision"]
    #[inline(always)]
    pub const fn hwver0(&self) -> &Hwver0 {
        &self.hwver0
    }
    #[doc = "0x1fd8 - Interrupt Status After Masking"]
    #[inline(always)]
    pub const fn irqstatmask(&self) -> &Irqstatmask {
        &self.irqstatmask
    }
    #[doc = "0x1fe0 - HW Version 1 TRNG Revision Number"]
    #[inline(always)]
    pub const fn hwver1(&self) -> &Hwver1 {
        &self.hwver1
    }
    #[doc = "0x1fec - Interrupt Set"]
    #[inline(always)]
    pub const fn irqset(&self) -> &Irqset {
        &self.irqset
    }
    #[doc = "0x1ff0 - SW Reset Control"]
    #[inline(always)]
    pub const fn swreset(&self) -> &Swreset {
        &self.swreset
    }
    #[doc = "0x1ff8 - Interrupt Status"]
    #[inline(always)]
    pub const fn irqstat(&self) -> &Irqstat {
        &self.irqstat
    }
}
#[doc = "OUT0 (rw) register accessor: Random Number Lower Word Readout Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out0`]
module"]
#[doc(alias = "OUT0")]
pub type Out0 = crate::Reg<out0::Out0Spec>;
#[doc = "Random Number Lower Word Readout Value"]
pub mod out0;
#[doc = "OUT1 (rw) register accessor: Random Number Upper Word Readout Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1`]
module"]
#[doc(alias = "OUT1")]
pub type Out1 = crate::Reg<out1::Out1Spec>;
#[doc = "Random Number Upper Word Readout Value"]
pub mod out1;
#[doc = "IRQFLAGSTAT (rw) register accessor: Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqflagstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqflagstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqflagstat`]
module"]
#[doc(alias = "IRQFLAGSTAT")]
pub type Irqflagstat = crate::Reg<irqflagstat::IrqflagstatSpec>;
#[doc = "Interrupt Status"]
pub mod irqflagstat;
#[doc = "IRQFLAGMASK (rw) register accessor: Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqflagmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqflagmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqflagmask`]
module"]
#[doc(alias = "IRQFLAGMASK")]
pub type Irqflagmask = crate::Reg<irqflagmask::IrqflagmaskSpec>;
#[doc = "Interrupt Mask"]
pub mod irqflagmask;
#[doc = "IRQFLAGCLR (rw) register accessor: Interrupt Flag Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqflagclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqflagclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqflagclr`]
module"]
#[doc(alias = "IRQFLAGCLR")]
pub type Irqflagclr = crate::Reg<irqflagclr::IrqflagclrSpec>;
#[doc = "Interrupt Flag Clear"]
pub mod irqflagclr;
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control"]
pub mod ctl;
#[doc = "CFG0 (rw) register accessor: Configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`]
module"]
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::Cfg0Spec>;
#[doc = "Configuration 0"]
pub mod cfg0;
#[doc = "ALARMCNT (rw) register accessor: Alarm Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarmcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarmcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarmcnt`]
module"]
#[doc(alias = "ALARMCNT")]
pub type Alarmcnt = crate::Reg<alarmcnt::AlarmcntSpec>;
#[doc = "Alarm Control"]
pub mod alarmcnt;
#[doc = "FROEN (rw) register accessor: FRO Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`froen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`froen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@froen`]
module"]
#[doc(alias = "FROEN")]
pub type Froen = crate::Reg<froen::FroenSpec>;
#[doc = "FRO Enable"]
pub mod froen;
#[doc = "FRODETUNE (rw) register accessor: FRO De-tune Bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frodetune::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frodetune::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frodetune`]
module"]
#[doc(alias = "FRODETUNE")]
pub type Frodetune = crate::Reg<frodetune::FrodetuneSpec>;
#[doc = "FRO De-tune Bit"]
pub mod frodetune;
#[doc = "ALARMMASK (rw) register accessor: Alarm Event\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarmmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarmmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarmmask`]
module"]
#[doc(alias = "ALARMMASK")]
pub type Alarmmask = crate::Reg<alarmmask::AlarmmaskSpec>;
#[doc = "Alarm Event"]
pub mod alarmmask;
#[doc = "ALARMSTOP (rw) register accessor: Alarm Shutdown\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarmstop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarmstop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarmstop`]
module"]
#[doc(alias = "ALARMSTOP")]
pub type Alarmstop = crate::Reg<alarmstop::AlarmstopSpec>;
#[doc = "Alarm Shutdown"]
pub mod alarmstop;
#[doc = "LFSR0 (rw) register accessor: LFSR Readout Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfsr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfsr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfsr0`]
module"]
#[doc(alias = "LFSR0")]
pub type Lfsr0 = crate::Reg<lfsr0::Lfsr0Spec>;
#[doc = "LFSR Readout Value"]
pub mod lfsr0;
#[doc = "LFSR1 (rw) register accessor: LFSR Readout Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfsr1`]
module"]
#[doc(alias = "LFSR1")]
pub type Lfsr1 = crate::Reg<lfsr1::Lfsr1Spec>;
#[doc = "LFSR Readout Value"]
pub mod lfsr1;
#[doc = "LFSR2 (rw) register accessor: LFSR Readout Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfsr2`]
module"]
#[doc(alias = "LFSR2")]
pub type Lfsr2 = crate::Reg<lfsr2::Lfsr2Spec>;
#[doc = "LFSR Readout Value"]
pub mod lfsr2;
#[doc = "HWOPT (rw) register accessor: TRNG Engine Options Information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwopt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwopt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwopt`]
module"]
#[doc(alias = "HWOPT")]
pub type Hwopt = crate::Reg<hwopt::HwoptSpec>;
#[doc = "TRNG Engine Options Information"]
pub mod hwopt;
#[doc = "HWVER0 (rw) register accessor: HW Version 0 EIP Number And Core Revision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwver0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwver0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwver0`]
module"]
#[doc(alias = "HWVER0")]
pub type Hwver0 = crate::Reg<hwver0::Hwver0Spec>;
#[doc = "HW Version 0 EIP Number And Core Revision"]
pub mod hwver0;
#[doc = "IRQSTATMASK (rw) register accessor: Interrupt Status After Masking\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqstatmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqstatmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqstatmask`]
module"]
#[doc(alias = "IRQSTATMASK")]
pub type Irqstatmask = crate::Reg<irqstatmask::IrqstatmaskSpec>;
#[doc = "Interrupt Status After Masking"]
pub mod irqstatmask;
#[doc = "HWVER1 (rw) register accessor: HW Version 1 TRNG Revision Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwver1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwver1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwver1`]
module"]
#[doc(alias = "HWVER1")]
pub type Hwver1 = crate::Reg<hwver1::Hwver1Spec>;
#[doc = "HW Version 1 TRNG Revision Number"]
pub mod hwver1;
#[doc = "IRQSET (rw) register accessor: Interrupt Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqset`]
module"]
#[doc(alias = "IRQSET")]
pub type Irqset = crate::Reg<irqset::IrqsetSpec>;
#[doc = "Interrupt Set"]
pub mod irqset;
#[doc = "SWRESET (rw) register accessor: SW Reset Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreset`]
module"]
#[doc(alias = "SWRESET")]
pub type Swreset = crate::Reg<swreset::SwresetSpec>;
#[doc = "SW Reset Control"]
pub mod swreset;
#[doc = "IRQSTAT (rw) register accessor: Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqstat`]
module"]
#[doc(alias = "IRQSTAT")]
pub type Irqstat = crate::Reg<irqstat::IrqstatSpec>;
#[doc = "Interrupt Status"]
pub mod irqstat;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    stat: Stat,
    result: Result,
    satcfg: Satcfg,
    trigsrc: Trigsrc,
    trigcnt: Trigcnt,
    trigcntload: Trigcntload,
    trigcntcfg: Trigcntcfg,
    prectl: Prectl,
    precnt: Precnt,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x08 - Result Result of last TDC conversion"]
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
    #[doc = "0x0c - Saturation Configuration"]
    #[inline(always)]
    pub const fn satcfg(&self) -> &Satcfg {
        &self.satcfg
    }
    #[doc = "0x10 - Trigger Source Select source and polarity for TDC start and stop events. See the Technical Reference Manual for event timing requirements."]
    #[inline(always)]
    pub const fn trigsrc(&self) -> &Trigsrc {
        &self.trigsrc
    }
    #[doc = "0x14 - Trigger Counter Stop-counter control and status."]
    #[inline(always)]
    pub const fn trigcnt(&self) -> &Trigcnt {
        &self.trigcnt
    }
    #[doc = "0x18 - Trigger Counter Load Stop-counter load."]
    #[inline(always)]
    pub const fn trigcntload(&self) -> &Trigcntload {
        &self.trigcntload
    }
    #[doc = "0x1c - Trigger Counter Configuration Stop-counter configuration."]
    #[inline(always)]
    pub const fn trigcntcfg(&self) -> &Trigcntcfg {
        &self.trigcntcfg
    }
    #[doc = "0x20 - Prescaler Control The prescaler can be used to count events that are faster than the AUX clock frequency. It can be used to: - count pulses on a specified event from the asynchronous event bus. - prescale a specified event from the asynchronous event bus. To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE. It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX clock frequency."]
    #[inline(always)]
    pub const fn prectl(&self) -> &Prectl {
        &self.prectl
    }
    #[doc = "0x24 - Prescaler Counter"]
    #[inline(always)]
    pub const fn precnt(&self) -> &Precnt {
        &self.precnt
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STAT (rw) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status"]
pub mod stat;
#[doc = "RESULT (rw) register accessor: Result Result of last TDC conversion\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`result::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`]
module"]
#[doc(alias = "RESULT")]
pub type Result = crate::Reg<result::ResultSpec>;
#[doc = "Result Result of last TDC conversion"]
pub mod result;
#[doc = "SATCFG (rw) register accessor: Saturation Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`satcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`satcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@satcfg`]
module"]
#[doc(alias = "SATCFG")]
pub type Satcfg = crate::Reg<satcfg::SatcfgSpec>;
#[doc = "Saturation Configuration"]
pub mod satcfg;
#[doc = "TRIGSRC (rw) register accessor: Trigger Source Select source and polarity for TDC start and stop events. See the Technical Reference Manual for event timing requirements.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigsrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigsrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigsrc`]
module"]
#[doc(alias = "TRIGSRC")]
pub type Trigsrc = crate::Reg<trigsrc::TrigsrcSpec>;
#[doc = "Trigger Source Select source and polarity for TDC start and stop events. See the Technical Reference Manual for event timing requirements."]
pub mod trigsrc;
#[doc = "TRIGCNT (rw) register accessor: Trigger Counter Stop-counter control and status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigcnt`]
module"]
#[doc(alias = "TRIGCNT")]
pub type Trigcnt = crate::Reg<trigcnt::TrigcntSpec>;
#[doc = "Trigger Counter Stop-counter control and status."]
pub mod trigcnt;
#[doc = "TRIGCNTLOAD (rw) register accessor: Trigger Counter Load Stop-counter load.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigcntload::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigcntload::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigcntload`]
module"]
#[doc(alias = "TRIGCNTLOAD")]
pub type Trigcntload = crate::Reg<trigcntload::TrigcntloadSpec>;
#[doc = "Trigger Counter Load Stop-counter load."]
pub mod trigcntload;
#[doc = "TRIGCNTCFG (rw) register accessor: Trigger Counter Configuration Stop-counter configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigcntcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigcntcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigcntcfg`]
module"]
#[doc(alias = "TRIGCNTCFG")]
pub type Trigcntcfg = crate::Reg<trigcntcfg::TrigcntcfgSpec>;
#[doc = "Trigger Counter Configuration Stop-counter configuration."]
pub mod trigcntcfg;
#[doc = "PRECTL (rw) register accessor: Prescaler Control The prescaler can be used to count events that are faster than the AUX clock frequency. It can be used to: - count pulses on a specified event from the asynchronous event bus. - prescale a specified event from the asynchronous event bus. To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE. It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX clock frequency.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prectl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prectl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prectl`]
module"]
#[doc(alias = "PRECTL")]
pub type Prectl = crate::Reg<prectl::PrectlSpec>;
#[doc = "Prescaler Control The prescaler can be used to count events that are faster than the AUX clock frequency. It can be used to: - count pulses on a specified event from the asynchronous event bus. - prescale a specified event from the asynchronous event bus. To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE. It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX clock frequency."]
pub mod prectl;
#[doc = "PRECNT (rw) register accessor: Prescaler Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`precnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`precnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@precnt`]
module"]
#[doc(alias = "PRECNT")]
pub type Precnt = crate::Reg<precnt::PrecntSpec>;
#[doc = "Prescaler Counter"]
pub mod precnt;

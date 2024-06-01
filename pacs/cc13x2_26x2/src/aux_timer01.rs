#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    t0cfg: T0cfg,
    t0ctl: T0ctl,
    t0target: T0target,
    t0cntr: T0cntr,
    t1cfg: T1cfg,
    t1ctl: T1ctl,
    t1target: T1target,
    t1cntr: T1cntr,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer 0 Configuration"]
    #[inline(always)]
    pub const fn t0cfg(&self) -> &T0cfg {
        &self.t0cfg
    }
    #[doc = "0x04 - Timer 0 Control"]
    #[inline(always)]
    pub const fn t0ctl(&self) -> &T0ctl {
        &self.t0ctl
    }
    #[doc = "0x08 - Timer 0 Target"]
    #[inline(always)]
    pub const fn t0target(&self) -> &T0target {
        &self.t0target
    }
    #[doc = "0x0c - Timer 0 Counter"]
    #[inline(always)]
    pub const fn t0cntr(&self) -> &T0cntr {
        &self.t0cntr
    }
    #[doc = "0x10 - Timer 1 Configuration"]
    #[inline(always)]
    pub const fn t1cfg(&self) -> &T1cfg {
        &self.t1cfg
    }
    #[doc = "0x14 - Timer 1 Control"]
    #[inline(always)]
    pub const fn t1ctl(&self) -> &T1ctl {
        &self.t1ctl
    }
    #[doc = "0x18 - Timer 1 Target Timer 1 counter target value"]
    #[inline(always)]
    pub const fn t1target(&self) -> &T1target {
        &self.t1target
    }
    #[doc = "0x1c - Timer 1 Counter"]
    #[inline(always)]
    pub const fn t1cntr(&self) -> &T1cntr {
        &self.t1cntr
    }
}
#[doc = "T0CFG (rw) register accessor: Timer 0 Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0cfg`]
module"]
#[doc(alias = "T0CFG")]
pub type T0cfg = crate::Reg<t0cfg::T0cfgSpec>;
#[doc = "Timer 0 Configuration"]
pub mod t0cfg;
#[doc = "T0CTL (rw) register accessor: Timer 0 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0ctl`]
module"]
#[doc(alias = "T0CTL")]
pub type T0ctl = crate::Reg<t0ctl::T0ctlSpec>;
#[doc = "Timer 0 Control"]
pub mod t0ctl;
#[doc = "T0TARGET (rw) register accessor: Timer 0 Target\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0target::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0target::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0target`]
module"]
#[doc(alias = "T0TARGET")]
pub type T0target = crate::Reg<t0target::T0targetSpec>;
#[doc = "Timer 0 Target"]
pub mod t0target;
#[doc = "T0CNTR (rw) register accessor: Timer 0 Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0cntr`]
module"]
#[doc(alias = "T0CNTR")]
pub type T0cntr = crate::Reg<t0cntr::T0cntrSpec>;
#[doc = "Timer 0 Counter"]
pub mod t0cntr;
#[doc = "T1CFG (rw) register accessor: Timer 1 Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1cfg`]
module"]
#[doc(alias = "T1CFG")]
pub type T1cfg = crate::Reg<t1cfg::T1cfgSpec>;
#[doc = "Timer 1 Configuration"]
pub mod t1cfg;
#[doc = "T1CTL (rw) register accessor: Timer 1 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1ctl`]
module"]
#[doc(alias = "T1CTL")]
pub type T1ctl = crate::Reg<t1ctl::T1ctlSpec>;
#[doc = "Timer 1 Control"]
pub mod t1ctl;
#[doc = "T1TARGET (rw) register accessor: Timer 1 Target Timer 1 counter target value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1target::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1target::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1target`]
module"]
#[doc(alias = "T1TARGET")]
pub type T1target = crate::Reg<t1target::T1targetSpec>;
#[doc = "Timer 1 Target Timer 1 counter target value"]
pub mod t1target;
#[doc = "T1CNTR (rw) register accessor: Timer 1 Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1cntr`]
module"]
#[doc(alias = "T1CNTR")]
pub type T1cntr = crate::Reg<t1cntr::T1cntrSpec>;
#[doc = "Timer 1 Counter"]
pub mod t1cntr;

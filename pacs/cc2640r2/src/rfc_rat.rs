#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ratcnt: Ratcnt,
    _reserved1: [u8; 0x78],
    ratch0val: Ratch0val,
    ratch1val: Ratch1val,
    ratch2val: Ratch2val,
    ratch3val: Ratch3val,
    ratch4val: Ratch4val,
    ratch5val: Ratch5val,
    ratch6val: Ratch6val,
    ratch7val: Ratch7val,
}
impl RegisterBlock {
    #[doc = "0x04 - Radio Timer Counter Value"]
    #[inline(always)]
    pub const fn ratcnt(&self) -> &Ratcnt {
        &self.ratcnt
    }
    #[doc = "0x80 - Timer Channel 0 Capture/Compare Register"]
    #[inline(always)]
    pub const fn ratch0val(&self) -> &Ratch0val {
        &self.ratch0val
    }
    #[doc = "0x84 - Timer Channel 1 Capture/Compare Register"]
    #[inline(always)]
    pub const fn ratch1val(&self) -> &Ratch1val {
        &self.ratch1val
    }
    #[doc = "0x88 - Timer Channel 2 Capture/Compare Register"]
    #[inline(always)]
    pub const fn ratch2val(&self) -> &Ratch2val {
        &self.ratch2val
    }
    #[doc = "0x8c - Timer Channel 3 Capture/Compare Register"]
    #[inline(always)]
    pub const fn ratch3val(&self) -> &Ratch3val {
        &self.ratch3val
    }
    #[doc = "0x90 - Timer Channel 4 Capture/Compare Register"]
    #[inline(always)]
    pub const fn ratch4val(&self) -> &Ratch4val {
        &self.ratch4val
    }
    #[doc = "0x94 - Timer Channel 5 Capture/Compare Register"]
    #[inline(always)]
    pub const fn ratch5val(&self) -> &Ratch5val {
        &self.ratch5val
    }
    #[doc = "0x98 - Timer Channel 6 Capture/Compare Register"]
    #[inline(always)]
    pub const fn ratch6val(&self) -> &Ratch6val {
        &self.ratch6val
    }
    #[doc = "0x9c - Timer Channel 7 Capture/Compare Register"]
    #[inline(always)]
    pub const fn ratch7val(&self) -> &Ratch7val {
        &self.ratch7val
    }
}
#[doc = "RATCNT (rw) register accessor: Radio Timer Counter Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ratcnt`]
module"]
#[doc(alias = "RATCNT")]
pub type Ratcnt = crate::Reg<ratcnt::RatcntSpec>;
#[doc = "Radio Timer Counter Value"]
pub mod ratcnt;
#[doc = "RATCH0VAL (rw) register accessor: Timer Channel 0 Capture/Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratch0val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratch0val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ratch0val`]
module"]
#[doc(alias = "RATCH0VAL")]
pub type Ratch0val = crate::Reg<ratch0val::Ratch0valSpec>;
#[doc = "Timer Channel 0 Capture/Compare Register"]
pub mod ratch0val;
#[doc = "RATCH1VAL (rw) register accessor: Timer Channel 1 Capture/Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratch1val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratch1val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ratch1val`]
module"]
#[doc(alias = "RATCH1VAL")]
pub type Ratch1val = crate::Reg<ratch1val::Ratch1valSpec>;
#[doc = "Timer Channel 1 Capture/Compare Register"]
pub mod ratch1val;
#[doc = "RATCH2VAL (rw) register accessor: Timer Channel 2 Capture/Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratch2val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratch2val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ratch2val`]
module"]
#[doc(alias = "RATCH2VAL")]
pub type Ratch2val = crate::Reg<ratch2val::Ratch2valSpec>;
#[doc = "Timer Channel 2 Capture/Compare Register"]
pub mod ratch2val;
#[doc = "RATCH3VAL (rw) register accessor: Timer Channel 3 Capture/Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratch3val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratch3val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ratch3val`]
module"]
#[doc(alias = "RATCH3VAL")]
pub type Ratch3val = crate::Reg<ratch3val::Ratch3valSpec>;
#[doc = "Timer Channel 3 Capture/Compare Register"]
pub mod ratch3val;
#[doc = "RATCH4VAL (rw) register accessor: Timer Channel 4 Capture/Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratch4val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratch4val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ratch4val`]
module"]
#[doc(alias = "RATCH4VAL")]
pub type Ratch4val = crate::Reg<ratch4val::Ratch4valSpec>;
#[doc = "Timer Channel 4 Capture/Compare Register"]
pub mod ratch4val;
#[doc = "RATCH5VAL (rw) register accessor: Timer Channel 5 Capture/Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratch5val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratch5val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ratch5val`]
module"]
#[doc(alias = "RATCH5VAL")]
pub type Ratch5val = crate::Reg<ratch5val::Ratch5valSpec>;
#[doc = "Timer Channel 5 Capture/Compare Register"]
pub mod ratch5val;
#[doc = "RATCH6VAL (rw) register accessor: Timer Channel 6 Capture/Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratch6val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratch6val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ratch6val`]
module"]
#[doc(alias = "RATCH6VAL")]
pub type Ratch6val = crate::Reg<ratch6val::Ratch6valSpec>;
#[doc = "Timer Channel 6 Capture/Compare Register"]
pub mod ratch6val;
#[doc = "RATCH7VAL (rw) register accessor: Timer Channel 7 Capture/Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratch7val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratch7val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ratch7val`]
module"]
#[doc(alias = "RATCH7VAL")]
pub type Ratch7val = crate::Reg<ratch7val::Ratch7valSpec>;
#[doc = "Timer Channel 7 Capture/Compare Register"]
pub mod ratch7val;

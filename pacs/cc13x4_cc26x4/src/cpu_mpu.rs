#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    type_: Type,
    ctrl: Ctrl,
    rnr: Rnr,
    rbar: Rbar,
    rlar: Rlar,
    rbar_a1: RbarA1,
    rlar_a1: RlarA1,
    rbar_a2: RbarA2,
    rlar_a2: RlarA2,
    rbar_a3: RbarA3,
    rlar_a3: RlarA3,
    _reserved11: [u8; 0x04],
    mair0: Mair0,
    _reserved12: [u8; 0x08],
    mair1: Mair1,
}
impl RegisterBlock {
    #[doc = "0x00 - The MPU Type Register indicates how many regions the MPU supports"]
    #[inline(always)]
    pub const fn type_(&self) -> &Type {
        &self.type_
    }
    #[doc = "0x04 - Enables the MPU and, when the MPU is enabled, controls whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - Selects the region currently accessed by MPU_RBAR and MPU_RLAR"]
    #[inline(always)]
    pub const fn rnr(&self) -> &Rnr {
        &self.rnr
    }
    #[doc = "0x0c - Provides indirect read and write access to the base address of the currently selected MPU region"]
    #[inline(always)]
    pub const fn rbar(&self) -> &Rbar {
        &self.rbar
    }
    #[doc = "0x10 - Provides indirect read and write access to the limit address of the currently selected MPU region"]
    #[inline(always)]
    pub const fn rlar(&self) -> &Rlar {
        &self.rlar
    }
    #[doc = "0x14 - Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\])"]
    #[inline(always)]
    pub const fn rbar_a1(&self) -> &RbarA1 {
        &self.rbar_a1
    }
    #[doc = "0x18 - Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\])"]
    #[inline(always)]
    pub const fn rlar_a1(&self) -> &RlarA1 {
        &self.rlar_a1
    }
    #[doc = "0x1c - Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\])"]
    #[inline(always)]
    pub const fn rbar_a2(&self) -> &RbarA2 {
        &self.rbar_a2
    }
    #[doc = "0x20 - Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\])"]
    #[inline(always)]
    pub const fn rlar_a2(&self) -> &RlarA2 {
        &self.rlar_a2
    }
    #[doc = "0x24 - Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\])"]
    #[inline(always)]
    pub const fn rbar_a3(&self) -> &RbarA3 {
        &self.rbar_a3
    }
    #[doc = "0x28 - Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\])"]
    #[inline(always)]
    pub const fn rlar_a3(&self) -> &RlarA3 {
        &self.rlar_a3
    }
    #[doc = "0x30 - Along with MPU_MAIR1, provides the memory attribute encodings corresponding to the AttrIndex values"]
    #[inline(always)]
    pub const fn mair0(&self) -> &Mair0 {
        &self.mair0
    }
    #[doc = "0x3c - Along with MPU_MAIR0, provides the memory attribute encodings corresponding to the AttrIndex values"]
    #[inline(always)]
    pub const fn mair1(&self) -> &Mair1 {
        &self.mair1
    }
}
#[doc = "TYPE (rw) register accessor: The MPU Type Register indicates how many regions the MPU supports\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`type_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`type_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@type_`]
module"]
#[doc(alias = "TYPE")]
pub type Type = crate::Reg<type_::TypeSpec>;
#[doc = "The MPU Type Register indicates how many regions the MPU supports"]
pub mod type_;
#[doc = "CTRL (rw) register accessor: Enables the MPU and, when the MPU is enabled, controls whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Enables the MPU and, when the MPU is enabled, controls whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
pub mod ctrl;
#[doc = "RNR (rw) register accessor: Selects the region currently accessed by MPU_RBAR and MPU_RLAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnr`]
module"]
#[doc(alias = "RNR")]
pub type Rnr = crate::Reg<rnr::RnrSpec>;
#[doc = "Selects the region currently accessed by MPU_RBAR and MPU_RLAR"]
pub mod rnr;
#[doc = "RBAR (rw) register accessor: Provides indirect read and write access to the base address of the currently selected MPU region\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbar`]
module"]
#[doc(alias = "RBAR")]
pub type Rbar = crate::Reg<rbar::RbarSpec>;
#[doc = "Provides indirect read and write access to the base address of the currently selected MPU region"]
pub mod rbar;
#[doc = "RLAR (rw) register accessor: Provides indirect read and write access to the limit address of the currently selected MPU region\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlar`]
module"]
#[doc(alias = "RLAR")]
pub type Rlar = crate::Reg<rlar::RlarSpec>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region"]
pub mod rlar;
#[doc = "RBAR_A1 (rw) register accessor: Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\])\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbar_a1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbar_a1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbar_a1`]
module"]
#[doc(alias = "RBAR_A1")]
pub type RbarA1 = crate::Reg<rbar_a1::RbarA1Spec>;
#[doc = "Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\])"]
pub mod rbar_a1;
#[doc = "RLAR_A1 (rw) register accessor: Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\])\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlar_a1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rlar_a1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlar_a1`]
module"]
#[doc(alias = "RLAR_A1")]
pub type RlarA1 = crate::Reg<rlar_a1::RlarA1Spec>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\])"]
pub mod rlar_a1;
#[doc = "RBAR_A2 (rw) register accessor: Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\])\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbar_a2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbar_a2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbar_a2`]
module"]
#[doc(alias = "RBAR_A2")]
pub type RbarA2 = crate::Reg<rbar_a2::RbarA2Spec>;
#[doc = "Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\])"]
pub mod rbar_a2;
#[doc = "RLAR_A2 (rw) register accessor: Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\])\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlar_a2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rlar_a2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlar_a2`]
module"]
#[doc(alias = "RLAR_A2")]
pub type RlarA2 = crate::Reg<rlar_a2::RlarA2Spec>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\])"]
pub mod rlar_a2;
#[doc = "RBAR_A3 (rw) register accessor: Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\])\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbar_a3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbar_a3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbar_a3`]
module"]
#[doc(alias = "RBAR_A3")]
pub type RbarA3 = crate::Reg<rbar_a3::RbarA3Spec>;
#[doc = "Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\])"]
pub mod rbar_a3;
#[doc = "RLAR_A3 (rw) register accessor: Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\])\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlar_a3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rlar_a3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlar_a3`]
module"]
#[doc(alias = "RLAR_A3")]
pub type RlarA3 = crate::Reg<rlar_a3::RlarA3Spec>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\])"]
pub mod rlar_a3;
#[doc = "MAIR0 (rw) register accessor: Along with MPU_MAIR1, provides the memory attribute encodings corresponding to the AttrIndex values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mair0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mair0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mair0`]
module"]
#[doc(alias = "MAIR0")]
pub type Mair0 = crate::Reg<mair0::Mair0Spec>;
#[doc = "Along with MPU_MAIR1, provides the memory attribute encodings corresponding to the AttrIndex values"]
pub mod mair0;
#[doc = "MAIR1 (rw) register accessor: Along with MPU_MAIR0, provides the memory attribute encodings corresponding to the AttrIndex values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mair1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mair1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mair1`]
module"]
#[doc(alias = "MAIR1")]
pub type Mair1 = crate::Reg<mair1::Mair1Spec>;
#[doc = "Along with MPU_MAIR0, provides the memory attribute encodings corresponding to the AttrIndex values"]
pub mod mair1;

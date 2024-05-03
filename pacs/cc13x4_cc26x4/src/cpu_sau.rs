#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    type_: Type,
    rnr: Rnr,
    rbar: Rbar,
    rlar: Rlar,
    sfsr: Sfsr,
    sfar: Sfar,
}
impl RegisterBlock {
    #[doc = "0x00 - Allows enabling of the Security Attribution Unit"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Indicates the number of regions implemented by the Security Attribution Unit"]
    #[inline(always)]
    pub const fn type_(&self) -> &Type {
        &self.type_
    }
    #[doc = "0x08 - Selects the region currently accessed by SAU_RBAR and SAU_RLAR"]
    #[inline(always)]
    pub const fn rnr(&self) -> &Rnr {
        &self.rnr
    }
    #[doc = "0x0c - Provides indirect read and write access to the base address of the currently selected SAU region"]
    #[inline(always)]
    pub const fn rbar(&self) -> &Rbar {
        &self.rbar
    }
    #[doc = "0x10 - Provides indirect read and write access to the limit address of the currently selected SAU region"]
    #[inline(always)]
    pub const fn rlar(&self) -> &Rlar {
        &self.rlar
    }
    #[doc = "0x14 - Provides information about any security related faults"]
    #[inline(always)]
    pub const fn sfsr(&self) -> &Sfsr {
        &self.sfsr
    }
    #[doc = "0x18 - Shows the address of the memory location that caused a Security violation"]
    #[inline(always)]
    pub const fn sfar(&self) -> &Sfar {
        &self.sfar
    }
}
#[doc = "CTRL (rw) register accessor: Allows enabling of the Security Attribution Unit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Allows enabling of the Security Attribution Unit"]
pub mod ctrl;
#[doc = "TYPE (rw) register accessor: Indicates the number of regions implemented by the Security Attribution Unit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`type_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`type_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@type_`]
module"]
#[doc(alias = "TYPE")]
pub type Type = crate::Reg<type_::TypeSpec>;
#[doc = "Indicates the number of regions implemented by the Security Attribution Unit"]
pub mod type_;
#[doc = "RNR (rw) register accessor: Selects the region currently accessed by SAU_RBAR and SAU_RLAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnr`]
module"]
#[doc(alias = "RNR")]
pub type Rnr = crate::Reg<rnr::RnrSpec>;
#[doc = "Selects the region currently accessed by SAU_RBAR and SAU_RLAR"]
pub mod rnr;
#[doc = "RBAR (rw) register accessor: Provides indirect read and write access to the base address of the currently selected SAU region\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbar`]
module"]
#[doc(alias = "RBAR")]
pub type Rbar = crate::Reg<rbar::RbarSpec>;
#[doc = "Provides indirect read and write access to the base address of the currently selected SAU region"]
pub mod rbar;
#[doc = "RLAR (rw) register accessor: Provides indirect read and write access to the limit address of the currently selected SAU region\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlar`]
module"]
#[doc(alias = "RLAR")]
pub type Rlar = crate::Reg<rlar::RlarSpec>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected SAU region"]
pub mod rlar;
#[doc = "SFSR (rw) register accessor: Provides information about any security related faults\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfsr`]
module"]
#[doc(alias = "SFSR")]
pub type Sfsr = crate::Reg<sfsr::SfsrSpec>;
#[doc = "Provides information about any security related faults"]
pub mod sfsr;
#[doc = "SFAR (rw) register accessor: Shows the address of the memory location that caused a Security violation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfar`]
module"]
#[doc(alias = "SFAR")]
pub type Sfar = crate::Reg<sfar::SfarSpec>;
#[doc = "Shows the address of the memory location that caused a Security violation"]
pub mod sfar;

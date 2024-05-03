#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ictr: Ictr,
    actlr: Actlr,
}
impl RegisterBlock {
    #[doc = "0x04 - Provides information about the interrupt controller"]
    #[inline(always)]
    pub const fn ictr(&self) -> &Ictr {
        &self.ictr
    }
    #[doc = "0x08 - Provides IMPLEMENTATION DEFINED configuration and control options"]
    #[inline(always)]
    pub const fn actlr(&self) -> &Actlr {
        &self.actlr
    }
}
#[doc = "ICTR (rw) register accessor: Provides information about the interrupt controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ictr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ictr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ictr`]
module"]
#[doc(alias = "ICTR")]
pub type Ictr = crate::Reg<ictr::IctrSpec>;
#[doc = "Provides information about the interrupt controller"]
pub mod ictr;
#[doc = "ACTLR (rw) register accessor: Provides IMPLEMENTATION DEFINED configuration and control options\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actlr`]
module"]
#[doc(alias = "ACTLR")]
pub type Actlr = crate::Reg<actlr::ActlrSpec>;
#[doc = "Provides IMPLEMENTATION DEFINED configuration and control options"]
pub mod actlr;

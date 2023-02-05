#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Provides information about the interrupt controller"]
    pub ictr: ICTR,
    #[doc = "0x08 - Provides IMPLEMENTATION DEFINED configuration and control options"]
    pub actlr: ACTLR,
}
#[doc = "ICTR (rw) register accessor: an alias for `Reg<ICTR_SPEC>`"]
pub type ICTR = crate::Reg<ictr::ICTR_SPEC>;
#[doc = "Provides information about the interrupt controller"]
pub mod ictr;
#[doc = "ACTLR (rw) register accessor: an alias for `Reg<ACTLR_SPEC>`"]
pub type ACTLR = crate::Reg<actlr::ACTLR_SPEC>;
#[doc = "Provides IMPLEMENTATION DEFINED configuration and control options"]
pub mod actlr;

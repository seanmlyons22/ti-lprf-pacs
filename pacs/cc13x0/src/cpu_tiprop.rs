#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved000: RESERVED000,
    _reserved1: [u8; 0x0ff4],
    #[doc = "0xff8 - Internal. Only to be used through TI provided API."]
    pub traceclkmux: TRACECLKMUX,
    #[doc = "0xffc - Internal. Only to be used through TI provided API."]
    pub dyn_cg: DYN_CG,
}
#[doc = "RESERVED000 (rw) register accessor: an alias for `Reg<RESERVED000_SPEC>`"]
pub type RESERVED000 = crate::Reg<reserved000::RESERVED000_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved000;
#[doc = "TRACECLKMUX (rw) register accessor: an alias for `Reg<TRACECLKMUX_SPEC>`"]
pub type TRACECLKMUX = crate::Reg<traceclkmux::TRACECLKMUX_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod traceclkmux;
#[doc = "DYN_CG (rw) register accessor: an alias for `Reg<DYN_CG_SPEC>`"]
pub type DYN_CG = crate::Reg<dyn_cg::DYN_CG_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dyn_cg;

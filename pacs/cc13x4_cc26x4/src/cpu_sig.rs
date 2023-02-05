#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Provides a mechanism for software to generate an interrupt"]
    pub stir: STIR,
}
#[doc = "STIR (rw) register accessor: an alias for `Reg<STIR_SPEC>`"]
pub type STIR = crate::Reg<stir::STIR_SPEC>;
#[doc = "Provides a mechanism for software to generate an interrupt"]
pub mod stir;

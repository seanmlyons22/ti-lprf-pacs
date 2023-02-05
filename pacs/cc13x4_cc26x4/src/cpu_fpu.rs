#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Holds control data for the Floating-point extension"]
    pub fpccr: FPCCR,
    #[doc = "0x08 - Holds the location of the unpopulated floating-point register space allocated on an exception stack frame"]
    pub fpcar: FPCAR,
    #[doc = "0x0c - Holds the default values for the floating-point status control data that the PE assigns to the FPSCR when it creates a new floating-point context"]
    pub fpdscr: FPDSCR,
    #[doc = "0x10 - Describes the features provided by the Floating-point Extension"]
    pub mvfr0: MVFR0,
    #[doc = "0x14 - Describes the features provided by the Floating-point Extension"]
    pub mvfr1: MVFR1,
    #[doc = "0x18 - Describes the features provided by the Floating-point Extension"]
    pub mvfr2: MVFR2,
}
#[doc = "FPCCR (rw) register accessor: an alias for `Reg<FPCCR_SPEC>`"]
pub type FPCCR = crate::Reg<fpccr::FPCCR_SPEC>;
#[doc = "Holds control data for the Floating-point extension"]
pub mod fpccr;
#[doc = "FPCAR (rw) register accessor: an alias for `Reg<FPCAR_SPEC>`"]
pub type FPCAR = crate::Reg<fpcar::FPCAR_SPEC>;
#[doc = "Holds the location of the unpopulated floating-point register space allocated on an exception stack frame"]
pub mod fpcar;
#[doc = "FPDSCR (rw) register accessor: an alias for `Reg<FPDSCR_SPEC>`"]
pub type FPDSCR = crate::Reg<fpdscr::FPDSCR_SPEC>;
#[doc = "Holds the default values for the floating-point status control data that the PE assigns to the FPSCR when it creates a new floating-point context"]
pub mod fpdscr;
#[doc = "MVFR0 (rw) register accessor: an alias for `Reg<MVFR0_SPEC>`"]
pub type MVFR0 = crate::Reg<mvfr0::MVFR0_SPEC>;
#[doc = "Describes the features provided by the Floating-point Extension"]
pub mod mvfr0;
#[doc = "MVFR1 (rw) register accessor: an alias for `Reg<MVFR1_SPEC>`"]
pub type MVFR1 = crate::Reg<mvfr1::MVFR1_SPEC>;
#[doc = "Describes the features provided by the Floating-point Extension"]
pub mod mvfr1;
#[doc = "MVFR2 (rw) register accessor: an alias for `Reg<MVFR2_SPEC>`"]
pub type MVFR2 = crate::Reg<mvfr2::MVFR2_SPEC>;
#[doc = "Describes the features provided by the Floating-point Extension"]
pub mod mvfr2;

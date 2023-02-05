#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub iostrmin: IOSTRMIN,
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    pub iostrmed: IOSTRMED,
    #[doc = "0x08 - Internal. Only to be used through TI provided API."]
    pub iostrmax: IOSTRMAX,
    #[doc = "0x0c - IO Latch Control Controls transparency of all latches holding I/O or configuration state from the MCU IOC"]
    pub ioclatch: IOCLATCH,
    #[doc = "0x10 - SCLK_LF External Output Control"]
    pub clk32kctl: CLK32KCTL,
    #[doc = "0x14 - TCK IO Pin Control"]
    pub tckctl: TCKCTL,
}
#[doc = "IOSTRMIN (rw) register accessor: an alias for `Reg<IOSTRMIN_SPEC>`"]
pub type IOSTRMIN = crate::Reg<iostrmin::IOSTRMIN_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmin;
#[doc = "IOSTRMED (rw) register accessor: an alias for `Reg<IOSTRMED_SPEC>`"]
pub type IOSTRMED = crate::Reg<iostrmed::IOSTRMED_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmed;
#[doc = "IOSTRMAX (rw) register accessor: an alias for `Reg<IOSTRMAX_SPEC>`"]
pub type IOSTRMAX = crate::Reg<iostrmax::IOSTRMAX_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmax;
#[doc = "IOCLATCH (rw) register accessor: an alias for `Reg<IOCLATCH_SPEC>`"]
pub type IOCLATCH = crate::Reg<ioclatch::IOCLATCH_SPEC>;
#[doc = "IO Latch Control Controls transparency of all latches holding I/O or configuration state from the MCU IOC"]
pub mod ioclatch;
#[doc = "CLK32KCTL (rw) register accessor: an alias for `Reg<CLK32KCTL_SPEC>`"]
pub type CLK32KCTL = crate::Reg<clk32kctl::CLK32KCTL_SPEC>;
#[doc = "SCLK_LF External Output Control"]
pub mod clk32kctl;
#[doc = "TCKCTL (rw) register accessor: an alias for `Reg<TCKCTL_SPEC>`"]
pub type TCKCTL = crate::Reg<tckctl::TCKCTL_SPEC>;
#[doc = "TCK IO Pin Control"]
pub mod tckctl;

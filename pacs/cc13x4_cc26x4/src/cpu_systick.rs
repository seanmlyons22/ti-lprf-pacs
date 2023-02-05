#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Controls the SysTick timer and provides status data `FTSSS"]
    pub csr: CSR,
    #[doc = "0x04 - Provides access SysTick timer counter reload value `FTSSS"]
    pub rvr: RVR,
    #[doc = "0x08 - Reads or clears the SysTick timer current counter value `FTSSS"]
    pub cvr: CVR,
    #[doc = "0x0c - Reads the SysTick timer calibration value and parameters `FTSSS"]
    pub calib: CALIB,
}
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Controls the SysTick timer and provides status data `FTSSS"]
pub mod csr;
#[doc = "RVR (rw) register accessor: an alias for `Reg<RVR_SPEC>`"]
pub type RVR = crate::Reg<rvr::RVR_SPEC>;
#[doc = "Provides access SysTick timer counter reload value `FTSSS"]
pub mod rvr;
#[doc = "CVR (rw) register accessor: an alias for `Reg<CVR_SPEC>`"]
pub type CVR = crate::Reg<cvr::CVR_SPEC>;
#[doc = "Reads or clears the SysTick timer current counter value `FTSSS"]
pub mod cvr;
#[doc = "CALIB (rw) register accessor: an alias for `Reg<CALIB_SPEC>`"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "Reads the SysTick timer calibration value and parameters `FTSSS"]
pub mod calib;

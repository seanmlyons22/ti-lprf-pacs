#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration"]
    pub load: LOAD,
    #[doc = "0x04 - Current Count Value"]
    pub value: VALUE,
    #[doc = "0x08 - Control"]
    pub ctl: CTL,
    #[doc = "0x0c - Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x10 - Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x14 - Masked Interrupt Status"]
    pub mis: MIS,
    _reserved6: [u8; 0x0400],
    #[doc = "0x418 - Test Mode"]
    pub test: TEST,
    #[doc = "0x41c - Interrupt Cause Test Mode"]
    pub int_caus: INT_CAUS,
    _reserved8: [u8; 0x07e0],
    #[doc = "0xc00 - Lock"]
    pub lock: LOCK,
}
#[doc = "LOAD (rw) register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "Configuration"]
pub mod load;
#[doc = "VALUE (rw) register accessor: an alias for `Reg<VALUE_SPEC>`"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "Current Count Value"]
pub mod value;
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear"]
pub mod icr;
#[doc = "RIS (rw) register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS (rw) register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Masked Interrupt Status"]
pub mod mis;
#[doc = "TEST (rw) register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test Mode"]
pub mod test;
#[doc = "INT_CAUS (rw) register accessor: an alias for `Reg<INT_CAUS_SPEC>`"]
pub type INT_CAUS = crate::Reg<int_caus::INT_CAUS_SPEC>;
#[doc = "Interrupt Cause Test Mode"]
pub mod int_caus;
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Lock"]
pub mod lock;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Parity Error Control Parity error check controls"]
    pub per_ctl: PER_CTL,
    #[doc = "0x04 - Parity Error Check Parity error check results"]
    pub per_chk: PER_CHK,
    #[doc = "0x08 - Parity Error Debug Parity error check debug address setting"]
    pub per_dbg: PER_DBG,
    #[doc = "0x0c - Memory Control Controls memory initialization"]
    pub mem_ctl: MEM_CTL,
}
#[doc = "PER_CTL (rw) register accessor: an alias for `Reg<PER_CTL_SPEC>`"]
pub type PER_CTL = crate::Reg<per_ctl::PER_CTL_SPEC>;
#[doc = "Parity Error Control Parity error check controls"]
pub mod per_ctl;
#[doc = "PER_CHK (rw) register accessor: an alias for `Reg<PER_CHK_SPEC>`"]
pub type PER_CHK = crate::Reg<per_chk::PER_CHK_SPEC>;
#[doc = "Parity Error Check Parity error check results"]
pub mod per_chk;
#[doc = "PER_DBG (rw) register accessor: an alias for `Reg<PER_DBG_SPEC>`"]
pub type PER_DBG = crate::Reg<per_dbg::PER_DBG_SPEC>;
#[doc = "Parity Error Debug Parity error check debug address setting"]
pub mod per_dbg;
#[doc = "MEM_CTL (rw) register accessor: an alias for `Reg<MEM_CTL_SPEC>`"]
pub type MEM_CTL = crate::Reg<mem_ctl::MEM_CTL_SPEC>;
#[doc = "Memory Control Controls memory initialization"]
pub mod mem_ctl;

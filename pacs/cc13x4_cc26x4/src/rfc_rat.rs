#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Radio Timer Counter Value"]
    pub ratcnt: RATCNT,
    _reserved1: [u8; 0x78],
    #[doc = "0x80 - Timer Channel 0 Capture/Compare Register"]
    pub ratch0val: RATCH0VAL,
    #[doc = "0x84 - Timer Channel 1 Capture/Compare Register"]
    pub ratch1val: RATCH1VAL,
    #[doc = "0x88 - Timer Channel 2 Capture/Compare Register"]
    pub ratch2val: RATCH2VAL,
    #[doc = "0x8c - Timer Channel 3 Capture/Compare Register"]
    pub ratch3val: RATCH3VAL,
    #[doc = "0x90 - Timer Channel 4 Capture/Compare Register"]
    pub ratch4val: RATCH4VAL,
    #[doc = "0x94 - Timer Channel 5 Capture/Compare Register"]
    pub ratch5val: RATCH5VAL,
    #[doc = "0x98 - Timer Channel 6 Capture/Compare Register"]
    pub ratch6val: RATCH6VAL,
    #[doc = "0x9c - Timer Channel 7 Capture/Compare Register"]
    pub ratch7val: RATCH7VAL,
}
#[doc = "RATCNT (rw) register accessor: an alias for `Reg<RATCNT_SPEC>`"]
pub type RATCNT = crate::Reg<ratcnt::RATCNT_SPEC>;
#[doc = "Radio Timer Counter Value"]
pub mod ratcnt;
#[doc = "RATCH0VAL (rw) register accessor: an alias for `Reg<RATCH0VAL_SPEC>`"]
pub type RATCH0VAL = crate::Reg<ratch0val::RATCH0VAL_SPEC>;
#[doc = "Timer Channel 0 Capture/Compare Register"]
pub mod ratch0val;
#[doc = "RATCH1VAL (rw) register accessor: an alias for `Reg<RATCH1VAL_SPEC>`"]
pub type RATCH1VAL = crate::Reg<ratch1val::RATCH1VAL_SPEC>;
#[doc = "Timer Channel 1 Capture/Compare Register"]
pub mod ratch1val;
#[doc = "RATCH2VAL (rw) register accessor: an alias for `Reg<RATCH2VAL_SPEC>`"]
pub type RATCH2VAL = crate::Reg<ratch2val::RATCH2VAL_SPEC>;
#[doc = "Timer Channel 2 Capture/Compare Register"]
pub mod ratch2val;
#[doc = "RATCH3VAL (rw) register accessor: an alias for `Reg<RATCH3VAL_SPEC>`"]
pub type RATCH3VAL = crate::Reg<ratch3val::RATCH3VAL_SPEC>;
#[doc = "Timer Channel 3 Capture/Compare Register"]
pub mod ratch3val;
#[doc = "RATCH4VAL (rw) register accessor: an alias for `Reg<RATCH4VAL_SPEC>`"]
pub type RATCH4VAL = crate::Reg<ratch4val::RATCH4VAL_SPEC>;
#[doc = "Timer Channel 4 Capture/Compare Register"]
pub mod ratch4val;
#[doc = "RATCH5VAL (rw) register accessor: an alias for `Reg<RATCH5VAL_SPEC>`"]
pub type RATCH5VAL = crate::Reg<ratch5val::RATCH5VAL_SPEC>;
#[doc = "Timer Channel 5 Capture/Compare Register"]
pub mod ratch5val;
#[doc = "RATCH6VAL (rw) register accessor: an alias for `Reg<RATCH6VAL_SPEC>`"]
pub type RATCH6VAL = crate::Reg<ratch6val::RATCH6VAL_SPEC>;
#[doc = "Timer Channel 6 Capture/Compare Register"]
pub mod ratch6val;
#[doc = "RATCH7VAL (rw) register accessor: an alias for `Reg<RATCH7VAL_SPEC>`"]
pub type RATCH7VAL = crate::Reg<ratch7val::RATCH7VAL_SPEC>;
#[doc = "Timer Channel 7 Capture/Compare Register"]
pub mod ratch7val;

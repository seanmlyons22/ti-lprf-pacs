#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The MPU Type Register indicates how many regions the MPU supports"]
    pub type_: TYPE,
    #[doc = "0x04 - Enables the MPU and, when the MPU is enabled, controls whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
    pub ctrl: CTRL,
    #[doc = "0x08 - Selects the region currently accessed by MPU_RBAR and MPU_RLAR"]
    pub rnr: RNR,
    #[doc = "0x0c - Provides indirect read and write access to the base address of the currently selected MPU region"]
    pub rbar: RBAR,
    #[doc = "0x10 - Provides indirect read and write access to the limit address of the currently selected MPU region"]
    pub rlar: RLAR,
    #[doc = "0x14 - Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\])"]
    pub rbar_a1: RBAR_A1,
    #[doc = "0x18 - Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\])"]
    pub rlar_a1: RLAR_A1,
    #[doc = "0x1c - Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\])"]
    pub rbar_a2: RBAR_A2,
    #[doc = "0x20 - Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\])"]
    pub rlar_a2: RLAR_A2,
    #[doc = "0x24 - Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\])"]
    pub rbar_a3: RBAR_A3,
    #[doc = "0x28 - Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\])"]
    pub rlar_a3: RLAR_A3,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - Along with MPU_MAIR1, provides the memory attribute encodings corresponding to the AttrIndex values"]
    pub mair0: MAIR0,
    _reserved12: [u8; 0x08],
    #[doc = "0x3c - Along with MPU_MAIR0, provides the memory attribute encodings corresponding to the AttrIndex values"]
    pub mair1: MAIR1,
}
#[doc = "TYPE (rw) register accessor: an alias for `Reg<TYPE_SPEC>`"]
pub type TYPE = crate::Reg<type_::TYPE_SPEC>;
#[doc = "The MPU Type Register indicates how many regions the MPU supports"]
pub mod type_;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Enables the MPU and, when the MPU is enabled, controls whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
pub mod ctrl;
#[doc = "RNR (rw) register accessor: an alias for `Reg<RNR_SPEC>`"]
pub type RNR = crate::Reg<rnr::RNR_SPEC>;
#[doc = "Selects the region currently accessed by MPU_RBAR and MPU_RLAR"]
pub mod rnr;
#[doc = "RBAR (rw) register accessor: an alias for `Reg<RBAR_SPEC>`"]
pub type RBAR = crate::Reg<rbar::RBAR_SPEC>;
#[doc = "Provides indirect read and write access to the base address of the currently selected MPU region"]
pub mod rbar;
#[doc = "RLAR (rw) register accessor: an alias for `Reg<RLAR_SPEC>`"]
pub type RLAR = crate::Reg<rlar::RLAR_SPEC>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region"]
pub mod rlar;
#[doc = "RBAR_A1 (rw) register accessor: an alias for `Reg<RBAR_A1_SPEC>`"]
pub type RBAR_A1 = crate::Reg<rbar_a1::RBAR_A1_SPEC>;
#[doc = "Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\])"]
pub mod rbar_a1;
#[doc = "RLAR_A1 (rw) register accessor: an alias for `Reg<RLAR_A1_SPEC>`"]
pub type RLAR_A1 = crate::Reg<rlar_a1::RLAR_A1_SPEC>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\])"]
pub mod rlar_a1;
#[doc = "RBAR_A2 (rw) register accessor: an alias for `Reg<RBAR_A2_SPEC>`"]
pub type RBAR_A2 = crate::Reg<rbar_a2::RBAR_A2_SPEC>;
#[doc = "Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\])"]
pub mod rbar_a2;
#[doc = "RLAR_A2 (rw) register accessor: an alias for `Reg<RLAR_A2_SPEC>`"]
pub type RLAR_A2 = crate::Reg<rlar_a2::RLAR_A2_SPEC>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\])"]
pub mod rlar_a2;
#[doc = "RBAR_A3 (rw) register accessor: an alias for `Reg<RBAR_A3_SPEC>`"]
pub type RBAR_A3 = crate::Reg<rbar_a3::RBAR_A3_SPEC>;
#[doc = "Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\])"]
pub mod rbar_a3;
#[doc = "RLAR_A3 (rw) register accessor: an alias for `Reg<RLAR_A3_SPEC>`"]
pub type RLAR_A3 = crate::Reg<rlar_a3::RLAR_A3_SPEC>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\])"]
pub mod rlar_a3;
#[doc = "MAIR0 (rw) register accessor: an alias for `Reg<MAIR0_SPEC>`"]
pub type MAIR0 = crate::Reg<mair0::MAIR0_SPEC>;
#[doc = "Along with MPU_MAIR1, provides the memory attribute encodings corresponding to the AttrIndex values"]
pub mod mair0;
#[doc = "MAIR1 (rw) register accessor: an alias for `Reg<MAIR1_SPEC>`"]
pub type MAIR1 = crate::Reg<mair1::MAIR1_SPEC>;
#[doc = "Along with MPU_MAIR0, provides the memory attribute encodings corresponding to the AttrIndex values"]
pub mod mair1;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control This register is used to enable the flash patch block."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators."]
    pub remap: REMAP,
    #[doc = "0x08 - Comparator 0"]
    pub comp0: COMP0,
    #[doc = "0x0c - Comparator 1"]
    pub comp1: COMP1,
    #[doc = "0x10 - Comparator 2"]
    pub comp2: COMP2,
    #[doc = "0x14 - Comparator 3"]
    pub comp3: COMP3,
    #[doc = "0x18 - Comparator 4"]
    pub comp4: COMP4,
    #[doc = "0x1c - Comparator 5"]
    pub comp5: COMP5,
    #[doc = "0x20 - Comparator 6"]
    pub comp6: COMP6,
    #[doc = "0x24 - Comparator 7"]
    pub comp7: COMP7,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control This register is used to enable the flash patch block."]
pub mod ctrl;
#[doc = "REMAP (rw) register accessor: an alias for `Reg<REMAP_SPEC>`"]
pub type REMAP = crate::Reg<remap::REMAP_SPEC>;
#[doc = "Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators."]
pub mod remap;
#[doc = "COMP0 (rw) register accessor: an alias for `Reg<COMP0_SPEC>`"]
pub type COMP0 = crate::Reg<comp0::COMP0_SPEC>;
#[doc = "Comparator 0"]
pub mod comp0;
#[doc = "COMP1 (rw) register accessor: an alias for `Reg<COMP1_SPEC>`"]
pub type COMP1 = crate::Reg<comp1::COMP1_SPEC>;
#[doc = "Comparator 1"]
pub mod comp1;
#[doc = "COMP2 (rw) register accessor: an alias for `Reg<COMP2_SPEC>`"]
pub type COMP2 = crate::Reg<comp2::COMP2_SPEC>;
#[doc = "Comparator 2"]
pub mod comp2;
#[doc = "COMP3 (rw) register accessor: an alias for `Reg<COMP3_SPEC>`"]
pub type COMP3 = crate::Reg<comp3::COMP3_SPEC>;
#[doc = "Comparator 3"]
pub mod comp3;
#[doc = "COMP4 (rw) register accessor: an alias for `Reg<COMP4_SPEC>`"]
pub type COMP4 = crate::Reg<comp4::COMP4_SPEC>;
#[doc = "Comparator 4"]
pub mod comp4;
#[doc = "COMP5 (rw) register accessor: an alias for `Reg<COMP5_SPEC>`"]
pub type COMP5 = crate::Reg<comp5::COMP5_SPEC>;
#[doc = "Comparator 5"]
pub mod comp5;
#[doc = "COMP6 (rw) register accessor: an alias for `Reg<COMP6_SPEC>`"]
pub type COMP6 = crate::Reg<comp6::COMP6_SPEC>;
#[doc = "Comparator 6"]
pub mod comp6;
#[doc = "COMP7 (rw) register accessor: an alias for `Reg<COMP7_SPEC>`"]
pub type COMP7 = crate::Reg<comp7::COMP7_SPEC>;
#[doc = "Comparator 7"]
pub mod comp7;

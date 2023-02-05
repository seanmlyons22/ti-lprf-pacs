#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Provides FPB implementation information, and the global enable for the FPB unit"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Indicates whether the implementation supports Flash Patch remap and, if it does, holds the target address for remap"]
    pub remap: REMAP,
    #[doc = "0x08 - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    pub comp0: COMP0,
    #[doc = "0x0c - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    pub comp1: COMP1,
    #[doc = "0x10 - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    pub comp2: COMP2,
    #[doc = "0x14 - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    pub comp3: COMP3,
    #[doc = "0x18 - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    pub comp4: COMP4,
    #[doc = "0x1c - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    pub comp5: COMP5,
    #[doc = "0x20 - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    pub comp6: COMP6,
    #[doc = "0x24 - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    pub comp7: COMP7,
    _reserved10: [u8; 0x0f94],
    #[doc = "0xfbc - Provides CoreSight discovery information for the FPB"]
    pub devarch: DEVARCH,
    _reserved11: [u8; 0x0c],
    #[doc = "0xfcc - Provides CoreSight discovery information for the FPB"]
    pub devtype: DEVTYPE,
    #[doc = "0xfd0 - Provides CoreSight discovery information for the FP"]
    pub pidr4: PIDR4,
    #[doc = "0xfd4 - Provides CoreSight discovery information for the FP"]
    pub pidr5: PIDR5,
    #[doc = "0xfd8 - Provides CoreSight discovery information for the FP"]
    pub pidr6: PIDR6,
    #[doc = "0xfdc - Provides CoreSight discovery information for the FP"]
    pub pidr7: PIDR7,
    #[doc = "0xfe0 - Provides CoreSight discovery information for the FP"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - Provides CoreSight discovery information for the FP"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - Provides CoreSight discovery information for the FP"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - Provides CoreSight discovery information for the FP"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - Provides CoreSight discovery information for the FP"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - Provides CoreSight discovery information for the FP"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - Provides CoreSight discovery information for the FP"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - Provides CoreSight discovery information for the FP"]
    pub cidr3: CIDR3,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Provides FPB implementation information, and the global enable for the FPB unit"]
pub mod ctrl;
#[doc = "REMAP (rw) register accessor: an alias for `Reg<REMAP_SPEC>`"]
pub type REMAP = crate::Reg<remap::REMAP_SPEC>;
#[doc = "Indicates whether the implementation supports Flash Patch remap and, if it does, holds the target address for remap"]
pub mod remap;
#[doc = "COMP0 (rw) register accessor: an alias for `Reg<COMP0_SPEC>`"]
pub type COMP0 = crate::Reg<comp0::COMP0_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod comp0;
#[doc = "COMP1 (rw) register accessor: an alias for `Reg<COMP1_SPEC>`"]
pub type COMP1 = crate::Reg<comp1::COMP1_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod comp1;
#[doc = "COMP2 (rw) register accessor: an alias for `Reg<COMP2_SPEC>`"]
pub type COMP2 = crate::Reg<comp2::COMP2_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod comp2;
#[doc = "COMP3 (rw) register accessor: an alias for `Reg<COMP3_SPEC>`"]
pub type COMP3 = crate::Reg<comp3::COMP3_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod comp3;
#[doc = "COMP4 (rw) register accessor: an alias for `Reg<COMP4_SPEC>`"]
pub type COMP4 = crate::Reg<comp4::COMP4_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod comp4;
#[doc = "COMP5 (rw) register accessor: an alias for `Reg<COMP5_SPEC>`"]
pub type COMP5 = crate::Reg<comp5::COMP5_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod comp5;
#[doc = "COMP6 (rw) register accessor: an alias for `Reg<COMP6_SPEC>`"]
pub type COMP6 = crate::Reg<comp6::COMP6_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod comp6;
#[doc = "COMP7 (rw) register accessor: an alias for `Reg<COMP7_SPEC>`"]
pub type COMP7 = crate::Reg<comp7::COMP7_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod comp7;
#[doc = "DEVARCH (rw) register accessor: an alias for `Reg<DEVARCH_SPEC>`"]
pub type DEVARCH = crate::Reg<devarch::DEVARCH_SPEC>;
#[doc = "Provides CoreSight discovery information for the FPB"]
pub mod devarch;
#[doc = "DEVTYPE (rw) register accessor: an alias for `Reg<DEVTYPE_SPEC>`"]
pub type DEVTYPE = crate::Reg<devtype::DEVTYPE_SPEC>;
#[doc = "Provides CoreSight discovery information for the FPB"]
pub mod devtype;
#[doc = "PIDR4 (rw) register accessor: an alias for `Reg<PIDR4_SPEC>`"]
pub type PIDR4 = crate::Reg<pidr4::PIDR4_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod pidr4;
#[doc = "PIDR5 (rw) register accessor: an alias for `Reg<PIDR5_SPEC>`"]
pub type PIDR5 = crate::Reg<pidr5::PIDR5_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod pidr5;
#[doc = "PIDR6 (rw) register accessor: an alias for `Reg<PIDR6_SPEC>`"]
pub type PIDR6 = crate::Reg<pidr6::PIDR6_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod pidr6;
#[doc = "PIDR7 (rw) register accessor: an alias for `Reg<PIDR7_SPEC>`"]
pub type PIDR7 = crate::Reg<pidr7::PIDR7_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod pidr7;
#[doc = "PIDR0 (rw) register accessor: an alias for `Reg<PIDR0_SPEC>`"]
pub type PIDR0 = crate::Reg<pidr0::PIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod pidr0;
#[doc = "PIDR1 (rw) register accessor: an alias for `Reg<PIDR1_SPEC>`"]
pub type PIDR1 = crate::Reg<pidr1::PIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod pidr1;
#[doc = "PIDR2 (rw) register accessor: an alias for `Reg<PIDR2_SPEC>`"]
pub type PIDR2 = crate::Reg<pidr2::PIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod pidr2;
#[doc = "PIDR3 (rw) register accessor: an alias for `Reg<PIDR3_SPEC>`"]
pub type PIDR3 = crate::Reg<pidr3::PIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod pidr3;
#[doc = "CIDR0 (rw) register accessor: an alias for `Reg<CIDR0_SPEC>`"]
pub type CIDR0 = crate::Reg<cidr0::CIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod cidr0;
#[doc = "CIDR1 (rw) register accessor: an alias for `Reg<CIDR1_SPEC>`"]
pub type CIDR1 = crate::Reg<cidr1::CIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod cidr1;
#[doc = "CIDR2 (rw) register accessor: an alias for `Reg<CIDR2_SPEC>`"]
pub type CIDR2 = crate::Reg<cidr2::CIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod cidr2;
#[doc = "CIDR3 (rw) register accessor: an alias for `Reg<CIDR3_SPEC>`"]
pub type CIDR3 = crate::Reg<cidr3::CIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod cidr3;

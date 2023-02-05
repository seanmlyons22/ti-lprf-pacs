#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Provides configuration and status information for the DWT unit, and used to control features of the unit"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Shows or sets the value of the processor cycle counter, CYCCNT"]
    pub cyccnt: CYCCNT,
    #[doc = "0x08 - CPI Count Register"]
    pub cpicnt: CPICNT,
    #[doc = "0x0c - Counts the total cycles spent in exception processing"]
    pub exccnt: EXCCNT,
    #[doc = "0x10 - Sleep Count Register"]
    pub sleepcnt: SLEEPCNT,
    #[doc = "0x14 - Increments on the additional cycles required to execute all load or store instructions"]
    pub lsucnt: LSUCNT,
    #[doc = "0x18 - Increments on the additional cycles required to execute all load or store instructions"]
    pub foldcnt: FOLDCNT,
    #[doc = "0x1c - Program Counter Sample Register"]
    pub pcsr: PCSR,
    #[doc = "0x20 - Provides a reference value for use by watchpoint comparator 0"]
    pub comp0: COMP0,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - Controls the operation of watchpoint comparator 0"]
    pub function0: FUNCTION0,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Provides a reference value for use by watchpoint comparator 1"]
    pub comp1: COMP1,
    _reserved11: [u8; 0x04],
    #[doc = "0x38 - Controls the operation of watchpoint comparator 1"]
    pub function1: FUNCTION1,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - Provides a reference value for use by watchpoint comparator 2"]
    pub comp2: COMP2,
    _reserved13: [u8; 0x04],
    #[doc = "0x48 - Controls the operation of watchpoint comparator 2"]
    pub function2: FUNCTION2,
    _reserved14: [u8; 0x04],
    #[doc = "0x50 - Provides a reference value for use by watchpoint comparator 3"]
    pub comp3: COMP3,
    _reserved15: [u8; 0x04],
    #[doc = "0x58 - Controls the operation of watchpoint comparator 3"]
    pub function3: FUNCTION3,
    _reserved16: [u8; 0x0f60],
    #[doc = "0xfbc - Provides CoreSight discovery information for the DWT"]
    pub devarch: DEVARCH,
    _reserved17: [u8; 0x0c],
    #[doc = "0xfcc - Provides CoreSight discovery information for the DWT"]
    pub devtype: DEVTYPE,
    #[doc = "0xfd0 - Provides CoreSight discovery information for the DWT"]
    pub pidr4: PIDR4,
    #[doc = "0xfd4 - Provides CoreSight discovery information for the DWT"]
    pub pidr5: PIDR5,
    #[doc = "0xfd8 - Provides CoreSight discovery information for the DWT"]
    pub pidr6: PIDR6,
    #[doc = "0xfdc - Provides CoreSight discovery information for the DWT"]
    pub pidr7: PIDR7,
    #[doc = "0xfe0 - Provides CoreSight discovery information for the DWT"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - Provides CoreSight discovery information for the DWT"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - Provides CoreSight discovery information for the DWT"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - Provides CoreSight discovery information for the DWT"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - Provides CoreSight discovery information for the DWT"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - Provides CoreSight discovery information for the DWT"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - Provides CoreSight discovery information for the DWT"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - Provides CoreSight discovery information for the DWT"]
    pub cidr3: CIDR3,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Provides configuration and status information for the DWT unit, and used to control features of the unit"]
pub mod ctrl;
#[doc = "CYCCNT (rw) register accessor: an alias for `Reg<CYCCNT_SPEC>`"]
pub type CYCCNT = crate::Reg<cyccnt::CYCCNT_SPEC>;
#[doc = "Shows or sets the value of the processor cycle counter, CYCCNT"]
pub mod cyccnt;
#[doc = "CPICNT (rw) register accessor: an alias for `Reg<CPICNT_SPEC>`"]
pub type CPICNT = crate::Reg<cpicnt::CPICNT_SPEC>;
#[doc = "CPI Count Register"]
pub mod cpicnt;
#[doc = "EXCCNT (rw) register accessor: an alias for `Reg<EXCCNT_SPEC>`"]
pub type EXCCNT = crate::Reg<exccnt::EXCCNT_SPEC>;
#[doc = "Counts the total cycles spent in exception processing"]
pub mod exccnt;
#[doc = "SLEEPCNT (rw) register accessor: an alias for `Reg<SLEEPCNT_SPEC>`"]
pub type SLEEPCNT = crate::Reg<sleepcnt::SLEEPCNT_SPEC>;
#[doc = "Sleep Count Register"]
pub mod sleepcnt;
#[doc = "LSUCNT (rw) register accessor: an alias for `Reg<LSUCNT_SPEC>`"]
pub type LSUCNT = crate::Reg<lsucnt::LSUCNT_SPEC>;
#[doc = "Increments on the additional cycles required to execute all load or store instructions"]
pub mod lsucnt;
#[doc = "FOLDCNT (rw) register accessor: an alias for `Reg<FOLDCNT_SPEC>`"]
pub type FOLDCNT = crate::Reg<foldcnt::FOLDCNT_SPEC>;
#[doc = "Increments on the additional cycles required to execute all load or store instructions"]
pub mod foldcnt;
#[doc = "PCSR (rw) register accessor: an alias for `Reg<PCSR_SPEC>`"]
pub type PCSR = crate::Reg<pcsr::PCSR_SPEC>;
#[doc = "Program Counter Sample Register"]
pub mod pcsr;
#[doc = "COMP0 (rw) register accessor: an alias for `Reg<COMP0_SPEC>`"]
pub type COMP0 = crate::Reg<comp0::COMP0_SPEC>;
#[doc = "Provides a reference value for use by watchpoint comparator 0"]
pub mod comp0;
#[doc = "FUNCTION0 (rw) register accessor: an alias for `Reg<FUNCTION0_SPEC>`"]
pub type FUNCTION0 = crate::Reg<function0::FUNCTION0_SPEC>;
#[doc = "Controls the operation of watchpoint comparator 0"]
pub mod function0;
#[doc = "COMP1 (rw) register accessor: an alias for `Reg<COMP1_SPEC>`"]
pub type COMP1 = crate::Reg<comp1::COMP1_SPEC>;
#[doc = "Provides a reference value for use by watchpoint comparator 1"]
pub mod comp1;
#[doc = "FUNCTION1 (rw) register accessor: an alias for `Reg<FUNCTION1_SPEC>`"]
pub type FUNCTION1 = crate::Reg<function1::FUNCTION1_SPEC>;
#[doc = "Controls the operation of watchpoint comparator 1"]
pub mod function1;
#[doc = "COMP2 (rw) register accessor: an alias for `Reg<COMP2_SPEC>`"]
pub type COMP2 = crate::Reg<comp2::COMP2_SPEC>;
#[doc = "Provides a reference value for use by watchpoint comparator 2"]
pub mod comp2;
#[doc = "FUNCTION2 (rw) register accessor: an alias for `Reg<FUNCTION2_SPEC>`"]
pub type FUNCTION2 = crate::Reg<function2::FUNCTION2_SPEC>;
#[doc = "Controls the operation of watchpoint comparator 2"]
pub mod function2;
#[doc = "COMP3 (rw) register accessor: an alias for `Reg<COMP3_SPEC>`"]
pub type COMP3 = crate::Reg<comp3::COMP3_SPEC>;
#[doc = "Provides a reference value for use by watchpoint comparator 3"]
pub mod comp3;
#[doc = "FUNCTION3 (rw) register accessor: an alias for `Reg<FUNCTION3_SPEC>`"]
pub type FUNCTION3 = crate::Reg<function3::FUNCTION3_SPEC>;
#[doc = "Controls the operation of watchpoint comparator 3"]
pub mod function3;
#[doc = "DEVARCH (rw) register accessor: an alias for `Reg<DEVARCH_SPEC>`"]
pub type DEVARCH = crate::Reg<devarch::DEVARCH_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod devarch;
#[doc = "DEVTYPE (rw) register accessor: an alias for `Reg<DEVTYPE_SPEC>`"]
pub type DEVTYPE = crate::Reg<devtype::DEVTYPE_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod devtype;
#[doc = "PIDR4 (rw) register accessor: an alias for `Reg<PIDR4_SPEC>`"]
pub type PIDR4 = crate::Reg<pidr4::PIDR4_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr4;
#[doc = "PIDR5 (rw) register accessor: an alias for `Reg<PIDR5_SPEC>`"]
pub type PIDR5 = crate::Reg<pidr5::PIDR5_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr5;
#[doc = "PIDR6 (rw) register accessor: an alias for `Reg<PIDR6_SPEC>`"]
pub type PIDR6 = crate::Reg<pidr6::PIDR6_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr6;
#[doc = "PIDR7 (rw) register accessor: an alias for `Reg<PIDR7_SPEC>`"]
pub type PIDR7 = crate::Reg<pidr7::PIDR7_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr7;
#[doc = "PIDR0 (rw) register accessor: an alias for `Reg<PIDR0_SPEC>`"]
pub type PIDR0 = crate::Reg<pidr0::PIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr0;
#[doc = "PIDR1 (rw) register accessor: an alias for `Reg<PIDR1_SPEC>`"]
pub type PIDR1 = crate::Reg<pidr1::PIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr1;
#[doc = "PIDR2 (rw) register accessor: an alias for `Reg<PIDR2_SPEC>`"]
pub type PIDR2 = crate::Reg<pidr2::PIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr2;
#[doc = "PIDR3 (rw) register accessor: an alias for `Reg<PIDR3_SPEC>`"]
pub type PIDR3 = crate::Reg<pidr3::PIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr3;
#[doc = "CIDR0 (rw) register accessor: an alias for `Reg<CIDR0_SPEC>`"]
pub type CIDR0 = crate::Reg<cidr0::CIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod cidr0;
#[doc = "CIDR1 (rw) register accessor: an alias for `Reg<CIDR1_SPEC>`"]
pub type CIDR1 = crate::Reg<cidr1::CIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod cidr1;
#[doc = "CIDR2 (rw) register accessor: an alias for `Reg<CIDR2_SPEC>`"]
pub type CIDR2 = crate::Reg<cidr2::CIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod cidr2;
#[doc = "CIDR3 (rw) register accessor: an alias for `Reg<CIDR3_SPEC>`"]
pub type CIDR3 = crate::Reg<cidr3::CIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod cidr3;

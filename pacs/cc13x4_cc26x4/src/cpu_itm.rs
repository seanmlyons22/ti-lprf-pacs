#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Provides the interface for generating Instrumentation packets"]
    pub stim0: STIM0,
    #[doc = "0x04 - Provides the interface for generating Instrumentation packets"]
    pub stim1: STIM1,
    #[doc = "0x08 - Provides the interface for generating Instrumentation packets"]
    pub stim2: STIM2,
    #[doc = "0x0c - Provides the interface for generating Instrumentation packets"]
    pub stim3: STIM3,
    #[doc = "0x10 - Provides the interface for generating Instrumentation packets"]
    pub stim4: STIM4,
    #[doc = "0x14 - Provides the interface for generating Instrumentation packets"]
    pub stim5: STIM5,
    #[doc = "0x18 - Provides the interface for generating Instrumentation packets"]
    pub stim6: STIM6,
    #[doc = "0x1c - Provides the interface for generating Instrumentation packets"]
    pub stim7: STIM7,
    #[doc = "0x20 - Provides the interface for generating Instrumentation packets"]
    pub stim8: STIM8,
    #[doc = "0x24 - Provides the interface for generating Instrumentation packets"]
    pub stim9: STIM9,
    #[doc = "0x28 - Provides the interface for generating Instrumentation packets"]
    pub stim10: STIM10,
    #[doc = "0x2c - Provides the interface for generating Instrumentation packets"]
    pub stim11: STIM11,
    #[doc = "0x30 - Provides the interface for generating Instrumentation packets"]
    pub stim12: STIM12,
    #[doc = "0x34 - Provides the interface for generating Instrumentation packets"]
    pub stim13: STIM13,
    #[doc = "0x38 - Provides the interface for generating Instrumentation packets"]
    pub stim14: STIM14,
    #[doc = "0x3c - Provides the interface for generating Instrumentation packets"]
    pub stim15: STIM15,
    #[doc = "0x40 - Provides the interface for generating Instrumentation packets"]
    pub stim16: STIM16,
    #[doc = "0x44 - Provides the interface for generating Instrumentation packets"]
    pub stim17: STIM17,
    #[doc = "0x48 - Provides the interface for generating Instrumentation packets"]
    pub stim18: STIM18,
    #[doc = "0x4c - Provides the interface for generating Instrumentation packets"]
    pub stim19: STIM19,
    #[doc = "0x50 - Provides the interface for generating Instrumentation packets"]
    pub stim20: STIM20,
    #[doc = "0x54 - Provides the interface for generating Instrumentation packets"]
    pub stim21: STIM21,
    #[doc = "0x58 - Provides the interface for generating Instrumentation packets"]
    pub stim22: STIM22,
    #[doc = "0x5c - Provides the interface for generating Instrumentation packets"]
    pub stim23: STIM23,
    #[doc = "0x60 - Provides the interface for generating Instrumentation packets"]
    pub stim24: STIM24,
    #[doc = "0x64 - Provides the interface for generating Instrumentation packets"]
    pub stim25: STIM25,
    #[doc = "0x68 - Provides the interface for generating Instrumentation packets"]
    pub stim26: STIM26,
    #[doc = "0x6c - Provides the interface for generating Instrumentation packets"]
    pub stim27: STIM27,
    #[doc = "0x70 - Provides the interface for generating Instrumentation packets"]
    pub stim28: STIM28,
    #[doc = "0x74 - Provides the interface for generating Instrumentation packets"]
    pub stim29: STIM29,
    #[doc = "0x78 - Provides the interface for generating Instrumentation packets"]
    pub stim30: STIM30,
    #[doc = "0x7c - Provides the interface for generating Instrumentation packets"]
    pub stim31: STIM31,
    _reserved32: [u8; 0x0d80],
    #[doc = "0xe00 - Provide an individual enable bit for each ITM_STIM register"]
    pub ter0: TER0,
    _reserved33: [u8; 0x3c],
    #[doc = "0xe40 - Controls which stimulus ports can be accessed by unprivileged code"]
    pub tpr: TPR,
    _reserved34: [u8; 0x3c],
    #[doc = "0xe80 - Configures and controls transfers through the ITM interface"]
    pub tcr: TCR,
    _reserved35: [u8; 0x6c],
    #[doc = "0xef0 - Integration Mode: Read ATB Ready"]
    pub int_atready: INT_ATREADY,
    _reserved36: [u8; 0x04],
    #[doc = "0xef8 - Integration Mode: Write ATB Valid"]
    pub int_atvalid: INT_ATVALID,
    _reserved37: [u8; 0x04],
    #[doc = "0xf00 - Integration Mode Control Register"]
    pub itctrl: ITCTRL,
    _reserved38: [u8; 0xb8],
    #[doc = "0xfbc - Provides CoreSight discovery information for the ITM"]
    pub devarch: DEVARCH,
    _reserved39: [u8; 0x0c],
    #[doc = "0xfcc - Provides CoreSight discovery information for the ITM"]
    pub devtype: DEVTYPE,
    #[doc = "0xfd0 - Provides CoreSight discovery information for the ITM"]
    pub pidr4: PIDR4,
    #[doc = "0xfd4 - Provides CoreSight discovery information for the ITM"]
    pub pidr5: PIDR5,
    #[doc = "0xfd8 - Provides CoreSight discovery information for the ITM"]
    pub pidr6: PIDR6,
    #[doc = "0xfdc - Provides CoreSight discovery information for the ITM"]
    pub pidr7: PIDR7,
    #[doc = "0xfe0 - Provides CoreSight discovery information for the ITM"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - Provides CoreSight discovery information for the ITM"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - Provides CoreSight discovery information for the ITM"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - Provides CoreSight discovery information for the ITM"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - Provides CoreSight discovery information for the ITM"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - Provides CoreSight discovery information for the ITM"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - Provides CoreSight discovery information for the ITM"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - Provides CoreSight discovery information for the ITM"]
    pub cidr3: CIDR3,
}
#[doc = "STIM0 (rw) register accessor: an alias for `Reg<STIM0_SPEC>`"]
pub type STIM0 = crate::Reg<stim0::STIM0_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim0;
#[doc = "STIM1 (rw) register accessor: an alias for `Reg<STIM1_SPEC>`"]
pub type STIM1 = crate::Reg<stim1::STIM1_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim1;
#[doc = "STIM2 (rw) register accessor: an alias for `Reg<STIM2_SPEC>`"]
pub type STIM2 = crate::Reg<stim2::STIM2_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim2;
#[doc = "STIM3 (rw) register accessor: an alias for `Reg<STIM3_SPEC>`"]
pub type STIM3 = crate::Reg<stim3::STIM3_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim3;
#[doc = "STIM4 (rw) register accessor: an alias for `Reg<STIM4_SPEC>`"]
pub type STIM4 = crate::Reg<stim4::STIM4_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim4;
#[doc = "STIM5 (rw) register accessor: an alias for `Reg<STIM5_SPEC>`"]
pub type STIM5 = crate::Reg<stim5::STIM5_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim5;
#[doc = "STIM6 (rw) register accessor: an alias for `Reg<STIM6_SPEC>`"]
pub type STIM6 = crate::Reg<stim6::STIM6_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim6;
#[doc = "STIM7 (rw) register accessor: an alias for `Reg<STIM7_SPEC>`"]
pub type STIM7 = crate::Reg<stim7::STIM7_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim7;
#[doc = "STIM8 (rw) register accessor: an alias for `Reg<STIM8_SPEC>`"]
pub type STIM8 = crate::Reg<stim8::STIM8_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim8;
#[doc = "STIM9 (rw) register accessor: an alias for `Reg<STIM9_SPEC>`"]
pub type STIM9 = crate::Reg<stim9::STIM9_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim9;
#[doc = "STIM10 (rw) register accessor: an alias for `Reg<STIM10_SPEC>`"]
pub type STIM10 = crate::Reg<stim10::STIM10_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim10;
#[doc = "STIM11 (rw) register accessor: an alias for `Reg<STIM11_SPEC>`"]
pub type STIM11 = crate::Reg<stim11::STIM11_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim11;
#[doc = "STIM12 (rw) register accessor: an alias for `Reg<STIM12_SPEC>`"]
pub type STIM12 = crate::Reg<stim12::STIM12_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim12;
#[doc = "STIM13 (rw) register accessor: an alias for `Reg<STIM13_SPEC>`"]
pub type STIM13 = crate::Reg<stim13::STIM13_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim13;
#[doc = "STIM14 (rw) register accessor: an alias for `Reg<STIM14_SPEC>`"]
pub type STIM14 = crate::Reg<stim14::STIM14_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim14;
#[doc = "STIM15 (rw) register accessor: an alias for `Reg<STIM15_SPEC>`"]
pub type STIM15 = crate::Reg<stim15::STIM15_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim15;
#[doc = "STIM16 (rw) register accessor: an alias for `Reg<STIM16_SPEC>`"]
pub type STIM16 = crate::Reg<stim16::STIM16_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim16;
#[doc = "STIM17 (rw) register accessor: an alias for `Reg<STIM17_SPEC>`"]
pub type STIM17 = crate::Reg<stim17::STIM17_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim17;
#[doc = "STIM18 (rw) register accessor: an alias for `Reg<STIM18_SPEC>`"]
pub type STIM18 = crate::Reg<stim18::STIM18_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim18;
#[doc = "STIM19 (rw) register accessor: an alias for `Reg<STIM19_SPEC>`"]
pub type STIM19 = crate::Reg<stim19::STIM19_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim19;
#[doc = "STIM20 (rw) register accessor: an alias for `Reg<STIM20_SPEC>`"]
pub type STIM20 = crate::Reg<stim20::STIM20_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim20;
#[doc = "STIM21 (rw) register accessor: an alias for `Reg<STIM21_SPEC>`"]
pub type STIM21 = crate::Reg<stim21::STIM21_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim21;
#[doc = "STIM22 (rw) register accessor: an alias for `Reg<STIM22_SPEC>`"]
pub type STIM22 = crate::Reg<stim22::STIM22_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim22;
#[doc = "STIM23 (rw) register accessor: an alias for `Reg<STIM23_SPEC>`"]
pub type STIM23 = crate::Reg<stim23::STIM23_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim23;
#[doc = "STIM24 (rw) register accessor: an alias for `Reg<STIM24_SPEC>`"]
pub type STIM24 = crate::Reg<stim24::STIM24_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim24;
#[doc = "STIM25 (rw) register accessor: an alias for `Reg<STIM25_SPEC>`"]
pub type STIM25 = crate::Reg<stim25::STIM25_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim25;
#[doc = "STIM26 (rw) register accessor: an alias for `Reg<STIM26_SPEC>`"]
pub type STIM26 = crate::Reg<stim26::STIM26_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim26;
#[doc = "STIM27 (rw) register accessor: an alias for `Reg<STIM27_SPEC>`"]
pub type STIM27 = crate::Reg<stim27::STIM27_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim27;
#[doc = "STIM28 (rw) register accessor: an alias for `Reg<STIM28_SPEC>`"]
pub type STIM28 = crate::Reg<stim28::STIM28_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim28;
#[doc = "STIM29 (rw) register accessor: an alias for `Reg<STIM29_SPEC>`"]
pub type STIM29 = crate::Reg<stim29::STIM29_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim29;
#[doc = "STIM30 (rw) register accessor: an alias for `Reg<STIM30_SPEC>`"]
pub type STIM30 = crate::Reg<stim30::STIM30_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim30;
#[doc = "STIM31 (rw) register accessor: an alias for `Reg<STIM31_SPEC>`"]
pub type STIM31 = crate::Reg<stim31::STIM31_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim31;
#[doc = "TER0 (rw) register accessor: an alias for `Reg<TER0_SPEC>`"]
pub type TER0 = crate::Reg<ter0::TER0_SPEC>;
#[doc = "Provide an individual enable bit for each ITM_STIM register"]
pub mod ter0;
#[doc = "TPR (rw) register accessor: an alias for `Reg<TPR_SPEC>`"]
pub type TPR = crate::Reg<tpr::TPR_SPEC>;
#[doc = "Controls which stimulus ports can be accessed by unprivileged code"]
pub mod tpr;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Configures and controls transfers through the ITM interface"]
pub mod tcr;
#[doc = "INT_ATREADY (rw) register accessor: an alias for `Reg<INT_ATREADY_SPEC>`"]
pub type INT_ATREADY = crate::Reg<int_atready::INT_ATREADY_SPEC>;
#[doc = "Integration Mode: Read ATB Ready"]
pub mod int_atready;
#[doc = "INT_ATVALID (rw) register accessor: an alias for `Reg<INT_ATVALID_SPEC>`"]
pub type INT_ATVALID = crate::Reg<int_atvalid::INT_ATVALID_SPEC>;
#[doc = "Integration Mode: Write ATB Valid"]
pub mod int_atvalid;
#[doc = "ITCTRL (rw) register accessor: an alias for `Reg<ITCTRL_SPEC>`"]
pub type ITCTRL = crate::Reg<itctrl::ITCTRL_SPEC>;
#[doc = "Integration Mode Control Register"]
pub mod itctrl;
#[doc = "DEVARCH (rw) register accessor: an alias for `Reg<DEVARCH_SPEC>`"]
pub type DEVARCH = crate::Reg<devarch::DEVARCH_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod devarch;
#[doc = "DEVTYPE (rw) register accessor: an alias for `Reg<DEVTYPE_SPEC>`"]
pub type DEVTYPE = crate::Reg<devtype::DEVTYPE_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod devtype;
#[doc = "PIDR4 (rw) register accessor: an alias for `Reg<PIDR4_SPEC>`"]
pub type PIDR4 = crate::Reg<pidr4::PIDR4_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr4;
#[doc = "PIDR5 (rw) register accessor: an alias for `Reg<PIDR5_SPEC>`"]
pub type PIDR5 = crate::Reg<pidr5::PIDR5_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr5;
#[doc = "PIDR6 (rw) register accessor: an alias for `Reg<PIDR6_SPEC>`"]
pub type PIDR6 = crate::Reg<pidr6::PIDR6_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr6;
#[doc = "PIDR7 (rw) register accessor: an alias for `Reg<PIDR7_SPEC>`"]
pub type PIDR7 = crate::Reg<pidr7::PIDR7_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr7;
#[doc = "PIDR0 (rw) register accessor: an alias for `Reg<PIDR0_SPEC>`"]
pub type PIDR0 = crate::Reg<pidr0::PIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr0;
#[doc = "PIDR1 (rw) register accessor: an alias for `Reg<PIDR1_SPEC>`"]
pub type PIDR1 = crate::Reg<pidr1::PIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr1;
#[doc = "PIDR2 (rw) register accessor: an alias for `Reg<PIDR2_SPEC>`"]
pub type PIDR2 = crate::Reg<pidr2::PIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr2;
#[doc = "PIDR3 (rw) register accessor: an alias for `Reg<PIDR3_SPEC>`"]
pub type PIDR3 = crate::Reg<pidr3::PIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr3;
#[doc = "CIDR0 (rw) register accessor: an alias for `Reg<CIDR0_SPEC>`"]
pub type CIDR0 = crate::Reg<cidr0::CIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod cidr0;
#[doc = "CIDR1 (rw) register accessor: an alias for `Reg<CIDR1_SPEC>`"]
pub type CIDR1 = crate::Reg<cidr1::CIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod cidr1;
#[doc = "CIDR2 (rw) register accessor: an alias for `Reg<CIDR2_SPEC>`"]
pub type CIDR2 = crate::Reg<cidr2::CIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod cidr2;
#[doc = "CIDR3 (rw) register accessor: an alias for `Reg<CIDR3_SPEC>`"]
pub type CIDR3 = crate::Reg<cidr3::CIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod cidr3;

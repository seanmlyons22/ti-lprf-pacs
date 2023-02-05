#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Stimulus Port 0"]
    pub stim0: STIM0,
    #[doc = "0x04 - Stimulus Port 1"]
    pub stim1: STIM1,
    #[doc = "0x08 - Stimulus Port 2"]
    pub stim2: STIM2,
    #[doc = "0x0c - Stimulus Port 3"]
    pub stim3: STIM3,
    #[doc = "0x10 - Stimulus Port 4"]
    pub stim4: STIM4,
    #[doc = "0x14 - Stimulus Port 5"]
    pub stim5: STIM5,
    #[doc = "0x18 - Stimulus Port 6"]
    pub stim6: STIM6,
    #[doc = "0x1c - Stimulus Port 7"]
    pub stim7: STIM7,
    #[doc = "0x20 - Stimulus Port 8"]
    pub stim8: STIM8,
    #[doc = "0x24 - Stimulus Port 9"]
    pub stim9: STIM9,
    #[doc = "0x28 - Stimulus Port 10"]
    pub stim10: STIM10,
    #[doc = "0x2c - Stimulus Port 11"]
    pub stim11: STIM11,
    #[doc = "0x30 - Stimulus Port 12"]
    pub stim12: STIM12,
    #[doc = "0x34 - Stimulus Port 13"]
    pub stim13: STIM13,
    #[doc = "0x38 - Stimulus Port 14"]
    pub stim14: STIM14,
    #[doc = "0x3c - Stimulus Port 15"]
    pub stim15: STIM15,
    #[doc = "0x40 - Stimulus Port 16"]
    pub stim16: STIM16,
    #[doc = "0x44 - Stimulus Port 17"]
    pub stim17: STIM17,
    #[doc = "0x48 - Stimulus Port 18"]
    pub stim18: STIM18,
    #[doc = "0x4c - Stimulus Port 19"]
    pub stim19: STIM19,
    #[doc = "0x50 - Stimulus Port 20"]
    pub stim20: STIM20,
    #[doc = "0x54 - Stimulus Port 21"]
    pub stim21: STIM21,
    #[doc = "0x58 - Stimulus Port 22"]
    pub stim22: STIM22,
    #[doc = "0x5c - Stimulus Port 23"]
    pub stim23: STIM23,
    #[doc = "0x60 - Stimulus Port 24"]
    pub stim24: STIM24,
    #[doc = "0x64 - Stimulus Port 25"]
    pub stim25: STIM25,
    #[doc = "0x68 - Stimulus Port 26"]
    pub stim26: STIM26,
    #[doc = "0x6c - Stimulus Port 27"]
    pub stim27: STIM27,
    #[doc = "0x70 - Stimulus Port 28"]
    pub stim28: STIM28,
    #[doc = "0x74 - Stimulus Port 29"]
    pub stim29: STIM29,
    #[doc = "0x78 - Stimulus Port 30"]
    pub stim30: STIM30,
    #[doc = "0x7c - Stimulus Port 31"]
    pub stim31: STIM31,
    _reserved32: [u8; 0x0d80],
    #[doc = "0xe00 - Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required."]
    pub ter: TER,
    _reserved33: [u8; 0x3c],
    #[doc = "0xe40 - Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode."]
    pub tpr: TPR,
    _reserved34: [u8; 0x3c],
    #[doc = "0xe80 - Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set."]
    pub tcr: TCR,
    _reserved35: [u8; 0x012c],
    #[doc = "0xfb0 - Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR."]
    pub lar: LAR,
    #[doc = "0xfb4 - Lock Status Use this register to enable write accesses to the Control Register."]
    pub lsr: LSR,
}
#[doc = "STIM0 (rw) register accessor: an alias for `Reg<STIM0_SPEC>`"]
pub type STIM0 = crate::Reg<stim0::STIM0_SPEC>;
#[doc = "Stimulus Port 0"]
pub mod stim0;
#[doc = "STIM1 (rw) register accessor: an alias for `Reg<STIM1_SPEC>`"]
pub type STIM1 = crate::Reg<stim1::STIM1_SPEC>;
#[doc = "Stimulus Port 1"]
pub mod stim1;
#[doc = "STIM2 (rw) register accessor: an alias for `Reg<STIM2_SPEC>`"]
pub type STIM2 = crate::Reg<stim2::STIM2_SPEC>;
#[doc = "Stimulus Port 2"]
pub mod stim2;
#[doc = "STIM3 (rw) register accessor: an alias for `Reg<STIM3_SPEC>`"]
pub type STIM3 = crate::Reg<stim3::STIM3_SPEC>;
#[doc = "Stimulus Port 3"]
pub mod stim3;
#[doc = "STIM4 (rw) register accessor: an alias for `Reg<STIM4_SPEC>`"]
pub type STIM4 = crate::Reg<stim4::STIM4_SPEC>;
#[doc = "Stimulus Port 4"]
pub mod stim4;
#[doc = "STIM5 (rw) register accessor: an alias for `Reg<STIM5_SPEC>`"]
pub type STIM5 = crate::Reg<stim5::STIM5_SPEC>;
#[doc = "Stimulus Port 5"]
pub mod stim5;
#[doc = "STIM6 (rw) register accessor: an alias for `Reg<STIM6_SPEC>`"]
pub type STIM6 = crate::Reg<stim6::STIM6_SPEC>;
#[doc = "Stimulus Port 6"]
pub mod stim6;
#[doc = "STIM7 (rw) register accessor: an alias for `Reg<STIM7_SPEC>`"]
pub type STIM7 = crate::Reg<stim7::STIM7_SPEC>;
#[doc = "Stimulus Port 7"]
pub mod stim7;
#[doc = "STIM8 (rw) register accessor: an alias for `Reg<STIM8_SPEC>`"]
pub type STIM8 = crate::Reg<stim8::STIM8_SPEC>;
#[doc = "Stimulus Port 8"]
pub mod stim8;
#[doc = "STIM9 (rw) register accessor: an alias for `Reg<STIM9_SPEC>`"]
pub type STIM9 = crate::Reg<stim9::STIM9_SPEC>;
#[doc = "Stimulus Port 9"]
pub mod stim9;
#[doc = "STIM10 (rw) register accessor: an alias for `Reg<STIM10_SPEC>`"]
pub type STIM10 = crate::Reg<stim10::STIM10_SPEC>;
#[doc = "Stimulus Port 10"]
pub mod stim10;
#[doc = "STIM11 (rw) register accessor: an alias for `Reg<STIM11_SPEC>`"]
pub type STIM11 = crate::Reg<stim11::STIM11_SPEC>;
#[doc = "Stimulus Port 11"]
pub mod stim11;
#[doc = "STIM12 (rw) register accessor: an alias for `Reg<STIM12_SPEC>`"]
pub type STIM12 = crate::Reg<stim12::STIM12_SPEC>;
#[doc = "Stimulus Port 12"]
pub mod stim12;
#[doc = "STIM13 (rw) register accessor: an alias for `Reg<STIM13_SPEC>`"]
pub type STIM13 = crate::Reg<stim13::STIM13_SPEC>;
#[doc = "Stimulus Port 13"]
pub mod stim13;
#[doc = "STIM14 (rw) register accessor: an alias for `Reg<STIM14_SPEC>`"]
pub type STIM14 = crate::Reg<stim14::STIM14_SPEC>;
#[doc = "Stimulus Port 14"]
pub mod stim14;
#[doc = "STIM15 (rw) register accessor: an alias for `Reg<STIM15_SPEC>`"]
pub type STIM15 = crate::Reg<stim15::STIM15_SPEC>;
#[doc = "Stimulus Port 15"]
pub mod stim15;
#[doc = "STIM16 (rw) register accessor: an alias for `Reg<STIM16_SPEC>`"]
pub type STIM16 = crate::Reg<stim16::STIM16_SPEC>;
#[doc = "Stimulus Port 16"]
pub mod stim16;
#[doc = "STIM17 (rw) register accessor: an alias for `Reg<STIM17_SPEC>`"]
pub type STIM17 = crate::Reg<stim17::STIM17_SPEC>;
#[doc = "Stimulus Port 17"]
pub mod stim17;
#[doc = "STIM18 (rw) register accessor: an alias for `Reg<STIM18_SPEC>`"]
pub type STIM18 = crate::Reg<stim18::STIM18_SPEC>;
#[doc = "Stimulus Port 18"]
pub mod stim18;
#[doc = "STIM19 (rw) register accessor: an alias for `Reg<STIM19_SPEC>`"]
pub type STIM19 = crate::Reg<stim19::STIM19_SPEC>;
#[doc = "Stimulus Port 19"]
pub mod stim19;
#[doc = "STIM20 (rw) register accessor: an alias for `Reg<STIM20_SPEC>`"]
pub type STIM20 = crate::Reg<stim20::STIM20_SPEC>;
#[doc = "Stimulus Port 20"]
pub mod stim20;
#[doc = "STIM21 (rw) register accessor: an alias for `Reg<STIM21_SPEC>`"]
pub type STIM21 = crate::Reg<stim21::STIM21_SPEC>;
#[doc = "Stimulus Port 21"]
pub mod stim21;
#[doc = "STIM22 (rw) register accessor: an alias for `Reg<STIM22_SPEC>`"]
pub type STIM22 = crate::Reg<stim22::STIM22_SPEC>;
#[doc = "Stimulus Port 22"]
pub mod stim22;
#[doc = "STIM23 (rw) register accessor: an alias for `Reg<STIM23_SPEC>`"]
pub type STIM23 = crate::Reg<stim23::STIM23_SPEC>;
#[doc = "Stimulus Port 23"]
pub mod stim23;
#[doc = "STIM24 (rw) register accessor: an alias for `Reg<STIM24_SPEC>`"]
pub type STIM24 = crate::Reg<stim24::STIM24_SPEC>;
#[doc = "Stimulus Port 24"]
pub mod stim24;
#[doc = "STIM25 (rw) register accessor: an alias for `Reg<STIM25_SPEC>`"]
pub type STIM25 = crate::Reg<stim25::STIM25_SPEC>;
#[doc = "Stimulus Port 25"]
pub mod stim25;
#[doc = "STIM26 (rw) register accessor: an alias for `Reg<STIM26_SPEC>`"]
pub type STIM26 = crate::Reg<stim26::STIM26_SPEC>;
#[doc = "Stimulus Port 26"]
pub mod stim26;
#[doc = "STIM27 (rw) register accessor: an alias for `Reg<STIM27_SPEC>`"]
pub type STIM27 = crate::Reg<stim27::STIM27_SPEC>;
#[doc = "Stimulus Port 27"]
pub mod stim27;
#[doc = "STIM28 (rw) register accessor: an alias for `Reg<STIM28_SPEC>`"]
pub type STIM28 = crate::Reg<stim28::STIM28_SPEC>;
#[doc = "Stimulus Port 28"]
pub mod stim28;
#[doc = "STIM29 (rw) register accessor: an alias for `Reg<STIM29_SPEC>`"]
pub type STIM29 = crate::Reg<stim29::STIM29_SPEC>;
#[doc = "Stimulus Port 29"]
pub mod stim29;
#[doc = "STIM30 (rw) register accessor: an alias for `Reg<STIM30_SPEC>`"]
pub type STIM30 = crate::Reg<stim30::STIM30_SPEC>;
#[doc = "Stimulus Port 30"]
pub mod stim30;
#[doc = "STIM31 (rw) register accessor: an alias for `Reg<STIM31_SPEC>`"]
pub type STIM31 = crate::Reg<stim31::STIM31_SPEC>;
#[doc = "Stimulus Port 31"]
pub mod stim31;
#[doc = "TER (rw) register accessor: an alias for `Reg<TER_SPEC>`"]
pub type TER = crate::Reg<ter::TER_SPEC>;
#[doc = "Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required."]
pub mod ter;
#[doc = "TPR (rw) register accessor: an alias for `Reg<TPR_SPEC>`"]
pub type TPR = crate::Reg<tpr::TPR_SPEC>;
#[doc = "Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode."]
pub mod tpr;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set."]
pub mod tcr;
#[doc = "LAR (rw) register accessor: an alias for `Reg<LAR_SPEC>`"]
pub type LAR = crate::Reg<lar::LAR_SPEC>;
#[doc = "Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR."]
pub mod lar;
#[doc = "LSR (rw) register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Lock Status Use this register to enable write accesses to the Control Register."]
pub mod lsr;

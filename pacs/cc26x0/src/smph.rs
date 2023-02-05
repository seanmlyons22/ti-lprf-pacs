#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU SEMAPHORE 0"]
    pub smph0: SMPH0,
    #[doc = "0x04 - MCU SEMAPHORE 1"]
    pub smph1: SMPH1,
    #[doc = "0x08 - MCU SEMAPHORE 2"]
    pub smph2: SMPH2,
    #[doc = "0x0c - MCU SEMAPHORE 3"]
    pub smph3: SMPH3,
    #[doc = "0x10 - MCU SEMAPHORE 4"]
    pub smph4: SMPH4,
    #[doc = "0x14 - MCU SEMAPHORE 5"]
    pub smph5: SMPH5,
    #[doc = "0x18 - MCU SEMAPHORE 6"]
    pub smph6: SMPH6,
    #[doc = "0x1c - MCU SEMAPHORE 7"]
    pub smph7: SMPH7,
    #[doc = "0x20 - MCU SEMAPHORE 8"]
    pub smph8: SMPH8,
    #[doc = "0x24 - MCU SEMAPHORE 9"]
    pub smph9: SMPH9,
    #[doc = "0x28 - MCU SEMAPHORE 10"]
    pub smph10: SMPH10,
    #[doc = "0x2c - MCU SEMAPHORE 11"]
    pub smph11: SMPH11,
    #[doc = "0x30 - MCU SEMAPHORE 12"]
    pub smph12: SMPH12,
    #[doc = "0x34 - MCU SEMAPHORE 13"]
    pub smph13: SMPH13,
    #[doc = "0x38 - MCU SEMAPHORE 14"]
    pub smph14: SMPH14,
    #[doc = "0x3c - MCU SEMAPHORE 15"]
    pub smph15: SMPH15,
    #[doc = "0x40 - MCU SEMAPHORE 16"]
    pub smph16: SMPH16,
    #[doc = "0x44 - MCU SEMAPHORE 17"]
    pub smph17: SMPH17,
    #[doc = "0x48 - MCU SEMAPHORE 18"]
    pub smph18: SMPH18,
    #[doc = "0x4c - MCU SEMAPHORE 19"]
    pub smph19: SMPH19,
    #[doc = "0x50 - MCU SEMAPHORE 20"]
    pub smph20: SMPH20,
    #[doc = "0x54 - MCU SEMAPHORE 21"]
    pub smph21: SMPH21,
    #[doc = "0x58 - MCU SEMAPHORE 22"]
    pub smph22: SMPH22,
    #[doc = "0x5c - MCU SEMAPHORE 23"]
    pub smph23: SMPH23,
    #[doc = "0x60 - MCU SEMAPHORE 24"]
    pub smph24: SMPH24,
    #[doc = "0x64 - MCU SEMAPHORE 25"]
    pub smph25: SMPH25,
    #[doc = "0x68 - MCU SEMAPHORE 26"]
    pub smph26: SMPH26,
    #[doc = "0x6c - MCU SEMAPHORE 27"]
    pub smph27: SMPH27,
    #[doc = "0x70 - MCU SEMAPHORE 28"]
    pub smph28: SMPH28,
    #[doc = "0x74 - MCU SEMAPHORE 29"]
    pub smph29: SMPH29,
    #[doc = "0x78 - MCU SEMAPHORE 30"]
    pub smph30: SMPH30,
    #[doc = "0x7c - MCU SEMAPHORE 31"]
    pub smph31: SMPH31,
    _reserved32: [u8; 0x0780],
    #[doc = "0x800 - MCU SEMAPHORE 0 ALIAS"]
    pub peek0: PEEK0,
    #[doc = "0x804 - MCU SEMAPHORE 1 ALIAS"]
    pub peek1: PEEK1,
    #[doc = "0x808 - MCU SEMAPHORE 2 ALIAS"]
    pub peek2: PEEK2,
    #[doc = "0x80c - MCU SEMAPHORE 3 ALIAS"]
    pub peek3: PEEK3,
    #[doc = "0x810 - MCU SEMAPHORE 4 ALIAS"]
    pub peek4: PEEK4,
    #[doc = "0x814 - MCU SEMAPHORE 5 ALIAS"]
    pub peek5: PEEK5,
    #[doc = "0x818 - MCU SEMAPHORE 6 ALIAS"]
    pub peek6: PEEK6,
    #[doc = "0x81c - MCU SEMAPHORE 7 ALIAS"]
    pub peek7: PEEK7,
    #[doc = "0x820 - MCU SEMAPHORE 8 ALIAS"]
    pub peek8: PEEK8,
    #[doc = "0x824 - MCU SEMAPHORE 9 ALIAS"]
    pub peek9: PEEK9,
    #[doc = "0x828 - MCU SEMAPHORE 10 ALIAS"]
    pub peek10: PEEK10,
    #[doc = "0x82c - MCU SEMAPHORE 11 ALIAS"]
    pub peek11: PEEK11,
    #[doc = "0x830 - MCU SEMAPHORE 12 ALIAS"]
    pub peek12: PEEK12,
    #[doc = "0x834 - MCU SEMAPHORE 13 ALIAS"]
    pub peek13: PEEK13,
    #[doc = "0x838 - MCU SEMAPHORE 14 ALIAS"]
    pub peek14: PEEK14,
    #[doc = "0x83c - MCU SEMAPHORE 15 ALIAS"]
    pub peek15: PEEK15,
    #[doc = "0x840 - MCU SEMAPHORE 16 ALIAS"]
    pub peek16: PEEK16,
    #[doc = "0x844 - MCU SEMAPHORE 17 ALIAS"]
    pub peek17: PEEK17,
    #[doc = "0x848 - MCU SEMAPHORE 18 ALIAS"]
    pub peek18: PEEK18,
    #[doc = "0x84c - MCU SEMAPHORE 19 ALIAS"]
    pub peek19: PEEK19,
    #[doc = "0x850 - MCU SEMAPHORE 20 ALIAS"]
    pub peek20: PEEK20,
    #[doc = "0x854 - MCU SEMAPHORE 21 ALIAS"]
    pub peek21: PEEK21,
    #[doc = "0x858 - MCU SEMAPHORE 22 ALIAS"]
    pub peek22: PEEK22,
    #[doc = "0x85c - MCU SEMAPHORE 23 ALIAS"]
    pub peek23: PEEK23,
    #[doc = "0x860 - MCU SEMAPHORE 24 ALIAS"]
    pub peek24: PEEK24,
    #[doc = "0x864 - MCU SEMAPHORE 25 ALIAS"]
    pub peek25: PEEK25,
    #[doc = "0x868 - MCU SEMAPHORE 26 ALIAS"]
    pub peek26: PEEK26,
    #[doc = "0x86c - MCU SEMAPHORE 27 ALIAS"]
    pub peek27: PEEK27,
    #[doc = "0x870 - MCU SEMAPHORE 28 ALIAS"]
    pub peek28: PEEK28,
    #[doc = "0x874 - MCU SEMAPHORE 29 ALIAS"]
    pub peek29: PEEK29,
    #[doc = "0x878 - MCU SEMAPHORE 30 ALIAS"]
    pub peek30: PEEK30,
    #[doc = "0x87c - MCU SEMAPHORE 31 ALIAS"]
    pub peek31: PEEK31,
}
#[doc = "SMPH0 (rw) register accessor: an alias for `Reg<SMPH0_SPEC>`"]
pub type SMPH0 = crate::Reg<smph0::SMPH0_SPEC>;
#[doc = "MCU SEMAPHORE 0"]
pub mod smph0;
#[doc = "SMPH1 (rw) register accessor: an alias for `Reg<SMPH1_SPEC>`"]
pub type SMPH1 = crate::Reg<smph1::SMPH1_SPEC>;
#[doc = "MCU SEMAPHORE 1"]
pub mod smph1;
#[doc = "SMPH2 (rw) register accessor: an alias for `Reg<SMPH2_SPEC>`"]
pub type SMPH2 = crate::Reg<smph2::SMPH2_SPEC>;
#[doc = "MCU SEMAPHORE 2"]
pub mod smph2;
#[doc = "SMPH3 (rw) register accessor: an alias for `Reg<SMPH3_SPEC>`"]
pub type SMPH3 = crate::Reg<smph3::SMPH3_SPEC>;
#[doc = "MCU SEMAPHORE 3"]
pub mod smph3;
#[doc = "SMPH4 (rw) register accessor: an alias for `Reg<SMPH4_SPEC>`"]
pub type SMPH4 = crate::Reg<smph4::SMPH4_SPEC>;
#[doc = "MCU SEMAPHORE 4"]
pub mod smph4;
#[doc = "SMPH5 (rw) register accessor: an alias for `Reg<SMPH5_SPEC>`"]
pub type SMPH5 = crate::Reg<smph5::SMPH5_SPEC>;
#[doc = "MCU SEMAPHORE 5"]
pub mod smph5;
#[doc = "SMPH6 (rw) register accessor: an alias for `Reg<SMPH6_SPEC>`"]
pub type SMPH6 = crate::Reg<smph6::SMPH6_SPEC>;
#[doc = "MCU SEMAPHORE 6"]
pub mod smph6;
#[doc = "SMPH7 (rw) register accessor: an alias for `Reg<SMPH7_SPEC>`"]
pub type SMPH7 = crate::Reg<smph7::SMPH7_SPEC>;
#[doc = "MCU SEMAPHORE 7"]
pub mod smph7;
#[doc = "SMPH8 (rw) register accessor: an alias for `Reg<SMPH8_SPEC>`"]
pub type SMPH8 = crate::Reg<smph8::SMPH8_SPEC>;
#[doc = "MCU SEMAPHORE 8"]
pub mod smph8;
#[doc = "SMPH9 (rw) register accessor: an alias for `Reg<SMPH9_SPEC>`"]
pub type SMPH9 = crate::Reg<smph9::SMPH9_SPEC>;
#[doc = "MCU SEMAPHORE 9"]
pub mod smph9;
#[doc = "SMPH10 (rw) register accessor: an alias for `Reg<SMPH10_SPEC>`"]
pub type SMPH10 = crate::Reg<smph10::SMPH10_SPEC>;
#[doc = "MCU SEMAPHORE 10"]
pub mod smph10;
#[doc = "SMPH11 (rw) register accessor: an alias for `Reg<SMPH11_SPEC>`"]
pub type SMPH11 = crate::Reg<smph11::SMPH11_SPEC>;
#[doc = "MCU SEMAPHORE 11"]
pub mod smph11;
#[doc = "SMPH12 (rw) register accessor: an alias for `Reg<SMPH12_SPEC>`"]
pub type SMPH12 = crate::Reg<smph12::SMPH12_SPEC>;
#[doc = "MCU SEMAPHORE 12"]
pub mod smph12;
#[doc = "SMPH13 (rw) register accessor: an alias for `Reg<SMPH13_SPEC>`"]
pub type SMPH13 = crate::Reg<smph13::SMPH13_SPEC>;
#[doc = "MCU SEMAPHORE 13"]
pub mod smph13;
#[doc = "SMPH14 (rw) register accessor: an alias for `Reg<SMPH14_SPEC>`"]
pub type SMPH14 = crate::Reg<smph14::SMPH14_SPEC>;
#[doc = "MCU SEMAPHORE 14"]
pub mod smph14;
#[doc = "SMPH15 (rw) register accessor: an alias for `Reg<SMPH15_SPEC>`"]
pub type SMPH15 = crate::Reg<smph15::SMPH15_SPEC>;
#[doc = "MCU SEMAPHORE 15"]
pub mod smph15;
#[doc = "SMPH16 (rw) register accessor: an alias for `Reg<SMPH16_SPEC>`"]
pub type SMPH16 = crate::Reg<smph16::SMPH16_SPEC>;
#[doc = "MCU SEMAPHORE 16"]
pub mod smph16;
#[doc = "SMPH17 (rw) register accessor: an alias for `Reg<SMPH17_SPEC>`"]
pub type SMPH17 = crate::Reg<smph17::SMPH17_SPEC>;
#[doc = "MCU SEMAPHORE 17"]
pub mod smph17;
#[doc = "SMPH18 (rw) register accessor: an alias for `Reg<SMPH18_SPEC>`"]
pub type SMPH18 = crate::Reg<smph18::SMPH18_SPEC>;
#[doc = "MCU SEMAPHORE 18"]
pub mod smph18;
#[doc = "SMPH19 (rw) register accessor: an alias for `Reg<SMPH19_SPEC>`"]
pub type SMPH19 = crate::Reg<smph19::SMPH19_SPEC>;
#[doc = "MCU SEMAPHORE 19"]
pub mod smph19;
#[doc = "SMPH20 (rw) register accessor: an alias for `Reg<SMPH20_SPEC>`"]
pub type SMPH20 = crate::Reg<smph20::SMPH20_SPEC>;
#[doc = "MCU SEMAPHORE 20"]
pub mod smph20;
#[doc = "SMPH21 (rw) register accessor: an alias for `Reg<SMPH21_SPEC>`"]
pub type SMPH21 = crate::Reg<smph21::SMPH21_SPEC>;
#[doc = "MCU SEMAPHORE 21"]
pub mod smph21;
#[doc = "SMPH22 (rw) register accessor: an alias for `Reg<SMPH22_SPEC>`"]
pub type SMPH22 = crate::Reg<smph22::SMPH22_SPEC>;
#[doc = "MCU SEMAPHORE 22"]
pub mod smph22;
#[doc = "SMPH23 (rw) register accessor: an alias for `Reg<SMPH23_SPEC>`"]
pub type SMPH23 = crate::Reg<smph23::SMPH23_SPEC>;
#[doc = "MCU SEMAPHORE 23"]
pub mod smph23;
#[doc = "SMPH24 (rw) register accessor: an alias for `Reg<SMPH24_SPEC>`"]
pub type SMPH24 = crate::Reg<smph24::SMPH24_SPEC>;
#[doc = "MCU SEMAPHORE 24"]
pub mod smph24;
#[doc = "SMPH25 (rw) register accessor: an alias for `Reg<SMPH25_SPEC>`"]
pub type SMPH25 = crate::Reg<smph25::SMPH25_SPEC>;
#[doc = "MCU SEMAPHORE 25"]
pub mod smph25;
#[doc = "SMPH26 (rw) register accessor: an alias for `Reg<SMPH26_SPEC>`"]
pub type SMPH26 = crate::Reg<smph26::SMPH26_SPEC>;
#[doc = "MCU SEMAPHORE 26"]
pub mod smph26;
#[doc = "SMPH27 (rw) register accessor: an alias for `Reg<SMPH27_SPEC>`"]
pub type SMPH27 = crate::Reg<smph27::SMPH27_SPEC>;
#[doc = "MCU SEMAPHORE 27"]
pub mod smph27;
#[doc = "SMPH28 (rw) register accessor: an alias for `Reg<SMPH28_SPEC>`"]
pub type SMPH28 = crate::Reg<smph28::SMPH28_SPEC>;
#[doc = "MCU SEMAPHORE 28"]
pub mod smph28;
#[doc = "SMPH29 (rw) register accessor: an alias for `Reg<SMPH29_SPEC>`"]
pub type SMPH29 = crate::Reg<smph29::SMPH29_SPEC>;
#[doc = "MCU SEMAPHORE 29"]
pub mod smph29;
#[doc = "SMPH30 (rw) register accessor: an alias for `Reg<SMPH30_SPEC>`"]
pub type SMPH30 = crate::Reg<smph30::SMPH30_SPEC>;
#[doc = "MCU SEMAPHORE 30"]
pub mod smph30;
#[doc = "SMPH31 (rw) register accessor: an alias for `Reg<SMPH31_SPEC>`"]
pub type SMPH31 = crate::Reg<smph31::SMPH31_SPEC>;
#[doc = "MCU SEMAPHORE 31"]
pub mod smph31;
#[doc = "PEEK0 (rw) register accessor: an alias for `Reg<PEEK0_SPEC>`"]
pub type PEEK0 = crate::Reg<peek0::PEEK0_SPEC>;
#[doc = "MCU SEMAPHORE 0 ALIAS"]
pub mod peek0;
#[doc = "PEEK1 (rw) register accessor: an alias for `Reg<PEEK1_SPEC>`"]
pub type PEEK1 = crate::Reg<peek1::PEEK1_SPEC>;
#[doc = "MCU SEMAPHORE 1 ALIAS"]
pub mod peek1;
#[doc = "PEEK2 (rw) register accessor: an alias for `Reg<PEEK2_SPEC>`"]
pub type PEEK2 = crate::Reg<peek2::PEEK2_SPEC>;
#[doc = "MCU SEMAPHORE 2 ALIAS"]
pub mod peek2;
#[doc = "PEEK3 (rw) register accessor: an alias for `Reg<PEEK3_SPEC>`"]
pub type PEEK3 = crate::Reg<peek3::PEEK3_SPEC>;
#[doc = "MCU SEMAPHORE 3 ALIAS"]
pub mod peek3;
#[doc = "PEEK4 (rw) register accessor: an alias for `Reg<PEEK4_SPEC>`"]
pub type PEEK4 = crate::Reg<peek4::PEEK4_SPEC>;
#[doc = "MCU SEMAPHORE 4 ALIAS"]
pub mod peek4;
#[doc = "PEEK5 (rw) register accessor: an alias for `Reg<PEEK5_SPEC>`"]
pub type PEEK5 = crate::Reg<peek5::PEEK5_SPEC>;
#[doc = "MCU SEMAPHORE 5 ALIAS"]
pub mod peek5;
#[doc = "PEEK6 (rw) register accessor: an alias for `Reg<PEEK6_SPEC>`"]
pub type PEEK6 = crate::Reg<peek6::PEEK6_SPEC>;
#[doc = "MCU SEMAPHORE 6 ALIAS"]
pub mod peek6;
#[doc = "PEEK7 (rw) register accessor: an alias for `Reg<PEEK7_SPEC>`"]
pub type PEEK7 = crate::Reg<peek7::PEEK7_SPEC>;
#[doc = "MCU SEMAPHORE 7 ALIAS"]
pub mod peek7;
#[doc = "PEEK8 (rw) register accessor: an alias for `Reg<PEEK8_SPEC>`"]
pub type PEEK8 = crate::Reg<peek8::PEEK8_SPEC>;
#[doc = "MCU SEMAPHORE 8 ALIAS"]
pub mod peek8;
#[doc = "PEEK9 (rw) register accessor: an alias for `Reg<PEEK9_SPEC>`"]
pub type PEEK9 = crate::Reg<peek9::PEEK9_SPEC>;
#[doc = "MCU SEMAPHORE 9 ALIAS"]
pub mod peek9;
#[doc = "PEEK10 (rw) register accessor: an alias for `Reg<PEEK10_SPEC>`"]
pub type PEEK10 = crate::Reg<peek10::PEEK10_SPEC>;
#[doc = "MCU SEMAPHORE 10 ALIAS"]
pub mod peek10;
#[doc = "PEEK11 (rw) register accessor: an alias for `Reg<PEEK11_SPEC>`"]
pub type PEEK11 = crate::Reg<peek11::PEEK11_SPEC>;
#[doc = "MCU SEMAPHORE 11 ALIAS"]
pub mod peek11;
#[doc = "PEEK12 (rw) register accessor: an alias for `Reg<PEEK12_SPEC>`"]
pub type PEEK12 = crate::Reg<peek12::PEEK12_SPEC>;
#[doc = "MCU SEMAPHORE 12 ALIAS"]
pub mod peek12;
#[doc = "PEEK13 (rw) register accessor: an alias for `Reg<PEEK13_SPEC>`"]
pub type PEEK13 = crate::Reg<peek13::PEEK13_SPEC>;
#[doc = "MCU SEMAPHORE 13 ALIAS"]
pub mod peek13;
#[doc = "PEEK14 (rw) register accessor: an alias for `Reg<PEEK14_SPEC>`"]
pub type PEEK14 = crate::Reg<peek14::PEEK14_SPEC>;
#[doc = "MCU SEMAPHORE 14 ALIAS"]
pub mod peek14;
#[doc = "PEEK15 (rw) register accessor: an alias for `Reg<PEEK15_SPEC>`"]
pub type PEEK15 = crate::Reg<peek15::PEEK15_SPEC>;
#[doc = "MCU SEMAPHORE 15 ALIAS"]
pub mod peek15;
#[doc = "PEEK16 (rw) register accessor: an alias for `Reg<PEEK16_SPEC>`"]
pub type PEEK16 = crate::Reg<peek16::PEEK16_SPEC>;
#[doc = "MCU SEMAPHORE 16 ALIAS"]
pub mod peek16;
#[doc = "PEEK17 (rw) register accessor: an alias for `Reg<PEEK17_SPEC>`"]
pub type PEEK17 = crate::Reg<peek17::PEEK17_SPEC>;
#[doc = "MCU SEMAPHORE 17 ALIAS"]
pub mod peek17;
#[doc = "PEEK18 (rw) register accessor: an alias for `Reg<PEEK18_SPEC>`"]
pub type PEEK18 = crate::Reg<peek18::PEEK18_SPEC>;
#[doc = "MCU SEMAPHORE 18 ALIAS"]
pub mod peek18;
#[doc = "PEEK19 (rw) register accessor: an alias for `Reg<PEEK19_SPEC>`"]
pub type PEEK19 = crate::Reg<peek19::PEEK19_SPEC>;
#[doc = "MCU SEMAPHORE 19 ALIAS"]
pub mod peek19;
#[doc = "PEEK20 (rw) register accessor: an alias for `Reg<PEEK20_SPEC>`"]
pub type PEEK20 = crate::Reg<peek20::PEEK20_SPEC>;
#[doc = "MCU SEMAPHORE 20 ALIAS"]
pub mod peek20;
#[doc = "PEEK21 (rw) register accessor: an alias for `Reg<PEEK21_SPEC>`"]
pub type PEEK21 = crate::Reg<peek21::PEEK21_SPEC>;
#[doc = "MCU SEMAPHORE 21 ALIAS"]
pub mod peek21;
#[doc = "PEEK22 (rw) register accessor: an alias for `Reg<PEEK22_SPEC>`"]
pub type PEEK22 = crate::Reg<peek22::PEEK22_SPEC>;
#[doc = "MCU SEMAPHORE 22 ALIAS"]
pub mod peek22;
#[doc = "PEEK23 (rw) register accessor: an alias for `Reg<PEEK23_SPEC>`"]
pub type PEEK23 = crate::Reg<peek23::PEEK23_SPEC>;
#[doc = "MCU SEMAPHORE 23 ALIAS"]
pub mod peek23;
#[doc = "PEEK24 (rw) register accessor: an alias for `Reg<PEEK24_SPEC>`"]
pub type PEEK24 = crate::Reg<peek24::PEEK24_SPEC>;
#[doc = "MCU SEMAPHORE 24 ALIAS"]
pub mod peek24;
#[doc = "PEEK25 (rw) register accessor: an alias for `Reg<PEEK25_SPEC>`"]
pub type PEEK25 = crate::Reg<peek25::PEEK25_SPEC>;
#[doc = "MCU SEMAPHORE 25 ALIAS"]
pub mod peek25;
#[doc = "PEEK26 (rw) register accessor: an alias for `Reg<PEEK26_SPEC>`"]
pub type PEEK26 = crate::Reg<peek26::PEEK26_SPEC>;
#[doc = "MCU SEMAPHORE 26 ALIAS"]
pub mod peek26;
#[doc = "PEEK27 (rw) register accessor: an alias for `Reg<PEEK27_SPEC>`"]
pub type PEEK27 = crate::Reg<peek27::PEEK27_SPEC>;
#[doc = "MCU SEMAPHORE 27 ALIAS"]
pub mod peek27;
#[doc = "PEEK28 (rw) register accessor: an alias for `Reg<PEEK28_SPEC>`"]
pub type PEEK28 = crate::Reg<peek28::PEEK28_SPEC>;
#[doc = "MCU SEMAPHORE 28 ALIAS"]
pub mod peek28;
#[doc = "PEEK29 (rw) register accessor: an alias for `Reg<PEEK29_SPEC>`"]
pub type PEEK29 = crate::Reg<peek29::PEEK29_SPEC>;
#[doc = "MCU SEMAPHORE 29 ALIAS"]
pub mod peek29;
#[doc = "PEEK30 (rw) register accessor: an alias for `Reg<PEEK30_SPEC>`"]
pub type PEEK30 = crate::Reg<peek30::PEEK30_SPEC>;
#[doc = "MCU SEMAPHORE 30 ALIAS"]
pub mod peek30;
#[doc = "PEEK31 (rw) register accessor: an alias for `Reg<PEEK31_SPEC>`"]
pub type PEEK31 = crate::Reg<peek31::PEEK31_SPEC>;
#[doc = "MCU SEMAPHORE 31 ALIAS"]
pub mod peek31;

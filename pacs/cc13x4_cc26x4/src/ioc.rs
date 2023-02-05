#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration of DIO0"]
    pub iocfg0: IOCFG0,
    #[doc = "0x04 - Configuration of DIO1"]
    pub iocfg1: IOCFG1,
    #[doc = "0x08 - Configuration of DIO2"]
    pub iocfg2: IOCFG2,
    #[doc = "0x0c - Configuration of DIO3"]
    pub iocfg3: IOCFG3,
    #[doc = "0x10 - Configuration of DIO4"]
    pub iocfg4: IOCFG4,
    #[doc = "0x14 - Configuration of DIO5"]
    pub iocfg5: IOCFG5,
    #[doc = "0x18 - Configuration of DIO6"]
    pub iocfg6: IOCFG6,
    #[doc = "0x1c - Configuration of DIO7"]
    pub iocfg7: IOCFG7,
    #[doc = "0x20 - Configuration of DIO8"]
    pub iocfg8: IOCFG8,
    #[doc = "0x24 - Configuration of DIO9"]
    pub iocfg9: IOCFG9,
    #[doc = "0x28 - Configuration of DIO10"]
    pub iocfg10: IOCFG10,
    #[doc = "0x2c - Configuration of DIO11"]
    pub iocfg11: IOCFG11,
    #[doc = "0x30 - Configuration of DIO12"]
    pub iocfg12: IOCFG12,
    #[doc = "0x34 - Configuration of DIO13"]
    pub iocfg13: IOCFG13,
    #[doc = "0x38 - Configuration of DIO14"]
    pub iocfg14: IOCFG14,
    #[doc = "0x3c - Configuration of DIO15"]
    pub iocfg15: IOCFG15,
    #[doc = "0x40 - Configuration of DIO16"]
    pub iocfg16: IOCFG16,
    #[doc = "0x44 - Configuration of DIO17"]
    pub iocfg17: IOCFG17,
    #[doc = "0x48 - Configuration of DIO18"]
    pub iocfg18: IOCFG18,
    #[doc = "0x4c - Configuration of DIO19"]
    pub iocfg19: IOCFG19,
    #[doc = "0x50 - Configuration of DIO20"]
    pub iocfg20: IOCFG20,
    #[doc = "0x54 - Configuration of DIO21"]
    pub iocfg21: IOCFG21,
    #[doc = "0x58 - Configuration of DIO22"]
    pub iocfg22: IOCFG22,
    #[doc = "0x5c - Configuration of DIO23"]
    pub iocfg23: IOCFG23,
    #[doc = "0x60 - Configuration of DIO24"]
    pub iocfg24: IOCFG24,
    #[doc = "0x64 - Configuration of DIO25"]
    pub iocfg25: IOCFG25,
    #[doc = "0x68 - Configuration of DIO26"]
    pub iocfg26: IOCFG26,
    #[doc = "0x6c - Configuration of DIO27"]
    pub iocfg27: IOCFG27,
    #[doc = "0x70 - Configuration of DIO28"]
    pub iocfg28: IOCFG28,
    #[doc = "0x74 - Configuration of DIO29"]
    pub iocfg29: IOCFG29,
    #[doc = "0x78 - Configuration of DIO30"]
    pub iocfg30: IOCFG30,
    #[doc = "0x7c - Configuration of DIO31"]
    pub iocfg31: IOCFG31,
    #[doc = "0x80 - Configuration of DIO32"]
    pub iocfg32: IOCFG32,
    #[doc = "0x84 - Configuration of DIO33"]
    pub iocfg33: IOCFG33,
    #[doc = "0x88 - Configuration of DIO34"]
    pub iocfg34: IOCFG34,
    #[doc = "0x8c - Configuration of DIO35"]
    pub iocfg35: IOCFG35,
    #[doc = "0x90 - Configuration of DIO36"]
    pub iocfg36: IOCFG36,
    #[doc = "0x94 - Configuration of DIO37"]
    pub iocfg37: IOCFG37,
    #[doc = "0x98 - Configuration of DIO38"]
    pub iocfg38: IOCFG38,
    #[doc = "0x9c - Configuration of DIO39"]
    pub iocfg39: IOCFG39,
    #[doc = "0xa0 - Configuration of DIO40"]
    pub iocfg40: IOCFG40,
    #[doc = "0xa4 - Configuration of DIO41"]
    pub iocfg41: IOCFG41,
    #[doc = "0xa8 - Configuration of DIO42"]
    pub iocfg42: IOCFG42,
    #[doc = "0xac - Configuration of DIO43"]
    pub iocfg43: IOCFG43,
    #[doc = "0xb0 - Configuration of DIO44"]
    pub iocfg44: IOCFG44,
    #[doc = "0xb4 - Configuration of DIO45"]
    pub iocfg45: IOCFG45,
    #[doc = "0xb8 - Configuration of DIO46"]
    pub iocfg46: IOCFG46,
    #[doc = "0xbc - Configuration of DIO47"]
    pub iocfg47: IOCFG47,
}
#[doc = "IOCFG0 (rw) register accessor: an alias for `Reg<IOCFG0_SPEC>`"]
pub type IOCFG0 = crate::Reg<iocfg0::IOCFG0_SPEC>;
#[doc = "Configuration of DIO0"]
pub mod iocfg0;
#[doc = "IOCFG1 (rw) register accessor: an alias for `Reg<IOCFG1_SPEC>`"]
pub type IOCFG1 = crate::Reg<iocfg1::IOCFG1_SPEC>;
#[doc = "Configuration of DIO1"]
pub mod iocfg1;
#[doc = "IOCFG2 (rw) register accessor: an alias for `Reg<IOCFG2_SPEC>`"]
pub type IOCFG2 = crate::Reg<iocfg2::IOCFG2_SPEC>;
#[doc = "Configuration of DIO2"]
pub mod iocfg2;
#[doc = "IOCFG3 (rw) register accessor: an alias for `Reg<IOCFG3_SPEC>`"]
pub type IOCFG3 = crate::Reg<iocfg3::IOCFG3_SPEC>;
#[doc = "Configuration of DIO3"]
pub mod iocfg3;
#[doc = "IOCFG4 (rw) register accessor: an alias for `Reg<IOCFG4_SPEC>`"]
pub type IOCFG4 = crate::Reg<iocfg4::IOCFG4_SPEC>;
#[doc = "Configuration of DIO4"]
pub mod iocfg4;
#[doc = "IOCFG5 (rw) register accessor: an alias for `Reg<IOCFG5_SPEC>`"]
pub type IOCFG5 = crate::Reg<iocfg5::IOCFG5_SPEC>;
#[doc = "Configuration of DIO5"]
pub mod iocfg5;
#[doc = "IOCFG6 (rw) register accessor: an alias for `Reg<IOCFG6_SPEC>`"]
pub type IOCFG6 = crate::Reg<iocfg6::IOCFG6_SPEC>;
#[doc = "Configuration of DIO6"]
pub mod iocfg6;
#[doc = "IOCFG7 (rw) register accessor: an alias for `Reg<IOCFG7_SPEC>`"]
pub type IOCFG7 = crate::Reg<iocfg7::IOCFG7_SPEC>;
#[doc = "Configuration of DIO7"]
pub mod iocfg7;
#[doc = "IOCFG8 (rw) register accessor: an alias for `Reg<IOCFG8_SPEC>`"]
pub type IOCFG8 = crate::Reg<iocfg8::IOCFG8_SPEC>;
#[doc = "Configuration of DIO8"]
pub mod iocfg8;
#[doc = "IOCFG9 (rw) register accessor: an alias for `Reg<IOCFG9_SPEC>`"]
pub type IOCFG9 = crate::Reg<iocfg9::IOCFG9_SPEC>;
#[doc = "Configuration of DIO9"]
pub mod iocfg9;
#[doc = "IOCFG10 (rw) register accessor: an alias for `Reg<IOCFG10_SPEC>`"]
pub type IOCFG10 = crate::Reg<iocfg10::IOCFG10_SPEC>;
#[doc = "Configuration of DIO10"]
pub mod iocfg10;
#[doc = "IOCFG11 (rw) register accessor: an alias for `Reg<IOCFG11_SPEC>`"]
pub type IOCFG11 = crate::Reg<iocfg11::IOCFG11_SPEC>;
#[doc = "Configuration of DIO11"]
pub mod iocfg11;
#[doc = "IOCFG12 (rw) register accessor: an alias for `Reg<IOCFG12_SPEC>`"]
pub type IOCFG12 = crate::Reg<iocfg12::IOCFG12_SPEC>;
#[doc = "Configuration of DIO12"]
pub mod iocfg12;
#[doc = "IOCFG13 (rw) register accessor: an alias for `Reg<IOCFG13_SPEC>`"]
pub type IOCFG13 = crate::Reg<iocfg13::IOCFG13_SPEC>;
#[doc = "Configuration of DIO13"]
pub mod iocfg13;
#[doc = "IOCFG14 (rw) register accessor: an alias for `Reg<IOCFG14_SPEC>`"]
pub type IOCFG14 = crate::Reg<iocfg14::IOCFG14_SPEC>;
#[doc = "Configuration of DIO14"]
pub mod iocfg14;
#[doc = "IOCFG15 (rw) register accessor: an alias for `Reg<IOCFG15_SPEC>`"]
pub type IOCFG15 = crate::Reg<iocfg15::IOCFG15_SPEC>;
#[doc = "Configuration of DIO15"]
pub mod iocfg15;
#[doc = "IOCFG16 (rw) register accessor: an alias for `Reg<IOCFG16_SPEC>`"]
pub type IOCFG16 = crate::Reg<iocfg16::IOCFG16_SPEC>;
#[doc = "Configuration of DIO16"]
pub mod iocfg16;
#[doc = "IOCFG17 (rw) register accessor: an alias for `Reg<IOCFG17_SPEC>`"]
pub type IOCFG17 = crate::Reg<iocfg17::IOCFG17_SPEC>;
#[doc = "Configuration of DIO17"]
pub mod iocfg17;
#[doc = "IOCFG18 (rw) register accessor: an alias for `Reg<IOCFG18_SPEC>`"]
pub type IOCFG18 = crate::Reg<iocfg18::IOCFG18_SPEC>;
#[doc = "Configuration of DIO18"]
pub mod iocfg18;
#[doc = "IOCFG19 (rw) register accessor: an alias for `Reg<IOCFG19_SPEC>`"]
pub type IOCFG19 = crate::Reg<iocfg19::IOCFG19_SPEC>;
#[doc = "Configuration of DIO19"]
pub mod iocfg19;
#[doc = "IOCFG20 (rw) register accessor: an alias for `Reg<IOCFG20_SPEC>`"]
pub type IOCFG20 = crate::Reg<iocfg20::IOCFG20_SPEC>;
#[doc = "Configuration of DIO20"]
pub mod iocfg20;
#[doc = "IOCFG21 (rw) register accessor: an alias for `Reg<IOCFG21_SPEC>`"]
pub type IOCFG21 = crate::Reg<iocfg21::IOCFG21_SPEC>;
#[doc = "Configuration of DIO21"]
pub mod iocfg21;
#[doc = "IOCFG22 (rw) register accessor: an alias for `Reg<IOCFG22_SPEC>`"]
pub type IOCFG22 = crate::Reg<iocfg22::IOCFG22_SPEC>;
#[doc = "Configuration of DIO22"]
pub mod iocfg22;
#[doc = "IOCFG23 (rw) register accessor: an alias for `Reg<IOCFG23_SPEC>`"]
pub type IOCFG23 = crate::Reg<iocfg23::IOCFG23_SPEC>;
#[doc = "Configuration of DIO23"]
pub mod iocfg23;
#[doc = "IOCFG24 (rw) register accessor: an alias for `Reg<IOCFG24_SPEC>`"]
pub type IOCFG24 = crate::Reg<iocfg24::IOCFG24_SPEC>;
#[doc = "Configuration of DIO24"]
pub mod iocfg24;
#[doc = "IOCFG25 (rw) register accessor: an alias for `Reg<IOCFG25_SPEC>`"]
pub type IOCFG25 = crate::Reg<iocfg25::IOCFG25_SPEC>;
#[doc = "Configuration of DIO25"]
pub mod iocfg25;
#[doc = "IOCFG26 (rw) register accessor: an alias for `Reg<IOCFG26_SPEC>`"]
pub type IOCFG26 = crate::Reg<iocfg26::IOCFG26_SPEC>;
#[doc = "Configuration of DIO26"]
pub mod iocfg26;
#[doc = "IOCFG27 (rw) register accessor: an alias for `Reg<IOCFG27_SPEC>`"]
pub type IOCFG27 = crate::Reg<iocfg27::IOCFG27_SPEC>;
#[doc = "Configuration of DIO27"]
pub mod iocfg27;
#[doc = "IOCFG28 (rw) register accessor: an alias for `Reg<IOCFG28_SPEC>`"]
pub type IOCFG28 = crate::Reg<iocfg28::IOCFG28_SPEC>;
#[doc = "Configuration of DIO28"]
pub mod iocfg28;
#[doc = "IOCFG29 (rw) register accessor: an alias for `Reg<IOCFG29_SPEC>`"]
pub type IOCFG29 = crate::Reg<iocfg29::IOCFG29_SPEC>;
#[doc = "Configuration of DIO29"]
pub mod iocfg29;
#[doc = "IOCFG30 (rw) register accessor: an alias for `Reg<IOCFG30_SPEC>`"]
pub type IOCFG30 = crate::Reg<iocfg30::IOCFG30_SPEC>;
#[doc = "Configuration of DIO30"]
pub mod iocfg30;
#[doc = "IOCFG31 (rw) register accessor: an alias for `Reg<IOCFG31_SPEC>`"]
pub type IOCFG31 = crate::Reg<iocfg31::IOCFG31_SPEC>;
#[doc = "Configuration of DIO31"]
pub mod iocfg31;
#[doc = "IOCFG32 (rw) register accessor: an alias for `Reg<IOCFG32_SPEC>`"]
pub type IOCFG32 = crate::Reg<iocfg32::IOCFG32_SPEC>;
#[doc = "Configuration of DIO32"]
pub mod iocfg32;
#[doc = "IOCFG33 (rw) register accessor: an alias for `Reg<IOCFG33_SPEC>`"]
pub type IOCFG33 = crate::Reg<iocfg33::IOCFG33_SPEC>;
#[doc = "Configuration of DIO33"]
pub mod iocfg33;
#[doc = "IOCFG34 (rw) register accessor: an alias for `Reg<IOCFG34_SPEC>`"]
pub type IOCFG34 = crate::Reg<iocfg34::IOCFG34_SPEC>;
#[doc = "Configuration of DIO34"]
pub mod iocfg34;
#[doc = "IOCFG35 (rw) register accessor: an alias for `Reg<IOCFG35_SPEC>`"]
pub type IOCFG35 = crate::Reg<iocfg35::IOCFG35_SPEC>;
#[doc = "Configuration of DIO35"]
pub mod iocfg35;
#[doc = "IOCFG36 (rw) register accessor: an alias for `Reg<IOCFG36_SPEC>`"]
pub type IOCFG36 = crate::Reg<iocfg36::IOCFG36_SPEC>;
#[doc = "Configuration of DIO36"]
pub mod iocfg36;
#[doc = "IOCFG37 (rw) register accessor: an alias for `Reg<IOCFG37_SPEC>`"]
pub type IOCFG37 = crate::Reg<iocfg37::IOCFG37_SPEC>;
#[doc = "Configuration of DIO37"]
pub mod iocfg37;
#[doc = "IOCFG38 (rw) register accessor: an alias for `Reg<IOCFG38_SPEC>`"]
pub type IOCFG38 = crate::Reg<iocfg38::IOCFG38_SPEC>;
#[doc = "Configuration of DIO38"]
pub mod iocfg38;
#[doc = "IOCFG39 (rw) register accessor: an alias for `Reg<IOCFG39_SPEC>`"]
pub type IOCFG39 = crate::Reg<iocfg39::IOCFG39_SPEC>;
#[doc = "Configuration of DIO39"]
pub mod iocfg39;
#[doc = "IOCFG40 (rw) register accessor: an alias for `Reg<IOCFG40_SPEC>`"]
pub type IOCFG40 = crate::Reg<iocfg40::IOCFG40_SPEC>;
#[doc = "Configuration of DIO40"]
pub mod iocfg40;
#[doc = "IOCFG41 (rw) register accessor: an alias for `Reg<IOCFG41_SPEC>`"]
pub type IOCFG41 = crate::Reg<iocfg41::IOCFG41_SPEC>;
#[doc = "Configuration of DIO41"]
pub mod iocfg41;
#[doc = "IOCFG42 (rw) register accessor: an alias for `Reg<IOCFG42_SPEC>`"]
pub type IOCFG42 = crate::Reg<iocfg42::IOCFG42_SPEC>;
#[doc = "Configuration of DIO42"]
pub mod iocfg42;
#[doc = "IOCFG43 (rw) register accessor: an alias for `Reg<IOCFG43_SPEC>`"]
pub type IOCFG43 = crate::Reg<iocfg43::IOCFG43_SPEC>;
#[doc = "Configuration of DIO43"]
pub mod iocfg43;
#[doc = "IOCFG44 (rw) register accessor: an alias for `Reg<IOCFG44_SPEC>`"]
pub type IOCFG44 = crate::Reg<iocfg44::IOCFG44_SPEC>;
#[doc = "Configuration of DIO44"]
pub mod iocfg44;
#[doc = "IOCFG45 (rw) register accessor: an alias for `Reg<IOCFG45_SPEC>`"]
pub type IOCFG45 = crate::Reg<iocfg45::IOCFG45_SPEC>;
#[doc = "Configuration of DIO45"]
pub mod iocfg45;
#[doc = "IOCFG46 (rw) register accessor: an alias for `Reg<IOCFG46_SPEC>`"]
pub type IOCFG46 = crate::Reg<iocfg46::IOCFG46_SPEC>;
#[doc = "Configuration of DIO46"]
pub mod iocfg46;
#[doc = "IOCFG47 (rw) register accessor: an alias for `Reg<IOCFG47_SPEC>`"]
pub type IOCFG47 = crate::Reg<iocfg47::IOCFG47_SPEC>;
#[doc = "Configuration of DIO47"]
pub mod iocfg47;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Signed Operand 0"]
    pub op0s: OP0S,
    #[doc = "0x04 - Unsigned Operand 0"]
    pub op0u: OP0U,
    #[doc = "0x08 - Signed Operand 1 and Multiply"]
    pub op1smul: OP1SMUL,
    #[doc = "0x0c - Unsigned Operand 1 and Multiply"]
    pub op1umul: OP1UMUL,
    #[doc = "0x10 - Signed Operand 1 and Multiply-Accumulate"]
    pub op1smac: OP1SMAC,
    #[doc = "0x14 - Unsigned Operand 1 and Multiply-Accumulate"]
    pub op1umac: OP1UMAC,
    #[doc = "0x18 - Signed Operand 1 and 16-bit Addition"]
    pub op1sadd16: OP1SADD16,
    #[doc = "0x1c - Unsigned Operand 1 and 16-bit Addition"]
    pub op1uadd16: OP1UADD16,
    #[doc = "0x20 - Signed Operand 1 and 32-bit Addition"]
    pub op1sadd32: OP1SADD32,
    #[doc = "0x24 - Unsigned Operand 1 and 32-bit Addition"]
    pub op1uadd32: OP1UADD32,
    #[doc = "0x28 - Count Leading Zero"]
    pub clz: CLZ,
    #[doc = "0x2c - Count Leading Sign"]
    pub cls: CLS,
    #[doc = "0x30 - Accumulator Shift Only one shift operation can be triggered per register write."]
    pub accshift: ACCSHIFT,
    #[doc = "0x34 - Accumulator Reset"]
    pub accreset: ACCRESET,
    #[doc = "0x38 - Accumulator Bits 15:0"]
    pub acc15_0: ACC15_0,
    #[doc = "0x3c - Accumulator Bits 16:1"]
    pub acc16_1: ACC16_1,
    #[doc = "0x40 - Accumulator Bits 17:2"]
    pub acc17_2: ACC17_2,
    #[doc = "0x44 - Accumulator Bits 18:3"]
    pub acc18_3: ACC18_3,
    #[doc = "0x48 - Accumulator Bits 19:4"]
    pub acc19_4: ACC19_4,
    #[doc = "0x4c - Accumulator Bits 20:5"]
    pub acc20_5: ACC20_5,
    #[doc = "0x50 - Accumulator Bits 21:6"]
    pub acc21_6: ACC21_6,
    #[doc = "0x54 - Accumulator Bits 22:7"]
    pub acc22_7: ACC22_7,
    #[doc = "0x58 - Accumulator Bits 23:8"]
    pub acc23_8: ACC23_8,
    #[doc = "0x5c - Accumulator Bits 24:9"]
    pub acc24_9: ACC24_9,
    #[doc = "0x60 - Accumulator Bits 25:10"]
    pub acc25_10: ACC25_10,
    #[doc = "0x64 - Accumulator Bits 26:11"]
    pub acc26_11: ACC26_11,
    #[doc = "0x68 - Accumulator Bits 27:12"]
    pub acc27_12: ACC27_12,
    #[doc = "0x6c - Accumulator Bits 28:13"]
    pub acc28_13: ACC28_13,
    #[doc = "0x70 - Accumulator Bits 29:14"]
    pub acc29_14: ACC29_14,
    #[doc = "0x74 - Accumulator Bits 30:15"]
    pub acc30_15: ACC30_15,
    #[doc = "0x78 - Accumulator Bits 31:16"]
    pub acc31_16: ACC31_16,
    #[doc = "0x7c - Accumulator Bits 32:17"]
    pub acc32_17: ACC32_17,
    #[doc = "0x80 - Accumulator Bits 33:18"]
    pub acc33_18: ACC33_18,
    #[doc = "0x84 - Accumulator Bits 34:19"]
    pub acc34_19: ACC34_19,
    #[doc = "0x88 - Accumulator Bits 35:20"]
    pub acc35_20: ACC35_20,
    #[doc = "0x8c - Accumulator Bits 36:21"]
    pub acc36_21: ACC36_21,
    #[doc = "0x90 - Accumulator Bits 37:22"]
    pub acc37_22: ACC37_22,
    #[doc = "0x94 - Accumulator Bits 38:23"]
    pub acc38_23: ACC38_23,
    #[doc = "0x98 - Accumulator Bits 39:24"]
    pub acc39_24: ACC39_24,
    #[doc = "0x9c - Accumulator Bits 39:32"]
    pub acc39_32: ACC39_32,
}
#[doc = "OP0S (rw) register accessor: an alias for `Reg<OP0S_SPEC>`"]
pub type OP0S = crate::Reg<op0s::OP0S_SPEC>;
#[doc = "Signed Operand 0"]
pub mod op0s;
#[doc = "OP0U (rw) register accessor: an alias for `Reg<OP0U_SPEC>`"]
pub type OP0U = crate::Reg<op0u::OP0U_SPEC>;
#[doc = "Unsigned Operand 0"]
pub mod op0u;
#[doc = "OP1SMUL (rw) register accessor: an alias for `Reg<OP1SMUL_SPEC>`"]
pub type OP1SMUL = crate::Reg<op1smul::OP1SMUL_SPEC>;
#[doc = "Signed Operand 1 and Multiply"]
pub mod op1smul;
#[doc = "OP1UMUL (rw) register accessor: an alias for `Reg<OP1UMUL_SPEC>`"]
pub type OP1UMUL = crate::Reg<op1umul::OP1UMUL_SPEC>;
#[doc = "Unsigned Operand 1 and Multiply"]
pub mod op1umul;
#[doc = "OP1SMAC (rw) register accessor: an alias for `Reg<OP1SMAC_SPEC>`"]
pub type OP1SMAC = crate::Reg<op1smac::OP1SMAC_SPEC>;
#[doc = "Signed Operand 1 and Multiply-Accumulate"]
pub mod op1smac;
#[doc = "OP1UMAC (rw) register accessor: an alias for `Reg<OP1UMAC_SPEC>`"]
pub type OP1UMAC = crate::Reg<op1umac::OP1UMAC_SPEC>;
#[doc = "Unsigned Operand 1 and Multiply-Accumulate"]
pub mod op1umac;
#[doc = "OP1SADD16 (rw) register accessor: an alias for `Reg<OP1SADD16_SPEC>`"]
pub type OP1SADD16 = crate::Reg<op1sadd16::OP1SADD16_SPEC>;
#[doc = "Signed Operand 1 and 16-bit Addition"]
pub mod op1sadd16;
#[doc = "OP1UADD16 (rw) register accessor: an alias for `Reg<OP1UADD16_SPEC>`"]
pub type OP1UADD16 = crate::Reg<op1uadd16::OP1UADD16_SPEC>;
#[doc = "Unsigned Operand 1 and 16-bit Addition"]
pub mod op1uadd16;
#[doc = "OP1SADD32 (rw) register accessor: an alias for `Reg<OP1SADD32_SPEC>`"]
pub type OP1SADD32 = crate::Reg<op1sadd32::OP1SADD32_SPEC>;
#[doc = "Signed Operand 1 and 32-bit Addition"]
pub mod op1sadd32;
#[doc = "OP1UADD32 (rw) register accessor: an alias for `Reg<OP1UADD32_SPEC>`"]
pub type OP1UADD32 = crate::Reg<op1uadd32::OP1UADD32_SPEC>;
#[doc = "Unsigned Operand 1 and 32-bit Addition"]
pub mod op1uadd32;
#[doc = "CLZ (rw) register accessor: an alias for `Reg<CLZ_SPEC>`"]
pub type CLZ = crate::Reg<clz::CLZ_SPEC>;
#[doc = "Count Leading Zero"]
pub mod clz;
#[doc = "CLS (rw) register accessor: an alias for `Reg<CLS_SPEC>`"]
pub type CLS = crate::Reg<cls::CLS_SPEC>;
#[doc = "Count Leading Sign"]
pub mod cls;
#[doc = "ACCSHIFT (rw) register accessor: an alias for `Reg<ACCSHIFT_SPEC>`"]
pub type ACCSHIFT = crate::Reg<accshift::ACCSHIFT_SPEC>;
#[doc = "Accumulator Shift Only one shift operation can be triggered per register write."]
pub mod accshift;
#[doc = "ACCRESET (rw) register accessor: an alias for `Reg<ACCRESET_SPEC>`"]
pub type ACCRESET = crate::Reg<accreset::ACCRESET_SPEC>;
#[doc = "Accumulator Reset"]
pub mod accreset;
#[doc = "ACC15_0 (rw) register accessor: an alias for `Reg<ACC15_0_SPEC>`"]
pub type ACC15_0 = crate::Reg<acc15_0::ACC15_0_SPEC>;
#[doc = "Accumulator Bits 15:0"]
pub mod acc15_0;
#[doc = "ACC16_1 (rw) register accessor: an alias for `Reg<ACC16_1_SPEC>`"]
pub type ACC16_1 = crate::Reg<acc16_1::ACC16_1_SPEC>;
#[doc = "Accumulator Bits 16:1"]
pub mod acc16_1;
#[doc = "ACC17_2 (rw) register accessor: an alias for `Reg<ACC17_2_SPEC>`"]
pub type ACC17_2 = crate::Reg<acc17_2::ACC17_2_SPEC>;
#[doc = "Accumulator Bits 17:2"]
pub mod acc17_2;
#[doc = "ACC18_3 (rw) register accessor: an alias for `Reg<ACC18_3_SPEC>`"]
pub type ACC18_3 = crate::Reg<acc18_3::ACC18_3_SPEC>;
#[doc = "Accumulator Bits 18:3"]
pub mod acc18_3;
#[doc = "ACC19_4 (rw) register accessor: an alias for `Reg<ACC19_4_SPEC>`"]
pub type ACC19_4 = crate::Reg<acc19_4::ACC19_4_SPEC>;
#[doc = "Accumulator Bits 19:4"]
pub mod acc19_4;
#[doc = "ACC20_5 (rw) register accessor: an alias for `Reg<ACC20_5_SPEC>`"]
pub type ACC20_5 = crate::Reg<acc20_5::ACC20_5_SPEC>;
#[doc = "Accumulator Bits 20:5"]
pub mod acc20_5;
#[doc = "ACC21_6 (rw) register accessor: an alias for `Reg<ACC21_6_SPEC>`"]
pub type ACC21_6 = crate::Reg<acc21_6::ACC21_6_SPEC>;
#[doc = "Accumulator Bits 21:6"]
pub mod acc21_6;
#[doc = "ACC22_7 (rw) register accessor: an alias for `Reg<ACC22_7_SPEC>`"]
pub type ACC22_7 = crate::Reg<acc22_7::ACC22_7_SPEC>;
#[doc = "Accumulator Bits 22:7"]
pub mod acc22_7;
#[doc = "ACC23_8 (rw) register accessor: an alias for `Reg<ACC23_8_SPEC>`"]
pub type ACC23_8 = crate::Reg<acc23_8::ACC23_8_SPEC>;
#[doc = "Accumulator Bits 23:8"]
pub mod acc23_8;
#[doc = "ACC24_9 (rw) register accessor: an alias for `Reg<ACC24_9_SPEC>`"]
pub type ACC24_9 = crate::Reg<acc24_9::ACC24_9_SPEC>;
#[doc = "Accumulator Bits 24:9"]
pub mod acc24_9;
#[doc = "ACC25_10 (rw) register accessor: an alias for `Reg<ACC25_10_SPEC>`"]
pub type ACC25_10 = crate::Reg<acc25_10::ACC25_10_SPEC>;
#[doc = "Accumulator Bits 25:10"]
pub mod acc25_10;
#[doc = "ACC26_11 (rw) register accessor: an alias for `Reg<ACC26_11_SPEC>`"]
pub type ACC26_11 = crate::Reg<acc26_11::ACC26_11_SPEC>;
#[doc = "Accumulator Bits 26:11"]
pub mod acc26_11;
#[doc = "ACC27_12 (rw) register accessor: an alias for `Reg<ACC27_12_SPEC>`"]
pub type ACC27_12 = crate::Reg<acc27_12::ACC27_12_SPEC>;
#[doc = "Accumulator Bits 27:12"]
pub mod acc27_12;
#[doc = "ACC28_13 (rw) register accessor: an alias for `Reg<ACC28_13_SPEC>`"]
pub type ACC28_13 = crate::Reg<acc28_13::ACC28_13_SPEC>;
#[doc = "Accumulator Bits 28:13"]
pub mod acc28_13;
#[doc = "ACC29_14 (rw) register accessor: an alias for `Reg<ACC29_14_SPEC>`"]
pub type ACC29_14 = crate::Reg<acc29_14::ACC29_14_SPEC>;
#[doc = "Accumulator Bits 29:14"]
pub mod acc29_14;
#[doc = "ACC30_15 (rw) register accessor: an alias for `Reg<ACC30_15_SPEC>`"]
pub type ACC30_15 = crate::Reg<acc30_15::ACC30_15_SPEC>;
#[doc = "Accumulator Bits 30:15"]
pub mod acc30_15;
#[doc = "ACC31_16 (rw) register accessor: an alias for `Reg<ACC31_16_SPEC>`"]
pub type ACC31_16 = crate::Reg<acc31_16::ACC31_16_SPEC>;
#[doc = "Accumulator Bits 31:16"]
pub mod acc31_16;
#[doc = "ACC32_17 (rw) register accessor: an alias for `Reg<ACC32_17_SPEC>`"]
pub type ACC32_17 = crate::Reg<acc32_17::ACC32_17_SPEC>;
#[doc = "Accumulator Bits 32:17"]
pub mod acc32_17;
#[doc = "ACC33_18 (rw) register accessor: an alias for `Reg<ACC33_18_SPEC>`"]
pub type ACC33_18 = crate::Reg<acc33_18::ACC33_18_SPEC>;
#[doc = "Accumulator Bits 33:18"]
pub mod acc33_18;
#[doc = "ACC34_19 (rw) register accessor: an alias for `Reg<ACC34_19_SPEC>`"]
pub type ACC34_19 = crate::Reg<acc34_19::ACC34_19_SPEC>;
#[doc = "Accumulator Bits 34:19"]
pub mod acc34_19;
#[doc = "ACC35_20 (rw) register accessor: an alias for `Reg<ACC35_20_SPEC>`"]
pub type ACC35_20 = crate::Reg<acc35_20::ACC35_20_SPEC>;
#[doc = "Accumulator Bits 35:20"]
pub mod acc35_20;
#[doc = "ACC36_21 (rw) register accessor: an alias for `Reg<ACC36_21_SPEC>`"]
pub type ACC36_21 = crate::Reg<acc36_21::ACC36_21_SPEC>;
#[doc = "Accumulator Bits 36:21"]
pub mod acc36_21;
#[doc = "ACC37_22 (rw) register accessor: an alias for `Reg<ACC37_22_SPEC>`"]
pub type ACC37_22 = crate::Reg<acc37_22::ACC37_22_SPEC>;
#[doc = "Accumulator Bits 37:22"]
pub mod acc37_22;
#[doc = "ACC38_23 (rw) register accessor: an alias for `Reg<ACC38_23_SPEC>`"]
pub type ACC38_23 = crate::Reg<acc38_23::ACC38_23_SPEC>;
#[doc = "Accumulator Bits 38:23"]
pub mod acc38_23;
#[doc = "ACC39_24 (rw) register accessor: an alias for `Reg<ACC39_24_SPEC>`"]
pub type ACC39_24 = crate::Reg<acc39_24::ACC39_24_SPEC>;
#[doc = "Accumulator Bits 39:24"]
pub mod acc39_24;
#[doc = "ACC39_32 (rw) register accessor: an alias for `Reg<ACC39_32_SPEC>`"]
pub type ACC39_32 = crate::Reg<acc39_32::ACC39_32_SPEC>;
#[doc = "Accumulator Bits 39:32"]
pub mod acc39_32;

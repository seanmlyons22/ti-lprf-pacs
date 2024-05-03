#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    op0s: Op0s,
    op0u: Op0u,
    op1smul: Op1smul,
    op1umul: Op1umul,
    op1smac: Op1smac,
    op1umac: Op1umac,
    op1sadd16: Op1sadd16,
    op1uadd16: Op1uadd16,
    op1sadd32: Op1sadd32,
    op1uadd32: Op1uadd32,
    clz: Clz,
    cls: Cls,
    accshift: Accshift,
    accreset: Accreset,
    acc15_0: Acc15_0,
    acc16_1: Acc16_1,
    acc17_2: Acc17_2,
    acc18_3: Acc18_3,
    acc19_4: Acc19_4,
    acc20_5: Acc20_5,
    acc21_6: Acc21_6,
    acc22_7: Acc22_7,
    acc23_8: Acc23_8,
    acc24_9: Acc24_9,
    acc25_10: Acc25_10,
    acc26_11: Acc26_11,
    acc27_12: Acc27_12,
    acc28_13: Acc28_13,
    acc29_14: Acc29_14,
    acc30_15: Acc30_15,
    acc31_16: Acc31_16,
    acc32_17: Acc32_17,
    acc33_18: Acc33_18,
    acc34_19: Acc34_19,
    acc35_20: Acc35_20,
    acc36_21: Acc36_21,
    acc37_22: Acc37_22,
    acc38_23: Acc38_23,
    acc39_24: Acc39_24,
    acc39_32: Acc39_32,
}
impl RegisterBlock {
    #[doc = "0x00 - Signed Operand 0"]
    #[inline(always)]
    pub const fn op0s(&self) -> &Op0s {
        &self.op0s
    }
    #[doc = "0x04 - Unsigned Operand 0"]
    #[inline(always)]
    pub const fn op0u(&self) -> &Op0u {
        &self.op0u
    }
    #[doc = "0x08 - Signed Operand 1 and Multiply"]
    #[inline(always)]
    pub const fn op1smul(&self) -> &Op1smul {
        &self.op1smul
    }
    #[doc = "0x0c - Unsigned Operand 1 and Multiply"]
    #[inline(always)]
    pub const fn op1umul(&self) -> &Op1umul {
        &self.op1umul
    }
    #[doc = "0x10 - Signed Operand 1 and Multiply-Accumulate"]
    #[inline(always)]
    pub const fn op1smac(&self) -> &Op1smac {
        &self.op1smac
    }
    #[doc = "0x14 - Unsigned Operand 1 and Multiply-Accumulate"]
    #[inline(always)]
    pub const fn op1umac(&self) -> &Op1umac {
        &self.op1umac
    }
    #[doc = "0x18 - Signed Operand 1 and 16-bit Addition"]
    #[inline(always)]
    pub const fn op1sadd16(&self) -> &Op1sadd16 {
        &self.op1sadd16
    }
    #[doc = "0x1c - Unsigned Operand 1 and 16-bit Addition"]
    #[inline(always)]
    pub const fn op1uadd16(&self) -> &Op1uadd16 {
        &self.op1uadd16
    }
    #[doc = "0x20 - Signed Operand 1 and 32-bit Addition"]
    #[inline(always)]
    pub const fn op1sadd32(&self) -> &Op1sadd32 {
        &self.op1sadd32
    }
    #[doc = "0x24 - Unsigned Operand 1 and 32-bit Addition"]
    #[inline(always)]
    pub const fn op1uadd32(&self) -> &Op1uadd32 {
        &self.op1uadd32
    }
    #[doc = "0x28 - Count Leading Zero"]
    #[inline(always)]
    pub const fn clz(&self) -> &Clz {
        &self.clz
    }
    #[doc = "0x2c - Count Leading Sign"]
    #[inline(always)]
    pub const fn cls(&self) -> &Cls {
        &self.cls
    }
    #[doc = "0x30 - Accumulator Shift Only one shift operation can be triggered per register write."]
    #[inline(always)]
    pub const fn accshift(&self) -> &Accshift {
        &self.accshift
    }
    #[doc = "0x34 - Accumulator Reset"]
    #[inline(always)]
    pub const fn accreset(&self) -> &Accreset {
        &self.accreset
    }
    #[doc = "0x38 - Accumulator Bits 15:0"]
    #[inline(always)]
    pub const fn acc15_0(&self) -> &Acc15_0 {
        &self.acc15_0
    }
    #[doc = "0x3c - Accumulator Bits 16:1"]
    #[inline(always)]
    pub const fn acc16_1(&self) -> &Acc16_1 {
        &self.acc16_1
    }
    #[doc = "0x40 - Accumulator Bits 17:2"]
    #[inline(always)]
    pub const fn acc17_2(&self) -> &Acc17_2 {
        &self.acc17_2
    }
    #[doc = "0x44 - Accumulator Bits 18:3"]
    #[inline(always)]
    pub const fn acc18_3(&self) -> &Acc18_3 {
        &self.acc18_3
    }
    #[doc = "0x48 - Accumulator Bits 19:4"]
    #[inline(always)]
    pub const fn acc19_4(&self) -> &Acc19_4 {
        &self.acc19_4
    }
    #[doc = "0x4c - Accumulator Bits 20:5"]
    #[inline(always)]
    pub const fn acc20_5(&self) -> &Acc20_5 {
        &self.acc20_5
    }
    #[doc = "0x50 - Accumulator Bits 21:6"]
    #[inline(always)]
    pub const fn acc21_6(&self) -> &Acc21_6 {
        &self.acc21_6
    }
    #[doc = "0x54 - Accumulator Bits 22:7"]
    #[inline(always)]
    pub const fn acc22_7(&self) -> &Acc22_7 {
        &self.acc22_7
    }
    #[doc = "0x58 - Accumulator Bits 23:8"]
    #[inline(always)]
    pub const fn acc23_8(&self) -> &Acc23_8 {
        &self.acc23_8
    }
    #[doc = "0x5c - Accumulator Bits 24:9"]
    #[inline(always)]
    pub const fn acc24_9(&self) -> &Acc24_9 {
        &self.acc24_9
    }
    #[doc = "0x60 - Accumulator Bits 25:10"]
    #[inline(always)]
    pub const fn acc25_10(&self) -> &Acc25_10 {
        &self.acc25_10
    }
    #[doc = "0x64 - Accumulator Bits 26:11"]
    #[inline(always)]
    pub const fn acc26_11(&self) -> &Acc26_11 {
        &self.acc26_11
    }
    #[doc = "0x68 - Accumulator Bits 27:12"]
    #[inline(always)]
    pub const fn acc27_12(&self) -> &Acc27_12 {
        &self.acc27_12
    }
    #[doc = "0x6c - Accumulator Bits 28:13"]
    #[inline(always)]
    pub const fn acc28_13(&self) -> &Acc28_13 {
        &self.acc28_13
    }
    #[doc = "0x70 - Accumulator Bits 29:14"]
    #[inline(always)]
    pub const fn acc29_14(&self) -> &Acc29_14 {
        &self.acc29_14
    }
    #[doc = "0x74 - Accumulator Bits 30:15"]
    #[inline(always)]
    pub const fn acc30_15(&self) -> &Acc30_15 {
        &self.acc30_15
    }
    #[doc = "0x78 - Accumulator Bits 31:16"]
    #[inline(always)]
    pub const fn acc31_16(&self) -> &Acc31_16 {
        &self.acc31_16
    }
    #[doc = "0x7c - Accumulator Bits 32:17"]
    #[inline(always)]
    pub const fn acc32_17(&self) -> &Acc32_17 {
        &self.acc32_17
    }
    #[doc = "0x80 - Accumulator Bits 33:18"]
    #[inline(always)]
    pub const fn acc33_18(&self) -> &Acc33_18 {
        &self.acc33_18
    }
    #[doc = "0x84 - Accumulator Bits 34:19"]
    #[inline(always)]
    pub const fn acc34_19(&self) -> &Acc34_19 {
        &self.acc34_19
    }
    #[doc = "0x88 - Accumulator Bits 35:20"]
    #[inline(always)]
    pub const fn acc35_20(&self) -> &Acc35_20 {
        &self.acc35_20
    }
    #[doc = "0x8c - Accumulator Bits 36:21"]
    #[inline(always)]
    pub const fn acc36_21(&self) -> &Acc36_21 {
        &self.acc36_21
    }
    #[doc = "0x90 - Accumulator Bits 37:22"]
    #[inline(always)]
    pub const fn acc37_22(&self) -> &Acc37_22 {
        &self.acc37_22
    }
    #[doc = "0x94 - Accumulator Bits 38:23"]
    #[inline(always)]
    pub const fn acc38_23(&self) -> &Acc38_23 {
        &self.acc38_23
    }
    #[doc = "0x98 - Accumulator Bits 39:24"]
    #[inline(always)]
    pub const fn acc39_24(&self) -> &Acc39_24 {
        &self.acc39_24
    }
    #[doc = "0x9c - Accumulator Bits 39:32"]
    #[inline(always)]
    pub const fn acc39_32(&self) -> &Acc39_32 {
        &self.acc39_32
    }
}
#[doc = "OP0S (rw) register accessor: Signed Operand 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op0s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op0s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op0s`]
module"]
#[doc(alias = "OP0S")]
pub type Op0s = crate::Reg<op0s::Op0sSpec>;
#[doc = "Signed Operand 0"]
pub mod op0s;
#[doc = "OP0U (rw) register accessor: Unsigned Operand 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op0u::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op0u::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op0u`]
module"]
#[doc(alias = "OP0U")]
pub type Op0u = crate::Reg<op0u::Op0uSpec>;
#[doc = "Unsigned Operand 0"]
pub mod op0u;
#[doc = "OP1SMUL (rw) register accessor: Signed Operand 1 and Multiply\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op1smul::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op1smul::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op1smul`]
module"]
#[doc(alias = "OP1SMUL")]
pub type Op1smul = crate::Reg<op1smul::Op1smulSpec>;
#[doc = "Signed Operand 1 and Multiply"]
pub mod op1smul;
#[doc = "OP1UMUL (rw) register accessor: Unsigned Operand 1 and Multiply\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op1umul::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op1umul::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op1umul`]
module"]
#[doc(alias = "OP1UMUL")]
pub type Op1umul = crate::Reg<op1umul::Op1umulSpec>;
#[doc = "Unsigned Operand 1 and Multiply"]
pub mod op1umul;
#[doc = "OP1SMAC (rw) register accessor: Signed Operand 1 and Multiply-Accumulate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op1smac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op1smac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op1smac`]
module"]
#[doc(alias = "OP1SMAC")]
pub type Op1smac = crate::Reg<op1smac::Op1smacSpec>;
#[doc = "Signed Operand 1 and Multiply-Accumulate"]
pub mod op1smac;
#[doc = "OP1UMAC (rw) register accessor: Unsigned Operand 1 and Multiply-Accumulate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op1umac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op1umac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op1umac`]
module"]
#[doc(alias = "OP1UMAC")]
pub type Op1umac = crate::Reg<op1umac::Op1umacSpec>;
#[doc = "Unsigned Operand 1 and Multiply-Accumulate"]
pub mod op1umac;
#[doc = "OP1SADD16 (rw) register accessor: Signed Operand 1 and 16-bit Addition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op1sadd16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op1sadd16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op1sadd16`]
module"]
#[doc(alias = "OP1SADD16")]
pub type Op1sadd16 = crate::Reg<op1sadd16::Op1sadd16Spec>;
#[doc = "Signed Operand 1 and 16-bit Addition"]
pub mod op1sadd16;
#[doc = "OP1UADD16 (rw) register accessor: Unsigned Operand 1 and 16-bit Addition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op1uadd16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op1uadd16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op1uadd16`]
module"]
#[doc(alias = "OP1UADD16")]
pub type Op1uadd16 = crate::Reg<op1uadd16::Op1uadd16Spec>;
#[doc = "Unsigned Operand 1 and 16-bit Addition"]
pub mod op1uadd16;
#[doc = "OP1SADD32 (rw) register accessor: Signed Operand 1 and 32-bit Addition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op1sadd32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op1sadd32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op1sadd32`]
module"]
#[doc(alias = "OP1SADD32")]
pub type Op1sadd32 = crate::Reg<op1sadd32::Op1sadd32Spec>;
#[doc = "Signed Operand 1 and 32-bit Addition"]
pub mod op1sadd32;
#[doc = "OP1UADD32 (rw) register accessor: Unsigned Operand 1 and 32-bit Addition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op1uadd32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op1uadd32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op1uadd32`]
module"]
#[doc(alias = "OP1UADD32")]
pub type Op1uadd32 = crate::Reg<op1uadd32::Op1uadd32Spec>;
#[doc = "Unsigned Operand 1 and 32-bit Addition"]
pub mod op1uadd32;
#[doc = "CLZ (rw) register accessor: Count Leading Zero\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clz`]
module"]
#[doc(alias = "CLZ")]
pub type Clz = crate::Reg<clz::ClzSpec>;
#[doc = "Count Leading Zero"]
pub mod clz;
#[doc = "CLS (rw) register accessor: Count Leading Sign\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cls::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cls::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cls`]
module"]
#[doc(alias = "CLS")]
pub type Cls = crate::Reg<cls::ClsSpec>;
#[doc = "Count Leading Sign"]
pub mod cls;
#[doc = "ACCSHIFT (rw) register accessor: Accumulator Shift Only one shift operation can be triggered per register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`accshift::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`accshift::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@accshift`]
module"]
#[doc(alias = "ACCSHIFT")]
pub type Accshift = crate::Reg<accshift::AccshiftSpec>;
#[doc = "Accumulator Shift Only one shift operation can be triggered per register write."]
pub mod accshift;
#[doc = "ACCRESET (rw) register accessor: Accumulator Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`accreset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`accreset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@accreset`]
module"]
#[doc(alias = "ACCRESET")]
pub type Accreset = crate::Reg<accreset::AccresetSpec>;
#[doc = "Accumulator Reset"]
pub mod accreset;
#[doc = "ACC15_0 (rw) register accessor: Accumulator Bits 15:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc15_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc15_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc15_0`]
module"]
#[doc(alias = "ACC15_0")]
pub type Acc15_0 = crate::Reg<acc15_0::Acc15_0Spec>;
#[doc = "Accumulator Bits 15:0"]
pub mod acc15_0;
#[doc = "ACC16_1 (rw) register accessor: Accumulator Bits 16:1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc16_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc16_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc16_1`]
module"]
#[doc(alias = "ACC16_1")]
pub type Acc16_1 = crate::Reg<acc16_1::Acc16_1Spec>;
#[doc = "Accumulator Bits 16:1"]
pub mod acc16_1;
#[doc = "ACC17_2 (rw) register accessor: Accumulator Bits 17:2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc17_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc17_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc17_2`]
module"]
#[doc(alias = "ACC17_2")]
pub type Acc17_2 = crate::Reg<acc17_2::Acc17_2Spec>;
#[doc = "Accumulator Bits 17:2"]
pub mod acc17_2;
#[doc = "ACC18_3 (rw) register accessor: Accumulator Bits 18:3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc18_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc18_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc18_3`]
module"]
#[doc(alias = "ACC18_3")]
pub type Acc18_3 = crate::Reg<acc18_3::Acc18_3Spec>;
#[doc = "Accumulator Bits 18:3"]
pub mod acc18_3;
#[doc = "ACC19_4 (rw) register accessor: Accumulator Bits 19:4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc19_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc19_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc19_4`]
module"]
#[doc(alias = "ACC19_4")]
pub type Acc19_4 = crate::Reg<acc19_4::Acc19_4Spec>;
#[doc = "Accumulator Bits 19:4"]
pub mod acc19_4;
#[doc = "ACC20_5 (rw) register accessor: Accumulator Bits 20:5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc20_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc20_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc20_5`]
module"]
#[doc(alias = "ACC20_5")]
pub type Acc20_5 = crate::Reg<acc20_5::Acc20_5Spec>;
#[doc = "Accumulator Bits 20:5"]
pub mod acc20_5;
#[doc = "ACC21_6 (rw) register accessor: Accumulator Bits 21:6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc21_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc21_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc21_6`]
module"]
#[doc(alias = "ACC21_6")]
pub type Acc21_6 = crate::Reg<acc21_6::Acc21_6Spec>;
#[doc = "Accumulator Bits 21:6"]
pub mod acc21_6;
#[doc = "ACC22_7 (rw) register accessor: Accumulator Bits 22:7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc22_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc22_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc22_7`]
module"]
#[doc(alias = "ACC22_7")]
pub type Acc22_7 = crate::Reg<acc22_7::Acc22_7Spec>;
#[doc = "Accumulator Bits 22:7"]
pub mod acc22_7;
#[doc = "ACC23_8 (rw) register accessor: Accumulator Bits 23:8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc23_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc23_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc23_8`]
module"]
#[doc(alias = "ACC23_8")]
pub type Acc23_8 = crate::Reg<acc23_8::Acc23_8Spec>;
#[doc = "Accumulator Bits 23:8"]
pub mod acc23_8;
#[doc = "ACC24_9 (rw) register accessor: Accumulator Bits 24:9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc24_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc24_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc24_9`]
module"]
#[doc(alias = "ACC24_9")]
pub type Acc24_9 = crate::Reg<acc24_9::Acc24_9Spec>;
#[doc = "Accumulator Bits 24:9"]
pub mod acc24_9;
#[doc = "ACC25_10 (rw) register accessor: Accumulator Bits 25:10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc25_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc25_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc25_10`]
module"]
#[doc(alias = "ACC25_10")]
pub type Acc25_10 = crate::Reg<acc25_10::Acc25_10Spec>;
#[doc = "Accumulator Bits 25:10"]
pub mod acc25_10;
#[doc = "ACC26_11 (rw) register accessor: Accumulator Bits 26:11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc26_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc26_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc26_11`]
module"]
#[doc(alias = "ACC26_11")]
pub type Acc26_11 = crate::Reg<acc26_11::Acc26_11Spec>;
#[doc = "Accumulator Bits 26:11"]
pub mod acc26_11;
#[doc = "ACC27_12 (rw) register accessor: Accumulator Bits 27:12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc27_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc27_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc27_12`]
module"]
#[doc(alias = "ACC27_12")]
pub type Acc27_12 = crate::Reg<acc27_12::Acc27_12Spec>;
#[doc = "Accumulator Bits 27:12"]
pub mod acc27_12;
#[doc = "ACC28_13 (rw) register accessor: Accumulator Bits 28:13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc28_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc28_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc28_13`]
module"]
#[doc(alias = "ACC28_13")]
pub type Acc28_13 = crate::Reg<acc28_13::Acc28_13Spec>;
#[doc = "Accumulator Bits 28:13"]
pub mod acc28_13;
#[doc = "ACC29_14 (rw) register accessor: Accumulator Bits 29:14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc29_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc29_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc29_14`]
module"]
#[doc(alias = "ACC29_14")]
pub type Acc29_14 = crate::Reg<acc29_14::Acc29_14Spec>;
#[doc = "Accumulator Bits 29:14"]
pub mod acc29_14;
#[doc = "ACC30_15 (rw) register accessor: Accumulator Bits 30:15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc30_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc30_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc30_15`]
module"]
#[doc(alias = "ACC30_15")]
pub type Acc30_15 = crate::Reg<acc30_15::Acc30_15Spec>;
#[doc = "Accumulator Bits 30:15"]
pub mod acc30_15;
#[doc = "ACC31_16 (rw) register accessor: Accumulator Bits 31:16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc31_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc31_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc31_16`]
module"]
#[doc(alias = "ACC31_16")]
pub type Acc31_16 = crate::Reg<acc31_16::Acc31_16Spec>;
#[doc = "Accumulator Bits 31:16"]
pub mod acc31_16;
#[doc = "ACC32_17 (rw) register accessor: Accumulator Bits 32:17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc32_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc32_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc32_17`]
module"]
#[doc(alias = "ACC32_17")]
pub type Acc32_17 = crate::Reg<acc32_17::Acc32_17Spec>;
#[doc = "Accumulator Bits 32:17"]
pub mod acc32_17;
#[doc = "ACC33_18 (rw) register accessor: Accumulator Bits 33:18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc33_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc33_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc33_18`]
module"]
#[doc(alias = "ACC33_18")]
pub type Acc33_18 = crate::Reg<acc33_18::Acc33_18Spec>;
#[doc = "Accumulator Bits 33:18"]
pub mod acc33_18;
#[doc = "ACC34_19 (rw) register accessor: Accumulator Bits 34:19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc34_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc34_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc34_19`]
module"]
#[doc(alias = "ACC34_19")]
pub type Acc34_19 = crate::Reg<acc34_19::Acc34_19Spec>;
#[doc = "Accumulator Bits 34:19"]
pub mod acc34_19;
#[doc = "ACC35_20 (rw) register accessor: Accumulator Bits 35:20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc35_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc35_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc35_20`]
module"]
#[doc(alias = "ACC35_20")]
pub type Acc35_20 = crate::Reg<acc35_20::Acc35_20Spec>;
#[doc = "Accumulator Bits 35:20"]
pub mod acc35_20;
#[doc = "ACC36_21 (rw) register accessor: Accumulator Bits 36:21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc36_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc36_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc36_21`]
module"]
#[doc(alias = "ACC36_21")]
pub type Acc36_21 = crate::Reg<acc36_21::Acc36_21Spec>;
#[doc = "Accumulator Bits 36:21"]
pub mod acc36_21;
#[doc = "ACC37_22 (rw) register accessor: Accumulator Bits 37:22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc37_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc37_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc37_22`]
module"]
#[doc(alias = "ACC37_22")]
pub type Acc37_22 = crate::Reg<acc37_22::Acc37_22Spec>;
#[doc = "Accumulator Bits 37:22"]
pub mod acc37_22;
#[doc = "ACC38_23 (rw) register accessor: Accumulator Bits 38:23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc38_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc38_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc38_23`]
module"]
#[doc(alias = "ACC38_23")]
pub type Acc38_23 = crate::Reg<acc38_23::Acc38_23Spec>;
#[doc = "Accumulator Bits 38:23"]
pub mod acc38_23;
#[doc = "ACC39_24 (rw) register accessor: Accumulator Bits 39:24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc39_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc39_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc39_24`]
module"]
#[doc(alias = "ACC39_24")]
pub type Acc39_24 = crate::Reg<acc39_24::Acc39_24Spec>;
#[doc = "Accumulator Bits 39:24"]
pub mod acc39_24;
#[doc = "ACC39_32 (rw) register accessor: Accumulator Bits 39:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc39_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc39_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc39_32`]
module"]
#[doc(alias = "ACC39_32")]
pub type Acc39_32 = crate::Reg<acc39_32::Acc39_32Spec>;
#[doc = "Accumulator Bits 39:32"]
pub mod acc39_32;

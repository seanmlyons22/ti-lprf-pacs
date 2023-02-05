#[doc = "Register `FUNCTION1` reader"]
pub struct R(crate::R<FUNCTION1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNCTION1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNCTION1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNCTION1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNCTION1` writer"]
pub struct W(crate::W<FUNCTION1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNCTION1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FUNCTION1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNCTION1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNCTION` reader - 3:0\\]
Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
pub type FUNCTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUNCTION` writer - 3:0\\]
Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
pub type FUNCTION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION1_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED4` reader - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED4` writer - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION1_SPEC, bool, O>;
#[doc = "Field `EMITRANGE` reader - 5:5\\]
Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
pub type EMITRANGE_R = crate::BitReader<bool>;
#[doc = "Field `EMITRANGE` writer - 5:5\\]
Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
pub type EMITRANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION1_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION1_SPEC, u8, u8, 2, O>;
#[doc = "Field `DATAVMATCH` reader - 8:8\\]
Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
pub type DATAVMATCH_R = crate::BitReader<bool>;
#[doc = "Field `DATAVMATCH` writer - 8:8\\]
Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
pub type DATAVMATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION1_SPEC, bool, O>;
#[doc = "Field `LNK1ENA` reader - 9:9\\]
Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)"]
pub type LNK1ENA_R = crate::BitReader<bool>;
#[doc = "Field `LNK1ENA` writer - 9:9\\]
Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)"]
pub type LNK1ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION1_SPEC, bool, O>;
#[doc = "Field `DATAVSIZE` reader - 11:10\\]
Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
pub type DATAVSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAVSIZE` writer - 11:10\\]
Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
pub type DATAVSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION1_SPEC, u8, u8, 2, O>;
#[doc = "Field `DATAVADDR0` reader - 15:12\\]
Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
pub type DATAVADDR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAVADDR0` writer - 15:12\\]
Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
pub type DATAVADDR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION1_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATAVADDR1` reader - 19:16\\]
Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
pub type DATAVADDR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAVADDR1` writer - 19:16\\]
Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
pub type DATAVADDR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION1_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED20` reader - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED20` writer - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION1_SPEC, u8, u8, 4, O>;
#[doc = "Field `MATCHED` reader - 24:24\\]
This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
pub type MATCHED_R = crate::BitReader<bool>;
#[doc = "Field `MATCHED` writer - 24:24\\]
This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
pub type MATCHED_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION1_SPEC, bool, O>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[inline(always)]
    pub fn emitrange(&self) -> EMITRANGE_R {
        EMITRANGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
    #[inline(always)]
    pub fn datavmatch(&self) -> DATAVMATCH_R {
        DATAVMATCH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)"]
    #[inline(always)]
    pub fn lnk1ena(&self) -> LNK1ENA_R {
        LNK1ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
    #[inline(always)]
    pub fn datavsize(&self) -> DATAVSIZE_R {
        DATAVSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
    #[inline(always)]
    pub fn datavaddr0(&self) -> DATAVADDR0_R {
        DATAVADDR0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
    #[inline(always)]
    pub fn datavaddr1(&self) -> DATAVADDR1_R {
        DATAVADDR1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[inline(always)]
    pub fn matched(&self) -> MATCHED_R {
        MATCHED_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FUNCTION_W<0> {
        FUNCTION_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[inline(always)]
    #[must_use]
    pub fn emitrange(&mut self) -> EMITRANGE_W<5> {
        EMITRANGE_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
    #[inline(always)]
    #[must_use]
    pub fn datavmatch(&mut self) -> DATAVMATCH_W<8> {
        DATAVMATCH_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)"]
    #[inline(always)]
    #[must_use]
    pub fn lnk1ena(&mut self) -> LNK1ENA_W<9> {
        LNK1ENA_W::new(self)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
    #[inline(always)]
    #[must_use]
    pub fn datavsize(&mut self) -> DATAVSIZE_W<10> {
        DATAVSIZE_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
    #[inline(always)]
    #[must_use]
    pub fn datavaddr0(&mut self) -> DATAVADDR0_W<12> {
        DATAVADDR0_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
    #[inline(always)]
    #[must_use]
    pub fn datavaddr1(&mut self) -> DATAVADDR1_W<16> {
        DATAVADDR1_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> RESERVED20_W<20> {
        RESERVED20_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn matched(&mut self) -> MATCHED_W<24> {
        MATCHED_W::new(self)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> RESERVED25_W<25> {
        RESERVED25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [function1](index.html) module"]
pub struct FUNCTION1_SPEC;
impl crate::RegisterSpec for FUNCTION1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [function1::R](R) reader structure"]
impl crate::Readable for FUNCTION1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [function1::W](W) writer structure"]
impl crate::Writable for FUNCTION1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUNCTION1 to value 0x0200"]
impl crate::Resettable for FUNCTION1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}

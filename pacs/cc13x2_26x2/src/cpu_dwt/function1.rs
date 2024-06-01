#[doc = "Register `FUNCTION1` reader"]
pub type R = crate::R<Function1Spec>;
#[doc = "Register `FUNCTION1` writer"]
pub type W = crate::W<Function1Spec>;
#[doc = "Field `FUNCTION` reader - 3:0\\]
Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
pub type FunctionR = crate::FieldReader;
#[doc = "Field `FUNCTION` writer - 3:0\\]
Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
pub type FunctionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED4` reader - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `RESERVED4` writer - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMITRANGE` reader - 5:5\\]
Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
pub type EmitrangeR = crate::BitReader;
#[doc = "Field `EMITRANGE` writer - 5:5\\]
Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
pub type EmitrangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATAVMATCH` reader - 8:8\\]
Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
pub type DatavmatchR = crate::BitReader;
#[doc = "Field `DATAVMATCH` writer - 8:8\\]
Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
pub type DatavmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LNK1ENA` reader - 9:9\\]
Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)"]
pub type Lnk1enaR = crate::BitReader;
#[doc = "Field `LNK1ENA` writer - 9:9\\]
Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)"]
pub type Lnk1enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAVSIZE` reader - 11:10\\]
Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
pub type DatavsizeR = crate::FieldReader;
#[doc = "Field `DATAVSIZE` writer - 11:10\\]
Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
pub type DatavsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATAVADDR0` reader - 15:12\\]
Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
pub type Datavaddr0R = crate::FieldReader;
#[doc = "Field `DATAVADDR0` writer - 15:12\\]
Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
pub type Datavaddr0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAVADDR1` reader - 19:16\\]
Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
pub type Datavaddr1R = crate::FieldReader;
#[doc = "Field `DATAVADDR1` writer - 19:16\\]
Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
pub type Datavaddr1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED20` reader - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader;
#[doc = "Field `RESERVED20` writer - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MATCHED` reader - 24:24\\]
This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
pub type MatchedR = crate::BitReader;
#[doc = "Field `MATCHED` writer - 24:24\\]
This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
pub type MatchedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[inline(always)]
    pub fn function(&self) -> FunctionR {
        FunctionR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[inline(always)]
    pub fn emitrange(&self) -> EmitrangeR {
        EmitrangeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
    #[inline(always)]
    pub fn datavmatch(&self) -> DatavmatchR {
        DatavmatchR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)"]
    #[inline(always)]
    pub fn lnk1ena(&self) -> Lnk1enaR {
        Lnk1enaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
    #[inline(always)]
    pub fn datavsize(&self) -> DatavsizeR {
        DatavsizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
    #[inline(always)]
    pub fn datavaddr0(&self) -> Datavaddr0R {
        Datavaddr0R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
    #[inline(always)]
    pub fn datavaddr1(&self) -> Datavaddr1R {
        Datavaddr1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[inline(always)]
    pub fn matched(&self) -> MatchedR {
        MatchedR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FunctionW<Function1Spec> {
        FunctionW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<Function1Spec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[inline(always)]
    #[must_use]
    pub fn emitrange(&mut self) -> EmitrangeW<Function1Spec> {
        EmitrangeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Function1Spec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
    #[inline(always)]
    #[must_use]
    pub fn datavmatch(&mut self) -> DatavmatchW<Function1Spec> {
        DatavmatchW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)"]
    #[inline(always)]
    #[must_use]
    pub fn lnk1ena(&mut self) -> Lnk1enaW<Function1Spec> {
        Lnk1enaW::new(self, 9)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
    #[inline(always)]
    #[must_use]
    pub fn datavsize(&mut self) -> DatavsizeW<Function1Spec> {
        DatavsizeW::new(self, 10)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
    #[inline(always)]
    #[must_use]
    pub fn datavaddr0(&mut self) -> Datavaddr0W<Function1Spec> {
        Datavaddr0W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
    #[inline(always)]
    #[must_use]
    pub fn datavaddr1(&mut self) -> Datavaddr1W<Function1Spec> {
        Datavaddr1W::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> Reserved20W<Function1Spec> {
        Reserved20W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn matched(&mut self) -> MatchedW<Function1Spec> {
        MatchedW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<Function1Spec> {
        Reserved25W::new(self, 25)
    }
}
#[doc = "Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`function1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`function1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Function1Spec;
impl crate::RegisterSpec for Function1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`function1::R`](R) reader structure"]
impl crate::Readable for Function1Spec {}
#[doc = "`write(|w| ..)` method takes [`function1::W`](W) writer structure"]
impl crate::Writable for Function1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUNCTION1 to value 0x0200"]
impl crate::Resettable for Function1Spec {
    const RESET_VALUE: u32 = 0x0200;
}

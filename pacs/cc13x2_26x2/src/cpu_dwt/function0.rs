#[doc = "Register `FUNCTION0` reader"]
pub type R = crate::R<Function0Spec>;
#[doc = "Register `FUNCTION0` writer"]
pub type W = crate::W<Function0Spec>;
#[doc = "Field `FUNCTION` reader - 3:0\\]
Function settings. 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
pub type FunctionR = crate::FieldReader;
#[doc = "Field `FUNCTION` writer - 3:0\\]
Function settings. 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
pub type FunctionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED4` reader - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `EMITRANGE` reader - 5:5\\]
Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
pub type EmitrangeR = crate::BitReader;
#[doc = "Field `EMITRANGE` writer - 5:5\\]
Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
pub type EmitrangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 6:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `CYCMATCH` reader - 7:7\\]
This bit is only available in comparator 0. When set, COMP0 will compare against the cycle counter (CYCCNT)."]
pub type CycmatchR = crate::BitReader;
#[doc = "Field `CYCMATCH` writer - 7:7\\]
This bit is only available in comparator 0. When set, COMP0 will compare against the cycle counter (CYCCNT)."]
pub type CycmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 23:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u16>;
#[doc = "Field `MATCHED` reader - 24:24\\]
This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
pub type MatchedR = crate::BitReader;
#[doc = "Field `MATCHED` writer - 24:24\\]
This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
pub type MatchedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Function settings. 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
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
    #[doc = "Bit 6 - 6:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
This bit is only available in comparator 0. When set, COMP0 will compare against the cycle counter (CYCCNT)."]
    #[inline(always)]
    pub fn cycmatch(&self) -> CycmatchR {
        CycmatchR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 0xffff) as u16)
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
Function settings. 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FunctionW<Function0Spec> {
        FunctionW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[inline(always)]
    #[must_use]
    pub fn emitrange(&mut self) -> EmitrangeW<Function0Spec> {
        EmitrangeW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
This bit is only available in comparator 0. When set, COMP0 will compare against the cycle counter (CYCCNT)."]
    #[inline(always)]
    #[must_use]
    pub fn cycmatch(&mut self) -> CycmatchW<Function0Spec> {
        CycmatchW::new(self, 7)
    }
    #[doc = "Bit 24 - 24:24\\]
This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn matched(&mut self) -> MatchedW<Function0Spec> {
        MatchedW::new(self, 24)
    }
}
#[doc = "Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`function0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`function0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Function0Spec;
impl crate::RegisterSpec for Function0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`function0::R`](R) reader structure"]
impl crate::Readable for Function0Spec {}
#[doc = "`write(|w| ..)` method takes [`function0::W`](W) writer structure"]
impl crate::Writable for Function0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUNCTION0 to value 0"]
impl crate::Resettable for Function0Spec {
    const RESET_VALUE: u32 = 0;
}

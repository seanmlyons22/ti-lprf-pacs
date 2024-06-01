#[doc = "Register `STCSR` reader"]
pub type R = crate::R<StcsrSpec>;
#[doc = "Register `STCSR` writer"]
pub type W = crate::W<StcsrSpec>;
#[doc = "Field `ENABLE` reader - 0:0\\]
Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - 0:0\\]
Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICKINT` reader - 1:1\\]
0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
pub type TickintR = crate::BitReader;
#[doc = "Field `TICKINT` writer - 1:1\\]
0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
pub type TickintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSOURCE` reader - 2:2\\]
Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
pub type ClksourceR = crate::BitReader;
#[doc = "Field `CLKSOURCE` writer - 2:2\\]
Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
pub type ClksourceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED3` writer - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `COUNTFLAG` reader - 16:16\\]
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
pub type CountflagR = crate::BitReader;
#[doc = "Field `COUNTFLAG` writer - 16:16\\]
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
pub type CountflagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
    #[inline(always)]
    pub fn tickint(&self) -> TickintR {
        TickintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
    #[inline(always)]
    pub fn clksource(&self) -> ClksourceR {
        ClksourceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:15 - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
    #[inline(always)]
    pub fn countflag(&self) -> CountflagR {
        CountflagR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<StcsrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TickintW<StcsrSpec> {
        TickintW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn clksource(&mut self) -> ClksourceW<StcsrSpec> {
        ClksourceW::new(self, 2)
    }
    #[doc = "Bits 3:15 - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<StcsrSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 16 - 16:16\\]
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
    #[inline(always)]
    #[must_use]
    pub fn countflag(&mut self) -> CountflagW<StcsrSpec> {
        CountflagW::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<StcsrSpec> {
        Reserved17W::new(self, 17)
    }
}
#[doc = "SysTick Control and Status This register enables the SysTick features and returns status flags related to SysTick.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcsrSpec;
impl crate::RegisterSpec for StcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcsr::R`](R) reader structure"]
impl crate::Readable for StcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`stcsr::W`](W) writer structure"]
impl crate::Writable for StcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCSR to value 0x04"]
impl crate::Resettable for StcsrSpec {
    const RESET_VALUE: u32 = 0x04;
}

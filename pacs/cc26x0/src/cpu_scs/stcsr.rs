#[doc = "Register `STCSR` reader"]
pub struct R(crate::R<STCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCSR` writer"]
pub struct W(crate::W<STCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCSR_SPEC>;
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
impl From<crate::W<STCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - 0:0\\]
Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STCSR_SPEC, bool, O>;
#[doc = "Field `TICKINT` reader - 1:1\\]
0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
pub type TICKINT_R = crate::BitReader<bool>;
#[doc = "Field `TICKINT` writer - 1:1\\]
0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
pub type TICKINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STCSR_SPEC, bool, O>;
#[doc = "Field `CLKSOURCE` reader - 2:2\\]
Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
pub type CLKSOURCE_R = crate::BitReader<bool>;
#[doc = "Field `CLKSOURCE` writer - 2:2\\]
Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
pub type CLKSOURCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STCSR_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED3` writer - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STCSR_SPEC, u16, u16, 13, O>;
#[doc = "Field `COUNTFLAG` reader - 16:16\\]
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
pub type COUNTFLAG_R = crate::BitReader<bool>;
#[doc = "Field `COUNTFLAG` writer - 16:16\\]
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
pub type COUNTFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, STCSR_SPEC, bool, O>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STCSR_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:15 - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TICKINT_W<1> {
        TICKINT_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn clksource(&mut self) -> CLKSOURCE_W<2> {
        CLKSOURCE_W::new(self)
    }
    #[doc = "Bits 3:15 - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
    #[inline(always)]
    #[must_use]
    pub fn countflag(&mut self) -> COUNTFLAG_W<16> {
        COUNTFLAG_W::new(self)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> RESERVED17_W<17> {
        RESERVED17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysTick Control and Status This register enables the SysTick features and returns status flags related to SysTick.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcsr](index.html) module"]
pub struct STCSR_SPEC;
impl crate::RegisterSpec for STCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcsr::R](R) reader structure"]
impl crate::Readable for STCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcsr::W](W) writer structure"]
impl crate::Writable for STCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCSR to value 0x04"]
impl crate::Resettable for STCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}

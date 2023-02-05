#[doc = "Register `SLEEPCNT` reader"]
pub struct R(crate::R<SLEEPCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLEEPCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLEEPCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLEEPCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLEEPCNT` writer"]
pub struct W(crate::W<SLEEPCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLEEPCNT_SPEC>;
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
impl From<crate::W<SLEEPCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLEEPCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEPCNT` reader - 7:0\\]
Counts one on each cycle when all of the following are true: - DWT_CTRL.SLEEPEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed, see DWT_CPICNT. - No load-store operation is in progress, see DWT_LSUCNT. - No exception-entry or exception-exit operation is in progress, see DWT_EXCCNT. - The PE is in a power saving mode. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE."]
pub type SLEEPCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEEPCNT` writer - 7:0\\]
Counts one on each cycle when all of the following are true: - DWT_CTRL.SLEEPEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed, see DWT_CPICNT. - No load-store operation is in progress, see DWT_LSUCNT. - No exception-entry or exception-exit operation is in progress, see DWT_EXCCNT. - The PE is in a power saving mode. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE."]
pub type SLEEPCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLEEPCNT_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLEEPCNT_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Counts one on each cycle when all of the following are true: - DWT_CTRL.SLEEPEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed, see DWT_CPICNT. - No load-store operation is in progress, see DWT_LSUCNT. - No exception-entry or exception-exit operation is in progress, see DWT_EXCCNT. - The PE is in a power saving mode. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE."]
    #[inline(always)]
    pub fn sleepcnt(&self) -> SLEEPCNT_R {
        SLEEPCNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Counts one on each cycle when all of the following are true: - DWT_CTRL.SLEEPEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed, see DWT_CPICNT. - No load-store operation is in progress, see DWT_LSUCNT. - No exception-entry or exception-exit operation is in progress, see DWT_EXCCNT. - The PE is in a power saving mode. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE."]
    #[inline(always)]
    #[must_use]
    pub fn sleepcnt(&mut self) -> SLEEPCNT_W<0> {
        SLEEPCNT_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepcnt](index.html) module"]
pub struct SLEEPCNT_SPEC;
impl crate::RegisterSpec for SLEEPCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sleepcnt::R](R) reader structure"]
impl crate::Readable for SLEEPCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sleepcnt::W](W) writer structure"]
impl crate::Writable for SLEEPCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLEEPCNT to value 0"]
impl crate::Resettable for SLEEPCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

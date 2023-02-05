#[doc = "Register `CPICNT` reader"]
pub struct R(crate::R<CPICNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPICNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPICNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPICNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPICNT` writer"]
pub struct W(crate::W<CPICNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPICNT_SPEC>;
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
impl From<crate::W<CPICNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPICNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPICNT` reader - 7:0\\]
Counts one on each cycle when all of the following are true: - DWT_CTRL.CPIEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed. - No load-store operation is in progress, see DWT_LSUCNT. - No exception-entry or exception-exit operation is in progress, see DWT_EXCCNT. - The PE is not in a power saving mode, see DWT_SLEEPCNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE."]
pub type CPICNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPICNT` writer - 7:0\\]
Counts one on each cycle when all of the following are true: - DWT_CTRL.CPIEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed. - No load-store operation is in progress, see DWT_LSUCNT. - No exception-entry or exception-exit operation is in progress, see DWT_EXCCNT. - The PE is not in a power saving mode, see DWT_SLEEPCNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE."]
pub type CPICNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPICNT_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPICNT_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Counts one on each cycle when all of the following are true: - DWT_CTRL.CPIEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed. - No load-store operation is in progress, see DWT_LSUCNT. - No exception-entry or exception-exit operation is in progress, see DWT_EXCCNT. - The PE is not in a power saving mode, see DWT_SLEEPCNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE."]
    #[inline(always)]
    pub fn cpicnt(&self) -> CPICNT_R {
        CPICNT_R::new((self.bits & 0xff) as u8)
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
Counts one on each cycle when all of the following are true: - DWT_CTRL.CPIEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed. - No load-store operation is in progress, see DWT_LSUCNT. - No exception-entry or exception-exit operation is in progress, see DWT_EXCCNT. - The PE is not in a power saving mode, see DWT_SLEEPCNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE."]
    #[inline(always)]
    #[must_use]
    pub fn cpicnt(&mut self) -> CPICNT_W<0> {
        CPICNT_W::new(self)
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
#[doc = "CPI Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpicnt](index.html) module"]
pub struct CPICNT_SPEC;
impl crate::RegisterSpec for CPICNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpicnt::R](R) reader structure"]
impl crate::Readable for CPICNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpicnt::W](W) writer structure"]
impl crate::Writable for CPICNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPICNT to value 0"]
impl crate::Resettable for CPICNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

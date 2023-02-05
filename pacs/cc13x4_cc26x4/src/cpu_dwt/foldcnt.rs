#[doc = "Register `FOLDCNT` reader"]
pub struct R(crate::R<FOLDCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FOLDCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FOLDCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FOLDCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FOLDCNT` writer"]
pub struct W(crate::W<FOLDCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FOLDCNT_SPEC>;
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
impl From<crate::W<FOLDCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FOLDCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FOLDCNT` reader - 7:0\\]
Counts on each cycle when all of the following are true: - DWT_CTRL.FOLDEVTENA == 1 and DEMCR.TRCENA == 1. - At least two instructions are executed, see DWT_CPICNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE. The counter is incremented by the number of instructions executed, minus one"]
pub type FOLDCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FOLDCNT` writer - 7:0\\]
Counts on each cycle when all of the following are true: - DWT_CTRL.FOLDEVTENA == 1 and DEMCR.TRCENA == 1. - At least two instructions are executed, see DWT_CPICNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE. The counter is incremented by the number of instructions executed, minus one"]
pub type FOLDCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FOLDCNT_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FOLDCNT_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Counts on each cycle when all of the following are true: - DWT_CTRL.FOLDEVTENA == 1 and DEMCR.TRCENA == 1. - At least two instructions are executed, see DWT_CPICNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE. The counter is incremented by the number of instructions executed, minus one"]
    #[inline(always)]
    pub fn foldcnt(&self) -> FOLDCNT_R {
        FOLDCNT_R::new((self.bits & 0xff) as u8)
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
Counts on each cycle when all of the following are true: - DWT_CTRL.FOLDEVTENA == 1 and DEMCR.TRCENA == 1. - At least two instructions are executed, see DWT_CPICNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE. The counter is incremented by the number of instructions executed, minus one"]
    #[inline(always)]
    #[must_use]
    pub fn foldcnt(&mut self) -> FOLDCNT_W<0> {
        FOLDCNT_W::new(self)
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
#[doc = "Increments on the additional cycles required to execute all load or store instructions\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [foldcnt](index.html) module"]
pub struct FOLDCNT_SPEC;
impl crate::RegisterSpec for FOLDCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [foldcnt::R](R) reader structure"]
impl crate::Readable for FOLDCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [foldcnt::W](W) writer structure"]
impl crate::Writable for FOLDCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FOLDCNT to value 0"]
impl crate::Resettable for FOLDCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

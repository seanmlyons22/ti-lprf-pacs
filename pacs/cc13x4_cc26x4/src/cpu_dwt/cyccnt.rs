#[doc = "Register `CYCCNT` reader"]
pub struct R(crate::R<CYCCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CYCCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CYCCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CYCCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CYCCNT` writer"]
pub struct W(crate::W<CYCCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CYCCNT_SPEC>;
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
impl From<crate::W<CYCCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CYCCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CYCCNT` reader - 31:0\\]
Increments one on each processor clock cycle when DWT_CTRL.CYCCNTENA == 1 and DEMCR.TRCENA == 1. On overflow, CYCCNT wraps to zero"]
pub type CYCCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CYCCNT` writer - 31:0\\]
Increments one on each processor clock cycle when DWT_CTRL.CYCCNTENA == 1 and DEMCR.TRCENA == 1. On overflow, CYCCNT wraps to zero"]
pub type CYCCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CYCCNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Increments one on each processor clock cycle when DWT_CTRL.CYCCNTENA == 1 and DEMCR.TRCENA == 1. On overflow, CYCCNT wraps to zero"]
    #[inline(always)]
    pub fn cyccnt(&self) -> CYCCNT_R {
        CYCCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Increments one on each processor clock cycle when DWT_CTRL.CYCCNTENA == 1 and DEMCR.TRCENA == 1. On overflow, CYCCNT wraps to zero"]
    #[inline(always)]
    #[must_use]
    pub fn cyccnt(&mut self) -> CYCCNT_W<0> {
        CYCCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shows or sets the value of the processor cycle counter, CYCCNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cyccnt](index.html) module"]
pub struct CYCCNT_SPEC;
impl crate::RegisterSpec for CYCCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cyccnt::R](R) reader structure"]
impl crate::Readable for CYCCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cyccnt::W](W) writer structure"]
impl crate::Writable for CYCCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CYCCNT to value 0"]
impl crate::Resettable for CYCCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

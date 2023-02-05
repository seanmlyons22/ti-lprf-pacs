#[doc = "Register `ITNS0` reader"]
pub struct R(crate::R<ITNS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITNS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITNS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITNS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ITNS0` writer"]
pub struct W(crate::W<ITNS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITNS0_SPEC>;
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
impl From<crate::W<ITNS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITNS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITNS` reader - 31:0\\]
For ITNS\\[m\\]
in NVIC_ITNS*n, the target Security state for interrupt 32*n+m"]
pub type ITNS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ITNS` writer - 31:0\\]
For ITNS\\[m\\]
in NVIC_ITNS*n, the target Security state for interrupt 32*n+m"]
pub type ITNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ITNS0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
For ITNS\\[m\\]
in NVIC_ITNS*n, the target Security state for interrupt 32*n+m"]
    #[inline(always)]
    pub fn itns(&self) -> ITNS_R {
        ITNS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
For ITNS\\[m\\]
in NVIC_ITNS*n, the target Security state for interrupt 32*n+m"]
    #[inline(always)]
    #[must_use]
    pub fn itns(&mut self) -> ITNS_W<0> {
        ITNS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itns0](index.html) module"]
pub struct ITNS0_SPEC;
impl crate::RegisterSpec for ITNS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itns0::R](R) reader structure"]
impl crate::Readable for ITNS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [itns0::W](W) writer structure"]
impl crate::Writable for ITNS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ITNS0 to value 0"]
impl crate::Resettable for ITNS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

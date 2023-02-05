#[doc = "Register `TER0` reader"]
pub struct R(crate::R<TER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TER0` writer"]
pub struct W(crate::W<TER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TER0_SPEC>;
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
impl From<crate::W<TER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STIMENA` reader - 31:0\\]
For STIMENA\\[m\\]
in ITM_TER*n, controls whether ITM_STIM(32*n + m) is enabled"]
pub type STIMENA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STIMENA` writer - 31:0\\]
For STIMENA\\[m\\]
in ITM_TER*n, controls whether ITM_STIM(32*n + m) is enabled"]
pub type STIMENA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TER0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
For STIMENA\\[m\\]
in ITM_TER*n, controls whether ITM_STIM(32*n + m) is enabled"]
    #[inline(always)]
    pub fn stimena(&self) -> STIMENA_R {
        STIMENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
For STIMENA\\[m\\]
in ITM_TER*n, controls whether ITM_STIM(32*n + m) is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn stimena(&mut self) -> STIMENA_W<0> {
        STIMENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provide an individual enable bit for each ITM_STIM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ter0](index.html) module"]
pub struct TER0_SPEC;
impl crate::RegisterSpec for TER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ter0::R](R) reader structure"]
impl crate::Readable for TER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ter0::W](W) writer structure"]
impl crate::Writable for TER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TER0 to value 0"]
impl crate::Resettable for TER0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

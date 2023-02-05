#[doc = "Register `TAILR` reader"]
pub struct R(crate::R<TAILR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAILR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAILR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAILR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAILR` writer"]
pub struct W(crate::W<TAILR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAILR_SPEC>;
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
impl From<crate::W<TAILR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAILR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAILR` reader - 31:0\\]
GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
pub type TAILR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TAILR` writer - 31:0\\]
GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
pub type TAILR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAILR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
    #[inline(always)]
    pub fn tailr(&self) -> TAILR_R {
        TAILR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
    #[inline(always)]
    #[must_use]
    pub fn tailr(&mut self) -> TAILR_W<0> {
        TAILR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer A Interval Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tailr](index.html) module"]
pub struct TAILR_SPEC;
impl crate::RegisterSpec for TAILR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tailr::R](R) reader structure"]
impl crate::Readable for TAILR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tailr::W](W) writer structure"]
impl crate::Writable for TAILR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAILR to value 0xffff_ffff"]
impl crate::Resettable for TAILR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

#[doc = "Register `TBILR` reader"]
pub struct R(crate::R<TBILR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBILR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBILR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBILR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBILR` writer"]
pub struct W(crate::W<TBILR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBILR_SPEC>;
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
impl From<crate::W<TBILR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBILR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBILR` reader - 31:0\\]
GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
pub type TBILR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TBILR` writer - 31:0\\]
GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
pub type TBILR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBILR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
    #[inline(always)]
    pub fn tbilr(&self) -> TBILR_R {
        TBILR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
    #[inline(always)]
    #[must_use]
    pub fn tbilr(&mut self) -> TBILR_W<0> {
        TBILR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer B Interval Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbilr](index.html) module"]
pub struct TBILR_SPEC;
impl crate::RegisterSpec for TBILR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbilr::R](R) reader structure"]
impl crate::Readable for TBILR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbilr::W](W) writer structure"]
impl crate::Writable for TBILR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBILR to value 0xffff"]
impl crate::Resettable for TBILR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}

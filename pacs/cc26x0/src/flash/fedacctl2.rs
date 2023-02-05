#[doc = "Register `FEDACCTL2` reader"]
pub struct R(crate::R<FEDACCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEDACCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEDACCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEDACCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEDACCTL2` writer"]
pub struct W(crate::W<FEDACCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEDACCTL2_SPEC>;
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
impl From<crate::W<FEDACCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEDACCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC_THRESHOLD` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type SEC_THRESHOLD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEC_THRESHOLD` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type SEC_THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FEDACCTL2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sec_threshold(&self) -> SEC_THRESHOLD_R {
        SEC_THRESHOLD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sec_threshold(&mut self) -> SEC_THRESHOLD_W<0> {
        SEC_THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fedacctl2](index.html) module"]
pub struct FEDACCTL2_SPEC;
impl crate::RegisterSpec for FEDACCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fedacctl2::R](R) reader structure"]
impl crate::Readable for FEDACCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fedacctl2::W](W) writer structure"]
impl crate::Writable for FEDACCTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEDACCTL2 to value 0"]
impl crate::Resettable for FEDACCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

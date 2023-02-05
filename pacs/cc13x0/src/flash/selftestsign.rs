#[doc = "Register `SELFTESTSIGN` reader"]
pub struct R(crate::R<SELFTESTSIGN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SELFTESTSIGN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SELFTESTSIGN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SELFTESTSIGN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SELFTESTSIGN` writer"]
pub struct W(crate::W<SELFTESTSIGN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SELFTESTSIGN_SPEC>;
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
impl From<crate::W<SELFTESTSIGN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SELFTESTSIGN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGNATURE` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type SIGNATURE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SIGNATURE` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type SIGNATURE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SELFTESTSIGN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn signature(&self) -> SIGNATURE_R {
        SIGNATURE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn signature(&mut self) -> SIGNATURE_W<0> {
        SIGNATURE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [selftestsign](index.html) module"]
pub struct SELFTESTSIGN_SPEC;
impl crate::RegisterSpec for SELFTESTSIGN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [selftestsign::R](R) reader structure"]
impl crate::Readable for SELFTESTSIGN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [selftestsign::W](W) writer structure"]
impl crate::Writable for SELFTESTSIGN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SELFTESTSIGN to value 0"]
impl crate::Resettable for SELFTESTSIGN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

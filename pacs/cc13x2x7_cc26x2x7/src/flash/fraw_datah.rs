#[doc = "Register `FRAW_DATAH` reader"]
pub struct R(crate::R<FRAW_DATAH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAW_DATAH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAW_DATAH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAW_DATAH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAW_DATAH` writer"]
pub struct W(crate::W<FRAW_DATAH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAW_DATAH_SPEC>;
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
impl From<crate::W<FRAW_DATAH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAW_DATAH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAW_DATAH` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FRAW_DATAH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRAW_DATAH` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FRAW_DATAH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAW_DATAH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fraw_datah(&self) -> FRAW_DATAH_R {
        FRAW_DATAH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fraw_datah(&mut self) -> FRAW_DATAH_W<0> {
        FRAW_DATAH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fraw_datah](index.html) module"]
pub struct FRAW_DATAH_SPEC;
impl crate::RegisterSpec for FRAW_DATAH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fraw_datah::R](R) reader structure"]
impl crate::Readable for FRAW_DATAH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fraw_datah::W](W) writer structure"]
impl crate::Writable for FRAW_DATAH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAW_DATAH to value 0"]
impl crate::Resettable for FRAW_DATAH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

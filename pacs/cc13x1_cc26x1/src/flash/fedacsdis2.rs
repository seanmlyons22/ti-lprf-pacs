#[doc = "Register `FEDACSDIS2` reader"]
pub struct R(crate::R<FEDACSDIS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEDACSDIS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEDACSDIS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEDACSDIS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEDACSDIS2` writer"]
pub struct W(crate::W<FEDACSDIS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEDACSDIS2_SPEC>;
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
impl From<crate::W<FEDACSDIS2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEDACSDIS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECTORID2` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type SECTORID2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SECTORID2` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type SECTORID2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FEDACSDIS2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sectorid2(&self) -> SECTORID2_R {
        SECTORID2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sectorid2(&mut self) -> SECTORID2_W<0> {
        SECTORID2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fedacsdis2](index.html) module"]
pub struct FEDACSDIS2_SPEC;
impl crate::RegisterSpec for FEDACSDIS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fedacsdis2::R](R) reader structure"]
impl crate::Readable for FEDACSDIS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fedacsdis2::W](W) writer structure"]
impl crate::Writable for FEDACSDIS2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEDACSDIS2 to value 0"]
impl crate::Resettable for FEDACSDIS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

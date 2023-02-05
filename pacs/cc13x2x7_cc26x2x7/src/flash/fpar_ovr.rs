#[doc = "Register `FPAR_OVR` reader"]
pub struct R(crate::R<FPAR_OVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPAR_OVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPAR_OVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPAR_OVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPAR_OVR` writer"]
pub struct W(crate::W<FPAR_OVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPAR_OVR_SPEC>;
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
impl From<crate::W<FPAR_OVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPAR_OVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT_INV_PAR` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type DAT_INV_PAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DAT_INV_PAR` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type DAT_INV_PAR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FPAR_OVR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dat_inv_par(&self) -> DAT_INV_PAR_R {
        DAT_INV_PAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dat_inv_par(&mut self) -> DAT_INV_PAR_W<0> {
        DAT_INV_PAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpar_ovr](index.html) module"]
pub struct FPAR_OVR_SPEC;
impl crate::RegisterSpec for FPAR_OVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpar_ovr::R](R) reader structure"]
impl crate::Readable for FPAR_OVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpar_ovr::W](W) writer structure"]
impl crate::Writable for FPAR_OVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPAR_OVR to value 0"]
impl crate::Resettable for FPAR_OVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

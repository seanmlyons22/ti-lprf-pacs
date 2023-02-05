#[doc = "Register `RCOSC_HF_TEMPCOMP` reader"]
pub struct R(crate::R<RCOSC_HF_TEMPCOMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCOSC_HF_TEMPCOMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCOSC_HF_TEMPCOMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCOSC_HF_TEMPCOMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCOSC_HF_TEMPCOMP` writer"]
pub struct W(crate::W<RCOSC_HF_TEMPCOMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCOSC_HF_TEMPCOMP_SPEC>;
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
impl From<crate::W<RCOSC_HF_TEMPCOMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCOSC_HF_TEMPCOMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRIMFRACT_SLOPE` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type CTRIMFRACT_SLOPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTRIMFRACT_SLOPE` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type CTRIMFRACT_SLOPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCOSC_HF_TEMPCOMP_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTRIMFRACT_QUAD` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type CTRIMFRACT_QUAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTRIMFRACT_QUAD` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type CTRIMFRACT_QUAD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCOSC_HF_TEMPCOMP_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTRIM` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type CTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTRIM` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type CTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCOSC_HF_TEMPCOMP_SPEC, u8, u8, 8, O>;
#[doc = "Field `FINE_RESISTOR` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type FINE_RESISTOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FINE_RESISTOR` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type FINE_RESISTOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCOSC_HF_TEMPCOMP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrimfract_slope(&self) -> CTRIMFRACT_SLOPE_R {
        CTRIMFRACT_SLOPE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrimfract_quad(&self) -> CTRIMFRACT_QUAD_R {
        CTRIMFRACT_QUAD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrim(&self) -> CTRIM_R {
        CTRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fine_resistor(&self) -> FINE_RESISTOR_R {
        FINE_RESISTOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ctrimfract_slope(&mut self) -> CTRIMFRACT_SLOPE_W<0> {
        CTRIMFRACT_SLOPE_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ctrimfract_quad(&mut self) -> CTRIMFRACT_QUAD_W<8> {
        CTRIMFRACT_QUAD_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ctrim(&mut self) -> CTRIM_W<16> {
        CTRIM_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fine_resistor(&mut self) -> FINE_RESISTOR_W<24> {
        FINE_RESISTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcosc_hf_tempcomp](index.html) module"]
pub struct RCOSC_HF_TEMPCOMP_SPEC;
impl crate::RegisterSpec for RCOSC_HF_TEMPCOMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcosc_hf_tempcomp::R](R) reader structure"]
impl crate::Readable for RCOSC_HF_TEMPCOMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcosc_hf_tempcomp::W](W) writer structure"]
impl crate::Writable for RCOSC_HF_TEMPCOMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCOSC_HF_TEMPCOMP to value 0x03"]
impl crate::Resettable for RCOSC_HF_TEMPCOMP_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}

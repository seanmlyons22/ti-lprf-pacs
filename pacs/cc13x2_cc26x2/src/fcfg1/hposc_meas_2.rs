#[doc = "Register `HPOSC_MEAS_2` reader"]
pub struct R(crate::R<HPOSC_MEAS_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPOSC_MEAS_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPOSC_MEAS_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPOSC_MEAS_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPOSC_MEAS_2` writer"]
pub struct W(crate::W<HPOSC_MEAS_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPOSC_MEAS_2_SPEC>;
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
impl From<crate::W<HPOSC_MEAS_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPOSC_MEAS_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPOSC_DT2` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_DT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPOSC_DT2` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_DT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HPOSC_MEAS_2_SPEC, u8, u8, 8, O>;
#[doc = "Field `HPOSC_T2` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_T2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPOSC_T2` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_T2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPOSC_MEAS_2_SPEC, u8, u8, 8, O>;
#[doc = "Field `HPOSC_D2` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_D2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HPOSC_D2` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_D2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HPOSC_MEAS_2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_dt2(&self) -> HPOSC_DT2_R {
        HPOSC_DT2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_t2(&self) -> HPOSC_T2_R {
        HPOSC_T2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_d2(&self) -> HPOSC_D2_R {
        HPOSC_D2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_dt2(&mut self) -> HPOSC_DT2_W<0> {
        HPOSC_DT2_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_t2(&mut self) -> HPOSC_T2_W<8> {
        HPOSC_T2_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_d2(&mut self) -> HPOSC_D2_W<16> {
        HPOSC_D2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hposc_meas_2](index.html) module"]
pub struct HPOSC_MEAS_2_SPEC;
impl crate::RegisterSpec for HPOSC_MEAS_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hposc_meas_2::R](R) reader structure"]
impl crate::Readable for HPOSC_MEAS_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hposc_meas_2::W](W) writer structure"]
impl crate::Writable for HPOSC_MEAS_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPOSC_MEAS_2 to value 0"]
impl crate::Resettable for HPOSC_MEAS_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

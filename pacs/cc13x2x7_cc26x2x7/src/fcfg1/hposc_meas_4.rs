#[doc = "Register `HPOSC_MEAS_4` reader"]
pub struct R(crate::R<HPOSC_MEAS_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPOSC_MEAS_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPOSC_MEAS_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPOSC_MEAS_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPOSC_MEAS_4` writer"]
pub struct W(crate::W<HPOSC_MEAS_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPOSC_MEAS_4_SPEC>;
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
impl From<crate::W<HPOSC_MEAS_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPOSC_MEAS_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPOSC_DT4` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_DT4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPOSC_DT4` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_DT4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HPOSC_MEAS_4_SPEC, u8, u8, 8, O>;
#[doc = "Field `HPOSC_T4` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_T4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPOSC_T4` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_T4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPOSC_MEAS_4_SPEC, u8, u8, 8, O>;
#[doc = "Field `HPOSC_D4` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_D4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HPOSC_D4` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_D4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HPOSC_MEAS_4_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_dt4(&self) -> HPOSC_DT4_R {
        HPOSC_DT4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_t4(&self) -> HPOSC_T4_R {
        HPOSC_T4_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_d4(&self) -> HPOSC_D4_R {
        HPOSC_D4_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_dt4(&mut self) -> HPOSC_DT4_W<0> {
        HPOSC_DT4_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_t4(&mut self) -> HPOSC_T4_W<8> {
        HPOSC_T4_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_d4(&mut self) -> HPOSC_D4_W<16> {
        HPOSC_D4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hposc_meas_4](index.html) module"]
pub struct HPOSC_MEAS_4_SPEC;
impl crate::RegisterSpec for HPOSC_MEAS_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hposc_meas_4::R](R) reader structure"]
impl crate::Readable for HPOSC_MEAS_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hposc_meas_4::W](W) writer structure"]
impl crate::Writable for HPOSC_MEAS_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPOSC_MEAS_4 to value 0"]
impl crate::Resettable for HPOSC_MEAS_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

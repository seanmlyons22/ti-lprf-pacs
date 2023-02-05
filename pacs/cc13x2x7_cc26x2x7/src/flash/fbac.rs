#[doc = "Register `FBAC` reader"]
pub struct R(crate::R<FBAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBAC` writer"]
pub struct W(crate::W<FBAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBAC_SPEC>;
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
impl From<crate::W<FBAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREADS` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type VREADS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREADS` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type VREADS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBAC_SPEC, u8, u8, 8, O>;
#[doc = "Field `BAGP` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type BAGP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BAGP` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type BAGP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBAC_SPEC, u8, u8, 8, O>;
#[doc = "Field `OTPPROTDIS` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type OTPPROTDIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OTPPROTDIS` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type OTPPROTDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBAC_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED19` reader - 31:18\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED19_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED19` writer - 31:18\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBAC_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vreads(&self) -> VREADS_R {
        VREADS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bagp(&self) -> BAGP_R {
        BAGP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn otpprotdis(&self) -> OTPPROTDIS_R {
        OTPPROTDIS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vreads(&mut self) -> VREADS_W<0> {
        VREADS_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bagp(&mut self) -> BAGP_W<8> {
        BAGP_W::new(self)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn otpprotdis(&mut self) -> OTPPROTDIS_W<16> {
        OTPPROTDIS_W::new(self)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> RESERVED19_W<18> {
        RESERVED19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbac](index.html) module"]
pub struct FBAC_SPEC;
impl crate::RegisterSpec for FBAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbac::R](R) reader structure"]
impl crate::Readable for FBAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbac::W](W) writer structure"]
impl crate::Writable for FBAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBAC to value 0x0f"]
impl crate::Resettable for FBAC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}

#[doc = "Register `FVSLP` reader"]
pub struct R(crate::R<FVSLP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FVSLP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FVSLP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FVSLP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FVSLP` writer"]
pub struct W(crate::W<FVSLP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FVSLP_SPEC>;
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
impl From<crate::W<FVSLP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FVSLP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED0` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVSLP_SPEC, u16, u16, 12, O>;
#[doc = "Field `VSL_P` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type VSL_P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSL_P` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type VSL_P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVSLP_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVSLP_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vsl_p(&self) -> VSL_P_R {
        VSL_P_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vsl_p(&mut self) -> VSL_P_W<12> {
        VSL_P_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvslp](index.html) module"]
pub struct FVSLP_SPEC;
impl crate::RegisterSpec for FVSLP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fvslp::R](R) reader structure"]
impl crate::Readable for FVSLP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fvslp::W](W) writer structure"]
impl crate::Writable for FVSLP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FVSLP to value 0x8000"]
impl crate::Resettable for FVSLP_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}

#[doc = "Register `FLASH_PP` reader"]
pub struct R(crate::R<FLASH_PP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_PP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_PP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_PP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_PP` writer"]
pub struct W(crate::W<FLASH_PP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_PP_SPEC>;
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
impl From<crate::W<FLASH_PP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_PP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAX_PP` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type MAX_PP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAX_PP` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type MAX_PP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_PP_SPEC, u16, u16, 16, O>;
#[doc = "Field `TRIM3P4` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type TRIM3P4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM3P4` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type TRIM3P4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_PP_SPEC, u8, u8, 8, O>;
#[doc = "Field `PUMP_SU` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type PUMP_SU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUMP_SU` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type PUMP_SU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_PP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_pp(&self) -> MAX_PP_R {
        MAX_PP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim3p4(&self) -> TRIM3P4_R {
        TRIM3P4_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pump_su(&self) -> PUMP_SU_R {
        PUMP_SU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn max_pp(&mut self) -> MAX_PP_W<0> {
        MAX_PP_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim3p4(&mut self) -> TRIM3P4_W<16> {
        TRIM3P4_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pump_su(&mut self) -> PUMP_SU_W<24> {
        PUMP_SU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_pp](index.html) module"]
pub struct FLASH_PP_SPEC;
impl crate::RegisterSpec for FLASH_PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_pp::R](R) reader structure"]
impl crate::Readable for FLASH_PP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_pp::W](W) writer structure"]
impl crate::Writable for FLASH_PP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_PP to value 0x14"]
impl crate::Resettable for FLASH_PP_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}

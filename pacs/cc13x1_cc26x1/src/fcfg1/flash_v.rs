#[doc = "Register `FLASH_V` reader"]
pub struct R(crate::R<FLASH_V_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_V_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_V_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_V_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_V` writer"]
pub struct W(crate::W<FLASH_V_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_V_SPEC>;
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
impl From<crate::W<FLASH_V_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_V_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM0P8` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type TRIM0P8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM0P8` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type TRIM0P8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_V_SPEC, u8, u8, 8, O>;
#[doc = "Field `V_READ` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type V_READ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `V_READ` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type V_READ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_V_SPEC, u8, u8, 8, O>;
#[doc = "Field `VWL_P` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type VWL_P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VWL_P` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type VWL_P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_V_SPEC, u8, u8, 8, O>;
#[doc = "Field `VSL_P` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type VSL_P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSL_P` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type VSL_P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_V_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim0p8(&self) -> TRIM0P8_R {
        TRIM0P8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v_read(&self) -> V_READ_R {
        V_READ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vwl_p(&self) -> VWL_P_R {
        VWL_P_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vsl_p(&self) -> VSL_P_R {
        VSL_P_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim0p8(&mut self) -> TRIM0P8_W<0> {
        TRIM0P8_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn v_read(&mut self) -> V_READ_W<8> {
        V_READ_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vwl_p(&mut self) -> VWL_P_W<16> {
        VWL_P_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vsl_p(&mut self) -> VSL_P_W<24> {
        VSL_P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_v](index.html) module"]
pub struct FLASH_V_SPEC;
impl crate::RegisterSpec for FLASH_V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_v::R](R) reader structure"]
impl crate::Readable for FLASH_V_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_v::W](W) writer structure"]
impl crate::Writable for FLASH_V_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_V to value 0"]
impl crate::Resettable for FLASH_V_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `FLASH_VHV_E` reader"]
pub struct R(crate::R<FLASH_VHV_E_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_VHV_E_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_VHV_E_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_VHV_E_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_VHV_E` writer"]
pub struct W(crate::W<FLASH_VHV_E_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_VHV_E_SPEC>;
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
impl From<crate::W<FLASH_VHV_E_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_VHV_E_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VHV_E_STEP_HIGHT` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type VHV_E_STEP_HIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VHV_E_STEP_HIGHT` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type VHV_E_STEP_HIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_VHV_E_SPEC, u16, u16, 16, O>;
#[doc = "Field `VHV_E_START` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type VHV_E_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VHV_E_START` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type VHV_E_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_VHV_E_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_e_step_hight(&self) -> VHV_E_STEP_HIGHT_R {
        VHV_E_STEP_HIGHT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_e_start(&self) -> VHV_E_START_R {
        VHV_E_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhv_e_step_hight(&mut self) -> VHV_E_STEP_HIGHT_W<0> {
        VHV_E_STEP_HIGHT_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhv_e_start(&mut self) -> VHV_E_START_W<16> {
        VHV_E_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_vhv_e](index.html) module"]
pub struct FLASH_VHV_E_SPEC;
impl crate::RegisterSpec for FLASH_VHV_E_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_vhv_e::R](R) reader structure"]
impl crate::Readable for FLASH_VHV_E_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_vhv_e::W](W) writer structure"]
impl crate::Writable for FLASH_VHV_E_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_VHV_E to value 0x01"]
impl crate::Resettable for FLASH_VHV_E_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

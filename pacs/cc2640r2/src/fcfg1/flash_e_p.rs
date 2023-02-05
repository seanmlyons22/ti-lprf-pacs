#[doc = "Register `FLASH_E_P` reader"]
pub struct R(crate::R<FLASH_E_P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_E_P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_E_P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_E_P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_E_P` writer"]
pub struct W(crate::W<FLASH_E_P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_E_P_SPEC>;
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
impl From<crate::W<FLASH_E_P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_E_P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVSU` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EVSU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EVSU` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EVSU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_E_P_SPEC, u8, u8, 8, O>;
#[doc = "Field `PVSU` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PVSU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PVSU` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PVSU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_E_P_SPEC, u8, u8, 8, O>;
#[doc = "Field `ESU` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type ESU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ESU` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type ESU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_E_P_SPEC, u8, u8, 8, O>;
#[doc = "Field `PSU` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type PSU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSU` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type PSU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_E_P_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn evsu(&self) -> EVSU_R {
        EVSU_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvsu(&self) -> PVSU_R {
        PVSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn esu(&self) -> ESU_R {
        ESU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psu(&self) -> PSU_R {
        PSU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn evsu(&mut self) -> EVSU_W<0> {
        EVSU_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pvsu(&mut self) -> PVSU_W<8> {
        PVSU_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn esu(&mut self) -> ESU_W<16> {
        ESU_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn psu(&mut self) -> PSU_W<24> {
        PSU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_e_p](index.html) module"]
pub struct FLASH_E_P_SPEC;
impl crate::RegisterSpec for FLASH_E_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_e_p::R](R) reader structure"]
impl crate::Readable for FLASH_E_P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_e_p::W](W) writer structure"]
impl crate::Writable for FLASH_E_P_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_E_P to value 0x1733_1a33"]
impl crate::Resettable for FLASH_E_P_SPEC {
    const RESET_VALUE: Self::Ux = 0x1733_1a33;
}

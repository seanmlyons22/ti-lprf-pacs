#[doc = "Register `FLASH_EH_SEQ` reader"]
pub struct R(crate::R<FLASH_EH_SEQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_EH_SEQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_EH_SEQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_EH_SEQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_EH_SEQ` writer"]
pub struct W(crate::W<FLASH_EH_SEQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_EH_SEQ_SPEC>;
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
impl From<crate::W<FLASH_EH_SEQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_EH_SEQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SM_FREQUENCY` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type SM_FREQUENCY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SM_FREQUENCY` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type SM_FREQUENCY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_EH_SEQ_SPEC, u16, u16, 12, O>;
#[doc = "Field `VSTAT` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type VSTAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSTAT` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type VSTAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_EH_SEQ_SPEC, u8, u8, 4, O>;
#[doc = "Field `SEQ` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type SEQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type SEQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_EH_SEQ_SPEC, u8, u8, 8, O>;
#[doc = "Field `EH` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type EH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EH` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type EH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_EH_SEQ_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sm_frequency(&self) -> SM_FREQUENCY_R {
        SM_FREQUENCY_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vstat(&self) -> VSTAT_R {
        VSTAT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn seq(&self) -> SEQ_R {
        SEQ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eh(&self) -> EH_R {
        EH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sm_frequency(&mut self) -> SM_FREQUENCY_W<0> {
        SM_FREQUENCY_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vstat(&mut self) -> VSTAT_W<12> {
        VSTAT_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn seq(&mut self) -> SEQ_W<16> {
        SEQ_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn eh(&mut self) -> EH_W<24> {
        EH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_eh_seq](index.html) module"]
pub struct FLASH_EH_SEQ_SPEC;
impl crate::RegisterSpec for FLASH_EH_SEQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_eh_seq::R](R) reader structure"]
impl crate::Readable for FLASH_EH_SEQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_eh_seq::W](W) writer structure"]
impl crate::Writable for FLASH_EH_SEQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_EH_SEQ to value 0x0200_f000"]
impl crate::Resettable for FLASH_EH_SEQ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_f000;
}

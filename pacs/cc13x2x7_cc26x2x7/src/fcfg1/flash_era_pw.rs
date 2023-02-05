#[doc = "Register `FLASH_ERA_PW` reader"]
pub struct R(crate::R<FLASH_ERA_PW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_ERA_PW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_ERA_PW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_ERA_PW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_ERA_PW` writer"]
pub struct W(crate::W<FLASH_ERA_PW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_ERA_PW_SPEC>;
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
impl From<crate::W<FLASH_ERA_PW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_ERA_PW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASE_PW` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type ERASE_PW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ERASE_PW` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type ERASE_PW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_ERA_PW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn erase_pw(&self) -> ERASE_PW_R {
        ERASE_PW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn erase_pw(&mut self) -> ERASE_PW_W<0> {
        ERASE_PW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_era_pw](index.html) module"]
pub struct FLASH_ERA_PW_SPEC;
impl crate::RegisterSpec for FLASH_ERA_PW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_era_pw::R](R) reader structure"]
impl crate::Readable for FLASH_ERA_PW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_era_pw::W](W) writer structure"]
impl crate::Writable for FLASH_ERA_PW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_ERA_PW to value 0x0fa0"]
impl crate::Resettable for FLASH_ERA_PW_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fa0;
}

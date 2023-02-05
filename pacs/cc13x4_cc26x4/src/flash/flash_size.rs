#[doc = "Register `FLASH_SIZE` reader"]
pub struct R(crate::R<FLASH_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_SIZE` writer"]
pub struct W(crate::W<FLASH_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SIZE_SPEC>;
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
impl From<crate::W<FLASH_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 6:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 6:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_SIZE_SPEC, u8, u8, 7, O>;
#[doc = "Field `SECTORS` reader - 9:7\\]
Internal. Only to be used through TI provided API."]
pub type SECTORS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECTORS` writer - 9:7\\]
Internal. Only to be used through TI provided API."]
pub type SECTORS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_SIZE_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED10` reader - 31:10\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED10_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_SIZE_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:9 - 9:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sectors(&self) -> SECTORS_R {
        SECTORS_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 7:9 - 9:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sectors(&mut self) -> SECTORS_W<7> {
        SECTORS_W::new(self)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_size](index.html) module"]
pub struct FLASH_SIZE_SPEC;
impl crate::RegisterSpec for FLASH_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_size::R](R) reader structure"]
impl crate::Readable for FLASH_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_size::W](W) writer structure"]
impl crate::Writable for FLASH_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_SIZE to value 0x0200"]
impl crate::Resettable for FLASH_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}

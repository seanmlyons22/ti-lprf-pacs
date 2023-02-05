#[doc = "Register `FLASH_VHV` reader"]
pub struct R(crate::R<FLASH_VHV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_VHV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_VHV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_VHV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_VHV` writer"]
pub struct W(crate::W<FLASH_VHV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_VHV_SPEC>;
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
impl From<crate::W<FLASH_VHV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_VHV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VHV_E` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VHV_E_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VHV_E` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VHV_E_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_VHV_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED0` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_VHV_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIM13_E` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type TRIM13_E_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM13_E` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type TRIM13_E_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_VHV_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED1` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_VHV_SPEC, u8, u8, 4, O>;
#[doc = "Field `VHV_P` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VHV_P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VHV_P` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VHV_P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_VHV_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED2` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_VHV_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIM13_P` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type TRIM13_P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM13_P` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type TRIM13_P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_VHV_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED3` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED3` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_VHV_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_e(&self) -> VHV_E_R {
        VHV_E_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_e(&self) -> TRIM13_E_R {
        TRIM13_E_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_p(&self) -> VHV_P_R {
        VHV_P_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_p(&self) -> TRIM13_P_R {
        TRIM13_P_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhv_e(&mut self) -> VHV_E_W<0> {
        VHV_E_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<4> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim13_e(&mut self) -> TRIM13_E_W<8> {
        TRIM13_E_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<12> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhv_p(&mut self) -> VHV_P_W<16> {
        VHV_P_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<20> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim13_p(&mut self) -> TRIM13_P_W<24> {
        TRIM13_P_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<28> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_vhv](index.html) module"]
pub struct FLASH_VHV_SPEC;
impl crate::RegisterSpec for FLASH_VHV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_vhv::R](R) reader structure"]
impl crate::Readable for FLASH_VHV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_vhv::W](W) writer structure"]
impl crate::Writable for FLASH_VHV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_VHV to value 0x04"]
impl crate::Resettable for FLASH_VHV_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}

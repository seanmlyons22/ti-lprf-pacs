#[doc = "Register `FLASH_VHV_PV` reader"]
pub struct R(crate::R<FLASH_VHV_PV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_VHV_PV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_VHV_PV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_VHV_PV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_VHV_PV` writer"]
pub struct W(crate::W<FLASH_VHV_PV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_VHV_PV_SPEC>;
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
impl From<crate::W<FLASH_VHV_PV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_VHV_PV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VINH` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type VINH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VINH` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type VINH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_VHV_PV_SPEC, u8, u8, 8, O>;
#[doc = "Field `VCG2P5` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type VCG2P5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCG2P5` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type VCG2P5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_VHV_PV_SPEC, u8, u8, 8, O>;
#[doc = "Field `VHV_PV` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VHV_PV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VHV_PV` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VHV_PV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_VHV_PV_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED0` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_VHV_PV_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIM13_PV` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type TRIM13_PV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM13_PV` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type TRIM13_PV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_VHV_PV_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED1` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_VHV_PV_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vinh(&self) -> VINH_R {
        VINH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vcg2p5(&self) -> VCG2P5_R {
        VCG2P5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_pv(&self) -> VHV_PV_R {
        VHV_PV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_pv(&self) -> TRIM13_PV_R {
        TRIM13_PV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vinh(&mut self) -> VINH_W<0> {
        VINH_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vcg2p5(&mut self) -> VCG2P5_W<8> {
        VCG2P5_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhv_pv(&mut self) -> VHV_PV_W<16> {
        VHV_PV_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<20> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim13_pv(&mut self) -> TRIM13_PV_W<24> {
        TRIM13_PV_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<28> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_vhv_pv](index.html) module"]
pub struct FLASH_VHV_PV_SPEC;
impl crate::RegisterSpec for FLASH_VHV_PV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_vhv_pv::R](R) reader structure"]
impl crate::Readable for FLASH_VHV_PV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_vhv_pv::W](W) writer structure"]
impl crate::Writable for FLASH_VHV_PV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_VHV_PV to value 0x0008_0001"]
impl crate::Resettable for FLASH_VHV_PV_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0001;
}

#[doc = "Register `FWPWRITE_ECC` reader"]
pub struct R(crate::R<FWPWRITE_ECC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWPWRITE_ECC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWPWRITE_ECC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWPWRITE_ECC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWPWRITE_ECC` writer"]
pub struct W(crate::W<FWPWRITE_ECC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWPWRITE_ECC_SPEC>;
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
impl From<crate::W<FWPWRITE_ECC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWPWRITE_ECC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCBYTES31_24` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type ECCBYTES31_24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECCBYTES31_24` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type ECCBYTES31_24_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FWPWRITE_ECC_SPEC, u8, u8, 8, O>;
#[doc = "Field `ECCBYTES23_16` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type ECCBYTES23_16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECCBYTES23_16` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type ECCBYTES23_16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FWPWRITE_ECC_SPEC, u8, u8, 8, O>;
#[doc = "Field `ECCBYTES15_08` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type ECCBYTES15_08_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECCBYTES15_08` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type ECCBYTES15_08_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FWPWRITE_ECC_SPEC, u8, u8, 8, O>;
#[doc = "Field `ECCBYTES07_00` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type ECCBYTES07_00_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECCBYTES07_00` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type ECCBYTES07_00_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FWPWRITE_ECC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes31_24(&self) -> ECCBYTES31_24_R {
        ECCBYTES31_24_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes23_16(&self) -> ECCBYTES23_16_R {
        ECCBYTES23_16_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes15_08(&self) -> ECCBYTES15_08_R {
        ECCBYTES15_08_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes07_00(&self) -> ECCBYTES07_00_R {
        ECCBYTES07_00_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn eccbytes31_24(&mut self) -> ECCBYTES31_24_W<0> {
        ECCBYTES31_24_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn eccbytes23_16(&mut self) -> ECCBYTES23_16_W<8> {
        ECCBYTES23_16_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn eccbytes15_08(&mut self) -> ECCBYTES15_08_W<16> {
        ECCBYTES15_08_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn eccbytes07_00(&mut self) -> ECCBYTES07_00_W<24> {
        ECCBYTES07_00_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite_ecc](index.html) module"]
pub struct FWPWRITE_ECC_SPEC;
impl crate::RegisterSpec for FWPWRITE_ECC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwpwrite_ecc::R](R) reader structure"]
impl crate::Readable for FWPWRITE_ECC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwpwrite_ecc::W](W) writer structure"]
impl crate::Writable for FWPWRITE_ECC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FWPWRITE_ECC to value 0xffff_ffff"]
impl crate::Resettable for FWPWRITE_ECC_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

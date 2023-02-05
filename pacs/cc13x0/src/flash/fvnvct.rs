#[doc = "Register `FVNVCT` reader"]
pub struct R(crate::R<FVNVCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FVNVCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FVNVCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FVNVCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FVNVCT` writer"]
pub struct W(crate::W<FVNVCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FVNVCT_SPEC>;
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
impl From<crate::W<FVNVCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FVNVCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VIN_CT` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type VIN_CT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VIN_CT` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type VIN_CT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVNVCT_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED5` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVNVCT_SPEC, u8, u8, 3, O>;
#[doc = "Field `VCG2P5CT` reader - 12:8\\]
Internal. Only to be used through TI provided API."]
pub type VCG2P5CT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCG2P5CT` writer - 12:8\\]
Internal. Only to be used through TI provided API."]
pub type VCG2P5CT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVNVCT_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED13` reader - 31:13\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED13_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED13` writer - 31:13\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVNVCT_SPEC, u32, u32, 19, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_ct(&self) -> VIN_CT_R {
        VIN_CT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vcg2p5ct(&self) -> VCG2P5CT_R {
        VCG2P5CT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_ct(&mut self) -> VIN_CT_W<0> {
        VIN_CT_W::new(self)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vcg2p5ct(&mut self) -> VCG2P5CT_W<8> {
        VCG2P5CT_W::new(self)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> RESERVED13_W<13> {
        RESERVED13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvnvct](index.html) module"]
pub struct FVNVCT_SPEC;
impl crate::RegisterSpec for FVNVCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fvnvct::R](R) reader structure"]
impl crate::Readable for FVNVCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fvnvct::W](W) writer structure"]
impl crate::Writable for FVNVCT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FVNVCT to value 0x0800"]
impl crate::Resettable for FVNVCT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}

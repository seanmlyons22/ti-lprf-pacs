#[doc = "Register `FTCR` reader"]
pub struct R(crate::R<FTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTCR` writer"]
pub struct W(crate::W<FTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTCR_SPEC>;
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
impl From<crate::W<FTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCR` reader - 6:0\\]
Internal. Only to be used through TI provided API."]
pub type TCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCR` writer - 6:0\\]
Internal. Only to be used through TI provided API."]
pub type TCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FTCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED7` writer - 31:7\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FTCR_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn tcr(&mut self) -> TCR_W<0> {
        TCR_W::new(self)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftcr](index.html) module"]
pub struct FTCR_SPEC;
impl crate::RegisterSpec for FTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftcr::R](R) reader structure"]
impl crate::Readable for FTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftcr::W](W) writer structure"]
impl crate::Writable for FTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTCR to value 0"]
impl crate::Resettable for FTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

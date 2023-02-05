#[doc = "Register `FEDACCTL1` reader"]
pub struct R(crate::R<FEDACCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEDACCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEDACCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEDACCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEDACCTL1` writer"]
pub struct W(crate::W<FEDACCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEDACCTL1_SPEC>;
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
impl From<crate::W<FEDACCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEDACCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDACEN` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type EDACEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EDACEN` writer - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type EDACEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FEDACCTL1_SPEC, u32, u32, 24, O>;
#[doc = "Field `SUSP_IGNR` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type SUSP_IGNR_R = crate::BitReader<bool>;
#[doc = "Field `SUSP_IGNR` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type SUSP_IGNR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEDACCTL1_SPEC, bool, O>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FEDACCTL1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn edacen(&self) -> EDACEN_R {
        EDACEN_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn susp_ignr(&self) -> SUSP_IGNR_R {
        SUSP_IGNR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn edacen(&mut self) -> EDACEN_W<0> {
        EDACEN_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn susp_ignr(&mut self) -> SUSP_IGNR_W<24> {
        SUSP_IGNR_W::new(self)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> RESERVED25_W<25> {
        RESERVED25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fedacctl1](index.html) module"]
pub struct FEDACCTL1_SPEC;
impl crate::RegisterSpec for FEDACCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fedacctl1::R](R) reader structure"]
impl crate::Readable for FEDACCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fedacctl1::W](W) writer structure"]
impl crate::Writable for FEDACCTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEDACCTL1 to value 0"]
impl crate::Resettable for FEDACCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

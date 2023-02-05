#[doc = "Register `EFUSECRA` reader"]
pub struct R(crate::R<EFUSECRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSECRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSECRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSECRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSECRA` writer"]
pub struct W(crate::W<EFUSECRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSECRA_SPEC>;
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
impl From<crate::W<EFUSECRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSECRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSECRA_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSECRA_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efusecra](index.html) module"]
pub struct EFUSECRA_SPEC;
impl crate::RegisterSpec for EFUSECRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efusecra::R](R) reader structure"]
impl crate::Readable for EFUSECRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efusecra::W](W) writer structure"]
impl crate::Writable for EFUSECRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFUSECRA to value 0"]
impl crate::Resettable for EFUSECRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

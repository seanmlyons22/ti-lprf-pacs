#[doc = "Register `TEMPP1` reader"]
pub struct R(crate::R<TEMPP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMPP1` writer"]
pub struct W(crate::W<TEMPP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMPP1_SPEC>;
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
impl From<crate::W<TEMPP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMPP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEMPP1_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEMPP1_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 0x3f) as u8)
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
    pub fn cfg(&mut self) -> CFG_W<0> {
        CFG_W::new(self)
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempp1](index.html) module"]
pub struct TEMPP1_SPEC;
impl crate::RegisterSpec for TEMPP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tempp1::R](R) reader structure"]
impl crate::Readable for TEMPP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tempp1::W](W) writer structure"]
impl crate::Writable for TEMPP1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEMPP1 to value 0"]
impl crate::Resettable for TEMPP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `STMPXCNTCAPT1` reader"]
pub struct R(crate::R<STMPXCNTCAPT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STMPXCNTCAPT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STMPXCNTCAPT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STMPXCNTCAPT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STMPXCNTCAPT1` writer"]
pub struct W(crate::W<STMPXCNTCAPT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STMPXCNTCAPT1_SPEC>;
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
impl From<crate::W<STMPXCNTCAPT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STMPXCNTCAPT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPT_VALUE` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type CAPT_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAPT_VALUE` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type CAPT_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STMPXCNTCAPT1_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STMPXCNTCAPT1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn capt_value(&self) -> CAPT_VALUE_R {
        CAPT_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn capt_value(&mut self) -> CAPT_VALUE_W<0> {
        CAPT_VALUE_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpxcntcapt1](index.html) module"]
pub struct STMPXCNTCAPT1_SPEC;
impl crate::RegisterSpec for STMPXCNTCAPT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stmpxcntcapt1::R](R) reader structure"]
impl crate::Readable for STMPXCNTCAPT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stmpxcntcapt1::W](W) writer structure"]
impl crate::Writable for STMPXCNTCAPT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STMPXCNTCAPT1 to value 0"]
impl crate::Resettable for STMPXCNTCAPT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

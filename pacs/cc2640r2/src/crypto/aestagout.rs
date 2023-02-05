#[doc = "Register `AESTAGOUT` reader"]
pub struct R(crate::R<AESTAGOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESTAGOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESTAGOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESTAGOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESTAGOUT` writer"]
pub struct W(crate::W<AESTAGOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESTAGOUT_SPEC>;
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
impl From<crate::W<AESTAGOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESTAGOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAG` reader - 31:0\\]
This register contains the authentication TAG for the combined and authentication-only modes."]
pub type TAG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TAG` writer - 31:0\\]
This register contains the authentication TAG for the combined and authentication-only modes."]
pub type TAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESTAGOUT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the authentication TAG for the combined and authentication-only modes."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the authentication TAG for the combined and authentication-only modes."]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TAG_W<0> {
        TAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Tag Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aestagout](index.html) module"]
pub struct AESTAGOUT_SPEC;
impl crate::RegisterSpec for AESTAGOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aestagout::R](R) reader structure"]
impl crate::Readable for AESTAGOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aestagout::W](W) writer structure"]
impl crate::Writable for AESTAGOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESTAGOUT to value 0"]
impl crate::Resettable for AESTAGOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

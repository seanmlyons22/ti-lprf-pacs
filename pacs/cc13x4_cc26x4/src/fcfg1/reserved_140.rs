#[doc = "Register `RESERVED_140` reader"]
pub struct R(crate::R<RESERVED_140_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESERVED_140_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESERVED_140_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESERVED_140_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESERVED_140` writer"]
pub struct W(crate::W<RESERVED_140_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESERVED_140_SPEC>;
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
impl From<crate::W<RESERVED_140_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESERVED_140_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved_140](index.html) module"]
pub struct RESERVED_140_SPEC;
impl crate::RegisterSpec for RESERVED_140_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reserved_140::R](R) reader structure"]
impl crate::Readable for RESERVED_140_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reserved_140::W](W) writer structure"]
impl crate::Writable for RESERVED_140_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESERVED_140 to value 0xffff_ffff"]
impl crate::Resettable for RESERVED_140_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

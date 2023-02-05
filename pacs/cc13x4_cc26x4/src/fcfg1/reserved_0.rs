#[doc = "Register `RESERVED_0` reader"]
pub struct R(crate::R<RESERVED_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESERVED_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESERVED_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESERVED_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESERVED_0` writer"]
pub struct W(crate::W<RESERVED_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESERVED_0_SPEC>;
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
impl From<crate::W<RESERVED_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESERVED_0_SPEC>) -> Self {
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
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved_0](index.html) module"]
pub struct RESERVED_0_SPEC;
impl crate::RegisterSpec for RESERVED_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reserved_0::R](R) reader structure"]
impl crate::Readable for RESERVED_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reserved_0::W](W) writer structure"]
impl crate::Writable for RESERVED_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESERVED_0 to value 0xffff_ffff"]
impl crate::Resettable for RESERVED_0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

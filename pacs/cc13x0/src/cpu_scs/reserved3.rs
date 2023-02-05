#[doc = "Register `RESERVED3` reader"]
pub struct R(crate::R<RESERVED3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESERVED3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESERVED3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESERVED3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESERVED3` writer"]
pub struct W(crate::W<RESERVED3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESERVED3_SPEC>;
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
impl From<crate::W<RESERVED3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESERVED3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED0` writer - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RESERVED3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved3](index.html) module"]
pub struct RESERVED3_SPEC;
impl crate::RegisterSpec for RESERVED3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reserved3::R](R) reader structure"]
impl crate::Readable for RESERVED3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reserved3::W](W) writer structure"]
impl crate::Writable for RESERVED3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESERVED3 to value 0"]
impl crate::Resettable for RESERVED3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

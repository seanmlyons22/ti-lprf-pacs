#[doc = "Register `PIDR6` reader"]
pub struct R(crate::R<PIDR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIDR6` writer"]
pub struct W(crate::W<PIDR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIDR6_SPEC>;
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
impl From<crate::W<PIDR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIDR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED0` writer - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIDR6_SPEC, u32, u32, 32, O>;
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
#[doc = "Provides CoreSight discovery information for the FP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr6](index.html) module"]
pub struct PIDR6_SPEC;
impl crate::RegisterSpec for PIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr6::R](R) reader structure"]
impl crate::Readable for PIDR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pidr6::W](W) writer structure"]
impl crate::Writable for PIDR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIDR6 to value 0"]
impl crate::Resettable for PIDR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

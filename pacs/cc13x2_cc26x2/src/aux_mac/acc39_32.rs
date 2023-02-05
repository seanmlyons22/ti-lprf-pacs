#[doc = "Register `ACC39_32` reader"]
pub struct R(crate::R<ACC39_32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACC39_32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACC39_32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACC39_32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACC39_32` writer"]
pub struct W(crate::W<ACC39_32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACC39_32_SPEC>;
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
impl From<crate::W<ACC39_32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACC39_32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - 7:0\\]
Value of the accumulator, bits 39:32. Write VALUE to initialize bits 39:32 of accumulator."]
pub type VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VALUE` writer - 7:0\\]
Value of the accumulator, bits 39:32. Write VALUE to initialize bits 39:32 of accumulator."]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACC39_32_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACC39_32_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Value of the accumulator, bits 39:32. Write VALUE to initialize bits 39:32 of accumulator."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Value of the accumulator, bits 39:32. Write VALUE to initialize bits 39:32 of accumulator."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Accumulator Bits 39:32\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc39_32](index.html) module"]
pub struct ACC39_32_SPEC;
impl crate::RegisterSpec for ACC39_32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acc39_32::R](R) reader structure"]
impl crate::Readable for ACC39_32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acc39_32::W](W) writer structure"]
impl crate::Writable for ACC39_32_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACC39_32 to value 0"]
impl crate::Resettable for ACC39_32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

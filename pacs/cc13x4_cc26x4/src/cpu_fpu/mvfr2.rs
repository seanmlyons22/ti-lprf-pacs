#[doc = "Register `MVFR2` reader"]
pub struct R(crate::R<MVFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MVFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MVFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MVFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MVFR2` writer"]
pub struct W(crate::W<MVFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MVFR2_SPEC>;
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
impl From<crate::W<MVFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MVFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `FPMisc` reader - 7:4\\]
Indicates support for miscellaneous FP features"]
pub type FPMISC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPMisc` writer - 7:4\\]
Indicates support for miscellaneous FP features"]
pub type FPMISC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR2_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates support for miscellaneous FP features"]
    #[inline(always)]
    pub fn fpmisc(&self) -> FPMISC_R {
        FPMISC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates support for miscellaneous FP features"]
    #[inline(always)]
    #[must_use]
    pub fn fpmisc(&mut self) -> FPMISC_W<4> {
        FPMISC_W::new(self)
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
#[doc = "Describes the features provided by the Floating-point Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mvfr2](index.html) module"]
pub struct MVFR2_SPEC;
impl crate::RegisterSpec for MVFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mvfr2::R](R) reader structure"]
impl crate::Readable for MVFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mvfr2::W](W) writer structure"]
impl crate::Writable for MVFR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MVFR2 to value 0"]
impl crate::Resettable for MVFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

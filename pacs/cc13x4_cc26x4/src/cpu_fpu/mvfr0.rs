#[doc = "Register `MVFR0` reader"]
pub struct R(crate::R<MVFR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MVFR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MVFR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MVFR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MVFR0` writer"]
pub struct W(crate::W<MVFR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MVFR0_SPEC>;
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
impl From<crate::W<MVFR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MVFR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIMDReg` reader - 3:0\\]
Indicates size of FP register file"]
pub type SIMDREG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIMDReg` writer - 3:0\\]
Indicates size of FP register file"]
pub type SIMDREG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `FPSP` reader - 7:4\\]
Indicates support for FP single-precision operations"]
pub type FPSP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPSP` writer - 7:4\\]
Indicates support for FP single-precision operations"]
pub type FPSP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `FPDP` reader - 11:8\\]
Indicates support for FP double-precision operations"]
pub type FPDP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPDP` writer - 11:8\\]
Indicates support for FP double-precision operations"]
pub type FPDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED12` writer - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `FPDivide` reader - 19:16\\]
Indicates the support for FP divide operations"]
pub type FPDIVIDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPDivide` writer - 19:16\\]
Indicates the support for FP divide operations"]
pub type FPDIVIDE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `FPSqrt` reader - 23:20\\]
Indicates the support for FP square root operations"]
pub type FPSQRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPSqrt` writer - 23:20\\]
Indicates the support for FP square root operations"]
pub type FPSQRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED24` reader - 27:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 27:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `FPRound` reader - 31:28\\]
Indicates the rounding modes supported by the FP Extension"]
pub type FPROUND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPRound` writer - 31:28\\]
Indicates the rounding modes supported by the FP Extension"]
pub type FPROUND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates size of FP register file"]
    #[inline(always)]
    pub fn simdreg(&self) -> SIMDREG_R {
        SIMDREG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates support for FP single-precision operations"]
    #[inline(always)]
    pub fn fpsp(&self) -> FPSP_R {
        FPSP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates support for FP double-precision operations"]
    #[inline(always)]
    pub fn fpdp(&self) -> FPDP_R {
        FPDP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the support for FP divide operations"]
    #[inline(always)]
    pub fn fpdivide(&self) -> FPDIVIDE_R {
        FPDIVIDE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the support for FP square root operations"]
    #[inline(always)]
    pub fn fpsqrt(&self) -> FPSQRT_R {
        FPSQRT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the rounding modes supported by the FP Extension"]
    #[inline(always)]
    pub fn fpround(&self) -> FPROUND_R {
        FPROUND_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates size of FP register file"]
    #[inline(always)]
    #[must_use]
    pub fn simdreg(&mut self) -> SIMDREG_W<0> {
        SIMDREG_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates support for FP single-precision operations"]
    #[inline(always)]
    #[must_use]
    pub fn fpsp(&mut self) -> FPSP_W<4> {
        FPSP_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates support for FP double-precision operations"]
    #[inline(always)]
    #[must_use]
    pub fn fpdp(&mut self) -> FPDP_W<8> {
        FPDP_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the support for FP divide operations"]
    #[inline(always)]
    #[must_use]
    pub fn fpdivide(&mut self) -> FPDIVIDE_W<16> {
        FPDIVIDE_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the support for FP square root operations"]
    #[inline(always)]
    #[must_use]
    pub fn fpsqrt(&mut self) -> FPSQRT_W<20> {
        FPSQRT_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the rounding modes supported by the FP Extension"]
    #[inline(always)]
    #[must_use]
    pub fn fpround(&mut self) -> FPROUND_W<28> {
        FPROUND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Describes the features provided by the Floating-point Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mvfr0](index.html) module"]
pub struct MVFR0_SPEC;
impl crate::RegisterSpec for MVFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mvfr0::R](R) reader structure"]
impl crate::Readable for MVFR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mvfr0::W](W) writer structure"]
impl crate::Writable for MVFR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MVFR0 to value 0"]
impl crate::Resettable for MVFR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

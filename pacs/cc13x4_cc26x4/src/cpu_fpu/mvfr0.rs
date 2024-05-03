#[doc = "Register `MVFR0` reader"]
pub type R = crate::R<Mvfr0Spec>;
#[doc = "Register `MVFR0` writer"]
pub type W = crate::W<Mvfr0Spec>;
#[doc = "Field `SIMDReg` reader - 3:0\\]
Indicates size of FP register file"]
pub type SimdregR = crate::FieldReader;
#[doc = "Field `SIMDReg` writer - 3:0\\]
Indicates size of FP register file"]
pub type SimdregW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FPSP` reader - 7:4\\]
Indicates support for FP single-precision operations"]
pub type FpspR = crate::FieldReader;
#[doc = "Field `FPSP` writer - 7:4\\]
Indicates support for FP single-precision operations"]
pub type FpspW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FPDP` reader - 11:8\\]
Indicates support for FP double-precision operations"]
pub type FpdpR = crate::FieldReader;
#[doc = "Field `FPDP` writer - 11:8\\]
Indicates support for FP double-precision operations"]
pub type FpdpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader;
#[doc = "Field `RESERVED12` writer - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FPDivide` reader - 19:16\\]
Indicates the support for FP divide operations"]
pub type FpdivideR = crate::FieldReader;
#[doc = "Field `FPDivide` writer - 19:16\\]
Indicates the support for FP divide operations"]
pub type FpdivideW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FPSqrt` reader - 23:20\\]
Indicates the support for FP square root operations"]
pub type FpsqrtR = crate::FieldReader;
#[doc = "Field `FPSqrt` writer - 23:20\\]
Indicates the support for FP square root operations"]
pub type FpsqrtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED24` reader - 27:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 27:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FPRound` reader - 31:28\\]
Indicates the rounding modes supported by the FP Extension"]
pub type FproundR = crate::FieldReader;
#[doc = "Field `FPRound` writer - 31:28\\]
Indicates the rounding modes supported by the FP Extension"]
pub type FproundW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates size of FP register file"]
    #[inline(always)]
    pub fn simdreg(&self) -> SimdregR {
        SimdregR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates support for FP single-precision operations"]
    #[inline(always)]
    pub fn fpsp(&self) -> FpspR {
        FpspR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates support for FP double-precision operations"]
    #[inline(always)]
    pub fn fpdp(&self) -> FpdpR {
        FpdpR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the support for FP divide operations"]
    #[inline(always)]
    pub fn fpdivide(&self) -> FpdivideR {
        FpdivideR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the support for FP square root operations"]
    #[inline(always)]
    pub fn fpsqrt(&self) -> FpsqrtR {
        FpsqrtR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the rounding modes supported by the FP Extension"]
    #[inline(always)]
    pub fn fpround(&self) -> FproundR {
        FproundR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates size of FP register file"]
    #[inline(always)]
    #[must_use]
    pub fn simdreg(&mut self) -> SimdregW<Mvfr0Spec> {
        SimdregW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates support for FP single-precision operations"]
    #[inline(always)]
    #[must_use]
    pub fn fpsp(&mut self) -> FpspW<Mvfr0Spec> {
        FpspW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates support for FP double-precision operations"]
    #[inline(always)]
    #[must_use]
    pub fn fpdp(&mut self) -> FpdpW<Mvfr0Spec> {
        FpdpW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<Mvfr0Spec> {
        Reserved12W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the support for FP divide operations"]
    #[inline(always)]
    #[must_use]
    pub fn fpdivide(&mut self) -> FpdivideW<Mvfr0Spec> {
        FpdivideW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the support for FP square root operations"]
    #[inline(always)]
    #[must_use]
    pub fn fpsqrt(&mut self) -> FpsqrtW<Mvfr0Spec> {
        FpsqrtW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<Mvfr0Spec> {
        Reserved24W::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the rounding modes supported by the FP Extension"]
    #[inline(always)]
    #[must_use]
    pub fn fpround(&mut self) -> FproundW<Mvfr0Spec> {
        FproundW::new(self, 28)
    }
}
#[doc = "Describes the features provided by the Floating-point Extension\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mvfr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mvfr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mvfr0Spec;
impl crate::RegisterSpec for Mvfr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mvfr0::R`](R) reader structure"]
impl crate::Readable for Mvfr0Spec {}
#[doc = "`write(|w| ..)` method takes [`mvfr0::W`](W) writer structure"]
impl crate::Writable for Mvfr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MVFR0 to value 0"]
impl crate::Resettable for Mvfr0Spec {
    const RESET_VALUE: u32 = 0;
}

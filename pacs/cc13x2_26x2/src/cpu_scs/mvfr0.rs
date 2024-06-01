#[doc = "Register `MVFR0` reader"]
pub type R = crate::R<Mvfr0Spec>;
#[doc = "Register `MVFR0` writer"]
pub type W = crate::W<Mvfr0Spec>;
#[doc = "Field `A_SIMD` reader - 3:0\\]
Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
pub type ASimdR = crate::FieldReader;
#[doc = "Field `A_SIMD` writer - 3:0\\]
Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
pub type ASimdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SINGLE_PRECISION` reader - 7:4\\]
Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
pub type SinglePrecisionR = crate::FieldReader;
#[doc = "Field `SINGLE_PRECISION` writer - 7:4\\]
Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
pub type SinglePrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DOUBLE_PRECISION` reader - 11:8\\]
Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported."]
pub type DoublePrecisionR = crate::FieldReader;
#[doc = "Field `DOUBLE_PRECISION` writer - 11:8\\]
Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported."]
pub type DoublePrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FP_EXCEPTION_TRAPPING` reader - 15:12\\]
Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported."]
pub type FpExceptionTrappingR = crate::FieldReader;
#[doc = "Field `FP_EXCEPTION_TRAPPING` writer - 15:12\\]
Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported."]
pub type FpExceptionTrappingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIVIDE` reader - 19:16\\]
Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
pub type DivideR = crate::FieldReader;
#[doc = "Field `DIVIDE` writer - 19:16\\]
Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
pub type DivideW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQUARE_ROOT` reader - 23:20\\]
Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
pub type SquareRootR = crate::FieldReader;
#[doc = "Field `SQUARE_ROOT` writer - 23:20\\]
Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
pub type SquareRootW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SHORT_VECTORS` reader - 27:24\\]
Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported."]
pub type ShortVectorsR = crate::FieldReader;
#[doc = "Field `SHORT_VECTORS` writer - 27:24\\]
Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported."]
pub type ShortVectorsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FP_ROUNDING_MODES` reader - 31:28\\]
Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
pub type FpRoundingModesR = crate::FieldReader;
#[doc = "Field `FP_ROUNDING_MODES` writer - 31:28\\]
Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
pub type FpRoundingModesW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
    #[inline(always)]
    pub fn a_simd(&self) -> ASimdR {
        ASimdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
    #[inline(always)]
    pub fn single_precision(&self) -> SinglePrecisionR {
        SinglePrecisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    pub fn double_precision(&self) -> DoublePrecisionR {
        DoublePrecisionR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    pub fn fp_exception_trapping(&self) -> FpExceptionTrappingR {
        FpExceptionTrappingR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn divide(&self) -> DivideR {
        DivideR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn square_root(&self) -> SquareRootR {
        SquareRootR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    pub fn short_vectors(&self) -> ShortVectorsR {
        ShortVectorsR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
    #[inline(always)]
    pub fn fp_rounding_modes(&self) -> FpRoundingModesR {
        FpRoundingModesR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
    #[inline(always)]
    #[must_use]
    pub fn a_simd(&mut self) -> ASimdW<Mvfr0Spec> {
        ASimdW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
    #[inline(always)]
    #[must_use]
    pub fn single_precision(&mut self) -> SinglePrecisionW<Mvfr0Spec> {
        SinglePrecisionW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    #[must_use]
    pub fn double_precision(&mut self) -> DoublePrecisionW<Mvfr0Spec> {
        DoublePrecisionW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    #[must_use]
    pub fn fp_exception_trapping(&mut self) -> FpExceptionTrappingW<Mvfr0Spec> {
        FpExceptionTrappingW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    #[must_use]
    pub fn divide(&mut self) -> DivideW<Mvfr0Spec> {
        DivideW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    #[must_use]
    pub fn square_root(&mut self) -> SquareRootW<Mvfr0Spec> {
        SquareRootW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    #[must_use]
    pub fn short_vectors(&mut self) -> ShortVectorsW<Mvfr0Spec> {
        ShortVectorsW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
    #[inline(always)]
    #[must_use]
    pub fn fp_rounding_modes(&mut self) -> FpRoundingModesW<Mvfr0Spec> {
        FpRoundingModesW::new(self, 28)
    }
}
#[doc = "Media and FP Feature 0 Describes the features provided by the Floating-point extension.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mvfr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mvfr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets MVFR0 to value 0x1011_0021"]
impl crate::Resettable for Mvfr0Spec {
    const RESET_VALUE: u32 = 0x1011_0021;
}

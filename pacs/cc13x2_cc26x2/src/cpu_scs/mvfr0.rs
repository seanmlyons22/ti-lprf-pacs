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
#[doc = "Field `A_SIMD` reader - 3:0\\]
Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
pub type A_SIMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `A_SIMD` writer - 3:0\\]
Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
pub type A_SIMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `SINGLE_PRECISION` reader - 7:4\\]
Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
pub type SINGLE_PRECISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SINGLE_PRECISION` writer - 7:4\\]
Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
pub type SINGLE_PRECISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DOUBLE_PRECISION` reader - 11:8\\]
Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported."]
pub type DOUBLE_PRECISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOUBLE_PRECISION` writer - 11:8\\]
Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported."]
pub type DOUBLE_PRECISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `FP_EXCEPTION_TRAPPING` reader - 15:12\\]
Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported."]
pub type FP_EXCEPTION_TRAPPING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FP_EXCEPTION_TRAPPING` writer - 15:12\\]
Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported."]
pub type FP_EXCEPTION_TRAPPING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DIVIDE` reader - 19:16\\]
Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
pub type DIVIDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVIDE` writer - 19:16\\]
Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
pub type DIVIDE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQUARE_ROOT` reader - 23:20\\]
Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
pub type SQUARE_ROOT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQUARE_ROOT` writer - 23:20\\]
Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
pub type SQUARE_ROOT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `SHORT_VECTORS` reader - 27:24\\]
Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported."]
pub type SHORT_VECTORS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHORT_VECTORS` writer - 27:24\\]
Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported."]
pub type SHORT_VECTORS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `FP_ROUNDING_MODES` reader - 31:28\\]
Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
pub type FP_ROUNDING_MODES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FP_ROUNDING_MODES` writer - 31:28\\]
Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
pub type FP_ROUNDING_MODES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MVFR0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
    #[inline(always)]
    pub fn a_simd(&self) -> A_SIMD_R {
        A_SIMD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
    #[inline(always)]
    pub fn single_precision(&self) -> SINGLE_PRECISION_R {
        SINGLE_PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    pub fn double_precision(&self) -> DOUBLE_PRECISION_R {
        DOUBLE_PRECISION_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    pub fn fp_exception_trapping(&self) -> FP_EXCEPTION_TRAPPING_R {
        FP_EXCEPTION_TRAPPING_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn divide(&self) -> DIVIDE_R {
        DIVIDE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn square_root(&self) -> SQUARE_ROOT_R {
        SQUARE_ROOT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    pub fn short_vectors(&self) -> SHORT_VECTORS_R {
        SHORT_VECTORS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
    #[inline(always)]
    pub fn fp_rounding_modes(&self) -> FP_ROUNDING_MODES_R {
        FP_ROUNDING_MODES_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
    #[inline(always)]
    #[must_use]
    pub fn a_simd(&mut self) -> A_SIMD_W<0> {
        A_SIMD_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
    #[inline(always)]
    #[must_use]
    pub fn single_precision(&mut self) -> SINGLE_PRECISION_W<4> {
        SINGLE_PRECISION_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    #[must_use]
    pub fn double_precision(&mut self) -> DOUBLE_PRECISION_W<8> {
        DOUBLE_PRECISION_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    #[must_use]
    pub fn fp_exception_trapping(&mut self) -> FP_EXCEPTION_TRAPPING_W<12> {
        FP_EXCEPTION_TRAPPING_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    #[must_use]
    pub fn divide(&mut self) -> DIVIDE_W<16> {
        DIVIDE_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    #[must_use]
    pub fn square_root(&mut self) -> SQUARE_ROOT_W<20> {
        SQUARE_ROOT_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    #[must_use]
    pub fn short_vectors(&mut self) -> SHORT_VECTORS_W<24> {
        SHORT_VECTORS_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
    #[inline(always)]
    #[must_use]
    pub fn fp_rounding_modes(&mut self) -> FP_ROUNDING_MODES_W<28> {
        FP_ROUNDING_MODES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Media and FP Feature 0 Describes the features provided by the Floating-point extension.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mvfr0](index.html) module"]
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
#[doc = "`reset()` method sets MVFR0 to value 0x1011_0021"]
impl crate::Resettable for MVFR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1011_0021;
}

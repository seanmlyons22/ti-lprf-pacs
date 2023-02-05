#[doc = "Register `MVFR1` reader"]
pub struct R(crate::R<MVFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MVFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MVFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MVFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MVFR1` writer"]
pub struct W(crate::W<MVFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MVFR1_SPEC>;
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
impl From<crate::W<MVFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MVFR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPFtZ` reader - 3:0\\]
Indicates whether subnormals are always flushed-to-zero"]
pub type FPFT_Z_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPFtZ` writer - 3:0\\]
Indicates whether subnormals are always flushed-to-zero"]
pub type FPFT_Z_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `FPDNaN` reader - 7:4\\]
Indicates whether the FP hardware implementation supports NaN propagation"]
pub type FPDNA_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPDNaN` writer - 7:4\\]
Indicates whether the FP hardware implementation supports NaN propagation"]
pub type FPDNA_N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED8` reader - 23:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED8` writer - 23:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `FPHP` reader - 27:24\\]
Indicates whether the FP Extension implements half-precision FP conversion instructions"]
pub type FPHP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPHP` writer - 27:24\\]
Indicates whether the FP Extension implements half-precision FP conversion instructions"]
pub type FPHP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `FMAC` reader - 31:28\\]
Indicates whether the FP Extension implements the fused multiply accumulate instructions"]
pub type FMAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FMAC` writer - 31:28\\]
Indicates whether the FP Extension implements the fused multiply accumulate instructions"]
pub type FMAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates whether subnormals are always flushed-to-zero"]
    #[inline(always)]
    pub fn fpft_z(&self) -> FPFT_Z_R {
        FPFT_Z_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates whether the FP hardware implementation supports NaN propagation"]
    #[inline(always)]
    pub fn fpdna_n(&self) -> FPDNA_N_R {
        FPDNA_N_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates whether the FP Extension implements half-precision FP conversion instructions"]
    #[inline(always)]
    pub fn fphp(&self) -> FPHP_R {
        FPHP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates whether the FP Extension implements the fused multiply accumulate instructions"]
    #[inline(always)]
    pub fn fmac(&self) -> FMAC_R {
        FMAC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates whether subnormals are always flushed-to-zero"]
    #[inline(always)]
    #[must_use]
    pub fn fpft_z(&mut self) -> FPFT_Z_W<0> {
        FPFT_Z_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates whether the FP hardware implementation supports NaN propagation"]
    #[inline(always)]
    #[must_use]
    pub fn fpdna_n(&mut self) -> FPDNA_N_W<4> {
        FPDNA_N_W::new(self)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates whether the FP Extension implements half-precision FP conversion instructions"]
    #[inline(always)]
    #[must_use]
    pub fn fphp(&mut self) -> FPHP_W<24> {
        FPHP_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates whether the FP Extension implements the fused multiply accumulate instructions"]
    #[inline(always)]
    #[must_use]
    pub fn fmac(&mut self) -> FMAC_W<28> {
        FMAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Describes the features provided by the Floating-point Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mvfr1](index.html) module"]
pub struct MVFR1_SPEC;
impl crate::RegisterSpec for MVFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mvfr1::R](R) reader structure"]
impl crate::Readable for MVFR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mvfr1::W](W) writer structure"]
impl crate::Writable for MVFR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MVFR1 to value 0"]
impl crate::Resettable for MVFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

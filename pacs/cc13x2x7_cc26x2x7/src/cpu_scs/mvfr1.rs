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
#[doc = "Field `FTZ_MODE` reader - 3:0\\]
Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
pub type FTZ_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTZ_MODE` writer - 3:0\\]
Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
pub type FTZ_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `D_NAN_MODE` reader - 7:4\\]
Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
pub type D_NAN_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D_NAN_MODE` writer - 7:4\\]
Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
pub type D_NAN_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED8` reader - 23:8\\]
Software should not rely on the value of a reserved."]
pub type RESERVED8_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED8` writer - 23:8\\]
Software should not rely on the value of a reserved."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `FP_HPFP` reader - 27:24\\]
Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
pub type FP_HPFP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FP_HPFP` writer - 27:24\\]
Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
pub type FP_HPFP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `FP_FUSED_MAC` reader - 31:28\\]
Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
pub type FP_FUSED_MAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FP_FUSED_MAC` writer - 31:28\\]
Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
pub type FP_FUSED_MAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MVFR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
    #[inline(always)]
    pub fn ftz_mode(&self) -> FTZ_MODE_R {
        FTZ_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
    #[inline(always)]
    pub fn d_nan_mode(&self) -> D_NAN_MODE_R {
        D_NAN_MODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Software should not rely on the value of a reserved."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn fp_hpfp(&self) -> FP_HPFP_R {
        FP_HPFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn fp_fused_mac(&self) -> FP_FUSED_MAC_R {
        FP_FUSED_MAC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
    #[inline(always)]
    #[must_use]
    pub fn ftz_mode(&mut self) -> FTZ_MODE_W<0> {
        FTZ_MODE_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
    #[inline(always)]
    #[must_use]
    pub fn d_nan_mode(&mut self) -> D_NAN_MODE_W<4> {
        D_NAN_MODE_W::new(self)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Software should not rely on the value of a reserved."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    #[must_use]
    pub fn fp_hpfp(&mut self) -> FP_HPFP_W<24> {
        FP_HPFP_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    #[must_use]
    pub fn fp_fused_mac(&mut self) -> FP_FUSED_MAC_W<28> {
        FP_FUSED_MAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Media and FP Feature 1 Describes the features provided by the Floating-point extension.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mvfr1](index.html) module"]
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
#[doc = "`reset()` method sets MVFR1 to value 0x1100_0011"]
impl crate::Resettable for MVFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1100_0011;
}

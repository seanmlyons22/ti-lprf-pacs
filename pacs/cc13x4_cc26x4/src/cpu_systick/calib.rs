#[doc = "Register `CALIB` reader"]
pub struct R(crate::R<CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALIB` writer"]
pub struct W(crate::W<CALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALIB_SPEC>;
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
impl From<crate::W<CALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TENMS` reader - 23:0\\]
Optionally, holds a reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. If this field is zero, the calibration value is not known"]
pub type TENMS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TENMS` writer - 23:0\\]
Optionally, holds a reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. If this field is zero, the calibration value is not known"]
pub type TENMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALIB_SPEC, u32, u32, 24, O>;
#[doc = "Field `RESERVED24` reader - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALIB_SPEC, u8, u8, 6, O>;
#[doc = "Field `SKEW` reader - 30:30\\]
Indicates whether the 10ms calibration value is exact"]
pub type SKEW_R = crate::BitReader<bool>;
#[doc = "Field `SKEW` writer - 30:30\\]
Indicates whether the 10ms calibration value is exact"]
pub type SKEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALIB_SPEC, bool, O>;
#[doc = "Field `NOREF` reader - 31:31\\]
Indicates whether the IMPLEMENTATION DEFINED reference clock is implemented"]
pub type NOREF_R = crate::BitReader<bool>;
#[doc = "Field `NOREF` writer - 31:31\\]
Indicates whether the IMPLEMENTATION DEFINED reference clock is implemented"]
pub type NOREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALIB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Optionally, holds a reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. If this field is zero, the calibration value is not known"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Indicates whether the 10ms calibration value is exact"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates whether the IMPLEMENTATION DEFINED reference clock is implemented"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Optionally, holds a reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. If this field is zero, the calibration value is not known"]
    #[inline(always)]
    #[must_use]
    pub fn tenms(&mut self) -> TENMS_W<0> {
        TENMS_W::new(self)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Indicates whether the 10ms calibration value is exact"]
    #[inline(always)]
    #[must_use]
    pub fn skew(&mut self) -> SKEW_W<30> {
        SKEW_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates whether the IMPLEMENTATION DEFINED reference clock is implemented"]
    #[inline(always)]
    #[must_use]
    pub fn noref(&mut self) -> NOREF_W<31> {
        NOREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reads the SysTick timer calibration value and parameters `FTSSS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](index.html) module"]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calib::R](R) reader structure"]
impl crate::Readable for CALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calib::W](W) writer structure"]
impl crate::Writable for CALIB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

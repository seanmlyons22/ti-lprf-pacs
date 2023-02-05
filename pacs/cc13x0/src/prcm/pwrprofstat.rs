#[doc = "Register `PWRPROFSTAT` reader"]
pub struct R(crate::R<PWRPROFSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRPROFSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRPROFSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRPROFSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRPROFSTAT` writer"]
pub struct W(crate::W<PWRPROFSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRPROFSTAT_SPEC>;
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
impl From<crate::W<PWRPROFSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRPROFSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - 7:0\\]
SW can use these bits to timestamp the application. These bits are also available through the testtap and can thus be used by the emulator to profile in real time."]
pub type VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VALUE` writer - 7:0\\]
SW can use these bits to timestamp the application. These bits are also available through the testtap and can thus be used by the emulator to profile in real time."]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWRPROFSTAT_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWRPROFSTAT_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
SW can use these bits to timestamp the application. These bits are also available through the testtap and can thus be used by the emulator to profile in real time."]
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
SW can use these bits to timestamp the application. These bits are also available through the testtap and can thus be used by the emulator to profile in real time."]
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
#[doc = "Power Profiler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrprofstat](index.html) module"]
pub struct PWRPROFSTAT_SPEC;
impl crate::RegisterSpec for PWRPROFSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrprofstat::R](R) reader structure"]
impl crate::Readable for PWRPROFSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrprofstat::W](W) writer structure"]
impl crate::Writable for PWRPROFSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRPROFSTAT to value 0x01"]
impl crate::Resettable for PWRPROFSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

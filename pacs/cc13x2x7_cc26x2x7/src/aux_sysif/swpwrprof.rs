#[doc = "Register `SWPWRPROF` reader"]
pub struct R(crate::R<SWPWRPROF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWPWRPROF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWPWRPROF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWPWRPROF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWPWRPROF` writer"]
pub struct W(crate::W<SWPWRPROF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWPWRPROF_SPEC>;
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
impl From<crate::W<SWPWRPROF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWPWRPROF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STAT` reader - 2:0\\]
Software status bits that can be read by the power profiler."]
pub type STAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STAT` writer - 2:0\\]
Software status bits that can be read by the power profiler."]
pub type STAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWPWRPROF_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SWPWRPROF_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Software status bits that can be read by the power profiler."]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Software status bits that can be read by the power profiler."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> STAT_W<0> {
        STAT_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Power Profiler\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpwrprof](index.html) module"]
pub struct SWPWRPROF_SPEC;
impl crate::RegisterSpec for SWPWRPROF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swpwrprof::R](R) reader structure"]
impl crate::Readable for SWPWRPROF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swpwrprof::W](W) writer structure"]
impl crate::Writable for SWPWRPROF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWPWRPROF to value 0"]
impl crate::Resettable for SWPWRPROF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

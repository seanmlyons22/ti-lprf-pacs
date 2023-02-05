#[doc = "Register `LFSR2` reader"]
pub struct R(crate::R<LFSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFSR2` writer"]
pub struct W(crate::W<LFSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFSR2_SPEC>;
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
impl From<crate::W<LFSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFSR_80_64` reader - 16:0\\]
Bits \\[80:64\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
pub type LFSR_80_64_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LFSR_80_64` writer - 16:0\\]
Bits \\[80:64\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
pub type LFSR_80_64_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LFSR2_SPEC, u32, u32, 17, O>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LFSR2_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Bits \\[80:64\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    pub fn lfsr_80_64(&self) -> LFSR_80_64_R {
        LFSR_80_64_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Bits \\[80:64\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr_80_64(&mut self) -> LFSR_80_64_W<0> {
        LFSR_80_64_W::new(self)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> RESERVED17_W<17> {
        RESERVED17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LFSR Readout Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfsr2](index.html) module"]
pub struct LFSR2_SPEC;
impl crate::RegisterSpec for LFSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfsr2::R](R) reader structure"]
impl crate::Readable for LFSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfsr2::W](W) writer structure"]
impl crate::Writable for LFSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFSR2 to value 0"]
impl crate::Resettable for LFSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

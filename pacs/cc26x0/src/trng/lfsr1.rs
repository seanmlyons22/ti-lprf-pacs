#[doc = "Register `LFSR1` reader"]
pub struct R(crate::R<LFSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFSR1` writer"]
pub struct W(crate::W<LFSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFSR1_SPEC>;
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
impl From<crate::W<LFSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFSR_63_32` reader - 31:0\\]
Bits \\[63:32\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
pub type LFSR_63_32_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LFSR_63_32` writer - 31:0\\]
Bits \\[63:32\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
pub type LFSR_63_32_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LFSR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bits \\[63:32\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    pub fn lfsr_63_32(&self) -> LFSR_63_32_R {
        LFSR_63_32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bits \\[63:32\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr_63_32(&mut self) -> LFSR_63_32_W<0> {
        LFSR_63_32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LFSR Readout Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfsr1](index.html) module"]
pub struct LFSR1_SPEC;
impl crate::RegisterSpec for LFSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfsr1::R](R) reader structure"]
impl crate::Readable for LFSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfsr1::W](W) writer structure"]
impl crate::Writable for LFSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFSR1 to value 0"]
impl crate::Resettable for LFSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

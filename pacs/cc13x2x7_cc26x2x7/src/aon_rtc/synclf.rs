#[doc = "Register `SYNCLF` reader"]
pub struct R(crate::R<SYNCLF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCLF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCLF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCLF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCLF` writer"]
pub struct W(crate::W<SYNCLF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCLF_SPEC>;
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
impl From<crate::W<SYNCLF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCLF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHASE` reader - 0:0\\]
This bit will always return the SCLK_LF phase. The return will delayed until a positive or negative edge of SCLK_LF is seen. 0: Falling edge of SCLK_LF 1: Rising edge of SCLK_LF"]
pub type PHASE_R = crate::BitReader<bool>;
#[doc = "Field `PHASE` writer - 0:0\\]
This bit will always return the SCLK_LF phase. The return will delayed until a positive or negative edge of SCLK_LF is seen. 0: Falling edge of SCLK_LF 1: Rising edge of SCLK_LF"]
pub type PHASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCLF_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYNCLF_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit will always return the SCLK_LF phase. The return will delayed until a positive or negative edge of SCLK_LF is seen. 0: Falling edge of SCLK_LF 1: Rising edge of SCLK_LF"]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit will always return the SCLK_LF phase. The return will delayed until a positive or negative edge of SCLK_LF is seen. 0: Falling edge of SCLK_LF 1: Rising edge of SCLK_LF"]
    #[inline(always)]
    #[must_use]
    pub fn phase(&mut self) -> PHASE_W<0> {
        PHASE_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronization to SCLK_LF This register is used for synchronizing MCU to positive or negative edge of SCLK_LF.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [synclf](index.html) module"]
pub struct SYNCLF_SPEC;
impl crate::RegisterSpec for SYNCLF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [synclf::R](R) reader structure"]
impl crate::Readable for SYNCLF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [synclf::W](W) writer structure"]
impl crate::Writable for SYNCLF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCLF to value 0"]
impl crate::Resettable for SYNCLF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

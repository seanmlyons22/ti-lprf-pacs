#[doc = "Register `TAR` reader"]
pub struct R(crate::R<TAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAR` writer"]
pub struct W(crate::W<TAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAR_SPEC>;
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
impl From<crate::W<TAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAR` reader - 31:0\\]
GPT Timer A Register Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer A Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
pub type TAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TAR` writer - 31:0\\]
GPT Timer A Register Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer A Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
pub type TAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Register Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer A Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Register Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer A Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<0> {
        TAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer A Register This register shows the current value of the Timer A counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPT is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the Timer B (TBR) register). In the16-bit Input Edge Count, Input Edge Time, and PWM modes, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\]
in the TAV register. To read the value of the prescalar in periodic snapshot mode, read the Timer A Prescale Snapshot (TAPS) register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tar](index.html) module"]
pub struct TAR_SPEC;
impl crate::RegisterSpec for TAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tar::R](R) reader structure"]
impl crate::Readable for TAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tar::W](W) writer structure"]
impl crate::Writable for TAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAR to value 0xffff_ffff"]
impl crate::Resettable for TAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

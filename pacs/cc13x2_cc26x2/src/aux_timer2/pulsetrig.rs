#[doc = "Register `PULSETRIG` reader"]
pub struct R(crate::R<PULSETRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULSETRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULSETRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULSETRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PULSETRIG` writer"]
pub struct W(crate::W<PULSETRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULSETRIG_SPEC>;
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
impl From<crate::W<PULSETRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULSETRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIG` reader - 0:0\\]
Pulse trigger. Write 1 to generate a pulse to AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE. Pulse width equals the duty cycle of AUX_SYSIF:TIMER2CLKCTL.SRC."]
pub type TRIG_R = crate::BitReader<bool>;
#[doc = "Field `TRIG` writer - 0:0\\]
Pulse trigger. Write 1 to generate a pulse to AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE. Pulse width equals the duty cycle of AUX_SYSIF:TIMER2CLKCTL.SRC."]
pub type TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULSETRIG_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PULSETRIG_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Pulse trigger. Write 1 to generate a pulse to AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE. Pulse width equals the duty cycle of AUX_SYSIF:TIMER2CLKCTL.SRC."]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new((self.bits & 1) != 0)
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
Pulse trigger. Write 1 to generate a pulse to AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE. Pulse width equals the duty cycle of AUX_SYSIF:TIMER2CLKCTL.SRC."]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TRIG_W<0> {
        TRIG_W::new(self)
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
#[doc = "Pulse Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulsetrig](index.html) module"]
pub struct PULSETRIG_SPEC;
impl crate::RegisterSpec for PULSETRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pulsetrig::R](R) reader structure"]
impl crate::Readable for PULSETRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pulsetrig::W](W) writer structure"]
impl crate::Writable for PULSETRIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PULSETRIG to value 0"]
impl crate::Resettable for PULSETRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

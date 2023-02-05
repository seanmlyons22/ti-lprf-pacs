#[doc = "Register `TIMERHALT` reader"]
pub struct R(crate::R<TIMERHALT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMERHALT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMERHALT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMERHALT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMERHALT` writer"]
pub struct W(crate::W<TIMERHALT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMERHALT_SPEC>;
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
impl From<crate::W<TIMERHALT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMERHALT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUX_TIMER0` reader - 0:0\\]
Halt AUX_TIMER01 Timer 0. 0: AUX_TIMER01 Timer 0 operates as normal. 1: Halt AUX_TIMER01 Timer 0 operation."]
pub type AUX_TIMER0_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TIMER0` writer - 0:0\\]
Halt AUX_TIMER01 Timer 0. 0: AUX_TIMER01 Timer 0 operates as normal. 1: Halt AUX_TIMER01 Timer 0 operation."]
pub type AUX_TIMER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERHALT_SPEC, bool, O>;
#[doc = "Field `AUX_TIMER1` reader - 1:1\\]
Halt AUX_TIMER01 Timer 1. 0: AUX_TIMER01 Timer 1 operates as normal. 1: Halt AUX_TIMER01 Timer 1 operation."]
pub type AUX_TIMER1_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TIMER1` writer - 1:1\\]
Halt AUX_TIMER01 Timer 1. 0: AUX_TIMER01 Timer 1 operates as normal. 1: Halt AUX_TIMER01 Timer 1 operation."]
pub type AUX_TIMER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERHALT_SPEC, bool, O>;
#[doc = "Field `AUX_TIMER2` reader - 2:2\\]
Halt AUX_TIMER2. 0: AUX_TIMER2 operates as normal. 1: Halt AUX_TIMER2 operation."]
pub type AUX_TIMER2_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TIMER2` writer - 2:2\\]
Halt AUX_TIMER2. 0: AUX_TIMER2 operates as normal. 1: Halt AUX_TIMER2 operation."]
pub type AUX_TIMER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERHALT_SPEC, bool, O>;
#[doc = "Field `PROGDLY` reader - 3:3\\]
Halt programmable delay. 0: AUX_EVCTL:PROGDLY.VALUE decrements as normal. 1: Halt AUX_EVCTL:PROGDLY.VALUE decrementation."]
pub type PROGDLY_R = crate::BitReader<bool>;
#[doc = "Field `PROGDLY` writer - 3:3\\]
Halt programmable delay. 0: AUX_EVCTL:PROGDLY.VALUE decrements as normal. 1: Halt AUX_EVCTL:PROGDLY.VALUE decrementation."]
pub type PROGDLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERHALT_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMERHALT_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Halt AUX_TIMER01 Timer 0. 0: AUX_TIMER01 Timer 0 operates as normal. 1: Halt AUX_TIMER01 Timer 0 operation."]
    #[inline(always)]
    pub fn aux_timer0(&self) -> AUX_TIMER0_R {
        AUX_TIMER0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Halt AUX_TIMER01 Timer 1. 0: AUX_TIMER01 Timer 1 operates as normal. 1: Halt AUX_TIMER01 Timer 1 operation."]
    #[inline(always)]
    pub fn aux_timer1(&self) -> AUX_TIMER1_R {
        AUX_TIMER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Halt AUX_TIMER2. 0: AUX_TIMER2 operates as normal. 1: Halt AUX_TIMER2 operation."]
    #[inline(always)]
    pub fn aux_timer2(&self) -> AUX_TIMER2_R {
        AUX_TIMER2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Halt programmable delay. 0: AUX_EVCTL:PROGDLY.VALUE decrements as normal. 1: Halt AUX_EVCTL:PROGDLY.VALUE decrementation."]
    #[inline(always)]
    pub fn progdly(&self) -> PROGDLY_R {
        PROGDLY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Halt AUX_TIMER01 Timer 0. 0: AUX_TIMER01 Timer 0 operates as normal. 1: Halt AUX_TIMER01 Timer 0 operation."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer0(&mut self) -> AUX_TIMER0_W<0> {
        AUX_TIMER0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Halt AUX_TIMER01 Timer 1. 0: AUX_TIMER01 Timer 1 operates as normal. 1: Halt AUX_TIMER01 Timer 1 operation."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer1(&mut self) -> AUX_TIMER1_W<1> {
        AUX_TIMER1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Halt AUX_TIMER2. 0: AUX_TIMER2 operates as normal. 1: Halt AUX_TIMER2 operation."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2(&mut self) -> AUX_TIMER2_W<2> {
        AUX_TIMER2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Halt programmable delay. 0: AUX_EVCTL:PROGDLY.VALUE decrements as normal. 1: Halt AUX_EVCTL:PROGDLY.VALUE decrementation."]
    #[inline(always)]
    #[must_use]
    pub fn progdly(&mut self) -> PROGDLY_W<3> {
        PROGDLY_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Halt Debug register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timerhalt](index.html) module"]
pub struct TIMERHALT_SPEC;
impl crate::RegisterSpec for TIMERHALT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timerhalt::R](R) reader structure"]
impl crate::Readable for TIMERHALT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timerhalt::W](W) writer structure"]
impl crate::Writable for TIMERHALT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMERHALT to value 0"]
impl crate::Resettable for TIMERHALT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

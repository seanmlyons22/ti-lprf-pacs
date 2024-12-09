#[doc = "Register `TIMERHALT` reader"]
pub type R = crate::R<TimerhaltSpec>;
#[doc = "Register `TIMERHALT` writer"]
pub type W = crate::W<TimerhaltSpec>;
#[doc = "Field `AUX_TIMER0` reader - 0:0\\]
Halt AUX_TIMER01 Timer 0. 0: AUX_TIMER01 Timer 0 operates as normal. 1: Halt AUX_TIMER01 Timer 0 operation."]
pub type AuxTimer0R = crate::BitReader;
#[doc = "Field `AUX_TIMER0` writer - 0:0\\]
Halt AUX_TIMER01 Timer 0. 0: AUX_TIMER01 Timer 0 operates as normal. 1: Halt AUX_TIMER01 Timer 0 operation."]
pub type AuxTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER1` reader - 1:1\\]
Halt AUX_TIMER01 Timer 1. 0: AUX_TIMER01 Timer 1 operates as normal. 1: Halt AUX_TIMER01 Timer 1 operation."]
pub type AuxTimer1R = crate::BitReader;
#[doc = "Field `AUX_TIMER1` writer - 1:1\\]
Halt AUX_TIMER01 Timer 1. 0: AUX_TIMER01 Timer 1 operates as normal. 1: Halt AUX_TIMER01 Timer 1 operation."]
pub type AuxTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2` reader - 2:2\\]
Halt AUX_TIMER2. 0: AUX_TIMER2 operates as normal. 1: Halt AUX_TIMER2 operation."]
pub type AuxTimer2R = crate::BitReader;
#[doc = "Field `AUX_TIMER2` writer - 2:2\\]
Halt AUX_TIMER2. 0: AUX_TIMER2 operates as normal. 1: Halt AUX_TIMER2 operation."]
pub type AuxTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGDLY` reader - 3:3\\]
Halt programmable delay. 0: AUX_EVCTL:PROGDLY.VALUE decrements as normal. 1: Halt AUX_EVCTL:PROGDLY.VALUE decrementation."]
pub type ProgdlyR = crate::BitReader;
#[doc = "Field `PROGDLY` writer - 3:3\\]
Halt programmable delay. 0: AUX_EVCTL:PROGDLY.VALUE decrements as normal. 1: Halt AUX_EVCTL:PROGDLY.VALUE decrementation."]
pub type ProgdlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Halt AUX_TIMER01 Timer 0. 0: AUX_TIMER01 Timer 0 operates as normal. 1: Halt AUX_TIMER01 Timer 0 operation."]
    #[inline(always)]
    pub fn aux_timer0(&self) -> AuxTimer0R {
        AuxTimer0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Halt AUX_TIMER01 Timer 1. 0: AUX_TIMER01 Timer 1 operates as normal. 1: Halt AUX_TIMER01 Timer 1 operation."]
    #[inline(always)]
    pub fn aux_timer1(&self) -> AuxTimer1R {
        AuxTimer1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Halt AUX_TIMER2. 0: AUX_TIMER2 operates as normal. 1: Halt AUX_TIMER2 operation."]
    #[inline(always)]
    pub fn aux_timer2(&self) -> AuxTimer2R {
        AuxTimer2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Halt programmable delay. 0: AUX_EVCTL:PROGDLY.VALUE decrements as normal. 1: Halt AUX_EVCTL:PROGDLY.VALUE decrementation."]
    #[inline(always)]
    pub fn progdly(&self) -> ProgdlyR {
        ProgdlyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Halt AUX_TIMER01 Timer 0. 0: AUX_TIMER01 Timer 0 operates as normal. 1: Halt AUX_TIMER01 Timer 0 operation."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer0(&mut self) -> AuxTimer0W<TimerhaltSpec> {
        AuxTimer0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Halt AUX_TIMER01 Timer 1. 0: AUX_TIMER01 Timer 1 operates as normal. 1: Halt AUX_TIMER01 Timer 1 operation."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer1(&mut self) -> AuxTimer1W<TimerhaltSpec> {
        AuxTimer1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Halt AUX_TIMER2. 0: AUX_TIMER2 operates as normal. 1: Halt AUX_TIMER2 operation."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2(&mut self) -> AuxTimer2W<TimerhaltSpec> {
        AuxTimer2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Halt programmable delay. 0: AUX_EVCTL:PROGDLY.VALUE decrements as normal. 1: Halt AUX_EVCTL:PROGDLY.VALUE decrementation."]
    #[inline(always)]
    #[must_use]
    pub fn progdly(&mut self) -> ProgdlyW<TimerhaltSpec> {
        ProgdlyW::new(self, 3)
    }
}
#[doc = "Timer Halt Debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timerhalt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timerhalt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerhaltSpec;
impl crate::RegisterSpec for TimerhaltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timerhalt::R`](R) reader structure"]
impl crate::Readable for TimerhaltSpec {}
#[doc = "`write(|w| ..)` method takes [`timerhalt::W`](W) writer structure"]
impl crate::Writable for TimerhaltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMERHALT to value 0"]
impl crate::Resettable for TimerhaltSpec {
    const RESET_VALUE: u32 = 0;
}

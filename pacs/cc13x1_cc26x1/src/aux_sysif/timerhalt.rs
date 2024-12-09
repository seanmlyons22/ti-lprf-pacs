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
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
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
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
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

#[doc = "Register `TAR` reader"]
pub type R = crate::R<TarSpec>;
#[doc = "Register `TAR` writer"]
pub type W = crate::W<TarSpec>;
#[doc = "Field `TAR` reader - 31:0\\]
GPT Timer A Register Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer A Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
pub type TarR = crate::FieldReader<u32>;
#[doc = "Field `TAR` writer - 31:0\\]
GPT Timer A Register Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer A Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
pub type TarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Register Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer A Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[inline(always)]
    pub fn tar(&self) -> TarR {
        TarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Register Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer A Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TarW<TarSpec> {
        TarW::new(self, 0)
    }
}
#[doc = "Timer A Register This register shows the current value of the Timer A counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPT is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the Timer B (TBR) register). In the16-bit Input Edge Count, Input Edge Time, and PWM modes, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\]
in the TAV register. To read the value of the prescalar in periodic snapshot mode, read the Timer A Prescale Snapshot (TAPS) register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TarSpec;
impl crate::RegisterSpec for TarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tar::R`](R) reader structure"]
impl crate::Readable for TarSpec {}
#[doc = "`write(|w| ..)` method takes [`tar::W`](W) writer structure"]
impl crate::Writable for TarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAR to value 0xffff_ffff"]
impl crate::Resettable for TarSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

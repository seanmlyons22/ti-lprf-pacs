#[doc = "Register `T0TARGET` reader"]
pub type R = crate::R<T0targetSpec>;
#[doc = "Register `T0TARGET` writer"]
pub type W = crate::W<T0targetSpec>;
#[doc = "Field `VALUE` reader - 15:0\\]
Timer 0 target value. Manual Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 0 counts to 1. AUX_TIMER0_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 0 counter value remains 0. AUX_TIMER0_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - 15:0\\]
Timer 0 target value. Manual Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 0 counts to 1. AUX_TIMER0_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 0 counter value remains 0. AUX_TIMER0_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Timer 0 target value. Manual Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 0 counts to 1. AUX_TIMER0_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 0 counter value remains 0. AUX_TIMER0_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Timer 0 target value. Manual Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 0 counts to 1. AUX_TIMER0_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 0 counter value remains 0. AUX_TIMER0_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<T0targetSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Timer 0 Target\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0target::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0target::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0targetSpec;
impl crate::RegisterSpec for T0targetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0target::R`](R) reader structure"]
impl crate::Readable for T0targetSpec {}
#[doc = "`write(|w| ..)` method takes [`t0target::W`](W) writer structure"]
impl crate::Writable for T0targetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T0TARGET to value 0"]
impl crate::Resettable for T0targetSpec {
    const RESET_VALUE: u32 = 0;
}

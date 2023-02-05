#[doc = "Register `T0TARGET` reader"]
pub struct R(crate::R<T0TARGET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0TARGET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0TARGET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0TARGET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0TARGET` writer"]
pub struct W(crate::W<T0TARGET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0TARGET_SPEC>;
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
impl From<crate::W<T0TARGET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0TARGET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - 15:0\\]
Timer 0 target value. Manual Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 0 counts to 1. AUX_TIMER0_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 0 counter value remains 0. AUX_TIMER0_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
pub type VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VALUE` writer - 15:0\\]
Timer 0 target value. Manual Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 0 counts to 1. AUX_TIMER0_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 0 counter value remains 0. AUX_TIMER0_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0TARGET_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, T0TARGET_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Timer 0 target value. Manual Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 0 counts to 1. AUX_TIMER0_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 0 counter value remains 0. AUX_TIMER0_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Timer 0 target value. Manual Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 0 counts to 1. AUX_TIMER0_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER0_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 0 counter value remains 0. AUX_TIMER0_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer 0 Target\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0target](index.html) module"]
pub struct T0TARGET_SPEC;
impl crate::RegisterSpec for T0TARGET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0target::R](R) reader structure"]
impl crate::Readable for T0TARGET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0target::W](W) writer structure"]
impl crate::Writable for T0TARGET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0TARGET to value 0"]
impl crate::Resettable for T0TARGET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

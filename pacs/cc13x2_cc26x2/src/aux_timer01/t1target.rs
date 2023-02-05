#[doc = "Register `T1TARGET` reader"]
pub struct R(crate::R<T1TARGET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T1TARGET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T1TARGET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T1TARGET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T1TARGET` writer"]
pub struct W(crate::W<T1TARGET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T1TARGET_SPEC>;
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
impl From<crate::W<T1TARGET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T1TARGET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - 15:0\\]
Timer 1 target value. Manual Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER1_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 1 counts to 1. AUX_TIMER1_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER1_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 1 counter value remains 0. AUX_TIMER1_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
pub type VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VALUE` writer - 15:0\\]
Timer 1 target value. Manual Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER1_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 1 counts to 1. AUX_TIMER1_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER1_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 1 counter value remains 0. AUX_TIMER1_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T1TARGET_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, T1TARGET_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Timer 1 target value. Manual Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER1_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 1 counts to 1. AUX_TIMER1_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER1_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 1 counter value remains 0. AUX_TIMER1_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
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
Timer 1 target value. Manual Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER1_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 1 counts to 1. AUX_TIMER1_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER1_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 1 counter value remains 0. AUX_TIMER1_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
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
#[doc = "Timer 1 Target Timer 1 counter target value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1target](index.html) module"]
pub struct T1TARGET_SPEC;
impl crate::RegisterSpec for T1TARGET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t1target::R](R) reader structure"]
impl crate::Readable for T1TARGET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t1target::W](W) writer structure"]
impl crate::Writable for T1TARGET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T1TARGET to value 0"]
impl crate::Resettable for T1TARGET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

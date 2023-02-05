#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SLEEPONEXIT` reader - 1:1\\]
Sleep on exit when returning from Handler mode to Thread mode. Enables interrupt driven applications to avoid returning to empty main application. 0: Do not sleep when returning to thread mode 1: Sleep on ISR exit"]
pub type SLEEPONEXIT_R = crate::BitReader<bool>;
#[doc = "Field `SLEEPONEXIT` writer - 1:1\\]
Sleep on exit when returning from Handler mode to Thread mode. Enables interrupt driven applications to avoid returning to empty main application. 0: Do not sleep when returning to thread mode 1: Sleep on ISR exit"]
pub type SLEEPONEXIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SLEEPDEEP` reader - 2:2\\]
Controls whether the processor uses sleep or deep sleep as its low power mode"]
pub type SLEEPDEEP_R = crate::BitReader<SLEEPDEEP_A>;
#[doc = "2:2\\]
Controls whether the processor uses sleep or deep sleep as its low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEPDEEP_A {
    #[doc = "1: Deep sleep"]
    DEEPSLEEP = 1,
    #[doc = "0: Sleep"]
    SLEEP = 0,
}
impl From<SLEEPDEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPDEEP_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEPDEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPDEEP_A {
        match self.bits {
            true => SLEEPDEEP_A::DEEPSLEEP,
            false => SLEEPDEEP_A::SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == SLEEPDEEP_A::DEEPSLEEP
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == SLEEPDEEP_A::SLEEP
    }
}
#[doc = "Field `SLEEPDEEP` writer - 2:2\\]
Controls whether the processor uses sleep or deep sleep as its low power mode"]
pub type SLEEPDEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, SLEEPDEEP_A, O>;
impl<'a, const O: u8> SLEEPDEEP_W<'a, O> {
    #[doc = "Deep sleep"]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(SLEEPDEEP_A::DEEPSLEEP)
    }
    #[doc = "Sleep"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut W {
        self.variant(SLEEPDEEP_A::SLEEP)
    }
}
#[doc = "Field `SLEEPDEEPS` reader - 3:3\\]
Sleep deep secure. This field controls whether the SLEEPDEEP bit is only accessible from the Secure state"]
pub type SLEEPDEEPS_R = crate::BitReader<bool>;
#[doc = "Field `SLEEPDEEPS` writer - 3:3\\]
Sleep deep secure. This field controls whether the SLEEPDEEP bit is only accessible from the Secure state"]
pub type SLEEPDEEPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SEVONPEND` reader - 4:4\\]
Send Event on Pending bit: 0: Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded 1: Enabled events and all interrupts, including disabled interrupts, can wakeup the processor. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction."]
pub type SEVONPEND_R = crate::BitReader<bool>;
#[doc = "Field `SEVONPEND` writer - 4:4\\]
Send Event on Pending bit: 0: Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded 1: Enabled events and all interrupts, including disabled interrupts, can wakeup the processor. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction."]
pub type SEVONPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCR_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sleep on exit when returning from Handler mode to Thread mode. Enables interrupt driven applications to avoid returning to empty main application. 0: Do not sleep when returning to thread mode 1: Sleep on ISR exit"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Controls whether the processor uses sleep or deep sleep as its low power mode"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Sleep deep secure. This field controls whether the SLEEPDEEP bit is only accessible from the Secure state"]
    #[inline(always)]
    pub fn sleepdeeps(&self) -> SLEEPDEEPS_R {
        SLEEPDEEPS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Send Event on Pending bit: 0: Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded 1: Enabled events and all interrupts, including disabled interrupts, can wakeup the processor. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction."]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Sleep on exit when returning from Handler mode to Thread mode. Enables interrupt driven applications to avoid returning to empty main application. 0: Do not sleep when returning to thread mode 1: Sleep on ISR exit"]
    #[inline(always)]
    #[must_use]
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W<1> {
        SLEEPONEXIT_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Controls whether the processor uses sleep or deep sleep as its low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W<2> {
        SLEEPDEEP_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Sleep deep secure. This field controls whether the SLEEPDEEP bit is only accessible from the Secure state"]
    #[inline(always)]
    #[must_use]
    pub fn sleepdeeps(&mut self) -> SLEEPDEEPS_W<3> {
        SLEEPDEEPS_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Send Event on Pending bit: 0: Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded 1: Enabled events and all interrupts, including disabled interrupts, can wakeup the processor. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction."]
    #[inline(always)]
    #[must_use]
    pub fn sevonpend(&mut self) -> SEVONPEND_W<4> {
        SEVONPEND_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEPONEXIT` reader - 1:1\\]
Sleep on exit when returning from Handler mode to Thread mode. Enables interrupt driven applications to avoid returning to empty main application. 0: Do not sleep when returning to thread mode 1: Sleep on ISR exit"]
pub type SleeponexitR = crate::BitReader;
#[doc = "Field `SLEEPONEXIT` writer - 1:1\\]
Sleep on exit when returning from Handler mode to Thread mode. Enables interrupt driven applications to avoid returning to empty main application. 0: Do not sleep when returning to thread mode 1: Sleep on ISR exit"]
pub type SleeponexitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "2:2\\]
Controls whether the processor uses sleep or deep sleep as its low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleepdeep {
    #[doc = "1: Deep sleep"]
    Deepsleep = 1,
    #[doc = "0: Sleep"]
    Sleep = 0,
}
impl From<Sleepdeep> for bool {
    #[inline(always)]
    fn from(variant: Sleepdeep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPDEEP` reader - 2:2\\]
Controls whether the processor uses sleep or deep sleep as its low power mode"]
pub type SleepdeepR = crate::BitReader<Sleepdeep>;
impl SleepdeepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleepdeep {
        match self.bits {
            true => Sleepdeep::Deepsleep,
            false => Sleepdeep::Sleep,
        }
    }
    #[doc = "Deep sleep"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == Sleepdeep::Deepsleep
    }
    #[doc = "Sleep"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == Sleepdeep::Sleep
    }
}
#[doc = "Field `SLEEPDEEP` writer - 2:2\\]
Controls whether the processor uses sleep or deep sleep as its low power mode"]
pub type SleepdeepW<'a, REG> = crate::BitWriter<'a, REG, Sleepdeep>;
impl<'a, REG> SleepdeepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deep sleep"]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepdeep::Deepsleep)
    }
    #[doc = "Sleep"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepdeep::Sleep)
    }
}
#[doc = "Field `SLEEPDEEPS` reader - 3:3\\]
Sleep deep secure. This field controls whether the SLEEPDEEP bit is only accessible from the Secure state"]
pub type SleepdeepsR = crate::BitReader;
#[doc = "Field `SLEEPDEEPS` writer - 3:3\\]
Sleep deep secure. This field controls whether the SLEEPDEEP bit is only accessible from the Secure state"]
pub type SleepdeepsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEVONPEND` reader - 4:4\\]
Send Event on Pending bit: 0: Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded 1: Enabled events and all interrupts, including disabled interrupts, can wakeup the processor. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction."]
pub type SevonpendR = crate::BitReader;
#[doc = "Field `SEVONPEND` writer - 4:4\\]
Send Event on Pending bit: 0: Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded 1: Enabled events and all interrupts, including disabled interrupts, can wakeup the processor. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction."]
pub type SevonpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sleep on exit when returning from Handler mode to Thread mode. Enables interrupt driven applications to avoid returning to empty main application. 0: Do not sleep when returning to thread mode 1: Sleep on ISR exit"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SleeponexitR {
        SleeponexitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Controls whether the processor uses sleep or deep sleep as its low power mode"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SleepdeepR {
        SleepdeepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Sleep deep secure. This field controls whether the SLEEPDEEP bit is only accessible from the Secure state"]
    #[inline(always)]
    pub fn sleepdeeps(&self) -> SleepdeepsR {
        SleepdeepsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Send Event on Pending bit: 0: Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded 1: Enabled events and all interrupts, including disabled interrupts, can wakeup the processor. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction."]
    #[inline(always)]
    pub fn sevonpend(&self) -> SevonpendR {
        SevonpendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<ScrSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sleep on exit when returning from Handler mode to Thread mode. Enables interrupt driven applications to avoid returning to empty main application. 0: Do not sleep when returning to thread mode 1: Sleep on ISR exit"]
    #[inline(always)]
    #[must_use]
    pub fn sleeponexit(&mut self) -> SleeponexitW<ScrSpec> {
        SleeponexitW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Controls whether the processor uses sleep or deep sleep as its low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleepdeep(&mut self) -> SleepdeepW<ScrSpec> {
        SleepdeepW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Sleep deep secure. This field controls whether the SLEEPDEEP bit is only accessible from the Secure state"]
    #[inline(always)]
    #[must_use]
    pub fn sleepdeeps(&mut self) -> SleepdeepsW<ScrSpec> {
        SleepdeepsW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Send Event on Pending bit: 0: Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded 1: Enabled events and all interrupts, including disabled interrupts, can wakeup the processor. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction."]
    #[inline(always)]
    #[must_use]
    pub fn sevonpend(&mut self) -> SevonpendW<ScrSpec> {
        SevonpendW::new(self, 4)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<ScrSpec> {
        Reserved5W::new(self, 5)
    }
}
#[doc = "System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {
    const RESET_VALUE: u32 = 0;
}

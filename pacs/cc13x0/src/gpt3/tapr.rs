#[doc = "Register `TAPR` reader"]
pub type R = crate::R<TaprSpec>;
#[doc = "Register `TAPR` writer"]
pub type W = crate::W<TaprSpec>;
#[doc = "Field `TAPSR` reader - 7:0\\]
Timer A Pre-scale. Prescaler ratio in one-shot and periodic count mode is TAPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
pub type TapsrR = crate::FieldReader;
#[doc = "Field `TAPSR` writer - 7:0\\]
Timer A Pre-scale. Prescaler ratio in one-shot and periodic count mode is TAPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
pub type TapsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Timer A Pre-scale. Prescaler ratio in one-shot and periodic count mode is TAPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
    #[inline(always)]
    pub fn tapsr(&self) -> TapsrR {
        TapsrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Timer A Pre-scale. Prescaler ratio in one-shot and periodic count mode is TAPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
    #[inline(always)]
    #[must_use]
    pub fn tapsr(&mut self) -> TapsrW<TaprSpec> {
        TapsrW::new(self, 0)
    }
}
#[doc = "Timer A Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TAR and TAV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tapr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tapr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaprSpec;
impl crate::RegisterSpec for TaprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tapr::R`](R) reader structure"]
impl crate::Readable for TaprSpec {}
#[doc = "`write(|w| ..)` method takes [`tapr::W`](W) writer structure"]
impl crate::Writable for TaprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAPR to value 0"]
impl crate::Resettable for TaprSpec {
    const RESET_VALUE: u32 = 0;
}

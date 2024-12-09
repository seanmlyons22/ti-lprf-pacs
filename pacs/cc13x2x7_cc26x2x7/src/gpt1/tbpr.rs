#[doc = "Register `TBPR` reader"]
pub type R = crate::R<TbprSpec>;
#[doc = "Register `TBPR` writer"]
pub type W = crate::W<TbprSpec>;
#[doc = "Field `TBPSR` reader - 7:0\\]
Timer B Pre-scale. Prescale ratio in one-shot and periodic count mode is TBPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
pub type TbpsrR = crate::FieldReader;
#[doc = "Field `TBPSR` writer - 7:0\\]
Timer B Pre-scale. Prescale ratio in one-shot and periodic count mode is TBPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
pub type TbpsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Timer B Pre-scale. Prescale ratio in one-shot and periodic count mode is TBPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
    #[inline(always)]
    pub fn tbpsr(&self) -> TbpsrR {
        TbpsrR::new((self.bits & 0xff) as u8)
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
Timer B Pre-scale. Prescale ratio in one-shot and periodic count mode is TBPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
    #[inline(always)]
    #[must_use]
    pub fn tbpsr(&mut self) -> TbpsrW<TbprSpec> {
        TbpsrW::new(self, 0)
    }
}
#[doc = "Timer B Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TBR and TBV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbprSpec;
impl crate::RegisterSpec for TbprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbpr::R`](R) reader structure"]
impl crate::Readable for TbprSpec {}
#[doc = "`write(|w| ..)` method takes [`tbpr::W`](W) writer structure"]
impl crate::Writable for TbprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBPR to value 0"]
impl crate::Resettable for TbprSpec {
    const RESET_VALUE: u32 = 0;
}

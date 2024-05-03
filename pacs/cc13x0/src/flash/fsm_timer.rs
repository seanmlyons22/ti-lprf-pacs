#[doc = "Register `FSM_TIMER` reader"]
pub type R = crate::R<FsmTimerSpec>;
#[doc = "Register `FSM_TIMER` writer"]
pub type W = crate::W<FsmTimerSpec>;
#[doc = "Field `FSM_TIMER` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmTimerR = crate::FieldReader<u32>;
#[doc = "Field `FSM_TIMER` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmTimerW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_timer(&self) -> FsmTimerR {
        FsmTimerR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_timer(&mut self) -> FsmTimerW<FsmTimerSpec> {
        FsmTimerW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmTimerSpec;
impl crate::RegisterSpec for FsmTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_timer::R`](R) reader structure"]
impl crate::Readable for FsmTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_timer::W`](W) writer structure"]
impl crate::Writable for FsmTimerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_TIMER to value 0"]
impl crate::Resettable for FsmTimerSpec {
    const RESET_VALUE: u32 = 0;
}

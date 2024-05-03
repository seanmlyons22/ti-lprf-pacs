#[doc = "Register `FSM_BSLE1` reader"]
pub type R = crate::R<FsmBsle1Spec>;
#[doc = "Register `FSM_BSLE1` writer"]
pub type W = crate::W<FsmBsle1Spec>;
#[doc = "Field `FSM_BSL1` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmBsl1R = crate::FieldReader<u32>;
#[doc = "Field `FSM_BSL1` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmBsl1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_bsl1(&self) -> FsmBsl1R {
        FsmBsl1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_bsl1(&mut self) -> FsmBsl1W<FsmBsle1Spec> {
        FsmBsl1W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_bsle1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_bsle1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmBsle1Spec;
impl crate::RegisterSpec for FsmBsle1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_bsle1::R`](R) reader structure"]
impl crate::Readable for FsmBsle1Spec {}
#[doc = "`write(|w| ..)` method takes [`fsm_bsle1::W`](W) writer structure"]
impl crate::Writable for FsmBsle1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_BSLE1 to value 0"]
impl crate::Resettable for FsmBsle1Spec {
    const RESET_VALUE: u32 = 0;
}

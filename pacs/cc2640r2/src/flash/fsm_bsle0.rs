#[doc = "Register `FSM_BSLE0` reader"]
pub type R = crate::R<FsmBsle0Spec>;
#[doc = "Register `FSM_BSLE0` writer"]
pub type W = crate::W<FsmBsle0Spec>;
#[doc = "Field `FSM_BSLE0` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmBsle0R = crate::FieldReader<u32>;
#[doc = "Field `FSM_BSLE0` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmBsle0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_bsle0(&self) -> FsmBsle0R {
        FsmBsle0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_bsle0(&mut self) -> FsmBsle0W<FsmBsle0Spec> {
        FsmBsle0W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_bsle0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_bsle0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmBsle0Spec;
impl crate::RegisterSpec for FsmBsle0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_bsle0::R`](R) reader structure"]
impl crate::Readable for FsmBsle0Spec {}
#[doc = "`write(|w| ..)` method takes [`fsm_bsle0::W`](W) writer structure"]
impl crate::Writable for FsmBsle0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_BSLE0 to value 0"]
impl crate::Resettable for FsmBsle0Spec {
    const RESET_VALUE: u32 = 0;
}

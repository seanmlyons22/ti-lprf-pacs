#[doc = "Register `FSM_ACC_PP` reader"]
pub type R = crate::R<FsmAccPpSpec>;
#[doc = "Register `FSM_ACC_PP` writer"]
pub type W = crate::W<FsmAccPpSpec>;
#[doc = "Field `FSM_ACC_PP` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmAccPpR = crate::FieldReader<u32>;
#[doc = "Field `FSM_ACC_PP` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmAccPpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_acc_pp(&self) -> FsmAccPpR {
        FsmAccPpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_acc_pp(&mut self) -> FsmAccPpW<FsmAccPpSpec> {
        FsmAccPpW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_acc_pp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_acc_pp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmAccPpSpec;
impl crate::RegisterSpec for FsmAccPpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_acc_pp::R`](R) reader structure"]
impl crate::Readable for FsmAccPpSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_acc_pp::W`](W) writer structure"]
impl crate::Writable for FsmAccPpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_ACC_PP to value 0"]
impl crate::Resettable for FsmAccPpSpec {
    const RESET_VALUE: u32 = 0;
}

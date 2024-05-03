#[doc = "Register `FSM_ERA_PW` reader"]
pub type R = crate::R<FsmEraPwSpec>;
#[doc = "Register `FSM_ERA_PW` writer"]
pub type W = crate::W<FsmEraPwSpec>;
#[doc = "Field `FSM_ERA_PW` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmEraPwR = crate::FieldReader<u32>;
#[doc = "Field `FSM_ERA_PW` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmEraPwW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_era_pw(&self) -> FsmEraPwR {
        FsmEraPwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_era_pw(&mut self) -> FsmEraPwW<FsmEraPwSpec> {
        FsmEraPwW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_era_pw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_era_pw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmEraPwSpec;
impl crate::RegisterSpec for FsmEraPwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_era_pw::R`](R) reader structure"]
impl crate::Readable for FsmEraPwSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_era_pw::W`](W) writer structure"]
impl crate::Writable for FsmEraPwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_ERA_PW to value 0"]
impl crate::Resettable for FsmEraPwSpec {
    const RESET_VALUE: u32 = 0;
}

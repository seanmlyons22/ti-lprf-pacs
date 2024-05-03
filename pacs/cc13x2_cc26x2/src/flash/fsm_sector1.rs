#[doc = "Register `FSM_SECTOR1` reader"]
pub type R = crate::R<FsmSector1Spec>;
#[doc = "Register `FSM_SECTOR1` writer"]
pub type W = crate::W<FsmSector1Spec>;
#[doc = "Field `FSM_SECTOR1` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmSector1R = crate::FieldReader<u32>;
#[doc = "Field `FSM_SECTOR1` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmSector1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_sector1(&self) -> FsmSector1R {
        FsmSector1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_sector1(&mut self) -> FsmSector1W<FsmSector1Spec> {
        FsmSector1W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_sector1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_sector1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmSector1Spec;
impl crate::RegisterSpec for FsmSector1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_sector1::R`](R) reader structure"]
impl crate::Readable for FsmSector1Spec {}
#[doc = "`write(|w| ..)` method takes [`fsm_sector1::W`](W) writer structure"]
impl crate::Writable for FsmSector1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_SECTOR1 to value 0xffff_ffff"]
impl crate::Resettable for FsmSector1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

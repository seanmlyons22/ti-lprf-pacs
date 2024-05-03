#[doc = "Register `FSM_SECTOR2` reader"]
pub type R = crate::R<FsmSector2Spec>;
#[doc = "Register `FSM_SECTOR2` writer"]
pub type W = crate::W<FsmSector2Spec>;
#[doc = "Field `FSM_SECTOR2` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmSector2R = crate::FieldReader<u32>;
#[doc = "Field `FSM_SECTOR2` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmSector2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_sector2(&self) -> FsmSector2R {
        FsmSector2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_sector2(&mut self) -> FsmSector2W<FsmSector2Spec> {
        FsmSector2W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_sector2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_sector2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmSector2Spec;
impl crate::RegisterSpec for FsmSector2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_sector2::R`](R) reader structure"]
impl crate::Readable for FsmSector2Spec {}
#[doc = "`write(|w| ..)` method takes [`fsm_sector2::W`](W) writer structure"]
impl crate::Writable for FsmSector2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_SECTOR2 to value 0x0fff"]
impl crate::Resettable for FsmSector2Spec {
    const RESET_VALUE: u32 = 0x0fff;
}

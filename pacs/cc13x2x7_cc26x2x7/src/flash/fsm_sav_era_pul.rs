#[doc = "Register `FSM_SAV_ERA_PUL` reader"]
pub type R = crate::R<FsmSavEraPulSpec>;
#[doc = "Register `FSM_SAV_ERA_PUL` writer"]
pub type W = crate::W<FsmSavEraPulSpec>;
#[doc = "Field `SAV_ERA_PUL` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type SavEraPulR = crate::FieldReader<u16>;
#[doc = "Field `SAV_ERA_PUL` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type SavEraPulW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_era_pul(&self) -> SavEraPulR {
        SavEraPulR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sav_era_pul(&mut self) -> SavEraPulW<FsmSavEraPulSpec> {
        SavEraPulW::new(self, 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<FsmSavEraPulSpec> {
        Reserved12W::new(self, 12)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_sav_era_pul::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_sav_era_pul::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmSavEraPulSpec;
impl crate::RegisterSpec for FsmSavEraPulSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_sav_era_pul::R`](R) reader structure"]
impl crate::Readable for FsmSavEraPulSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_sav_era_pul::W`](W) writer structure"]
impl crate::Writable for FsmSavEraPulSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_SAV_ERA_PUL to value 0"]
impl crate::Resettable for FsmSavEraPulSpec {
    const RESET_VALUE: u32 = 0;
}

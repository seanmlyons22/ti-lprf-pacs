#[doc = "Register `FSM_PUL_CNTR` reader"]
pub type R = crate::R<FsmPulCntrSpec>;
#[doc = "Register `FSM_PUL_CNTR` writer"]
pub type W = crate::W<FsmPulCntrSpec>;
#[doc = "Field `PUL_CNTR` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type PulCntrR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12R = crate::FieldReader;
#[doc = "Field `CUR_EC_LEVEL` reader - 24:16\\]
Internal. Only to be used through TI provided API."]
pub type CurEcLevelR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pul_cntr(&self) -> PulCntrR {
        PulCntrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cur_ec_level(&self) -> CurEcLevelR {
        CurEcLevelR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_pul_cntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_pul_cntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmPulCntrSpec;
impl crate::RegisterSpec for FsmPulCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_pul_cntr::R`](R) reader structure"]
impl crate::Readable for FsmPulCntrSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_pul_cntr::W`](W) writer structure"]
impl crate::Writable for FsmPulCntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_PUL_CNTR to value 0"]
impl crate::Resettable for FsmPulCntrSpec {
    const RESET_VALUE: u32 = 0;
}

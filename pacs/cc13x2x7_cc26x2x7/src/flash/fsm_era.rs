#[doc = "Register `FSM_ERA` reader"]
pub type R = crate::R<FsmEraSpec>;
#[doc = "Register `FSM_ERA` writer"]
pub type W = crate::W<FsmEraSpec>;
#[doc = "Field `ERA_ADDR` reader - 22:0\\]
Internal. Only to be used through TI provided API."]
pub type EraAddrR = crate::FieldReader<u32>;
#[doc = "Field `ERA_BANK` reader - 25:23\\]
Internal. Only to be used through TI provided API."]
pub type EraBankR = crate::FieldReader;
#[doc = "Field `RESERVED26` reader - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type Reserved26R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:22 - 22:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_addr(&self) -> EraAddrR {
        EraAddrR::new(self.bits & 0x007f_ffff)
    }
    #[doc = "Bits 23:25 - 25:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_bank(&self) -> EraBankR {
        EraBankR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved26(&self) -> Reserved26R {
        Reserved26R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_era::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_era::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmEraSpec;
impl crate::RegisterSpec for FsmEraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_era::R`](R) reader structure"]
impl crate::Readable for FsmEraSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_era::W`](W) writer structure"]
impl crate::Writable for FsmEraSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_ERA to value 0"]
impl crate::Resettable for FsmEraSpec {
    const RESET_VALUE: u32 = 0;
}

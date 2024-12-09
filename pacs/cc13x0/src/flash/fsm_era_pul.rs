#[doc = "Register `FSM_ERA_PUL` reader"]
pub type R = crate::R<FsmEraPulSpec>;
#[doc = "Register `FSM_ERA_PUL` writer"]
pub type W = crate::W<FsmEraPulSpec>;
#[doc = "Field `MAX_ERA_PUL` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type MaxEraPulR = crate::FieldReader<u16>;
#[doc = "Field `MAX_ERA_PUL` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type MaxEraPulW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12R = crate::FieldReader;
#[doc = "Field `MAX_EC_LEVEL` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type MaxEcLevelR = crate::FieldReader;
#[doc = "Field `MAX_EC_LEVEL` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type MaxEcLevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type Reserved20R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_era_pul(&self) -> MaxEraPulR {
        MaxEraPulR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_ec_level(&self) -> MaxEcLevelR {
        MaxEcLevelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn max_era_pul(&mut self) -> MaxEraPulW<FsmEraPulSpec> {
        MaxEraPulW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn max_ec_level(&mut self) -> MaxEcLevelW<FsmEraPulSpec> {
        MaxEcLevelW::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_era_pul::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_era_pul::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmEraPulSpec;
impl crate::RegisterSpec for FsmEraPulSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_era_pul::R`](R) reader structure"]
impl crate::Readable for FsmEraPulSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_era_pul::W`](W) writer structure"]
impl crate::Writable for FsmEraPulSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_ERA_PUL to value 0x0004_0bb8"]
impl crate::Resettable for FsmEraPulSpec {
    const RESET_VALUE: u32 = 0x0004_0bb8;
}

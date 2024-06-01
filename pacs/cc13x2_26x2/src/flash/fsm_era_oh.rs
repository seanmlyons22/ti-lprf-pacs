#[doc = "Register `FSM_ERA_OH` reader"]
pub type R = crate::R<FsmEraOhSpec>;
#[doc = "Register `FSM_ERA_OH` writer"]
pub type W = crate::W<FsmEraOhSpec>;
#[doc = "Field `ERA_OH` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type EraOhR = crate::FieldReader<u16>;
#[doc = "Field `ERA_OH` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type EraOhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_oh(&self) -> EraOhR {
        EraOhR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn era_oh(&mut self) -> EraOhW<FsmEraOhSpec> {
        EraOhW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<FsmEraOhSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_era_oh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_era_oh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmEraOhSpec;
impl crate::RegisterSpec for FsmEraOhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_era_oh::R`](R) reader structure"]
impl crate::Readable for FsmEraOhSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_era_oh::W`](W) writer structure"]
impl crate::Writable for FsmEraOhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_ERA_OH to value 0x01"]
impl crate::Resettable for FsmEraOhSpec {
    const RESET_VALUE: u32 = 0x01;
}

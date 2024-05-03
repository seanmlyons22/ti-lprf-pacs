#[doc = "Register `FSM_STEP_SIZE` reader"]
pub type R = crate::R<FsmStepSizeSpec>;
#[doc = "Register `FSM_STEP_SIZE` writer"]
pub type W = crate::W<FsmStepSizeSpec>;
#[doc = "Field `RESERVED0` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED0` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EC_STEP_SIZE` reader - 24:16\\]
Internal. Only to be used through TI provided API."]
pub type EcStepSizeR = crate::FieldReader<u16>;
#[doc = "Field `EC_STEP_SIZE` writer - 24:16\\]
Internal. Only to be used through TI provided API."]
pub type EcStepSizeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ec_step_size(&self) -> EcStepSizeR {
        EcStepSizeR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<FsmStepSizeSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ec_step_size(&mut self) -> EcStepSizeW<FsmStepSizeSpec> {
        EcStepSizeW::new(self, 16)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<FsmStepSizeSpec> {
        Reserved25W::new(self, 25)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_step_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_step_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmStepSizeSpec;
impl crate::RegisterSpec for FsmStepSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_step_size::R`](R) reader structure"]
impl crate::Readable for FsmStepSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_step_size::W`](W) writer structure"]
impl crate::Writable for FsmStepSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_STEP_SIZE to value 0"]
impl crate::Resettable for FsmStepSizeSpec {
    const RESET_VALUE: u32 = 0;
}

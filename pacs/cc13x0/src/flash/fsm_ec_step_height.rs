#[doc = "Register `FSM_EC_STEP_HEIGHT` reader"]
pub type R = crate::R<FsmEcStepHeightSpec>;
#[doc = "Register `FSM_EC_STEP_HEIGHT` writer"]
pub type W = crate::W<FsmEcStepHeightSpec>;
#[doc = "Field `EC_STEP_HEIGHT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type EcStepHeightR = crate::FieldReader;
#[doc = "Field `EC_STEP_HEIGHT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type EcStepHeightW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ec_step_height(&self) -> EcStepHeightR {
        EcStepHeightR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ec_step_height(&mut self) -> EcStepHeightW<FsmEcStepHeightSpec> {
        EcStepHeightW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_ec_step_height::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_ec_step_height::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmEcStepHeightSpec;
impl crate::RegisterSpec for FsmEcStepHeightSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_ec_step_height::R`](R) reader structure"]
impl crate::Readable for FsmEcStepHeightSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_ec_step_height::W`](W) writer structure"]
impl crate::Writable for FsmEcStepHeightSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_EC_STEP_HEIGHT to value 0"]
impl crate::Resettable for FsmEcStepHeightSpec {
    const RESET_VALUE: u32 = 0;
}

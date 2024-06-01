#[doc = "Register `FSM_CMD` reader"]
pub type R = crate::R<FsmCmdSpec>;
#[doc = "Register `FSM_CMD` writer"]
pub type W = crate::W<FsmCmdSpec>;
#[doc = "Field `FSMCMD` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmcmdR = crate::FieldReader;
#[doc = "Field `FSMCMD` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmcmdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsmcmd(&self) -> FsmcmdR {
        FsmcmdR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsmcmd(&mut self) -> FsmcmdW<FsmCmdSpec> {
        FsmcmdW::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<FsmCmdSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmCmdSpec;
impl crate::RegisterSpec for FsmCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_cmd::R`](R) reader structure"]
impl crate::Readable for FsmCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_cmd::W`](W) writer structure"]
impl crate::Writable for FsmCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_CMD to value 0"]
impl crate::Resettable for FsmCmdSpec {
    const RESET_VALUE: u32 = 0;
}

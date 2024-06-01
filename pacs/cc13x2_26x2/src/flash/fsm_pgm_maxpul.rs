#[doc = "Register `FSM_PGM_MAXPUL` reader"]
pub type R = crate::R<FsmPgmMaxpulSpec>;
#[doc = "Register `FSM_PGM_MAXPUL` writer"]
pub type W = crate::W<FsmPgmMaxpulSpec>;
#[doc = "Field `FSM_PGM_MAXPUL` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmPgmMaxpulR = crate::FieldReader<u16>;
#[doc = "Field `FSM_PGM_MAXPUL` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmPgmMaxpulW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
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
    pub fn fsm_pgm_maxpul(&self) -> FsmPgmMaxpulR {
        FsmPgmMaxpulR::new((self.bits & 0x0fff) as u16)
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
    pub fn fsm_pgm_maxpul(&mut self) -> FsmPgmMaxpulW<FsmPgmMaxpulSpec> {
        FsmPgmMaxpulW::new(self, 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<FsmPgmMaxpulSpec> {
        Reserved12W::new(self, 12)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_pgm_maxpul::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_pgm_maxpul::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmPgmMaxpulSpec;
impl crate::RegisterSpec for FsmPgmMaxpulSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_pgm_maxpul::R`](R) reader structure"]
impl crate::Readable for FsmPgmMaxpulSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_pgm_maxpul::W`](W) writer structure"]
impl crate::Writable for FsmPgmMaxpulSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_PGM_MAXPUL to value 0"]
impl crate::Resettable for FsmPgmMaxpulSpec {
    const RESET_VALUE: u32 = 0;
}

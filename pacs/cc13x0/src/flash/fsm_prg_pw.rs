#[doc = "Register `FSM_PRG_PW` reader"]
pub type R = crate::R<FsmPrgPwSpec>;
#[doc = "Register `FSM_PRG_PW` writer"]
pub type W = crate::W<FsmPrgPwSpec>;
#[doc = "Field `PROG_PUL_WIDTH` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type ProgPulWidthR = crate::FieldReader<u16>;
#[doc = "Field `PROG_PUL_WIDTH` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type ProgPulWidthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    pub fn prog_pul_width(&self) -> ProgPulWidthR {
        ProgPulWidthR::new((self.bits & 0xffff) as u16)
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
    pub fn prog_pul_width(&mut self) -> ProgPulWidthW<FsmPrgPwSpec> {
        ProgPulWidthW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<FsmPrgPwSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_prg_pw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_prg_pw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmPrgPwSpec;
impl crate::RegisterSpec for FsmPrgPwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_prg_pw::R`](R) reader structure"]
impl crate::Readable for FsmPrgPwSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_prg_pw::W`](W) writer structure"]
impl crate::Writable for FsmPrgPwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_PRG_PW to value 0"]
impl crate::Resettable for FsmPrgPwSpec {
    const RESET_VALUE: u32 = 0;
}

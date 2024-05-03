#[doc = "Register `FSM_PE_OSU` reader"]
pub type R = crate::R<FsmPeOsuSpec>;
#[doc = "Register `FSM_PE_OSU` writer"]
pub type W = crate::W<FsmPeOsuSpec>;
#[doc = "Field `ERA_OSU` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EraOsuR = crate::FieldReader;
#[doc = "Field `ERA_OSU` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EraOsuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PGM_OSU` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PgmOsuR = crate::FieldReader;
#[doc = "Field `PGM_OSU` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PgmOsuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_osu(&self) -> EraOsuR {
        EraOsuR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_osu(&self) -> PgmOsuR {
        PgmOsuR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn era_osu(&mut self) -> EraOsuW<FsmPeOsuSpec> {
        EraOsuW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_osu(&mut self) -> PgmOsuW<FsmPeOsuSpec> {
        PgmOsuW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<FsmPeOsuSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_pe_osu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_pe_osu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmPeOsuSpec;
impl crate::RegisterSpec for FsmPeOsuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_pe_osu::R`](R) reader structure"]
impl crate::Readable for FsmPeOsuSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_pe_osu::W`](W) writer structure"]
impl crate::Writable for FsmPeOsuSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_PE_OSU to value 0"]
impl crate::Resettable for FsmPeOsuSpec {
    const RESET_VALUE: u32 = 0;
}

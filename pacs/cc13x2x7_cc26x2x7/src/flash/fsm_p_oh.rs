#[doc = "Register `FSM_P_OH` reader"]
pub type R = crate::R<FsmPOhSpec>;
#[doc = "Register `FSM_P_OH` writer"]
pub type W = crate::W<FsmPOhSpec>;
#[doc = "Field `RESERVED0` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `PGM_OH` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PgmOhR = crate::FieldReader;
#[doc = "Field `PGM_OH` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PgmOhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_oh(&self) -> PgmOhR {
        PgmOhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_oh(&mut self) -> PgmOhW<FsmPOhSpec> {
        PgmOhW::new(self, 8)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_p_oh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_p_oh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmPOhSpec;
impl crate::RegisterSpec for FsmPOhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_p_oh::R`](R) reader structure"]
impl crate::Readable for FsmPOhSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_p_oh::W`](W) writer structure"]
impl crate::Writable for FsmPOhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_P_OH to value 0x0100"]
impl crate::Resettable for FsmPOhSpec {
    const RESET_VALUE: u32 = 0x0100;
}

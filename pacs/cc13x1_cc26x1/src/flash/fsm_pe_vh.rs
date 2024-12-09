#[doc = "Register `FSM_PE_VH` reader"]
pub type R = crate::R<FsmPeVhSpec>;
#[doc = "Register `FSM_PE_VH` writer"]
pub type W = crate::W<FsmPeVhSpec>;
#[doc = "Field `ERA_VH` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EraVhR = crate::FieldReader;
#[doc = "Field `PGM_VH` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PgmVhR = crate::FieldReader;
#[doc = "Field `PGM_VH` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PgmVhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_vh(&self) -> EraVhR {
        EraVhR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_vh(&self) -> PgmVhR {
        PgmVhR::new(((self.bits >> 8) & 0xff) as u8)
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
    pub fn pgm_vh(&mut self) -> PgmVhW<FsmPeVhSpec> {
        PgmVhW::new(self, 8)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_pe_vh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_pe_vh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmPeVhSpec;
impl crate::RegisterSpec for FsmPeVhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_pe_vh::R`](R) reader structure"]
impl crate::Readable for FsmPeVhSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_pe_vh::W`](W) writer structure"]
impl crate::Writable for FsmPeVhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_PE_VH to value 0x0100"]
impl crate::Resettable for FsmPeVhSpec {
    const RESET_VALUE: u32 = 0x0100;
}

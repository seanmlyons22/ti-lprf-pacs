#[doc = "Register `FSM_PGM` reader"]
pub type R = crate::R<FsmPgmSpec>;
#[doc = "Register `FSM_PGM` writer"]
pub type W = crate::W<FsmPgmSpec>;
#[doc = "Field `PGM_ADDR` reader - 22:0\\]
Internal. Only to be used through TI provided API."]
pub type PgmAddrR = crate::FieldReader<u32>;
#[doc = "Field `PGM_ADDR` writer - 22:0\\]
Internal. Only to be used through TI provided API."]
pub type PgmAddrW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
#[doc = "Field `PGM_BANK` reader - 25:23\\]
Internal. Only to be used through TI provided API."]
pub type PgmBankR = crate::FieldReader;
#[doc = "Field `PGM_BANK` writer - 25:23\\]
Internal. Only to be used through TI provided API."]
pub type PgmBankW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED26` reader - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type Reserved26R = crate::FieldReader;
#[doc = "Field `RESERVED26` writer - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type Reserved26W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:22 - 22:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_addr(&self) -> PgmAddrR {
        PgmAddrR::new(self.bits & 0x007f_ffff)
    }
    #[doc = "Bits 23:25 - 25:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_bank(&self) -> PgmBankR {
        PgmBankR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved26(&self) -> Reserved26R {
        Reserved26R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:22 - 22:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_addr(&mut self) -> PgmAddrW<FsmPgmSpec> {
        PgmAddrW::new(self, 0)
    }
    #[doc = "Bits 23:25 - 25:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_bank(&mut self) -> PgmBankW<FsmPgmSpec> {
        PgmBankW::new(self, 23)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved26(&mut self) -> Reserved26W<FsmPgmSpec> {
        Reserved26W::new(self, 26)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_pgm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_pgm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmPgmSpec;
impl crate::RegisterSpec for FsmPgmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_pgm::R`](R) reader structure"]
impl crate::Readable for FsmPgmSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_pgm::W`](W) writer structure"]
impl crate::Writable for FsmPgmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_PGM to value 0"]
impl crate::Resettable for FsmPgmSpec {
    const RESET_VALUE: u32 = 0;
}

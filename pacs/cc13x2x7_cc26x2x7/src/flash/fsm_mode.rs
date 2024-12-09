#[doc = "Register `FSM_MODE` reader"]
pub type R = crate::R<FsmModeSpec>;
#[doc = "Register `FSM_MODE` writer"]
pub type W = crate::W<FsmModeSpec>;
#[doc = "Field `CMD` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type CmdR = crate::FieldReader;
#[doc = "Field `MODE` reader - 5:3\\]
Internal. Only to be used through TI provided API."]
pub type ModeR = crate::FieldReader;
#[doc = "Field `SAV_ERA_MODE` reader - 8:6\\]
Internal. Only to be used through TI provided API."]
pub type SavEraModeR = crate::FieldReader;
#[doc = "Field `SAV_PGM_CMD` reader - 11:9\\]
Internal. Only to be used through TI provided API."]
pub type SavPgmCmdR = crate::FieldReader;
#[doc = "Field `SUBMODE` reader - 13:12\\]
Internal. Only to be used through TI provided API."]
pub type SubmodeR = crate::FieldReader;
#[doc = "Field `ERA_SUBMODE` reader - 15:14\\]
Internal. Only to be used through TI provided API."]
pub type EraSubmodeR = crate::FieldReader;
#[doc = "Field `PGM_SUBMODE` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type PgmSubmodeR = crate::FieldReader;
#[doc = "Field `RDV_SUBMODE` reader - 19:18\\]
Internal. Only to be used through TI provided API."]
pub type RdvSubmodeR = crate::FieldReader;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type Reserved20R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_era_mode(&self) -> SavEraModeR {
        SavEraModeR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_pgm_cmd(&self) -> SavPgmCmdR {
        SavPgmCmdR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn submode(&self) -> SubmodeR {
        SubmodeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_submode(&self) -> EraSubmodeR {
        EraSubmodeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_submode(&self) -> PgmSubmodeR {
        PgmSubmodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rdv_submode(&self) -> RdvSubmodeR {
        RdvSubmodeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmModeSpec;
impl crate::RegisterSpec for FsmModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_mode::R`](R) reader structure"]
impl crate::Readable for FsmModeSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_mode::W`](W) writer structure"]
impl crate::Writable for FsmModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_MODE to value 0"]
impl crate::Resettable for FsmModeSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `FSM_ST_MACHINE` reader"]
pub type R = crate::R<FsmStMachineSpec>;
#[doc = "Register `FSM_ST_MACHINE` writer"]
pub type W = crate::W<FsmStMachineSpec>;
#[doc = "Field `OVERRIDE` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type OverrideR = crate::BitReader;
#[doc = "Field `OVERRIDE` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type OverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV_DATA` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type InvDataR = crate::BitReader;
#[doc = "Field `INV_DATA` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type InvDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_EN` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type CmdEnR = crate::BitReader;
#[doc = "Field `CMD_EN` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type CmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_TST_EN` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type DisTstEnR = crate::BitReader;
#[doc = "Field `DIS_TST_EN` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type DisTstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREC_STOP_EN` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type PrecStopEnR = crate::BitReader;
#[doc = "Field `PREC_STOP_EN` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type PrecStopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGM_SEC_COF_EN` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type PgmSecCofEnR = crate::BitReader;
#[doc = "Field `PGM_SEC_COF_EN` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type PgmSecCofEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `RESERVED6` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_SHORT_ROW` reader - 10:7\\]
Internal. Only to be used through TI provided API."]
pub type DbgShortRowR = crate::FieldReader;
#[doc = "Field `DBG_SHORT_ROW` writer - 10:7\\]
Internal. Only to be used through TI provided API."]
pub type DbgShortRowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DO_REDU_COL` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type DoReduColR = crate::BitReader;
#[doc = "Field `DO_REDU_COL` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type DoReduColW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED12` reader - 13:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12R = crate::FieldReader;
#[doc = "Field `RESERVED12` writer - 13:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ONE_TIME_GOOD` reader - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type OneTimeGoodR = crate::BitReader;
#[doc = "Field `ONE_TIME_GOOD` writer - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type OneTimeGoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED15` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `RESERVED15` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RV_INT_EN` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type RvIntEnR = crate::BitReader;
#[doc = "Field `RV_INT_EN` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type RvIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RV_RES` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RvResR = crate::BitReader;
#[doc = "Field `RV_RES` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RvResW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RV_SEC_EN` reader - 18:18\\]
Internal. Only to be used through TI provided API."]
pub type RvSecEnR = crate::BitReader;
#[doc = "Field `RV_SEC_EN` writer - 18:18\\]
Internal. Only to be used through TI provided API."]
pub type RvSecEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANDOM` reader - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type RandomR = crate::BitReader;
#[doc = "Field `RANDOM` writer - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type RandomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPV_ALLOWED` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type CmpvAllowedR = crate::BitReader;
#[doc = "Field `CMPV_ALLOWED` writer - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type CmpvAllowedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALL_BANKS` reader - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type AllBanksR = crate::BitReader;
#[doc = "Field `ALL_BANKS` writer - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type AllBanksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSM_INT_EN` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type FsmIntEnR = crate::BitReader;
#[doc = "Field `FSM_INT_EN` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type FsmIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO_PRECOND` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type DoPrecondR = crate::BitReader;
#[doc = "Field `DO_PRECOND` writer - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type DoPrecondW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn override_(&self) -> OverrideR {
        OverrideR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inv_data(&self) -> InvDataR {
        InvDataR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmd_en(&self) -> CmdEnR {
        CmdEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_tst_en(&self) -> DisTstEnR {
        DisTstEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn prec_stop_en(&self) -> PrecStopEnR {
        PrecStopEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_sec_cof_en(&self) -> PgmSecCofEnR {
        PgmSecCofEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - 10:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dbg_short_row(&self) -> DbgShortRowR {
        DbgShortRowR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn do_redu_col(&self) -> DoReduColR {
        DoReduColR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn one_time_good(&self) -> OneTimeGoodR {
        OneTimeGoodR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rv_int_en(&self) -> RvIntEnR {
        RvIntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rv_res(&self) -> RvResR {
        RvResR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rv_sec_en(&self) -> RvSecEnR {
        RvSecEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn random(&self) -> RandomR {
        RandomR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmpv_allowed(&self) -> CmpvAllowedR {
        CmpvAllowedR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn all_banks(&self) -> AllBanksR {
        AllBanksR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_int_en(&self) -> FsmIntEnR {
        FsmIntEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn do_precond(&self) -> DoPrecondR {
        DoPrecondR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn override_(&mut self) -> OverrideW<FsmStMachineSpec> {
        OverrideW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn inv_data(&mut self) -> InvDataW<FsmStMachineSpec> {
        InvDataW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_en(&mut self) -> CmdEnW<FsmStMachineSpec> {
        CmdEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_tst_en(&mut self) -> DisTstEnW<FsmStMachineSpec> {
        DisTstEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn prec_stop_en(&mut self) -> PrecStopEnW<FsmStMachineSpec> {
        PrecStopEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_sec_cof_en(&mut self) -> PgmSecCofEnW<FsmStMachineSpec> {
        PgmSecCofEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<FsmStMachineSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bits 7:10 - 10:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_short_row(&mut self) -> DbgShortRowW<FsmStMachineSpec> {
        DbgShortRowW::new(self, 7)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn do_redu_col(&mut self) -> DoReduColW<FsmStMachineSpec> {
        DoReduColW::new(self, 11)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<FsmStMachineSpec> {
        Reserved12W::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn one_time_good(&mut self) -> OneTimeGoodW<FsmStMachineSpec> {
        OneTimeGoodW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<FsmStMachineSpec> {
        Reserved15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rv_int_en(&mut self) -> RvIntEnW<FsmStMachineSpec> {
        RvIntEnW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rv_res(&mut self) -> RvResW<FsmStMachineSpec> {
        RvResW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rv_sec_en(&mut self) -> RvSecEnW<FsmStMachineSpec> {
        RvSecEnW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn random(&mut self) -> RandomW<FsmStMachineSpec> {
        RandomW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cmpv_allowed(&mut self) -> CmpvAllowedW<FsmStMachineSpec> {
        CmpvAllowedW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn all_banks(&mut self) -> AllBanksW<FsmStMachineSpec> {
        AllBanksW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_int_en(&mut self) -> FsmIntEnW<FsmStMachineSpec> {
        FsmIntEnW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn do_precond(&mut self) -> DoPrecondW<FsmStMachineSpec> {
        DoPrecondW::new(self, 23)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<FsmStMachineSpec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_st_machine::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_st_machine::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmStMachineSpec;
impl crate::RegisterSpec for FsmStMachineSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_st_machine::R`](R) reader structure"]
impl crate::Readable for FsmStMachineSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_st_machine::W`](W) writer structure"]
impl crate::Writable for FsmStMachineSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_ST_MACHINE to value 0x0080_0500"]
impl crate::Resettable for FsmStMachineSpec {
    const RESET_VALUE: u32 = 0x0080_0500;
}

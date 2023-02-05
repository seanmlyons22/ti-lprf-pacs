#[doc = "Register `FSM_ST_MACHINE` reader"]
pub struct R(crate::R<FSM_ST_MACHINE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_ST_MACHINE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_ST_MACHINE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_ST_MACHINE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_ST_MACHINE` writer"]
pub struct W(crate::W<FSM_ST_MACHINE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_ST_MACHINE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FSM_ST_MACHINE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_ST_MACHINE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVERRIDE` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `OVERRIDE` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `INV_DATA` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type INV_DATA_R = crate::BitReader<bool>;
#[doc = "Field `INV_DATA` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type INV_DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `CMD_EN` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMD_EN` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `DIS_TST_EN` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type DIS_TST_EN_R = crate::BitReader<bool>;
#[doc = "Field `DIS_TST_EN` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type DIS_TST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `PREC_STOP_EN` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type PREC_STOP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PREC_STOP_EN` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type PREC_STOP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `PGM_SEC_COF_EN` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type PGM_SEC_COF_EN_R = crate::BitReader<bool>;
#[doc = "Field `PGM_SEC_COF_EN` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type PGM_SEC_COF_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED6` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `DBG_SHORT_ROW` reader - 10:7\\]
Internal. Only to be used through TI provided API."]
pub type DBG_SHORT_ROW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBG_SHORT_ROW` writer - 10:7\\]
Internal. Only to be used through TI provided API."]
pub type DBG_SHORT_ROW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_ST_MACHINE_SPEC, u8, u8, 4, O>;
#[doc = "Field `DO_REDU_COL` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type DO_REDU_COL_R = crate::BitReader<bool>;
#[doc = "Field `DO_REDU_COL` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type DO_REDU_COL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `RESERVED12` reader - 13:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED12` writer - 13:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_ST_MACHINE_SPEC, u8, u8, 2, O>;
#[doc = "Field `ONE_TIME_GOOD` reader - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type ONE_TIME_GOOD_R = crate::BitReader<bool>;
#[doc = "Field `ONE_TIME_GOOD` writer - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type ONE_TIME_GOOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `RESERVED15` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED15_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED15` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `RV_INT_EN` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type RV_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RV_INT_EN` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type RV_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `RV_RES` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RV_RES_R = crate::BitReader<bool>;
#[doc = "Field `RV_RES` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RV_RES_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `RV_SEC_EN` reader - 18:18\\]
Internal. Only to be used through TI provided API."]
pub type RV_SEC_EN_R = crate::BitReader<bool>;
#[doc = "Field `RV_SEC_EN` writer - 18:18\\]
Internal. Only to be used through TI provided API."]
pub type RV_SEC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `RANDOM` reader - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type RANDOM_R = crate::BitReader<bool>;
#[doc = "Field `RANDOM` writer - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type RANDOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `CMPV_ALLOWED` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type CMPV_ALLOWED_R = crate::BitReader<bool>;
#[doc = "Field `CMPV_ALLOWED` writer - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type CMPV_ALLOWED_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `ALL_BANKS` reader - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type ALL_BANKS_R = crate::BitReader<bool>;
#[doc = "Field `ALL_BANKS` writer - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type ALL_BANKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `FSM_INT_EN` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type FSM_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `FSM_INT_EN` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type FSM_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `DO_PRECOND` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type DO_PRECOND_R = crate::BitReader<bool>;
#[doc = "Field `DO_PRECOND` writer - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type DO_PRECOND_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ST_MACHINE_SPEC, bool, O>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED24_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_ST_MACHINE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn override_(&self) -> OVERRIDE_R {
        OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inv_data(&self) -> INV_DATA_R {
        INV_DATA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmd_en(&self) -> CMD_EN_R {
        CMD_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_tst_en(&self) -> DIS_TST_EN_R {
        DIS_TST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn prec_stop_en(&self) -> PREC_STOP_EN_R {
        PREC_STOP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_sec_cof_en(&self) -> PGM_SEC_COF_EN_R {
        PGM_SEC_COF_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - 10:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dbg_short_row(&self) -> DBG_SHORT_ROW_R {
        DBG_SHORT_ROW_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn do_redu_col(&self) -> DO_REDU_COL_R {
        DO_REDU_COL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn one_time_good(&self) -> ONE_TIME_GOOD_R {
        ONE_TIME_GOOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rv_int_en(&self) -> RV_INT_EN_R {
        RV_INT_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rv_res(&self) -> RV_RES_R {
        RV_RES_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rv_sec_en(&self) -> RV_SEC_EN_R {
        RV_SEC_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn random(&self) -> RANDOM_R {
        RANDOM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmpv_allowed(&self) -> CMPV_ALLOWED_R {
        CMPV_ALLOWED_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn all_banks(&self) -> ALL_BANKS_R {
        ALL_BANKS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_int_en(&self) -> FSM_INT_EN_R {
        FSM_INT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn do_precond(&self) -> DO_PRECOND_R {
        DO_PRECOND_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn override_(&mut self) -> OVERRIDE_W<0> {
        OVERRIDE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn inv_data(&mut self) -> INV_DATA_W<1> {
        INV_DATA_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_en(&mut self) -> CMD_EN_W<2> {
        CMD_EN_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_tst_en(&mut self) -> DIS_TST_EN_W<3> {
        DIS_TST_EN_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn prec_stop_en(&mut self) -> PREC_STOP_EN_W<4> {
        PREC_STOP_EN_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_sec_cof_en(&mut self) -> PGM_SEC_COF_EN_W<5> {
        PGM_SEC_COF_EN_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bits 7:10 - 10:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_short_row(&mut self) -> DBG_SHORT_ROW_W<7> {
        DBG_SHORT_ROW_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn do_redu_col(&mut self) -> DO_REDU_COL_W<11> {
        DO_REDU_COL_W::new(self)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn one_time_good(&mut self) -> ONE_TIME_GOOD_W<14> {
        ONE_TIME_GOOD_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> RESERVED15_W<15> {
        RESERVED15_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rv_int_en(&mut self) -> RV_INT_EN_W<16> {
        RV_INT_EN_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rv_res(&mut self) -> RV_RES_W<17> {
        RV_RES_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rv_sec_en(&mut self) -> RV_SEC_EN_W<18> {
        RV_SEC_EN_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn random(&mut self) -> RANDOM_W<19> {
        RANDOM_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cmpv_allowed(&mut self) -> CMPV_ALLOWED_W<20> {
        CMPV_ALLOWED_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn all_banks(&mut self) -> ALL_BANKS_W<21> {
        ALL_BANKS_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_int_en(&mut self) -> FSM_INT_EN_W<22> {
        FSM_INT_EN_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn do_precond(&mut self) -> DO_PRECOND_W<23> {
        DO_PRECOND_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_st_machine](index.html) module"]
pub struct FSM_ST_MACHINE_SPEC;
impl crate::RegisterSpec for FSM_ST_MACHINE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_st_machine::R](R) reader structure"]
impl crate::Readable for FSM_ST_MACHINE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_st_machine::W](W) writer structure"]
impl crate::Writable for FSM_ST_MACHINE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_ST_MACHINE to value 0x0080_0500"]
impl crate::Resettable for FSM_ST_MACHINE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_0500;
}

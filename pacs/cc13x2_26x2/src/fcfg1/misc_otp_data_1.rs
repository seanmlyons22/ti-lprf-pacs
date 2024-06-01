#[doc = "Register `MISC_OTP_DATA_1` reader"]
pub type R = crate::R<MiscOtpData1Spec>;
#[doc = "Register `MISC_OTP_DATA_1` writer"]
pub type W = crate::W<MiscOtpData1Spec>;
#[doc = "Field `IDAC_STEP` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type IdacStepR = crate::FieldReader;
#[doc = "Field `IDAC_STEP` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type IdacStepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPM_IBIAS_WAIT_CNT` reader - 9:4\\]
Internal. Only to be used through TI provided API."]
pub type LpmIbiasWaitCntR = crate::FieldReader;
#[doc = "Field `LPM_IBIAS_WAIT_CNT` writer - 9:4\\]
Internal. Only to be used through TI provided API."]
pub type LpmIbiasWaitCntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HPM_IBIAS_WAIT_CNT` reader - 19:10\\]
Internal. Only to be used through TI provided API."]
pub type HpmIbiasWaitCntR = crate::FieldReader<u16>;
#[doc = "Field `HPM_IBIAS_WAIT_CNT` writer - 19:10\\]
Internal. Only to be used through TI provided API."]
pub type HpmIbiasWaitCntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DBLR_LOOP_FILTER_RESET_VOLTAGE` reader - 21:20\\]
Internal. Only to be used through TI provided API."]
pub type DblrLoopFilterResetVoltageR = crate::FieldReader;
#[doc = "Field `DBLR_LOOP_FILTER_RESET_VOLTAGE` writer - 21:20\\]
Internal. Only to be used through TI provided API."]
pub type DblrLoopFilterResetVoltageW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_BUF_ITRIM` reader - 23:22\\]
Internal. Only to be used through TI provided API."]
pub type LpBufItrimR = crate::FieldReader;
#[doc = "Field `LP_BUF_ITRIM` writer - 23:22\\]
Internal. Only to be used through TI provided API."]
pub type LpBufItrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_BUF_ITRIM` reader - 26:24\\]
Internal. Only to be used through TI provided API."]
pub type HpBufItrimR = crate::FieldReader;
#[doc = "Field `HP_BUF_ITRIM` writer - 26:24\\]
Internal. Only to be used through TI provided API."]
pub type HpBufItrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PEAK_DET_ITRIM` reader - 28:27\\]
Internal. Only to be used through TI provided API."]
pub type PeakDetItrimR = crate::FieldReader;
#[doc = "Field `PEAK_DET_ITRIM` writer - 28:27\\]
Internal. Only to be used through TI provided API."]
pub type PeakDetItrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn idac_step(&self) -> IdacStepR {
        IdacStepR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - 9:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt(&self) -> LpmIbiasWaitCntR {
        LpmIbiasWaitCntR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:19 - 19:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpm_ibias_wait_cnt(&self) -> HpmIbiasWaitCntR {
        HpmIbiasWaitCntR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dblr_loop_filter_reset_voltage(&self) -> DblrLoopFilterResetVoltageR {
        DblrLoopFilterResetVoltageR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lp_buf_itrim(&self) -> LpBufItrimR {
        LpBufItrimR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hp_buf_itrim(&self) -> HpBufItrimR {
        HpBufItrimR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn peak_det_itrim(&self) -> PeakDetItrimR {
        PeakDetItrimR::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn idac_step(&mut self) -> IdacStepW<MiscOtpData1Spec> {
        IdacStepW::new(self, 0)
    }
    #[doc = "Bits 4:9 - 9:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_ibias_wait_cnt(&mut self) -> LpmIbiasWaitCntW<MiscOtpData1Spec> {
        LpmIbiasWaitCntW::new(self, 4)
    }
    #[doc = "Bits 10:19 - 19:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hpm_ibias_wait_cnt(&mut self) -> HpmIbiasWaitCntW<MiscOtpData1Spec> {
        HpmIbiasWaitCntW::new(self, 10)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dblr_loop_filter_reset_voltage(
        &mut self,
    ) -> DblrLoopFilterResetVoltageW<MiscOtpData1Spec> {
        DblrLoopFilterResetVoltageW::new(self, 20)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lp_buf_itrim(&mut self) -> LpBufItrimW<MiscOtpData1Spec> {
        LpBufItrimW::new(self, 22)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hp_buf_itrim(&mut self) -> HpBufItrimW<MiscOtpData1Spec> {
        HpBufItrimW::new(self, 24)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn peak_det_itrim(&mut self) -> PeakDetItrimW<MiscOtpData1Spec> {
        PeakDetItrimW::new(self, 27)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_otp_data_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_otp_data_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscOtpData1Spec;
impl crate::RegisterSpec for MiscOtpData1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_otp_data_1::R`](R) reader structure"]
impl crate::Readable for MiscOtpData1Spec {}
#[doc = "`write(|w| ..)` method takes [`misc_otp_data_1::W`](W) writer structure"]
impl crate::Writable for MiscOtpData1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC_OTP_DATA_1 to value 0xe084_03f8"]
impl crate::Resettable for MiscOtpData1Spec {
    const RESET_VALUE: u32 = 0xe084_03f8;
}

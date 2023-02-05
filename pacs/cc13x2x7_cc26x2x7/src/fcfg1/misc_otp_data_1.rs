#[doc = "Register `MISC_OTP_DATA_1` reader"]
pub struct R(crate::R<MISC_OTP_DATA_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_OTP_DATA_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_OTP_DATA_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_OTP_DATA_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_OTP_DATA_1` writer"]
pub struct W(crate::W<MISC_OTP_DATA_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_OTP_DATA_1_SPEC>;
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
impl From<crate::W<MISC_OTP_DATA_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_OTP_DATA_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDAC_STEP` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type IDAC_STEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDAC_STEP` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type IDAC_STEP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_OTP_DATA_1_SPEC, u8, u8, 4, O>;
#[doc = "Field `LPM_IBIAS_WAIT_CNT` reader - 9:4\\]
Internal. Only to be used through TI provided API."]
pub type LPM_IBIAS_WAIT_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPM_IBIAS_WAIT_CNT` writer - 9:4\\]
Internal. Only to be used through TI provided API."]
pub type LPM_IBIAS_WAIT_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_OTP_DATA_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `HPM_IBIAS_WAIT_CNT` reader - 19:10\\]
Internal. Only to be used through TI provided API."]
pub type HPM_IBIAS_WAIT_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HPM_IBIAS_WAIT_CNT` writer - 19:10\\]
Internal. Only to be used through TI provided API."]
pub type HPM_IBIAS_WAIT_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_OTP_DATA_1_SPEC, u16, u16, 10, O>;
#[doc = "Field `DBLR_LOOP_FILTER_RESET_VOLTAGE` reader - 21:20\\]
Internal. Only to be used through TI provided API."]
pub type DBLR_LOOP_FILTER_RESET_VOLTAGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBLR_LOOP_FILTER_RESET_VOLTAGE` writer - 21:20\\]
Internal. Only to be used through TI provided API."]
pub type DBLR_LOOP_FILTER_RESET_VOLTAGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_OTP_DATA_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `LP_BUF_ITRIM` reader - 23:22\\]
Internal. Only to be used through TI provided API."]
pub type LP_BUF_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LP_BUF_ITRIM` writer - 23:22\\]
Internal. Only to be used through TI provided API."]
pub type LP_BUF_ITRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_OTP_DATA_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `HP_BUF_ITRIM` reader - 26:24\\]
Internal. Only to be used through TI provided API."]
pub type HP_BUF_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HP_BUF_ITRIM` writer - 26:24\\]
Internal. Only to be used through TI provided API."]
pub type HP_BUF_ITRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_OTP_DATA_1_SPEC, u8, u8, 3, O>;
#[doc = "Field `PEAK_DET_ITRIM` reader - 28:27\\]
Internal. Only to be used through TI provided API."]
pub type PEAK_DET_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PEAK_DET_ITRIM` writer - 28:27\\]
Internal. Only to be used through TI provided API."]
pub type PEAK_DET_ITRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_OTP_DATA_1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn idac_step(&self) -> IDAC_STEP_R {
        IDAC_STEP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - 9:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt(&self) -> LPM_IBIAS_WAIT_CNT_R {
        LPM_IBIAS_WAIT_CNT_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:19 - 19:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpm_ibias_wait_cnt(&self) -> HPM_IBIAS_WAIT_CNT_R {
        HPM_IBIAS_WAIT_CNT_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dblr_loop_filter_reset_voltage(&self) -> DBLR_LOOP_FILTER_RESET_VOLTAGE_R {
        DBLR_LOOP_FILTER_RESET_VOLTAGE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lp_buf_itrim(&self) -> LP_BUF_ITRIM_R {
        LP_BUF_ITRIM_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hp_buf_itrim(&self) -> HP_BUF_ITRIM_R {
        HP_BUF_ITRIM_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn peak_det_itrim(&self) -> PEAK_DET_ITRIM_R {
        PEAK_DET_ITRIM_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn idac_step(&mut self) -> IDAC_STEP_W<0> {
        IDAC_STEP_W::new(self)
    }
    #[doc = "Bits 4:9 - 9:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_ibias_wait_cnt(&mut self) -> LPM_IBIAS_WAIT_CNT_W<4> {
        LPM_IBIAS_WAIT_CNT_W::new(self)
    }
    #[doc = "Bits 10:19 - 19:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hpm_ibias_wait_cnt(&mut self) -> HPM_IBIAS_WAIT_CNT_W<10> {
        HPM_IBIAS_WAIT_CNT_W::new(self)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dblr_loop_filter_reset_voltage(&mut self) -> DBLR_LOOP_FILTER_RESET_VOLTAGE_W<20> {
        DBLR_LOOP_FILTER_RESET_VOLTAGE_W::new(self)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lp_buf_itrim(&mut self) -> LP_BUF_ITRIM_W<22> {
        LP_BUF_ITRIM_W::new(self)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hp_buf_itrim(&mut self) -> HP_BUF_ITRIM_W<24> {
        HP_BUF_ITRIM_W::new(self)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn peak_det_itrim(&mut self) -> PEAK_DET_ITRIM_W<27> {
        PEAK_DET_ITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_otp_data_1](index.html) module"]
pub struct MISC_OTP_DATA_1_SPEC;
impl crate::RegisterSpec for MISC_OTP_DATA_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_otp_data_1::R](R) reader structure"]
impl crate::Readable for MISC_OTP_DATA_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_otp_data_1::W](W) writer structure"]
impl crate::Writable for MISC_OTP_DATA_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC_OTP_DATA_1 to value 0xe084_03f8"]
impl crate::Resettable for MISC_OTP_DATA_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xe084_03f8;
}

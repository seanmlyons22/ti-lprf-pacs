#[doc = "Register `STAT2` reader"]
pub struct R(crate::R<STAT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT2` writer"]
pub struct W(crate::W<STAT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT2_SPEC>;
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
impl From<crate::W<STAT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSC_HF_RF_FREQGOOD` reader - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
pub type XOSC_HF_RF_FREQGOOD_R = crate::BitReader<bool>;
#[doc = "Field `XOSC_HF_RF_FREQGOOD` writer - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
pub type XOSC_HF_RF_FREQGOOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT2_SPEC, bool, O>;
#[doc = "Field `XOSC_HF_FREQGOOD` reader - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
pub type XOSC_HF_FREQGOOD_R = crate::BitReader<bool>;
#[doc = "Field `XOSC_HF_FREQGOOD` writer - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
pub type XOSC_HF_FREQGOOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT2_SPEC, bool, O>;
#[doc = "Field `XOSC_HF_AMPGOOD` reader - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
pub type XOSC_HF_AMPGOOD_R = crate::BitReader<bool>;
#[doc = "Field `XOSC_HF_AMPGOOD` writer - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
pub type XOSC_HF_AMPGOOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT2_SPEC, bool, O>;
#[doc = "Field `AMPCOMP_REQ` reader - 3:3\\]
ampcomp_req"]
pub type AMPCOMP_REQ_R = crate::BitReader<bool>;
#[doc = "Field `AMPCOMP_REQ` writer - 3:3\\]
ampcomp_req"]
pub type AMPCOMP_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT2_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED4` writer - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT2_SPEC, u8, u8, 8, O>;
#[doc = "Field `RAMPSTATE` reader - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
pub type RAMPSTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAMPSTATE` writer - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
pub type RAMPSTATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT2_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED16` reader - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED16` writer - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT2_SPEC, u8, u8, 7, O>;
#[doc = "Field `HPM_RAMP3_THMET` reader - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
pub type HPM_RAMP3_THMET_R = crate::BitReader<bool>;
#[doc = "Field `HPM_RAMP3_THMET` writer - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
pub type HPM_RAMP3_THMET_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT2_SPEC, bool, O>;
#[doc = "Field `HPM_RAMP2_THMET` reader - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
pub type HPM_RAMP2_THMET_R = crate::BitReader<bool>;
#[doc = "Field `HPM_RAMP2_THMET` writer - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
pub type HPM_RAMP2_THMET_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT2_SPEC, bool, O>;
#[doc = "Field `HPM_RAMP1_THMET` reader - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
pub type HPM_RAMP1_THMET_R = crate::BitReader<bool>;
#[doc = "Field `HPM_RAMP1_THMET` writer - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
pub type HPM_RAMP1_THMET_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT2_SPEC, bool, O>;
#[doc = "Field `ADC_DCBIAS` reader - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
pub type ADC_DCBIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_DCBIAS` writer - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
pub type ADC_DCBIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT2_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
    #[inline(always)]
    pub fn xosc_hf_rf_freqgood(&self) -> XOSC_HF_RF_FREQGOOD_R {
        XOSC_HF_RF_FREQGOOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
    #[inline(always)]
    pub fn xosc_hf_freqgood(&self) -> XOSC_HF_FREQGOOD_R {
        XOSC_HF_FREQGOOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
    #[inline(always)]
    pub fn xosc_hf_ampgood(&self) -> XOSC_HF_AMPGOOD_R {
        XOSC_HF_AMPGOOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
ampcomp_req"]
    #[inline(always)]
    pub fn ampcomp_req(&self) -> AMPCOMP_REQ_R {
        AMPCOMP_REQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
    #[inline(always)]
    pub fn rampstate(&self) -> RAMPSTATE_R {
        RAMPSTATE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
    #[inline(always)]
    pub fn hpm_ramp3_thmet(&self) -> HPM_RAMP3_THMET_R {
        HPM_RAMP3_THMET_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
    #[inline(always)]
    pub fn hpm_ramp2_thmet(&self) -> HPM_RAMP2_THMET_R {
        HPM_RAMP2_THMET_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
    #[inline(always)]
    pub fn hpm_ramp1_thmet(&self) -> HPM_RAMP1_THMET_R {
        HPM_RAMP1_THMET_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
    #[inline(always)]
    pub fn adc_dcbias(&self) -> ADC_DCBIAS_R {
        ADC_DCBIAS_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_rf_freqgood(&mut self) -> XOSC_HF_RF_FREQGOOD_W<0> {
        XOSC_HF_RF_FREQGOOD_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_freqgood(&mut self) -> XOSC_HF_FREQGOOD_W<1> {
        XOSC_HF_FREQGOOD_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_ampgood(&mut self) -> XOSC_HF_AMPGOOD_W<2> {
        XOSC_HF_AMPGOOD_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
ampcomp_req"]
    #[inline(always)]
    #[must_use]
    pub fn ampcomp_req(&mut self) -> AMPCOMP_REQ_W<3> {
        AMPCOMP_REQ_W::new(self)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
    #[inline(always)]
    #[must_use]
    pub fn rampstate(&mut self) -> RAMPSTATE_W<12> {
        RAMPSTATE_W::new(self)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
    #[inline(always)]
    #[must_use]
    pub fn hpm_ramp3_thmet(&mut self) -> HPM_RAMP3_THMET_W<23> {
        HPM_RAMP3_THMET_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
    #[inline(always)]
    #[must_use]
    pub fn hpm_ramp2_thmet(&mut self) -> HPM_RAMP2_THMET_W<24> {
        HPM_RAMP2_THMET_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
    #[inline(always)]
    #[must_use]
    pub fn hpm_ramp1_thmet(&mut self) -> HPM_RAMP1_THMET_W<25> {
        HPM_RAMP1_THMET_W::new(self)
    }
    #[doc = "Bits 26:31 - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcbias(&mut self) -> ADC_DCBIAS_W<26> {
        ADC_DCBIAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status 2 This register contains status signals from AMPCOMP FSM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat2](index.html) module"]
pub struct STAT2_SPEC;
impl crate::RegisterSpec for STAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat2::R](R) reader structure"]
impl crate::Readable for STAT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat2::W](W) writer structure"]
impl crate::Writable for STAT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT2 to value 0"]
impl crate::Resettable for STAT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

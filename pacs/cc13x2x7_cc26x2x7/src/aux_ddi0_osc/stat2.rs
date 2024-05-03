#[doc = "Register `STAT2` reader"]
pub type R = crate::R<Stat2Spec>;
#[doc = "Register `STAT2` writer"]
pub type W = crate::W<Stat2Spec>;
#[doc = "Field `XOSC_HF_RF_FREQGOOD` reader - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
pub type XoscHfRfFreqgoodR = crate::BitReader;
#[doc = "Field `XOSC_HF_RF_FREQGOOD` writer - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
pub type XoscHfRfFreqgoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSC_HF_FREQGOOD` reader - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
pub type XoscHfFreqgoodR = crate::BitReader;
#[doc = "Field `XOSC_HF_FREQGOOD` writer - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
pub type XoscHfFreqgoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSC_HF_AMPGOOD` reader - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
pub type XoscHfAmpgoodR = crate::BitReader;
#[doc = "Field `XOSC_HF_AMPGOOD` writer - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
pub type XoscHfAmpgoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMPCOMP_REQ` reader - 3:3\\]
ampcomp_req"]
pub type AmpcompReqR = crate::BitReader;
#[doc = "Field `AMPCOMP_REQ` writer - 3:3\\]
ampcomp_req"]
pub type AmpcompReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RAMPSTATE` reader - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
pub type RampstateR = crate::FieldReader;
#[doc = "Field `RAMPSTATE` writer - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
pub type RampstateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED16` reader - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader;
#[doc = "Field `RESERVED16` writer - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HPM_RAMP3_THMET` reader - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
pub type HpmRamp3ThmetR = crate::BitReader;
#[doc = "Field `HPM_RAMP3_THMET` writer - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
pub type HpmRamp3ThmetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPM_RAMP2_THMET` reader - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
pub type HpmRamp2ThmetR = crate::BitReader;
#[doc = "Field `HPM_RAMP2_THMET` writer - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
pub type HpmRamp2ThmetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPM_RAMP1_THMET` reader - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
pub type HpmRamp1ThmetR = crate::BitReader;
#[doc = "Field `HPM_RAMP1_THMET` writer - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
pub type HpmRamp1ThmetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_DCBIAS` reader - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
pub type AdcDcbiasR = crate::FieldReader;
#[doc = "Field `ADC_DCBIAS` writer - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
pub type AdcDcbiasW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
    #[inline(always)]
    pub fn xosc_hf_rf_freqgood(&self) -> XoscHfRfFreqgoodR {
        XoscHfRfFreqgoodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
    #[inline(always)]
    pub fn xosc_hf_freqgood(&self) -> XoscHfFreqgoodR {
        XoscHfFreqgoodR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
    #[inline(always)]
    pub fn xosc_hf_ampgood(&self) -> XoscHfAmpgoodR {
        XoscHfAmpgoodR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
ampcomp_req"]
    #[inline(always)]
    pub fn ampcomp_req(&self) -> AmpcompReqR {
        AmpcompReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
    #[inline(always)]
    pub fn rampstate(&self) -> RampstateR {
        RampstateR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
    #[inline(always)]
    pub fn hpm_ramp3_thmet(&self) -> HpmRamp3ThmetR {
        HpmRamp3ThmetR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
    #[inline(always)]
    pub fn hpm_ramp2_thmet(&self) -> HpmRamp2ThmetR {
        HpmRamp2ThmetR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
    #[inline(always)]
    pub fn hpm_ramp1_thmet(&self) -> HpmRamp1ThmetR {
        HpmRamp1ThmetR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
    #[inline(always)]
    pub fn adc_dcbias(&self) -> AdcDcbiasR {
        AdcDcbiasR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_rf_freqgood(&mut self) -> XoscHfRfFreqgoodW<Stat2Spec> {
        XoscHfRfFreqgoodW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_freqgood(&mut self) -> XoscHfFreqgoodW<Stat2Spec> {
        XoscHfFreqgoodW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_ampgood(&mut self) -> XoscHfAmpgoodW<Stat2Spec> {
        XoscHfAmpgoodW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
ampcomp_req"]
    #[inline(always)]
    #[must_use]
    pub fn ampcomp_req(&mut self) -> AmpcompReqW<Stat2Spec> {
        AmpcompReqW::new(self, 3)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<Stat2Spec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bits 12:15 - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
    #[inline(always)]
    #[must_use]
    pub fn rampstate(&mut self) -> RampstateW<Stat2Spec> {
        RampstateW::new(self, 12)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Stat2Spec> {
        Reserved16W::new(self, 16)
    }
    #[doc = "Bit 23 - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
    #[inline(always)]
    #[must_use]
    pub fn hpm_ramp3_thmet(&mut self) -> HpmRamp3ThmetW<Stat2Spec> {
        HpmRamp3ThmetW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
    #[inline(always)]
    #[must_use]
    pub fn hpm_ramp2_thmet(&mut self) -> HpmRamp2ThmetW<Stat2Spec> {
        HpmRamp2ThmetW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
    #[inline(always)]
    #[must_use]
    pub fn hpm_ramp1_thmet(&mut self) -> HpmRamp1ThmetW<Stat2Spec> {
        HpmRamp1ThmetW::new(self, 25)
    }
    #[doc = "Bits 26:31 - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcbias(&mut self) -> AdcDcbiasW<Stat2Spec> {
        AdcDcbiasW::new(self, 26)
    }
}
#[doc = "Status 2 This register contains status signals from AMPCOMP FSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stat2Spec;
impl crate::RegisterSpec for Stat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat2::R`](R) reader structure"]
impl crate::Readable for Stat2Spec {}
#[doc = "`write(|w| ..)` method takes [`stat2::W`](W) writer structure"]
impl crate::Writable for Stat2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT2 to value 0"]
impl crate::Resettable for Stat2Spec {
    const RESET_VALUE: u32 = 0;
}

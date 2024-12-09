#[doc = "Register `OSC_CONF` reader"]
pub type R = crate::R<OscConfSpec>;
#[doc = "Register `OSC_CONF` writer"]
pub type W = crate::W<OscConfSpec>;
#[doc = "Field `HPOSC_DIV3_BYPASS` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type HposcDiv3BypassR = crate::BitReader;
#[doc = "Field `HPOSC_SERIES_CAP` reader - 2:1\\]
Internal. Only to be used through TI provided API."]
pub type HposcSeriesCapR = crate::FieldReader;
#[doc = "Field `RESERVED2` reader - 4:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `HPOSC_BIAS_RECHARGE_DELAY` reader - 6:5\\]
Internal. Only to be used through TI provided API."]
pub type HposcBiasRechargeDelayR = crate::FieldReader;
#[doc = "Field `HPOSC_FILTER_EN` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type HposcFilterEnR = crate::BitReader;
#[doc = "Field `HPOSC_BIAS_RES_SET` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type HposcBiasResSetR = crate::FieldReader;
#[doc = "Field `HPOSC_CURRMIRR_RATIO` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type HposcCurrmirrRatioR = crate::FieldReader;
#[doc = "Field `HPOSC_BIAS_HOLD_MODE_EN` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type HposcBiasHoldModeEnR = crate::BitReader;
#[doc = "Field `HPOSC_OPTION` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type HposcOptionR = crate::BitReader;
#[doc = "Field `XOSC_OPTION` reader - 18:18\\]
0: XOSC_HF unavailable (may not be bonded out) 1: XOSC_HF available (default)"]
pub type XoscOptionR = crate::BitReader;
#[doc = "Field `XOSC_HF_FAST_START` reader - 20:19\\]
Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START."]
pub type XoscHfFastStartR = crate::FieldReader;
#[doc = "Field `XOSCLF_CMIRRWR_RATIO` reader - 24:21\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO."]
pub type XosclfCmirrwrRatioR = crate::FieldReader;
#[doc = "Field `XOSCLF_REGULATOR_TRIM` reader - 26:25\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM."]
pub type XosclfRegulatorTrimR = crate::FieldReader;
#[doc = "Field `ATESTLF_RCOSCLF_IBIAS_TRIM` reader - 27:27\\]
Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM."]
pub type AtestlfRcosclfIbiasTrimR = crate::BitReader;
#[doc = "Field `ADC_SH_MODE_EN` reader - 28:28\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN."]
pub type AdcShModeEnR = crate::BitReader;
#[doc = "Field `ADC_SH_VBUF_EN` reader - 29:29\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN."]
pub type AdcShVbufEnR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_div3_bypass(&self) -> HposcDiv3BypassR {
        HposcDiv3BypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_series_cap(&self) -> HposcSeriesCapR {
        HposcSeriesCapR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_bias_recharge_delay(&self) -> HposcBiasRechargeDelayR {
        HposcBiasRechargeDelayR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_filter_en(&self) -> HposcFilterEnR {
        HposcFilterEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_bias_res_set(&self) -> HposcBiasResSetR {
        HposcBiasResSetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_currmirr_ratio(&self) -> HposcCurrmirrRatioR {
        HposcCurrmirrRatioR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_bias_hold_mode_en(&self) -> HposcBiasHoldModeEnR {
        HposcBiasHoldModeEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_option(&self) -> HposcOptionR {
        HposcOptionR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: XOSC_HF unavailable (may not be bonded out) 1: XOSC_HF available (default)"]
    #[inline(always)]
    pub fn xosc_option(&self) -> XoscOptionR {
        XoscOptionR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START."]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&self) -> XoscHfFastStartR {
        XoscHfFastStartR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO."]
    #[inline(always)]
    pub fn xosclf_cmirrwr_ratio(&self) -> XosclfCmirrwrRatioR {
        XosclfCmirrwrRatioR::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM."]
    #[inline(always)]
    pub fn xosclf_regulator_trim(&self) -> XosclfRegulatorTrimR {
        XosclfRegulatorTrimR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM."]
    #[inline(always)]
    pub fn atestlf_rcosclf_ibias_trim(&self) -> AtestlfRcosclfIbiasTrimR {
        AtestlfRcosclfIbiasTrimR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN."]
    #[inline(always)]
    pub fn adc_sh_mode_en(&self) -> AdcShModeEnR {
        AdcShModeEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN."]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&self) -> AdcShVbufEnR {
        AdcShVbufEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {}
#[doc = "OSC Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscConfSpec;
impl crate::RegisterSpec for OscConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc_conf::R`](R) reader structure"]
impl crate::Readable for OscConfSpec {}
#[doc = "`write(|w| ..)` method takes [`osc_conf::W`](W) writer structure"]
impl crate::Writable for OscConfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSC_CONF to value 0xf009_0000"]
impl crate::Resettable for OscConfSpec {
    const RESET_VALUE: u32 = 0xf009_0000;
}

#[doc = "Register `OSC_CONF` reader"]
pub struct R(crate::R<OSC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_CONF` writer"]
pub struct W(crate::W<OSC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_CONF_SPEC>;
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
impl From<crate::W<OSC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPOSC_DIV3_BYPASS` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_DIV3_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `HPOSC_DIV3_BYPASS` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_DIV3_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_CONF_SPEC, bool, O>;
#[doc = "Field `HPOSC_SERIES_CAP` reader - 2:1\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_SERIES_CAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPOSC_SERIES_CAP` writer - 2:1\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_SERIES_CAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED2` reader - 4:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 4:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSC_CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `HPOSC_BIAS_RECHARGE_DELAY` reader - 6:5\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_BIAS_RECHARGE_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPOSC_BIAS_RECHARGE_DELAY` writer - 6:5\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_BIAS_RECHARGE_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `HPOSC_FILTER_EN` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_FILTER_EN_R = crate::BitReader<bool>;
#[doc = "Field `HPOSC_FILTER_EN` writer - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_CONF_SPEC, bool, O>;
#[doc = "Field `HPOSC_BIAS_RES_SET` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_BIAS_RES_SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPOSC_BIAS_RES_SET` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_BIAS_RES_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONF_SPEC, u8, u8, 4, O>;
#[doc = "Field `HPOSC_CURRMIRR_RATIO` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_CURRMIRR_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPOSC_CURRMIRR_RATIO` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_CURRMIRR_RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONF_SPEC, u8, u8, 4, O>;
#[doc = "Field `HPOSC_BIAS_HOLD_MODE_EN` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_BIAS_HOLD_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `HPOSC_BIAS_HOLD_MODE_EN` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_BIAS_HOLD_MODE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OSC_CONF_SPEC, bool, O>;
#[doc = "Field `HPOSC_OPTION` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_OPTION_R = crate::BitReader<bool>;
#[doc = "Field `HPOSC_OPTION` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type HPOSC_OPTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_CONF_SPEC, bool, O>;
#[doc = "Field `XOSC_OPTION` reader - 18:18\\]
0: XOSC_HF unavailable (may not be bonded out) 1: XOSC_HF available (default)"]
pub type XOSC_OPTION_R = crate::BitReader<bool>;
#[doc = "Field `XOSC_OPTION` writer - 18:18\\]
0: XOSC_HF unavailable (may not be bonded out) 1: XOSC_HF available (default)"]
pub type XOSC_OPTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_CONF_SPEC, bool, O>;
#[doc = "Field `XOSC_HF_FAST_START` reader - 20:19\\]
Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START."]
pub type XOSC_HF_FAST_START_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XOSC_HF_FAST_START` writer - 20:19\\]
Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START."]
pub type XOSC_HF_FAST_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `XOSCLF_CMIRRWR_RATIO` reader - 24:21\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO."]
pub type XOSCLF_CMIRRWR_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XOSCLF_CMIRRWR_RATIO` writer - 24:21\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO."]
pub type XOSCLF_CMIRRWR_RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONF_SPEC, u8, u8, 4, O>;
#[doc = "Field `XOSCLF_REGULATOR_TRIM` reader - 26:25\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM."]
pub type XOSCLF_REGULATOR_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XOSCLF_REGULATOR_TRIM` writer - 26:25\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM."]
pub type XOSCLF_REGULATOR_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `ATESTLF_RCOSCLF_IBIAS_TRIM` reader - 27:27\\]
Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM."]
pub type ATESTLF_RCOSCLF_IBIAS_TRIM_R = crate::BitReader<bool>;
#[doc = "Field `ATESTLF_RCOSCLF_IBIAS_TRIM` writer - 27:27\\]
Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM."]
pub type ATESTLF_RCOSCLF_IBIAS_TRIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OSC_CONF_SPEC, bool, O>;
#[doc = "Field `ADC_SH_MODE_EN` reader - 28:28\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN."]
pub type ADC_SH_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SH_MODE_EN` writer - 28:28\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN."]
pub type ADC_SH_MODE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_CONF_SPEC, bool, O>;
#[doc = "Field `ADC_SH_VBUF_EN` reader - 29:29\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN."]
pub type ADC_SH_VBUF_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SH_VBUF_EN` writer - 29:29\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN."]
pub type ADC_SH_VBUF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_CONF_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSC_CONF_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_div3_bypass(&self) -> HPOSC_DIV3_BYPASS_R {
        HPOSC_DIV3_BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_series_cap(&self) -> HPOSC_SERIES_CAP_R {
        HPOSC_SERIES_CAP_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_bias_recharge_delay(&self) -> HPOSC_BIAS_RECHARGE_DELAY_R {
        HPOSC_BIAS_RECHARGE_DELAY_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_filter_en(&self) -> HPOSC_FILTER_EN_R {
        HPOSC_FILTER_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_bias_res_set(&self) -> HPOSC_BIAS_RES_SET_R {
        HPOSC_BIAS_RES_SET_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_currmirr_ratio(&self) -> HPOSC_CURRMIRR_RATIO_R {
        HPOSC_CURRMIRR_RATIO_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_bias_hold_mode_en(&self) -> HPOSC_BIAS_HOLD_MODE_EN_R {
        HPOSC_BIAS_HOLD_MODE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_option(&self) -> HPOSC_OPTION_R {
        HPOSC_OPTION_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: XOSC_HF unavailable (may not be bonded out) 1: XOSC_HF available (default)"]
    #[inline(always)]
    pub fn xosc_option(&self) -> XOSC_OPTION_R {
        XOSC_OPTION_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START."]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&self) -> XOSC_HF_FAST_START_R {
        XOSC_HF_FAST_START_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO."]
    #[inline(always)]
    pub fn xosclf_cmirrwr_ratio(&self) -> XOSCLF_CMIRRWR_RATIO_R {
        XOSCLF_CMIRRWR_RATIO_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM."]
    #[inline(always)]
    pub fn xosclf_regulator_trim(&self) -> XOSCLF_REGULATOR_TRIM_R {
        XOSCLF_REGULATOR_TRIM_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM."]
    #[inline(always)]
    pub fn atestlf_rcosclf_ibias_trim(&self) -> ATESTLF_RCOSCLF_IBIAS_TRIM_R {
        ATESTLF_RCOSCLF_IBIAS_TRIM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN."]
    #[inline(always)]
    pub fn adc_sh_mode_en(&self) -> ADC_SH_MODE_EN_R {
        ADC_SH_MODE_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN."]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&self) -> ADC_SH_VBUF_EN_R {
        ADC_SH_VBUF_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_div3_bypass(&mut self) -> HPOSC_DIV3_BYPASS_W<0> {
        HPOSC_DIV3_BYPASS_W::new(self)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_series_cap(&mut self) -> HPOSC_SERIES_CAP_W<1> {
        HPOSC_SERIES_CAP_W::new(self)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<3> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_bias_recharge_delay(&mut self) -> HPOSC_BIAS_RECHARGE_DELAY_W<5> {
        HPOSC_BIAS_RECHARGE_DELAY_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_filter_en(&mut self) -> HPOSC_FILTER_EN_W<7> {
        HPOSC_FILTER_EN_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_bias_res_set(&mut self) -> HPOSC_BIAS_RES_SET_W<8> {
        HPOSC_BIAS_RES_SET_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_currmirr_ratio(&mut self) -> HPOSC_CURRMIRR_RATIO_W<12> {
        HPOSC_CURRMIRR_RATIO_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_bias_hold_mode_en(&mut self) -> HPOSC_BIAS_HOLD_MODE_EN_W<16> {
        HPOSC_BIAS_HOLD_MODE_EN_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_option(&mut self) -> HPOSC_OPTION_W<17> {
        HPOSC_OPTION_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
0: XOSC_HF unavailable (may not be bonded out) 1: XOSC_HF available (default)"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_option(&mut self) -> XOSC_OPTION_W<18> {
        XOSC_OPTION_W::new(self)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_fast_start(&mut self) -> XOSC_HF_FAST_START_W<19> {
        XOSC_HF_FAST_START_W::new(self)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO."]
    #[inline(always)]
    #[must_use]
    pub fn xosclf_cmirrwr_ratio(&mut self) -> XOSCLF_CMIRRWR_RATIO_W<21> {
        XOSCLF_CMIRRWR_RATIO_W::new(self)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM."]
    #[inline(always)]
    #[must_use]
    pub fn xosclf_regulator_trim(&mut self) -> XOSCLF_REGULATOR_TRIM_W<25> {
        XOSCLF_REGULATOR_TRIM_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM."]
    #[inline(always)]
    #[must_use]
    pub fn atestlf_rcosclf_ibias_trim(&mut self) -> ATESTLF_RCOSCLF_IBIAS_TRIM_W<27> {
        ATESTLF_RCOSCLF_IBIAS_TRIM_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN."]
    #[inline(always)]
    #[must_use]
    pub fn adc_sh_mode_en(&mut self) -> ADC_SH_MODE_EN_W<28> {
        ADC_SH_MODE_EN_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN."]
    #[inline(always)]
    #[must_use]
    pub fn adc_sh_vbuf_en(&mut self) -> ADC_SH_VBUF_EN_W<29> {
        ADC_SH_VBUF_EN_W::new(self)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<30> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_conf](index.html) module"]
pub struct OSC_CONF_SPEC;
impl crate::RegisterSpec for OSC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_conf::R](R) reader structure"]
impl crate::Readable for OSC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_conf::W](W) writer structure"]
impl crate::Writable for OSC_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC_CONF to value 0xf009_0000"]
impl crate::Resettable for OSC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0xf009_0000;
}

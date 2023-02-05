#[doc = "Register `OSCRIS` reader"]
pub struct R(crate::R<OSCRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCRIS` writer"]
pub struct W(crate::W<OSCRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCRIS_SPEC>;
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
impl From<crate::W<OSCRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCOSCHFRIS` reader - 0:0\\]
The RCOSCHFRIS interrupt indicates when the RCOSC_HF oscillator has been qualified for use as a clock source When the RCOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The RCOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: RCOSC_HF has not been qualified 1: RCOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCHFC"]
pub type RCOSCHFRIS_R = crate::BitReader<bool>;
#[doc = "Field `RCOSCHFRIS` writer - 0:0\\]
The RCOSCHFRIS interrupt indicates when the RCOSC_HF oscillator has been qualified for use as a clock source When the RCOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The RCOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: RCOSC_HF has not been qualified 1: RCOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCHFC"]
pub type RCOSCHFRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCRIS_SPEC, bool, O>;
#[doc = "Field `XOSCHFRIS` reader - 1:1\\]
The XOSCHFRIS interrupt indicates when the XOSC_HF oscillator has been qualified for use as a clock source. XOSCHFRIS is also used in TCXO mode (when DDI_0_OSC:XOSCHFCTL.TCXO_MODE is 1). When the XOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The XOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: XOSC_HF has not been qualified 1: XOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCHFC"]
pub type XOSCHFRIS_R = crate::BitReader<bool>;
#[doc = "Field `XOSCHFRIS` writer - 1:1\\]
The XOSCHFRIS interrupt indicates when the XOSC_HF oscillator has been qualified for use as a clock source. XOSCHFRIS is also used in TCXO mode (when DDI_0_OSC:XOSCHFCTL.TCXO_MODE is 1). When the XOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The XOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: XOSC_HF has not been qualified 1: XOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCHFC"]
pub type XOSCHFRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCRIS_SPEC, bool, O>;
#[doc = "Field `RCOSCLFRIS` reader - 2:2\\]
The RCOSCLFRIS interrupt indicates when the output of the RCOSC_LF oscillator has been qualified with respect to frequency. The RCOSCLFRIS interrupt status goes high when the RCOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, RCOSCLFRIS interrupt status remains high, and further qualification is turned off until the RCOSC_LF oscillator is disabled. RCOSCLFRIS interrupt status will go low only at initial power-on, or after the RCOSC_LF oscillator has been disabled when being deselected as a clock source. 0: RCOSCLF has not been qualified 1: RCOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCLFC"]
pub type RCOSCLFRIS_R = crate::BitReader<bool>;
#[doc = "Field `RCOSCLFRIS` writer - 2:2\\]
The RCOSCLFRIS interrupt indicates when the output of the RCOSC_LF oscillator has been qualified with respect to frequency. The RCOSCLFRIS interrupt status goes high when the RCOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, RCOSCLFRIS interrupt status remains high, and further qualification is turned off until the RCOSC_LF oscillator is disabled. RCOSCLFRIS interrupt status will go low only at initial power-on, or after the RCOSC_LF oscillator has been disabled when being deselected as a clock source. 0: RCOSCLF has not been qualified 1: RCOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCLFC"]
pub type RCOSCLFRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCRIS_SPEC, bool, O>;
#[doc = "Field `RCOSCDLFRIS` reader - 3:3\\]
The RCOSCDLFRIS interrupt indicates when the RCOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When RCOSCDLFRIS is high, RCOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have RCOSC_HF selected as clock source, the RCOSC_HF source is automatically disabled and the RCOSCDLFRIS interrupt status will go low. If the SCLK_LF or ACLK_REF source is changed from RCOSC_HF derived to XOSC_HF derived low-frequency clock and the new source has not been qualified, then the clock will remain running on the original source. The RCOSCDLFRIS interrupt will then remain high. 0: RCOSCDLF has not been qualified 1: RCOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCDLFC"]
pub type RCOSCDLFRIS_R = crate::BitReader<bool>;
#[doc = "Field `RCOSCDLFRIS` writer - 3:3\\]
The RCOSCDLFRIS interrupt indicates when the RCOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When RCOSCDLFRIS is high, RCOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have RCOSC_HF selected as clock source, the RCOSC_HF source is automatically disabled and the RCOSCDLFRIS interrupt status will go low. If the SCLK_LF or ACLK_REF source is changed from RCOSC_HF derived to XOSC_HF derived low-frequency clock and the new source has not been qualified, then the clock will remain running on the original source. The RCOSCDLFRIS interrupt will then remain high. 0: RCOSCDLF has not been qualified 1: RCOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCDLFC"]
pub type RCOSCDLFRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCRIS_SPEC, bool, O>;
#[doc = "Field `XOSCLFRIS` reader - 4:4\\]
The XOSCLFRIS interrupt indicates when the output of the XOSC_LF oscillator has been qualified with respect to frequency. The XOSCLFRIS interrupt status goes high when the XOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, XOSCLFRIS interrupt status remains high, and further qualification is turned off until the XOSC_LF oscillator is disabled. XOSCLFRIS interrupt status will go low only at initial power-on, or after the XOSC_LF oscillator has been disabled when being deselected as a clock source. 0: XOSCLF has not been qualified 1: XOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCLFC"]
pub type XOSCLFRIS_R = crate::BitReader<bool>;
#[doc = "Field `XOSCLFRIS` writer - 4:4\\]
The XOSCLFRIS interrupt indicates when the output of the XOSC_LF oscillator has been qualified with respect to frequency. The XOSCLFRIS interrupt status goes high when the XOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, XOSCLFRIS interrupt status remains high, and further qualification is turned off until the XOSC_LF oscillator is disabled. XOSCLFRIS interrupt status will go low only at initial power-on, or after the XOSC_LF oscillator has been disabled when being deselected as a clock source. 0: XOSCLF has not been qualified 1: XOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCLFC"]
pub type XOSCLFRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCRIS_SPEC, bool, O>;
#[doc = "Field `XOSCDLFRIS` reader - 5:5\\]
The XOSCDLFRIS interrupt indicates when the XOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When XOSCDLFRIS is high, XOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have XOSC_HF selected as clock source, the XOSC_HF source is automatically disabled and the XOSCDLFRIS interrupt status will go low. 0: XOSCDLF has not been qualified 1: XOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCDLFC"]
pub type XOSCDLFRIS_R = crate::BitReader<bool>;
#[doc = "Field `XOSCDLFRIS` writer - 5:5\\]
The XOSCDLFRIS interrupt indicates when the XOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When XOSCDLFRIS is high, XOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have XOSC_HF selected as clock source, the XOSC_HF source is automatically disabled and the XOSCDLFRIS interrupt status will go low. 0: XOSCDLF has not been qualified 1: XOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCDLFC"]
pub type XOSCDLFRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCRIS_SPEC, bool, O>;
#[doc = "Field `LFSRCDONERIS` reader - 6:6\\]
SCLK_LF source switch done. The DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL register field is used to request that the SCLK_LF source shall be changed. After an SCLK_LF clock source change is requested, the new source may need to be enabled and qualified before switching of clock source can be done. The interrupt LFRSRCDONERIS goes high to indicate that the SCLK_LF clock source switching has been performed. LFRSRCDONERIS will go low again when the next clock source change is requested by writing to DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL . 0: Indicates SCLK_LF source switch has not completed 1: Indicates SCLK_LF source switch has completed Interrupt is qualified regardless of OSCIMSC.LFSRCDONEIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.LFSRCDONEC"]
pub type LFSRCDONERIS_R = crate::BitReader<bool>;
#[doc = "Field `LFSRCDONERIS` writer - 6:6\\]
SCLK_LF source switch done. The DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL register field is used to request that the SCLK_LF source shall be changed. After an SCLK_LF clock source change is requested, the new source may need to be enabled and qualified before switching of clock source can be done. The interrupt LFRSRCDONERIS goes high to indicate that the SCLK_LF clock source switching has been performed. LFRSRCDONERIS will go low again when the next clock source change is requested by writing to DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL . 0: Indicates SCLK_LF source switch has not completed 1: Indicates SCLK_LF source switch has completed Interrupt is qualified regardless of OSCIMSC.LFSRCDONEIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.LFSRCDONEC"]
pub type LFSRCDONERIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCRIS_SPEC, bool, O>;
#[doc = "Field `HFSRCPENDRIS` reader - 7:7\\]
SCLK_HF source switch pending interrupt. After a write to DDI_0_OSC:CTL0.SCLK_HF_SRC_SEL leads to a SCLK_HF source change request, then the requested SCLK_HF source will be enabled and qualified. When the new source is ready to be used as a clock source, then the interrupt HSSRCPENDRIS will go high. When the Flash allows SCLK_HF source switching to take place after flash memory read access is disabled. At this time the actual SCLK_HF clock source switch will be performed, and the interrupt status HSSRCPENDRIS will go low. 0: Indicates SCLK_HF source is not ready to be switched 1: Indicates SCLK_HF source is ready to be switched Interrupt is qualified regardless of OSCIMSC.HFSRCPENDIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.HFSRCPENDC"]
pub type HFSRCPENDRIS_R = crate::BitReader<bool>;
#[doc = "Field `HFSRCPENDRIS` writer - 7:7\\]
SCLK_HF source switch pending interrupt. After a write to DDI_0_OSC:CTL0.SCLK_HF_SRC_SEL leads to a SCLK_HF source change request, then the requested SCLK_HF source will be enabled and qualified. When the new source is ready to be used as a clock source, then the interrupt HSSRCPENDRIS will go high. When the Flash allows SCLK_HF source switching to take place after flash memory read access is disabled. At this time the actual SCLK_HF clock source switch will be performed, and the interrupt status HSSRCPENDRIS will go low. 0: Indicates SCLK_HF source is not ready to be switched 1: Indicates SCLK_HF source is ready to be switched Interrupt is qualified regardless of OSCIMSC.HFSRCPENDIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.HFSRCPENDC"]
pub type HFSRCPENDRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCRIS_SPEC, bool, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCRIS_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
The RCOSCHFRIS interrupt indicates when the RCOSC_HF oscillator has been qualified for use as a clock source When the RCOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The RCOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: RCOSC_HF has not been qualified 1: RCOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCHFC"]
    #[inline(always)]
    pub fn rcoschfris(&self) -> RCOSCHFRIS_R {
        RCOSCHFRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
The XOSCHFRIS interrupt indicates when the XOSC_HF oscillator has been qualified for use as a clock source. XOSCHFRIS is also used in TCXO mode (when DDI_0_OSC:XOSCHFCTL.TCXO_MODE is 1). When the XOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The XOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: XOSC_HF has not been qualified 1: XOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCHFC"]
    #[inline(always)]
    pub fn xoschfris(&self) -> XOSCHFRIS_R {
        XOSCHFRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
The RCOSCLFRIS interrupt indicates when the output of the RCOSC_LF oscillator has been qualified with respect to frequency. The RCOSCLFRIS interrupt status goes high when the RCOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, RCOSCLFRIS interrupt status remains high, and further qualification is turned off until the RCOSC_LF oscillator is disabled. RCOSCLFRIS interrupt status will go low only at initial power-on, or after the RCOSC_LF oscillator has been disabled when being deselected as a clock source. 0: RCOSCLF has not been qualified 1: RCOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCLFC"]
    #[inline(always)]
    pub fn rcosclfris(&self) -> RCOSCLFRIS_R {
        RCOSCLFRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
The RCOSCDLFRIS interrupt indicates when the RCOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When RCOSCDLFRIS is high, RCOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have RCOSC_HF selected as clock source, the RCOSC_HF source is automatically disabled and the RCOSCDLFRIS interrupt status will go low. If the SCLK_LF or ACLK_REF source is changed from RCOSC_HF derived to XOSC_HF derived low-frequency clock and the new source has not been qualified, then the clock will remain running on the original source. The RCOSCDLFRIS interrupt will then remain high. 0: RCOSCDLF has not been qualified 1: RCOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCDLFC"]
    #[inline(always)]
    pub fn rcoscdlfris(&self) -> RCOSCDLFRIS_R {
        RCOSCDLFRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
The XOSCLFRIS interrupt indicates when the output of the XOSC_LF oscillator has been qualified with respect to frequency. The XOSCLFRIS interrupt status goes high when the XOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, XOSCLFRIS interrupt status remains high, and further qualification is turned off until the XOSC_LF oscillator is disabled. XOSCLFRIS interrupt status will go low only at initial power-on, or after the XOSC_LF oscillator has been disabled when being deselected as a clock source. 0: XOSCLF has not been qualified 1: XOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCLFC"]
    #[inline(always)]
    pub fn xosclfris(&self) -> XOSCLFRIS_R {
        XOSCLFRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
The XOSCDLFRIS interrupt indicates when the XOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When XOSCDLFRIS is high, XOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have XOSC_HF selected as clock source, the XOSC_HF source is automatically disabled and the XOSCDLFRIS interrupt status will go low. 0: XOSCDLF has not been qualified 1: XOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCDLFC"]
    #[inline(always)]
    pub fn xoscdlfris(&self) -> XOSCDLFRIS_R {
        XOSCDLFRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
SCLK_LF source switch done. The DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL register field is used to request that the SCLK_LF source shall be changed. After an SCLK_LF clock source change is requested, the new source may need to be enabled and qualified before switching of clock source can be done. The interrupt LFRSRCDONERIS goes high to indicate that the SCLK_LF clock source switching has been performed. LFRSRCDONERIS will go low again when the next clock source change is requested by writing to DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL . 0: Indicates SCLK_LF source switch has not completed 1: Indicates SCLK_LF source switch has completed Interrupt is qualified regardless of OSCIMSC.LFSRCDONEIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.LFSRCDONEC"]
    #[inline(always)]
    pub fn lfsrcdoneris(&self) -> LFSRCDONERIS_R {
        LFSRCDONERIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
SCLK_HF source switch pending interrupt. After a write to DDI_0_OSC:CTL0.SCLK_HF_SRC_SEL leads to a SCLK_HF source change request, then the requested SCLK_HF source will be enabled and qualified. When the new source is ready to be used as a clock source, then the interrupt HSSRCPENDRIS will go high. When the Flash allows SCLK_HF source switching to take place after flash memory read access is disabled. At this time the actual SCLK_HF clock source switch will be performed, and the interrupt status HSSRCPENDRIS will go low. 0: Indicates SCLK_HF source is not ready to be switched 1: Indicates SCLK_HF source is ready to be switched Interrupt is qualified regardless of OSCIMSC.HFSRCPENDIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.HFSRCPENDC"]
    #[inline(always)]
    pub fn hfsrcpendris(&self) -> HFSRCPENDRIS_R {
        HFSRCPENDRIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
The RCOSCHFRIS interrupt indicates when the RCOSC_HF oscillator has been qualified for use as a clock source When the RCOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The RCOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: RCOSC_HF has not been qualified 1: RCOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCHFC"]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfris(&mut self) -> RCOSCHFRIS_W<0> {
        RCOSCHFRIS_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
The XOSCHFRIS interrupt indicates when the XOSC_HF oscillator has been qualified for use as a clock source. XOSCHFRIS is also used in TCXO mode (when DDI_0_OSC:XOSCHFCTL.TCXO_MODE is 1). When the XOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The XOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: XOSC_HF has not been qualified 1: XOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCHFC"]
    #[inline(always)]
    #[must_use]
    pub fn xoschfris(&mut self) -> XOSCHFRIS_W<1> {
        XOSCHFRIS_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
The RCOSCLFRIS interrupt indicates when the output of the RCOSC_LF oscillator has been qualified with respect to frequency. The RCOSCLFRIS interrupt status goes high when the RCOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, RCOSCLFRIS interrupt status remains high, and further qualification is turned off until the RCOSC_LF oscillator is disabled. RCOSCLFRIS interrupt status will go low only at initial power-on, or after the RCOSC_LF oscillator has been disabled when being deselected as a clock source. 0: RCOSCLF has not been qualified 1: RCOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCLFC"]
    #[inline(always)]
    #[must_use]
    pub fn rcosclfris(&mut self) -> RCOSCLFRIS_W<2> {
        RCOSCLFRIS_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
The RCOSCDLFRIS interrupt indicates when the RCOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When RCOSCDLFRIS is high, RCOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have RCOSC_HF selected as clock source, the RCOSC_HF source is automatically disabled and the RCOSCDLFRIS interrupt status will go low. If the SCLK_LF or ACLK_REF source is changed from RCOSC_HF derived to XOSC_HF derived low-frequency clock and the new source has not been qualified, then the clock will remain running on the original source. The RCOSCDLFRIS interrupt will then remain high. 0: RCOSCDLF has not been qualified 1: RCOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCDLFC"]
    #[inline(always)]
    #[must_use]
    pub fn rcoscdlfris(&mut self) -> RCOSCDLFRIS_W<3> {
        RCOSCDLFRIS_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
The XOSCLFRIS interrupt indicates when the output of the XOSC_LF oscillator has been qualified with respect to frequency. The XOSCLFRIS interrupt status goes high when the XOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, XOSCLFRIS interrupt status remains high, and further qualification is turned off until the XOSC_LF oscillator is disabled. XOSCLFRIS interrupt status will go low only at initial power-on, or after the XOSC_LF oscillator has been disabled when being deselected as a clock source. 0: XOSCLF has not been qualified 1: XOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCLFC"]
    #[inline(always)]
    #[must_use]
    pub fn xosclfris(&mut self) -> XOSCLFRIS_W<4> {
        XOSCLFRIS_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
The XOSCDLFRIS interrupt indicates when the XOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When XOSCDLFRIS is high, XOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have XOSC_HF selected as clock source, the XOSC_HF source is automatically disabled and the XOSCDLFRIS interrupt status will go low. 0: XOSCDLF has not been qualified 1: XOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCDLFC"]
    #[inline(always)]
    #[must_use]
    pub fn xoscdlfris(&mut self) -> XOSCDLFRIS_W<5> {
        XOSCDLFRIS_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
SCLK_LF source switch done. The DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL register field is used to request that the SCLK_LF source shall be changed. After an SCLK_LF clock source change is requested, the new source may need to be enabled and qualified before switching of clock source can be done. The interrupt LFRSRCDONERIS goes high to indicate that the SCLK_LF clock source switching has been performed. LFRSRCDONERIS will go low again when the next clock source change is requested by writing to DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL . 0: Indicates SCLK_LF source switch has not completed 1: Indicates SCLK_LF source switch has completed Interrupt is qualified regardless of OSCIMSC.LFSRCDONEIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.LFSRCDONEC"]
    #[inline(always)]
    #[must_use]
    pub fn lfsrcdoneris(&mut self) -> LFSRCDONERIS_W<6> {
        LFSRCDONERIS_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
SCLK_HF source switch pending interrupt. After a write to DDI_0_OSC:CTL0.SCLK_HF_SRC_SEL leads to a SCLK_HF source change request, then the requested SCLK_HF source will be enabled and qualified. When the new source is ready to be used as a clock source, then the interrupt HSSRCPENDRIS will go high. When the Flash allows SCLK_HF source switching to take place after flash memory read access is disabled. At this time the actual SCLK_HF clock source switch will be performed, and the interrupt status HSSRCPENDRIS will go low. 0: Indicates SCLK_HF source is not ready to be switched 1: Indicates SCLK_HF source is ready to be switched Interrupt is qualified regardless of OSCIMSC.HFSRCPENDIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.HFSRCPENDC"]
    #[inline(always)]
    #[must_use]
    pub fn hfsrcpendris(&mut self) -> HFSRCPENDRIS_W<7> {
        HFSRCPENDRIS_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscris](index.html) module"]
pub struct OSCRIS_SPEC;
impl crate::RegisterSpec for OSCRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oscris::R](R) reader structure"]
impl crate::Readable for OSCRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oscris::W](W) writer structure"]
impl crate::Writable for OSCRIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCRIS to value 0"]
impl crate::Resettable for OSCRIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

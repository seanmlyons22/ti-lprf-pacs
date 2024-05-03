#[doc = "Register `OSCRIS` reader"]
pub type R = crate::R<OscrisSpec>;
#[doc = "Register `OSCRIS` writer"]
pub type W = crate::W<OscrisSpec>;
#[doc = "Field `RCOSCHFRIS` reader - 0:0\\]
The RCOSCHFRIS interrupt indicates when the RCOSC_HF oscillator has been qualified for use as a clock source When the RCOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The RCOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: RCOSC_HF has not been qualified 1: RCOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCHFC"]
pub type RcoschfrisR = crate::BitReader;
#[doc = "Field `RCOSCHFRIS` writer - 0:0\\]
The RCOSCHFRIS interrupt indicates when the RCOSC_HF oscillator has been qualified for use as a clock source When the RCOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The RCOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: RCOSC_HF has not been qualified 1: RCOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCHFC"]
pub type RcoschfrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCHFRIS` reader - 1:1\\]
The XOSCHFRIS interrupt indicates when the XOSC_HF oscillator has been qualified for use as a clock source. XOSCHFRIS is also used in TCXO mode (when DDI_0_OSC:XOSCHFCTL.TCXO_MODE is 1). When the XOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The XOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: XOSC_HF has not been qualified 1: XOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCHFC"]
pub type XoschfrisR = crate::BitReader;
#[doc = "Field `XOSCHFRIS` writer - 1:1\\]
The XOSCHFRIS interrupt indicates when the XOSC_HF oscillator has been qualified for use as a clock source. XOSCHFRIS is also used in TCXO mode (when DDI_0_OSC:XOSCHFCTL.TCXO_MODE is 1). When the XOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The XOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: XOSC_HF has not been qualified 1: XOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCHFC"]
pub type XoschfrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOSCLFRIS` reader - 2:2\\]
The RCOSCLFRIS interrupt indicates when the output of the RCOSC_LF oscillator has been qualified with respect to frequency. The RCOSCLFRIS interrupt status goes high when the RCOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, RCOSCLFRIS interrupt status remains high, and further qualification is turned off until the RCOSC_LF oscillator is disabled. RCOSCLFRIS interrupt status will go low only at initial power-on, or after the RCOSC_LF oscillator has been disabled when being deselected as a clock source. 0: RCOSCLF has not been qualified 1: RCOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCLFC"]
pub type RcosclfrisR = crate::BitReader;
#[doc = "Field `RCOSCLFRIS` writer - 2:2\\]
The RCOSCLFRIS interrupt indicates when the output of the RCOSC_LF oscillator has been qualified with respect to frequency. The RCOSCLFRIS interrupt status goes high when the RCOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, RCOSCLFRIS interrupt status remains high, and further qualification is turned off until the RCOSC_LF oscillator is disabled. RCOSCLFRIS interrupt status will go low only at initial power-on, or after the RCOSC_LF oscillator has been disabled when being deselected as a clock source. 0: RCOSCLF has not been qualified 1: RCOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCLFC"]
pub type RcosclfrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOSCDLFRIS` reader - 3:3\\]
The RCOSCDLFRIS interrupt indicates when the RCOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When RCOSCDLFRIS is high, RCOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have RCOSC_HF selected as clock source, the RCOSC_HF source is automatically disabled and the RCOSCDLFRIS interrupt status will go low. If the SCLK_LF or ACLK_REF source is changed from RCOSC_HF derived to XOSC_HF derived low-frequency clock and the new source has not been qualified, then the clock will remain running on the original source. The RCOSCDLFRIS interrupt will then remain high. 0: RCOSCDLF has not been qualified 1: RCOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCDLFC"]
pub type RcoscdlfrisR = crate::BitReader;
#[doc = "Field `RCOSCDLFRIS` writer - 3:3\\]
The RCOSCDLFRIS interrupt indicates when the RCOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When RCOSCDLFRIS is high, RCOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have RCOSC_HF selected as clock source, the RCOSC_HF source is automatically disabled and the RCOSCDLFRIS interrupt status will go low. If the SCLK_LF or ACLK_REF source is changed from RCOSC_HF derived to XOSC_HF derived low-frequency clock and the new source has not been qualified, then the clock will remain running on the original source. The RCOSCDLFRIS interrupt will then remain high. 0: RCOSCDLF has not been qualified 1: RCOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCDLFC"]
pub type RcoscdlfrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCLFRIS` reader - 4:4\\]
The XOSCLFRIS interrupt indicates when the output of the XOSC_LF oscillator has been qualified with respect to frequency. The XOSCLFRIS interrupt status goes high when the XOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, XOSCLFRIS interrupt status remains high, and further qualification is turned off until the XOSC_LF oscillator is disabled. XOSCLFRIS interrupt status will go low only at initial power-on, or after the XOSC_LF oscillator has been disabled when being deselected as a clock source. 0: XOSCLF has not been qualified 1: XOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCLFC"]
pub type XosclfrisR = crate::BitReader;
#[doc = "Field `XOSCLFRIS` writer - 4:4\\]
The XOSCLFRIS interrupt indicates when the output of the XOSC_LF oscillator has been qualified with respect to frequency. The XOSCLFRIS interrupt status goes high when the XOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, XOSCLFRIS interrupt status remains high, and further qualification is turned off until the XOSC_LF oscillator is disabled. XOSCLFRIS interrupt status will go low only at initial power-on, or after the XOSC_LF oscillator has been disabled when being deselected as a clock source. 0: XOSCLF has not been qualified 1: XOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCLFC"]
pub type XosclfrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCDLFRIS` reader - 5:5\\]
The XOSCDLFRIS interrupt indicates when the XOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When XOSCDLFRIS is high, XOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have XOSC_HF selected as clock source, the XOSC_HF source is automatically disabled and the XOSCDLFRIS interrupt status will go low. 0: XOSCDLF has not been qualified 1: XOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCDLFC"]
pub type XoscdlfrisR = crate::BitReader;
#[doc = "Field `XOSCDLFRIS` writer - 5:5\\]
The XOSCDLFRIS interrupt indicates when the XOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When XOSCDLFRIS is high, XOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have XOSC_HF selected as clock source, the XOSC_HF source is automatically disabled and the XOSCDLFRIS interrupt status will go low. 0: XOSCDLF has not been qualified 1: XOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCDLFC"]
pub type XoscdlfrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFSRCDONERIS` reader - 6:6\\]
SCLK_LF source switch done. The DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL register field is used to request that the SCLK_LF source shall be changed. After an SCLK_LF clock source change is requested, the new source may need to be enabled and qualified before switching of clock source can be done. The interrupt LFRSRCDONERIS goes high to indicate that the SCLK_LF clock source switching has been performed. LFRSRCDONERIS will go low again when the next clock source change is requested by writing to DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL . 0: Indicates SCLK_LF source switch has not completed 1: Indicates SCLK_LF source switch has completed Interrupt is qualified regardless of OSCIMSC.LFSRCDONEIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.LFSRCDONEC"]
pub type LfsrcdonerisR = crate::BitReader;
#[doc = "Field `LFSRCDONERIS` writer - 6:6\\]
SCLK_LF source switch done. The DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL register field is used to request that the SCLK_LF source shall be changed. After an SCLK_LF clock source change is requested, the new source may need to be enabled and qualified before switching of clock source can be done. The interrupt LFRSRCDONERIS goes high to indicate that the SCLK_LF clock source switching has been performed. LFRSRCDONERIS will go low again when the next clock source change is requested by writing to DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL . 0: Indicates SCLK_LF source switch has not completed 1: Indicates SCLK_LF source switch has completed Interrupt is qualified regardless of OSCIMSC.LFSRCDONEIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.LFSRCDONEC"]
pub type LfsrcdonerisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFSRCPENDRIS` reader - 7:7\\]
SCLK_HF source switch pending interrupt. After a write to DDI_0_OSC:CTL0.SCLK_HF_SRC_SEL leads to a SCLK_HF source change request, then the requested SCLK_HF source will be enabled and qualified. When the new source is ready to be used as a clock source, then the interrupt HSSRCPENDRIS will go high. When the Flash allows SCLK_HF source switching to take place after flash memory read access is disabled. At this time the actual SCLK_HF clock source switch will be performed, and the interrupt status HSSRCPENDRIS will go low. 0: Indicates SCLK_HF source is not ready to be switched 1: Indicates SCLK_HF source is ready to be switched Interrupt is qualified regardless of OSCIMSC.HFSRCPENDIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.HFSRCPENDC"]
pub type HfsrcpendrisR = crate::BitReader;
#[doc = "Field `HFSRCPENDRIS` writer - 7:7\\]
SCLK_HF source switch pending interrupt. After a write to DDI_0_OSC:CTL0.SCLK_HF_SRC_SEL leads to a SCLK_HF source change request, then the requested SCLK_HF source will be enabled and qualified. When the new source is ready to be used as a clock source, then the interrupt HSSRCPENDRIS will go high. When the Flash allows SCLK_HF source switching to take place after flash memory read access is disabled. At this time the actual SCLK_HF clock source switch will be performed, and the interrupt status HSSRCPENDRIS will go low. 0: Indicates SCLK_HF source is not ready to be switched 1: Indicates SCLK_HF source is ready to be switched Interrupt is qualified regardless of OSCIMSC.HFSRCPENDIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.HFSRCPENDC"]
pub type HfsrcpendrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
The RCOSCHFRIS interrupt indicates when the RCOSC_HF oscillator has been qualified for use as a clock source When the RCOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The RCOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: RCOSC_HF has not been qualified 1: RCOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCHFC"]
    #[inline(always)]
    pub fn rcoschfris(&self) -> RcoschfrisR {
        RcoschfrisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
The XOSCHFRIS interrupt indicates when the XOSC_HF oscillator has been qualified for use as a clock source. XOSCHFRIS is also used in TCXO mode (when DDI_0_OSC:XOSCHFCTL.TCXO_MODE is 1). When the XOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The XOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: XOSC_HF has not been qualified 1: XOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCHFC"]
    #[inline(always)]
    pub fn xoschfris(&self) -> XoschfrisR {
        XoschfrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
The RCOSCLFRIS interrupt indicates when the output of the RCOSC_LF oscillator has been qualified with respect to frequency. The RCOSCLFRIS interrupt status goes high when the RCOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, RCOSCLFRIS interrupt status remains high, and further qualification is turned off until the RCOSC_LF oscillator is disabled. RCOSCLFRIS interrupt status will go low only at initial power-on, or after the RCOSC_LF oscillator has been disabled when being deselected as a clock source. 0: RCOSCLF has not been qualified 1: RCOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCLFC"]
    #[inline(always)]
    pub fn rcosclfris(&self) -> RcosclfrisR {
        RcosclfrisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
The RCOSCDLFRIS interrupt indicates when the RCOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When RCOSCDLFRIS is high, RCOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have RCOSC_HF selected as clock source, the RCOSC_HF source is automatically disabled and the RCOSCDLFRIS interrupt status will go low. If the SCLK_LF or ACLK_REF source is changed from RCOSC_HF derived to XOSC_HF derived low-frequency clock and the new source has not been qualified, then the clock will remain running on the original source. The RCOSCDLFRIS interrupt will then remain high. 0: RCOSCDLF has not been qualified 1: RCOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCDLFC"]
    #[inline(always)]
    pub fn rcoscdlfris(&self) -> RcoscdlfrisR {
        RcoscdlfrisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
The XOSCLFRIS interrupt indicates when the output of the XOSC_LF oscillator has been qualified with respect to frequency. The XOSCLFRIS interrupt status goes high when the XOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, XOSCLFRIS interrupt status remains high, and further qualification is turned off until the XOSC_LF oscillator is disabled. XOSCLFRIS interrupt status will go low only at initial power-on, or after the XOSC_LF oscillator has been disabled when being deselected as a clock source. 0: XOSCLF has not been qualified 1: XOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCLFC"]
    #[inline(always)]
    pub fn xosclfris(&self) -> XosclfrisR {
        XosclfrisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
The XOSCDLFRIS interrupt indicates when the XOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When XOSCDLFRIS is high, XOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have XOSC_HF selected as clock source, the XOSC_HF source is automatically disabled and the XOSCDLFRIS interrupt status will go low. 0: XOSCDLF has not been qualified 1: XOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCDLFC"]
    #[inline(always)]
    pub fn xoscdlfris(&self) -> XoscdlfrisR {
        XoscdlfrisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
SCLK_LF source switch done. The DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL register field is used to request that the SCLK_LF source shall be changed. After an SCLK_LF clock source change is requested, the new source may need to be enabled and qualified before switching of clock source can be done. The interrupt LFRSRCDONERIS goes high to indicate that the SCLK_LF clock source switching has been performed. LFRSRCDONERIS will go low again when the next clock source change is requested by writing to DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL . 0: Indicates SCLK_LF source switch has not completed 1: Indicates SCLK_LF source switch has completed Interrupt is qualified regardless of OSCIMSC.LFSRCDONEIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.LFSRCDONEC"]
    #[inline(always)]
    pub fn lfsrcdoneris(&self) -> LfsrcdonerisR {
        LfsrcdonerisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
SCLK_HF source switch pending interrupt. After a write to DDI_0_OSC:CTL0.SCLK_HF_SRC_SEL leads to a SCLK_HF source change request, then the requested SCLK_HF source will be enabled and qualified. When the new source is ready to be used as a clock source, then the interrupt HSSRCPENDRIS will go high. When the Flash allows SCLK_HF source switching to take place after flash memory read access is disabled. At this time the actual SCLK_HF clock source switch will be performed, and the interrupt status HSSRCPENDRIS will go low. 0: Indicates SCLK_HF source is not ready to be switched 1: Indicates SCLK_HF source is ready to be switched Interrupt is qualified regardless of OSCIMSC.HFSRCPENDIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.HFSRCPENDC"]
    #[inline(always)]
    pub fn hfsrcpendris(&self) -> HfsrcpendrisR {
        HfsrcpendrisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
The RCOSCHFRIS interrupt indicates when the RCOSC_HF oscillator has been qualified for use as a clock source When the RCOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The RCOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: RCOSC_HF has not been qualified 1: RCOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCHFC"]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfris(&mut self) -> RcoschfrisW<OscrisSpec> {
        RcoschfrisW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
The XOSCHFRIS interrupt indicates when the XOSC_HF oscillator has been qualified for use as a clock source. XOSCHFRIS is also used in TCXO mode (when DDI_0_OSC:XOSCHFCTL.TCXO_MODE is 1). When the XOSCHFRIS interrupt is high, the oscillator is qualified and will be used as a clock source when selected. The XOSCHFRIS interrupt goes low when the oscillator is disabled after being deselected as a clock source. 0: XOSC_HF has not been qualified 1: XOSC_HF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCHFC"]
    #[inline(always)]
    #[must_use]
    pub fn xoschfris(&mut self) -> XoschfrisW<OscrisSpec> {
        XoschfrisW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
The RCOSCLFRIS interrupt indicates when the output of the RCOSC_LF oscillator has been qualified with respect to frequency. The RCOSCLFRIS interrupt status goes high when the RCOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, RCOSCLFRIS interrupt status remains high, and further qualification is turned off until the RCOSC_LF oscillator is disabled. RCOSCLFRIS interrupt status will go low only at initial power-on, or after the RCOSC_LF oscillator has been disabled when being deselected as a clock source. 0: RCOSCLF has not been qualified 1: RCOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCLFC"]
    #[inline(always)]
    #[must_use]
    pub fn rcosclfris(&mut self) -> RcosclfrisW<OscrisSpec> {
        RcosclfrisW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
The RCOSCDLFRIS interrupt indicates when the RCOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When RCOSCDLFRIS is high, RCOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have RCOSC_HF selected as clock source, the RCOSC_HF source is automatically disabled and the RCOSCDLFRIS interrupt status will go low. If the SCLK_LF or ACLK_REF source is changed from RCOSC_HF derived to XOSC_HF derived low-frequency clock and the new source has not been qualified, then the clock will remain running on the original source. The RCOSCDLFRIS interrupt will then remain high. 0: RCOSCDLF has not been qualified 1: RCOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.RCOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCDLFC"]
    #[inline(always)]
    #[must_use]
    pub fn rcoscdlfris(&mut self) -> RcoscdlfrisW<OscrisSpec> {
        RcoscdlfrisW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
The XOSCLFRIS interrupt indicates when the output of the XOSC_LF oscillator has been qualified with respect to frequency. The XOSCLFRIS interrupt status goes high when the XOSC_LF oscillator is ready to be used as a clock source. After the clock qualification is successful, XOSCLFRIS interrupt status remains high, and further qualification is turned off until the XOSC_LF oscillator is disabled. XOSCLFRIS interrupt status will go low only at initial power-on, or after the XOSC_LF oscillator has been disabled when being deselected as a clock source. 0: XOSCLF has not been qualified 1: XOSCLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCLFC"]
    #[inline(always)]
    #[must_use]
    pub fn xosclfris(&mut self) -> XosclfrisW<OscrisSpec> {
        XosclfrisW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
The XOSCDLFRIS interrupt indicates when the XOSC_HF oscillator is ready to be used as a derived low-frequency clock source for SCLK_LF or ACLK_REF. When XOSCDLFRIS is high, XOSC_HF will be used as source for SCLK_LF when selected. When none of the system clocks have XOSC_HF selected as clock source, the XOSC_HF source is automatically disabled and the XOSCDLFRIS interrupt status will go low. 0: XOSCDLF has not been qualified 1: XOSCDLF has been qualified Interrupt is qualified regardless of OSCIMSC.XOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCDLFC"]
    #[inline(always)]
    #[must_use]
    pub fn xoscdlfris(&mut self) -> XoscdlfrisW<OscrisSpec> {
        XoscdlfrisW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
SCLK_LF source switch done. The DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL register field is used to request that the SCLK_LF source shall be changed. After an SCLK_LF clock source change is requested, the new source may need to be enabled and qualified before switching of clock source can be done. The interrupt LFRSRCDONERIS goes high to indicate that the SCLK_LF clock source switching has been performed. LFRSRCDONERIS will go low again when the next clock source change is requested by writing to DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL . 0: Indicates SCLK_LF source switch has not completed 1: Indicates SCLK_LF source switch has completed Interrupt is qualified regardless of OSCIMSC.LFSRCDONEIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.LFSRCDONEC"]
    #[inline(always)]
    #[must_use]
    pub fn lfsrcdoneris(&mut self) -> LfsrcdonerisW<OscrisSpec> {
        LfsrcdonerisW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
SCLK_HF source switch pending interrupt. After a write to DDI_0_OSC:CTL0.SCLK_HF_SRC_SEL leads to a SCLK_HF source change request, then the requested SCLK_HF source will be enabled and qualified. When the new source is ready to be used as a clock source, then the interrupt HSSRCPENDRIS will go high. When the Flash allows SCLK_HF source switching to take place after flash memory read access is disabled. At this time the actual SCLK_HF clock source switch will be performed, and the interrupt status HSSRCPENDRIS will go low. 0: Indicates SCLK_HF source is not ready to be switched 1: Indicates SCLK_HF source is ready to be switched Interrupt is qualified regardless of OSCIMSC.HFSRCPENDIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.HFSRCPENDC"]
    #[inline(always)]
    #[must_use]
    pub fn hfsrcpendris(&mut self) -> HfsrcpendrisW<OscrisSpec> {
        HfsrcpendrisW::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<OscrisSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Oscillator Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscrisSpec;
impl crate::RegisterSpec for OscrisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oscris::R`](R) reader structure"]
impl crate::Readable for OscrisSpec {}
#[doc = "`write(|w| ..)` method takes [`oscris::W`](W) writer structure"]
impl crate::Writable for OscrisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCRIS to value 0"]
impl crate::Resettable for OscrisSpec {
    const RESET_VALUE: u32 = 0;
}

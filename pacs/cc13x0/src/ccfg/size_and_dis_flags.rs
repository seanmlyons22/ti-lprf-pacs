#[doc = "Register `SIZE_AND_DIS_FLAGS` reader"]
pub struct R(crate::R<SIZE_AND_DIS_FLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIZE_AND_DIS_FLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIZE_AND_DIS_FLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIZE_AND_DIS_FLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIZE_AND_DIS_FLAGS` writer"]
pub struct W(crate::W<SIZE_AND_DIS_FLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIZE_AND_DIS_FLAGS_SPEC>;
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
impl From<crate::W<SIZE_AND_DIS_FLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIZE_AND_DIS_FLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS_XOSC_OVR` reader - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
pub type DIS_XOSC_OVR_R = crate::BitReader<bool>;
#[doc = "Field `DIS_XOSC_OVR` writer - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
pub type DIS_XOSC_OVR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SIZE_AND_DIS_FLAGS_SPEC, bool, O>;
#[doc = "Field `DIS_ALT_DCDC_SETTING` reader - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub type DIS_ALT_DCDC_SETTING_R = crate::BitReader<bool>;
#[doc = "Field `DIS_ALT_DCDC_SETTING` writer - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub type DIS_ALT_DCDC_SETTING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SIZE_AND_DIS_FLAGS_SPEC, bool, O>;
#[doc = "Field `DIS_GPRAM` reader - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
pub type DIS_GPRAM_R = crate::BitReader<bool>;
#[doc = "Field `DIS_GPRAM` writer - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
pub type DIS_GPRAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIZE_AND_DIS_FLAGS_SPEC, bool, O>;
#[doc = "Field `DIS_TCXO` reader - 3:3\\]
Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
pub type DIS_TCXO_R = crate::BitReader<bool>;
#[doc = "Field `DIS_TCXO` writer - 3:3\\]
Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
pub type DIS_TCXO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIZE_AND_DIS_FLAGS_SPEC, bool, O>;
#[doc = "Field `DISABLE_FLAGS` reader - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type DISABLE_FLAGS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DISABLE_FLAGS` writer - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type DISABLE_FLAGS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIZE_AND_DIS_FLAGS_SPEC, u16, u16, 12, O>;
#[doc = "Field `SIZE_OF_CCFG` reader - 31:16\\]
Total size of CCFG in bytes."]
pub type SIZE_OF_CCFG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SIZE_OF_CCFG` writer - 31:16\\]
Total size of CCFG in bytes."]
pub type SIZE_OF_CCFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIZE_AND_DIS_FLAGS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
    #[inline(always)]
    pub fn dis_xosc_ovr(&self) -> DIS_XOSC_OVR_R {
        DIS_XOSC_OVR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dis_alt_dcdc_setting(&self) -> DIS_ALT_DCDC_SETTING_R {
        DIS_ALT_DCDC_SETTING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
    #[inline(always)]
    pub fn dis_gpram(&self) -> DIS_GPRAM_R {
        DIS_GPRAM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
    #[inline(always)]
    pub fn dis_tcxo(&self) -> DIS_TCXO_R {
        DIS_TCXO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn disable_flags(&self) -> DISABLE_FLAGS_R {
        DISABLE_FLAGS_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Total size of CCFG in bytes."]
    #[inline(always)]
    pub fn size_of_ccfg(&self) -> SIZE_OF_CCFG_R {
        SIZE_OF_CCFG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
    #[inline(always)]
    #[must_use]
    pub fn dis_xosc_ovr(&mut self) -> DIS_XOSC_OVR_W<0> {
        DIS_XOSC_OVR_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    #[must_use]
    pub fn dis_alt_dcdc_setting(&mut self) -> DIS_ALT_DCDC_SETTING_W<1> {
        DIS_ALT_DCDC_SETTING_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
    #[inline(always)]
    #[must_use]
    pub fn dis_gpram(&mut self) -> DIS_GPRAM_W<2> {
        DIS_GPRAM_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
    #[inline(always)]
    #[must_use]
    pub fn dis_tcxo(&mut self) -> DIS_TCXO_W<3> {
        DIS_TCXO_W::new(self)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn disable_flags(&mut self) -> DISABLE_FLAGS_W<4> {
        DISABLE_FLAGS_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Total size of CCFG in bytes."]
    #[inline(always)]
    #[must_use]
    pub fn size_of_ccfg(&mut self) -> SIZE_OF_CCFG_W<16> {
        SIZE_OF_CCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCFG Size and Disable Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [size_and_dis_flags](index.html) module"]
pub struct SIZE_AND_DIS_FLAGS_SPEC;
impl crate::RegisterSpec for SIZE_AND_DIS_FLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [size_and_dis_flags::R](R) reader structure"]
impl crate::Readable for SIZE_AND_DIS_FLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [size_and_dis_flags::W](W) writer structure"]
impl crate::Writable for SIZE_AND_DIS_FLAGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIZE_AND_DIS_FLAGS to value 0xffff_ffff"]
impl crate::Resettable for SIZE_AND_DIS_FLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

#[doc = "Register `SIZE_AND_DIS_FLAGS` reader"]
pub type R = crate::R<SizeAndDisFlagsSpec>;
#[doc = "Register `SIZE_AND_DIS_FLAGS` writer"]
pub type W = crate::W<SizeAndDisFlagsSpec>;
#[doc = "Field `DIS_XOSC_OVR` reader - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
pub type DisXoscOvrR = crate::BitReader;
#[doc = "Field `DIS_XOSC_OVR` writer - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
pub type DisXoscOvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_ALT_DCDC_SETTING` reader - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub type DisAltDcdcSettingR = crate::BitReader;
#[doc = "Field `DIS_ALT_DCDC_SETTING` writer - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub type DisAltDcdcSettingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_GPRAM` reader - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
pub type DisGpramR = crate::BitReader;
#[doc = "Field `DIS_GPRAM` writer - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
pub type DisGpramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_TCXO` reader - 3:3\\]
Deprecated. Must be set to 1."]
pub type DisTcxoR = crate::BitReader;
#[doc = "Field `DIS_TCXO` writer - 3:3\\]
Deprecated. Must be set to 1."]
pub type DisTcxoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_FLAGS` reader - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type DisableFlagsR = crate::FieldReader<u16>;
#[doc = "Field `DISABLE_FLAGS` writer - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type DisableFlagsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SIZE_OF_CCFG` reader - 31:16\\]
Total size of CCFG in bytes."]
pub type SizeOfCcfgR = crate::FieldReader<u16>;
#[doc = "Field `SIZE_OF_CCFG` writer - 31:16\\]
Total size of CCFG in bytes."]
pub type SizeOfCcfgW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
    #[inline(always)]
    pub fn dis_xosc_ovr(&self) -> DisXoscOvrR {
        DisXoscOvrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dis_alt_dcdc_setting(&self) -> DisAltDcdcSettingR {
        DisAltDcdcSettingR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
    #[inline(always)]
    pub fn dis_gpram(&self) -> DisGpramR {
        DisGpramR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Deprecated. Must be set to 1."]
    #[inline(always)]
    pub fn dis_tcxo(&self) -> DisTcxoR {
        DisTcxoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn disable_flags(&self) -> DisableFlagsR {
        DisableFlagsR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Total size of CCFG in bytes."]
    #[inline(always)]
    pub fn size_of_ccfg(&self) -> SizeOfCcfgR {
        SizeOfCcfgR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
    #[inline(always)]
    #[must_use]
    pub fn dis_xosc_ovr(&mut self) -> DisXoscOvrW<SizeAndDisFlagsSpec> {
        DisXoscOvrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    #[must_use]
    pub fn dis_alt_dcdc_setting(&mut self) -> DisAltDcdcSettingW<SizeAndDisFlagsSpec> {
        DisAltDcdcSettingW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
    #[inline(always)]
    #[must_use]
    pub fn dis_gpram(&mut self) -> DisGpramW<SizeAndDisFlagsSpec> {
        DisGpramW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Deprecated. Must be set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn dis_tcxo(&mut self) -> DisTcxoW<SizeAndDisFlagsSpec> {
        DisTcxoW::new(self, 3)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn disable_flags(&mut self) -> DisableFlagsW<SizeAndDisFlagsSpec> {
        DisableFlagsW::new(self, 4)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Total size of CCFG in bytes."]
    #[inline(always)]
    #[must_use]
    pub fn size_of_ccfg(&mut self) -> SizeOfCcfgW<SizeAndDisFlagsSpec> {
        SizeOfCcfgW::new(self, 16)
    }
}
#[doc = "CCFG Size and Disable Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`size_and_dis_flags::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`size_and_dis_flags::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SizeAndDisFlagsSpec;
impl crate::RegisterSpec for SizeAndDisFlagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`size_and_dis_flags::R`](R) reader structure"]
impl crate::Readable for SizeAndDisFlagsSpec {}
#[doc = "`write(|w| ..)` method takes [`size_and_dis_flags::W`](W) writer structure"]
impl crate::Writable for SizeAndDisFlagsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIZE_AND_DIS_FLAGS to value 0xffff_ffff"]
impl crate::Resettable for SizeAndDisFlagsSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

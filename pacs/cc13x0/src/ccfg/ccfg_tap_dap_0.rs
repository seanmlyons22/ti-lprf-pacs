#[doc = "Register `CCFG_TAP_DAP_0` reader"]
pub type R = crate::R<CcfgTapDap0Spec>;
#[doc = "Register `CCFG_TAP_DAP_0` writer"]
pub type W = crate::W<CcfgTapDap0Spec>;
#[doc = "Field `TEST_TAP_ENABLE` reader - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
pub type TestTapEnableR = crate::FieldReader;
#[doc = "Field `TEST_TAP_ENABLE` writer - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
pub type TestTapEnableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRCM_TAP_ENABLE` reader - 15:8\\]
Enable PRCM TAP. 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PRCM TAP access will remain disabled out of power-up/system-reset."]
pub type PrcmTapEnableR = crate::FieldReader;
#[doc = "Field `PRCM_TAP_ENABLE` writer - 15:8\\]
Enable PRCM TAP. 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PRCM TAP access will remain disabled out of power-up/system-reset."]
pub type PrcmTapEnableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPU_DAP_ENABLE` reader - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
pub type CpuDapEnableR = crate::FieldReader;
#[doc = "Field `CPU_DAP_ENABLE` writer - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
pub type CpuDapEnableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn test_tap_enable(&self) -> TestTapEnableR {
        TestTapEnableR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PRCM TAP. 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PRCM TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn prcm_tap_enable(&self) -> PrcmTapEnableR {
        PrcmTapEnableR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn cpu_dap_enable(&self) -> CpuDapEnableR {
        CpuDapEnableR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    #[must_use]
    pub fn test_tap_enable(&mut self) -> TestTapEnableW<CcfgTapDap0Spec> {
        TestTapEnableW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PRCM TAP. 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PRCM TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    #[must_use]
    pub fn prcm_tap_enable(&mut self) -> PrcmTapEnableW<CcfgTapDap0Spec> {
        PrcmTapEnableW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_dap_enable(&mut self) -> CpuDapEnableW<CcfgTapDap0Spec> {
        CpuDapEnableW::new(self, 16)
    }
}
#[doc = "Test Access Points Enable 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_tap_dap_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_tap_dap_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgTapDap0Spec;
impl crate::RegisterSpec for CcfgTapDap0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_tap_dap_0::R`](R) reader structure"]
impl crate::Readable for CcfgTapDap0Spec {}
#[doc = "`write(|w| ..)` method takes [`ccfg_tap_dap_0::W`](W) writer structure"]
impl crate::Writable for CcfgTapDap0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCFG_TAP_DAP_0 to value 0xffc5_c5c5"]
impl crate::Resettable for CcfgTapDap0Spec {
    const RESET_VALUE: u32 = 0xffc5_c5c5;
}

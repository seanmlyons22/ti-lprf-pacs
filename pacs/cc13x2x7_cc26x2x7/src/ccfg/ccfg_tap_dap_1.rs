#[doc = "Register `CCFG_TAP_DAP_1` reader"]
pub type R = crate::R<CcfgTapDap1Spec>;
#[doc = "Register `CCFG_TAP_DAP_1` writer"]
pub type W = crate::W<CcfgTapDap1Spec>;
#[doc = "Field `AON_TAP_ENABLE` reader - 7:0\\]
Enable AON TAP 0xC5: AON TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: AON TAP access will remain disabled out of power-up/system-reset."]
pub type AonTapEnableR = crate::FieldReader;
#[doc = "Field `AON_TAP_ENABLE` writer - 7:0\\]
Enable AON TAP 0xC5: AON TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: AON TAP access will remain disabled out of power-up/system-reset."]
pub type AonTapEnableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PBIST1_TAP_ENABLE` reader - 15:8\\]
Enable PBIST1 TAP. 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST1 TAP access will remain disabled out of power-up/system-reset."]
pub type Pbist1TapEnableR = crate::FieldReader;
#[doc = "Field `PBIST1_TAP_ENABLE` writer - 15:8\\]
Enable PBIST1 TAP. 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST1 TAP access will remain disabled out of power-up/system-reset."]
pub type Pbist1TapEnableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PBIST2_TAP_ENABLE` reader - 23:16\\]
Enable PBIST2 TAP. 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST2 TAP access will remain disabled out of power-up/system-reset."]
pub type Pbist2TapEnableR = crate::FieldReader;
#[doc = "Field `PBIST2_TAP_ENABLE` writer - 23:16\\]
Enable PBIST2 TAP. 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST2 TAP access will remain disabled out of power-up/system-reset."]
pub type Pbist2TapEnableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Enable AON TAP 0xC5: AON TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: AON TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn aon_tap_enable(&self) -> AonTapEnableR {
        AonTapEnableR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PBIST1 TAP. 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST1 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn pbist1_tap_enable(&self) -> Pbist1TapEnableR {
        Pbist1TapEnableR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Enable PBIST2 TAP. 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST2 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn pbist2_tap_enable(&self) -> Pbist2TapEnableR {
        Pbist2TapEnableR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Enable AON TAP 0xC5: AON TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: AON TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    #[must_use]
    pub fn aon_tap_enable(&mut self) -> AonTapEnableW<CcfgTapDap1Spec> {
        AonTapEnableW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PBIST1 TAP. 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST1 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    #[must_use]
    pub fn pbist1_tap_enable(&mut self) -> Pbist1TapEnableW<CcfgTapDap1Spec> {
        Pbist1TapEnableW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Enable PBIST2 TAP. 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST2 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    #[must_use]
    pub fn pbist2_tap_enable(&mut self) -> Pbist2TapEnableW<CcfgTapDap1Spec> {
        Pbist2TapEnableW::new(self, 16)
    }
}
#[doc = "Test Access Points Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_tap_dap_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_tap_dap_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgTapDap1Spec;
impl crate::RegisterSpec for CcfgTapDap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_tap_dap_1::R`](R) reader structure"]
impl crate::Readable for CcfgTapDap1Spec {}
#[doc = "`write(|w| ..)` method takes [`ccfg_tap_dap_1::W`](W) writer structure"]
impl crate::Writable for CcfgTapDap1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCFG_TAP_DAP_1 to value 0xffc5_c5c5"]
impl crate::Resettable for CcfgTapDap1Spec {
    const RESET_VALUE: u32 = 0xffc5_c5c5;
}

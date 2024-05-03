#[doc = "Register `CCFG_TI_OPTIONS` reader"]
pub type R = crate::R<CcfgTiOptionsSpec>;
#[doc = "Register `CCFG_TI_OPTIONS` writer"]
pub type W = crate::W<CcfgTiOptionsSpec>;
#[doc = "Field `TI_FA_ENABLE` reader - 7:0\\]
TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
pub type TiFaEnableR = crate::FieldReader;
#[doc = "Field `TI_FA_ENABLE` writer - 7:0\\]
TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
pub type TiFaEnableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDAU_CFG_ENABLE` reader - 15:8\\]
IDAU configuration. 0xC5: Disable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG. All other values: Enable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG."]
pub type IdauCfgEnableR = crate::FieldReader;
#[doc = "Field `IDAU_CFG_ENABLE` writer - 15:8\\]
IDAU configuration. 0xC5: Disable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG. All other values: Enable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG."]
pub type IdauCfgEnableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `C_FA_DIS` reader - 23:16\\]
Option to disable failure analysis without customer password. If C_FA_DIS != 0xC5, CKEY (CKEY0.KEY, CKEY1.KEY, CKEY2.KEY, CKEY2.KEY) must be provided to TI for failure analysis to be possible. 0xC5: Failure analysis without customer password is enabled All other values: Failure analysis without customer password is disabled"]
pub type CFaDisR = crate::FieldReader;
#[doc = "Field `C_FA_DIS` writer - 23:16\\]
Option to disable failure analysis without customer password. If C_FA_DIS != 0xC5, CKEY (CKEY0.KEY, CKEY1.KEY, CKEY2.KEY, CKEY2.KEY) must be provided to TI for failure analysis to be possible. 0xC5: Failure analysis without customer password is enabled All other values: Failure analysis without customer password is disabled"]
pub type CFaDisW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
    #[inline(always)]
    pub fn ti_fa_enable(&self) -> TiFaEnableR {
        TiFaEnableR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
IDAU configuration. 0xC5: Disable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG. All other values: Enable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG."]
    #[inline(always)]
    pub fn idau_cfg_enable(&self) -> IdauCfgEnableR {
        IdauCfgEnableR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Option to disable failure analysis without customer password. If C_FA_DIS != 0xC5, CKEY (CKEY0.KEY, CKEY1.KEY, CKEY2.KEY, CKEY2.KEY) must be provided to TI for failure analysis to be possible. 0xC5: Failure analysis without customer password is enabled All other values: Failure analysis without customer password is disabled"]
    #[inline(always)]
    pub fn c_fa_dis(&self) -> CFaDisR {
        CFaDisR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
    #[inline(always)]
    #[must_use]
    pub fn ti_fa_enable(&mut self) -> TiFaEnableW<CcfgTiOptionsSpec> {
        TiFaEnableW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
IDAU configuration. 0xC5: Disable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG. All other values: Enable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG."]
    #[inline(always)]
    #[must_use]
    pub fn idau_cfg_enable(&mut self) -> IdauCfgEnableW<CcfgTiOptionsSpec> {
        IdauCfgEnableW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Option to disable failure analysis without customer password. If C_FA_DIS != 0xC5, CKEY (CKEY0.KEY, CKEY1.KEY, CKEY2.KEY, CKEY2.KEY) must be provided to TI for failure analysis to be possible. 0xC5: Failure analysis without customer password is enabled All other values: Failure analysis without customer password is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn c_fa_dis(&mut self) -> CFaDisW<CcfgTiOptionsSpec> {
        CFaDisW::new(self, 16)
    }
}
#[doc = "TI Options\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_ti_options::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_ti_options::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgTiOptionsSpec;
impl crate::RegisterSpec for CcfgTiOptionsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_ti_options::R`](R) reader structure"]
impl crate::Readable for CcfgTiOptionsSpec {}
#[doc = "`write(|w| ..)` method takes [`ccfg_ti_options::W`](W) writer structure"]
impl crate::Writable for CcfgTiOptionsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCFG_TI_OPTIONS to value 0xffc5_c5c5"]
impl crate::Resettable for CcfgTiOptionsSpec {
    const RESET_VALUE: u32 = 0xffc5_c5c5;
}

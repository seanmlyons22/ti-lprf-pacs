#[doc = "Register `CCFG_TI_OPTIONS` reader"]
pub struct R(crate::R<CCFG_TI_OPTIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_TI_OPTIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_TI_OPTIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_TI_OPTIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_TI_OPTIONS` writer"]
pub struct W(crate::W<CCFG_TI_OPTIONS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_TI_OPTIONS_SPEC>;
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
impl From<crate::W<CCFG_TI_OPTIONS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_TI_OPTIONS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI_FA_ENABLE` reader - 7:0\\]
TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
pub type TI_FA_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TI_FA_ENABLE` writer - 7:0\\]
TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
pub type TI_FA_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCFG_TI_OPTIONS_SPEC, u8, u8, 8, O>;
#[doc = "Field `IDAU_CFG_ENABLE` reader - 15:8\\]
IDAU configuration. 0xC5: Disable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG. All other values: Enable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG."]
pub type IDAU_CFG_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDAU_CFG_ENABLE` writer - 15:8\\]
IDAU configuration. 0xC5: Disable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG. All other values: Enable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG."]
pub type IDAU_CFG_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCFG_TI_OPTIONS_SPEC, u8, u8, 8, O>;
#[doc = "Field `C_FA_DIS` reader - 23:16\\]
Option to disable failure analysis without customer password. If C_FA_DIS != 0xC5, CKEY (CKEY0.KEY, CKEY1.KEY, CKEY2.KEY, CKEY2.KEY) must be provided to TI for failure analysis to be possible. 0xC5: Failure analysis without customer password is enabled All other values: Failure analysis without customer password is disabled"]
pub type C_FA_DIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `C_FA_DIS` writer - 23:16\\]
Option to disable failure analysis without customer password. If C_FA_DIS != 0xC5, CKEY (CKEY0.KEY, CKEY1.KEY, CKEY2.KEY, CKEY2.KEY) must be provided to TI for failure analysis to be possible. 0xC5: Failure analysis without customer password is enabled All other values: Failure analysis without customer password is disabled"]
pub type C_FA_DIS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCFG_TI_OPTIONS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
    #[inline(always)]
    pub fn ti_fa_enable(&self) -> TI_FA_ENABLE_R {
        TI_FA_ENABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
IDAU configuration. 0xC5: Disable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG. All other values: Enable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG."]
    #[inline(always)]
    pub fn idau_cfg_enable(&self) -> IDAU_CFG_ENABLE_R {
        IDAU_CFG_ENABLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Option to disable failure analysis without customer password. If C_FA_DIS != 0xC5, CKEY (CKEY0.KEY, CKEY1.KEY, CKEY2.KEY, CKEY2.KEY) must be provided to TI for failure analysis to be possible. 0xC5: Failure analysis without customer password is enabled All other values: Failure analysis without customer password is disabled"]
    #[inline(always)]
    pub fn c_fa_dis(&self) -> C_FA_DIS_R {
        C_FA_DIS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
    #[inline(always)]
    #[must_use]
    pub fn ti_fa_enable(&mut self) -> TI_FA_ENABLE_W<0> {
        TI_FA_ENABLE_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
IDAU configuration. 0xC5: Disable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG. All other values: Enable IDAU configuration controlled by TRUSTZONE_FLASH_CFG and TRUSTZONE_SRAM_CFG."]
    #[inline(always)]
    #[must_use]
    pub fn idau_cfg_enable(&mut self) -> IDAU_CFG_ENABLE_W<8> {
        IDAU_CFG_ENABLE_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Option to disable failure analysis without customer password. If C_FA_DIS != 0xC5, CKEY (CKEY0.KEY, CKEY1.KEY, CKEY2.KEY, CKEY2.KEY) must be provided to TI for failure analysis to be possible. 0xC5: Failure analysis without customer password is enabled All other values: Failure analysis without customer password is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn c_fa_dis(&mut self) -> C_FA_DIS_W<16> {
        C_FA_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TI Options\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_ti_options](index.html) module"]
pub struct CCFG_TI_OPTIONS_SPEC;
impl crate::RegisterSpec for CCFG_TI_OPTIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_ti_options::R](R) reader structure"]
impl crate::Readable for CCFG_TI_OPTIONS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_ti_options::W](W) writer structure"]
impl crate::Writable for CCFG_TI_OPTIONS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_TI_OPTIONS to value 0xffc5_c5c5"]
impl crate::Resettable for CCFG_TI_OPTIONS_SPEC {
    const RESET_VALUE: Self::Ux = 0xffc5_c5c5;
}

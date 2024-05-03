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
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
    #[inline(always)]
    pub fn ti_fa_enable(&self) -> TiFaEnableR {
        TiFaEnableR::new((self.bits & 0xff) as u8)
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
#[doc = "`reset()` method sets CCFG_TI_OPTIONS to value 0xffff_ffc5"]
impl crate::Resettable for CcfgTiOptionsSpec {
    const RESET_VALUE: u32 = 0xffff_ffc5;
}

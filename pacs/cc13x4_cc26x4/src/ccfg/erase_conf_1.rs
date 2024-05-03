#[doc = "Register `ERASE_CONF_1` reader"]
pub type R = crate::R<EraseConf1Spec>;
#[doc = "Register `ERASE_CONF_1` writer"]
pub type W = crate::W<EraseConf1Spec>;
#[doc = "Field `WEPROT_CCFG_N` reader - 0:0\\]
WriteErase protect the CCFG sector Setting this bit = 0 will set FLASH:WEPROT_AUX_BY1.WEPROT_B0_CCFG_BY1 = 1 during boot and hence WriteErase protect the CCFG"]
pub type WeprotCcfgNR = crate::BitReader;
#[doc = "Field `WEPROT_CCFG_N` writer - 0:0\\]
WriteErase protect the CCFG sector Setting this bit = 0 will set FLASH:WEPROT_AUX_BY1.WEPROT_B0_CCFG_BY1 = 1 during boot and hence WriteErase protect the CCFG"]
pub type WeprotCcfgNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
WriteErase protect the CCFG sector Setting this bit = 0 will set FLASH:WEPROT_AUX_BY1.WEPROT_B0_CCFG_BY1 = 1 during boot and hence WriteErase protect the CCFG"]
    #[inline(always)]
    pub fn weprot_ccfg_n(&self) -> WeprotCcfgNR {
        WeprotCcfgNR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
WriteErase protect the CCFG sector Setting this bit = 0 will set FLASH:WEPROT_AUX_BY1.WEPROT_B0_CCFG_BY1 = 1 during boot and hence WriteErase protect the CCFG"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_ccfg_n(&mut self) -> WeprotCcfgNW<EraseConf1Spec> {
        WeprotCcfgNW::new(self, 0)
    }
}
#[doc = "Erase Configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`erase_conf_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`erase_conf_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EraseConf1Spec;
impl crate::RegisterSpec for EraseConf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erase_conf_1::R`](R) reader structure"]
impl crate::Readable for EraseConf1Spec {}
#[doc = "`write(|w| ..)` method takes [`erase_conf_1::W`](W) writer structure"]
impl crate::Writable for EraseConf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERASE_CONF_1 to value 0xffff_ffff"]
impl crate::Resettable for EraseConf1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

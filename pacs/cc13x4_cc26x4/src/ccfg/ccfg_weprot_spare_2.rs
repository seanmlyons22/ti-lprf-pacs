#[doc = "Register `CCFG_WEPROT_SPARE_2` reader"]
pub type R = crate::R<CcfgWeprotSpare2Spec>;
#[doc = "Register `CCFG_WEPROT_SPARE_2` writer"]
pub type W = crate::W<CcfgWeprotSpare2Spec>;
impl W {}
#[doc = "Spare register for WriteErase configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_weprot_spare_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_weprot_spare_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgWeprotSpare2Spec;
impl crate::RegisterSpec for CcfgWeprotSpare2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_weprot_spare_2::R`](R) reader structure"]
impl crate::Readable for CcfgWeprotSpare2Spec {}
#[doc = "`write(|w| ..)` method takes [`ccfg_weprot_spare_2::W`](W) writer structure"]
impl crate::Writable for CcfgWeprotSpare2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCFG_WEPROT_SPARE_2 to value 0xffff_ffff"]
impl crate::Resettable for CcfgWeprotSpare2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

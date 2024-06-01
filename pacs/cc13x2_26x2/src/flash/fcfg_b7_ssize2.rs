#[doc = "Register `FCFG_B7_SSIZE2` reader"]
pub type R = crate::R<FcfgB7Ssize2Spec>;
#[doc = "Register `FCFG_B7_SSIZE2` writer"]
pub type W = crate::W<FcfgB7Ssize2Spec>;
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b7_ssize2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b7_ssize2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgB7Ssize2Spec;
impl crate::RegisterSpec for FcfgB7Ssize2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg_b7_ssize2::R`](R) reader structure"]
impl crate::Readable for FcfgB7Ssize2Spec {}
#[doc = "`write(|w| ..)` method takes [`fcfg_b7_ssize2::W`](W) writer structure"]
impl crate::Writable for FcfgB7Ssize2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG_B7_SSIZE2 to value 0"]
impl crate::Resettable for FcfgB7Ssize2Spec {
    const RESET_VALUE: u32 = 0;
}

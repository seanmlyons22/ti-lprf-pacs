#[doc = "Register `FCFG_B5_SSIZE3` reader"]
pub type R = crate::R<FcfgB5Ssize3Spec>;
#[doc = "Register `FCFG_B5_SSIZE3` writer"]
pub type W = crate::W<FcfgB5Ssize3Spec>;
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b5_ssize3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b5_ssize3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgB5Ssize3Spec;
impl crate::RegisterSpec for FcfgB5Ssize3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg_b5_ssize3::R`](R) reader structure"]
impl crate::Readable for FcfgB5Ssize3Spec {}
#[doc = "`write(|w| ..)` method takes [`fcfg_b5_ssize3::W`](W) writer structure"]
impl crate::Writable for FcfgB5Ssize3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG_B5_SSIZE3 to value 0"]
impl crate::Resettable for FcfgB5Ssize3Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `FCFG_B3_SSIZE1` reader"]
pub type R = crate::R<FcfgB3Ssize1Spec>;
#[doc = "Register `FCFG_B3_SSIZE1` writer"]
pub type W = crate::W<FcfgB3Ssize1Spec>;
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b3_ssize1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b3_ssize1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgB3Ssize1Spec;
impl crate::RegisterSpec for FcfgB3Ssize1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg_b3_ssize1::R`](R) reader structure"]
impl crate::Readable for FcfgB3Ssize1Spec {}
#[doc = "`write(|w| ..)` method takes [`fcfg_b3_ssize1::W`](W) writer structure"]
impl crate::Writable for FcfgB3Ssize1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG_B3_SSIZE1 to value 0"]
impl crate::Resettable for FcfgB3Ssize1Spec {
    const RESET_VALUE: u32 = 0;
}

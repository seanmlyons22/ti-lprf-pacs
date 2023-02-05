#[doc = "Register `RESERVED3` reader"]
pub type R = crate::R<Reserved3Spec>;
#[doc = "Register `RESERVED3` writer"]
pub type W = crate::W<Reserved3Spec>;
impl W {}
#[doc = "UART Reserved Area\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved3Spec;
impl crate::RegisterSpec for Reserved3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved3::R`](R) reader structure"]
impl crate::Readable for Reserved3Spec {}
#[doc = "`write(|w| ..)` method takes [`reserved3::W`](W) writer structure"]
impl crate::Writable for Reserved3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESERVED3 to value 0"]
impl crate::Resettable for Reserved3Spec {
    const RESET_VALUE: u32 = 0;
}

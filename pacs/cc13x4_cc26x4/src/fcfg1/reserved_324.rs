#[doc = "Register `RESERVED_324` reader"]
pub type R = crate::R<Reserved324Spec>;
#[doc = "Register `RESERVED_324` writer"]
pub type W = crate::W<Reserved324Spec>;
impl W {}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved_324::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved_324::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved324Spec;
impl crate::RegisterSpec for Reserved324Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved_324::R`](R) reader structure"]
impl crate::Readable for Reserved324Spec {}
#[doc = "`write(|w| ..)` method takes [`reserved_324::W`](W) writer structure"]
impl crate::Writable for Reserved324Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESERVED_324 to value 0xffff_ffff"]
impl crate::Resettable for Reserved324Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

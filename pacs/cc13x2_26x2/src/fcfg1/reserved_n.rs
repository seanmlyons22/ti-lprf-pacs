#[doc = "Register `RESERVED_N` reader"]
pub type R = crate::R<ReservedNSpec>;
#[doc = "Register `RESERVED_N` writer"]
pub type W = crate::W<ReservedNSpec>;
impl W {}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved_n::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved_n::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReservedNSpec;
impl crate::RegisterSpec for ReservedNSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved_n::R`](R) reader structure"]
impl crate::Readable for ReservedNSpec {}
#[doc = "`write(|w| ..)` method takes [`reserved_n::W`](W) writer structure"]
impl crate::Writable for ReservedNSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESERVED_N to value 0xffff_ffff"]
impl crate::Resettable for ReservedNSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

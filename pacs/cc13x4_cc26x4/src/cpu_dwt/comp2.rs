#[doc = "Register `COMP2` reader"]
pub type R = crate::R<Comp2Spec>;
#[doc = "Register `COMP2` writer"]
pub type W = crate::W<Comp2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<Comp2Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Provides a reference value for use by watchpoint comparator 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp2Spec;
impl crate::RegisterSpec for Comp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp2::R`](R) reader structure"]
impl crate::Readable for Comp2Spec {}
#[doc = "`write(|w| ..)` method takes [`comp2::W`](W) writer structure"]
impl crate::Writable for Comp2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP2 to value 0"]
impl crate::Resettable for Comp2Spec {
    const RESET_VALUE: u32 = 0;
}

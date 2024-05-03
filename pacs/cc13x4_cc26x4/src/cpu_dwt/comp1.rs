#[doc = "Register `COMP1` reader"]
pub type R = crate::R<Comp1Spec>;
#[doc = "Register `COMP1` writer"]
pub type W = crate::W<Comp1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<Comp1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Provides a reference value for use by watchpoint comparator 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp1Spec;
impl crate::RegisterSpec for Comp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1::R`](R) reader structure"]
impl crate::Readable for Comp1Spec {}
#[doc = "`write(|w| ..)` method takes [`comp1::W`](W) writer structure"]
impl crate::Writable for Comp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP1 to value 0"]
impl crate::Resettable for Comp1Spec {
    const RESET_VALUE: u32 = 0;
}

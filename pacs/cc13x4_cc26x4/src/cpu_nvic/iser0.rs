#[doc = "Register `ISER0` reader"]
pub type R = crate::R<Iser0Spec>;
#[doc = "Register `ISER0` writer"]
pub type W = crate::W<Iser0Spec>;
#[doc = "Field `SETENA` reader - 31:0\\]
For SETENA\\[m\\]
in NVIC_ISER*n, indicates whether interrupt 32*n + m is enabled"]
pub type SetenaR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
For SETENA\\[m\\]
in NVIC_ISER*n, indicates whether interrupt 32*n + m is enabled"]
    #[inline(always)]
    pub fn setena(&self) -> SetenaR {
        SetenaR::new(self.bits)
    }
}
impl W {}
#[doc = "Enables or reads the enabled state of each group of 32 interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iser0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iser0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iser0Spec;
impl crate::RegisterSpec for Iser0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iser0::R`](R) reader structure"]
impl crate::Readable for Iser0Spec {}
#[doc = "`write(|w| ..)` method takes [`iser0::W`](W) writer structure"]
impl crate::Writable for Iser0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISER0 to value 0"]
impl crate::Resettable for Iser0Spec {
    const RESET_VALUE: u32 = 0;
}

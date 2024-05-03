#[doc = "Register `ICER0` reader"]
pub type R = crate::R<Icer0Spec>;
#[doc = "Register `ICER0` writer"]
pub type W = crate::W<Icer0Spec>;
#[doc = "Field `CLRENA` reader - 31:0\\]
For CLRENA\\[m\\]
in NVIC_ICER*n, indicates whether interrupt 32*n + m is enabled"]
pub type ClrenaR = crate::FieldReader<u32>;
#[doc = "Field `CLRENA` writer - 31:0\\]
For CLRENA\\[m\\]
in NVIC_ICER*n, indicates whether interrupt 32*n + m is enabled"]
pub type ClrenaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
For CLRENA\\[m\\]
in NVIC_ICER*n, indicates whether interrupt 32*n + m is enabled"]
    #[inline(always)]
    pub fn clrena(&self) -> ClrenaR {
        ClrenaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
For CLRENA\\[m\\]
in NVIC_ICER*n, indicates whether interrupt 32*n + m is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clrena(&mut self) -> ClrenaW<Icer0Spec> {
        ClrenaW::new(self, 0)
    }
}
#[doc = "Clears or reads the enabled state of each group of 32 interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icer0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icer0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icer0Spec;
impl crate::RegisterSpec for Icer0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icer0::R`](R) reader structure"]
impl crate::Readable for Icer0Spec {}
#[doc = "`write(|w| ..)` method takes [`icer0::W`](W) writer structure"]
impl crate::Writable for Icer0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICER0 to value 0"]
impl crate::Resettable for Icer0Spec {
    const RESET_VALUE: u32 = 0;
}

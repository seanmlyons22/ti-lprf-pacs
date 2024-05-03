#[doc = "Register `ITNS0` reader"]
pub type R = crate::R<Itns0Spec>;
#[doc = "Register `ITNS0` writer"]
pub type W = crate::W<Itns0Spec>;
#[doc = "Field `ITNS` reader - 31:0\\]
For ITNS\\[m\\]
in NVIC_ITNS*n, the target Security state for interrupt 32*n+m"]
pub type ItnsR = crate::FieldReader<u32>;
#[doc = "Field `ITNS` writer - 31:0\\]
For ITNS\\[m\\]
in NVIC_ITNS*n, the target Security state for interrupt 32*n+m"]
pub type ItnsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
For ITNS\\[m\\]
in NVIC_ITNS*n, the target Security state for interrupt 32*n+m"]
    #[inline(always)]
    pub fn itns(&self) -> ItnsR {
        ItnsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
For ITNS\\[m\\]
in NVIC_ITNS*n, the target Security state for interrupt 32*n+m"]
    #[inline(always)]
    #[must_use]
    pub fn itns(&mut self) -> ItnsW<Itns0Spec> {
        ItnsW::new(self, 0)
    }
}
#[doc = "For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itns0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itns0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itns0Spec;
impl crate::RegisterSpec for Itns0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itns0::R`](R) reader structure"]
impl crate::Readable for Itns0Spec {}
#[doc = "`write(|w| ..)` method takes [`itns0::W`](W) writer structure"]
impl crate::Writable for Itns0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITNS0 to value 0"]
impl crate::Resettable for Itns0Spec {
    const RESET_VALUE: u32 = 0;
}

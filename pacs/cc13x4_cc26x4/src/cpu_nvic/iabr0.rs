#[doc = "Register `IABR0` reader"]
pub type R = crate::R<Iabr0Spec>;
#[doc = "Register `IABR0` writer"]
pub type W = crate::W<Iabr0Spec>;
#[doc = "Field `ACTIVE` reader - 31:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
pub type ActiveR = crate::FieldReader<u32>;
#[doc = "Field `ACTIVE` writer - 31:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
pub type ActiveW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ActiveW<Iabr0Spec> {
        ActiveW::new(self, 0)
    }
}
#[doc = "For each group of 32 interrupts, shows the active state of each interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iabr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iabr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iabr0Spec;
impl crate::RegisterSpec for Iabr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iabr0::R`](R) reader structure"]
impl crate::Readable for Iabr0Spec {}
#[doc = "`write(|w| ..)` method takes [`iabr0::W`](W) writer structure"]
impl crate::Writable for Iabr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IABR0 to value 0"]
impl crate::Resettable for Iabr0Spec {
    const RESET_VALUE: u32 = 0;
}

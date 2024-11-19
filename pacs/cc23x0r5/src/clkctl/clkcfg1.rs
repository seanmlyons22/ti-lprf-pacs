#[doc = "Register `CLKCFG1` reader"]
pub type R = crate::R<Clkcfg1Spec>;
#[doc = "Register `CLKCFG1` writer"]
pub type W = crate::W<Clkcfg1Spec>;
#[doc = "Field `RESERVED0` reader - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED0` writer - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<Clkcfg1Spec> {
        Reserved0W::new(self, 0)
    }
}
#[doc = "Clock Configuration Register 1. This register shows the IP clock configuration for the system. The configuration is updated through CLKENSET1 and CLKENCLR1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkcfg1Spec;
impl crate::RegisterSpec for Clkcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcfg1::R`](R) reader structure"]
impl crate::Readable for Clkcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`clkcfg1::W`](W) writer structure"]
impl crate::Writable for Clkcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCFG1 to value 0"]
impl crate::Resettable for Clkcfg1Spec {
    const RESET_VALUE: u32 = 0;
}
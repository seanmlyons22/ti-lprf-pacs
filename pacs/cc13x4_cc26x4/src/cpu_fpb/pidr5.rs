#[doc = "Register `PIDR5` reader"]
pub type R = crate::R<Pidr5Spec>;
#[doc = "Register `PIDR5` writer"]
pub type W = crate::W<Pidr5Spec>;
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
    pub fn reserved0(&mut self) -> Reserved0W<Pidr5Spec> {
        Reserved0W::new(self, 0)
    }
}
#[doc = "Provides CoreSight discovery information for the FP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr5Spec;
impl crate::RegisterSpec for Pidr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr5::R`](R) reader structure"]
impl crate::Readable for Pidr5Spec {}
#[doc = "`write(|w| ..)` method takes [`pidr5::W`](W) writer structure"]
impl crate::Writable for Pidr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIDR5 to value 0"]
impl crate::Resettable for Pidr5Spec {
    const RESET_VALUE: u32 = 0;
}

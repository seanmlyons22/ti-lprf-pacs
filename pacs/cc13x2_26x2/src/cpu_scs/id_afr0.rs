#[doc = "Register `ID_AFR0` reader"]
pub type R = crate::R<IdAfr0Spec>;
#[doc = "Register `ID_AFR0` writer"]
pub type W = crate::W<IdAfr0Spec>;
#[doc = "Field `RESERVED0` reader - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits)
    }
}
impl W {}
#[doc = "Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_afr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_afr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdAfr0Spec;
impl crate::RegisterSpec for IdAfr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_afr0::R`](R) reader structure"]
impl crate::Readable for IdAfr0Spec {}
#[doc = "`write(|w| ..)` method takes [`id_afr0::W`](W) writer structure"]
impl crate::Writable for IdAfr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_AFR0 to value 0"]
impl crate::Resettable for IdAfr0Spec {
    const RESET_VALUE: u32 = 0;
}

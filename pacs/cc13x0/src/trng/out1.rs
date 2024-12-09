#[doc = "Register `OUT1` reader"]
pub type R = crate::R<Out1Spec>;
#[doc = "Register `OUT1` writer"]
pub type W = crate::W<Out1Spec>;
#[doc = "Field `VALUE_63_32` reader - 31:0\\]
MSW of 64-bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
pub type Value63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MSW of 64-bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
    #[inline(always)]
    pub fn value_63_32(&self) -> Value63_32R {
        Value63_32R::new(self.bits)
    }
}
impl W {}
#[doc = "Random Number Upper Word Readout Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Out1Spec;
impl crate::RegisterSpec for Out1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out1::R`](R) reader structure"]
impl crate::Readable for Out1Spec {}
#[doc = "`write(|w| ..)` method takes [`out1::W`](W) writer structure"]
impl crate::Writable for Out1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT1 to value 0"]
impl crate::Resettable for Out1Spec {
    const RESET_VALUE: u32 = 0;
}

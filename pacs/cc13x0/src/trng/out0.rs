#[doc = "Register `OUT0` reader"]
pub type R = crate::R<Out0Spec>;
#[doc = "Register `OUT0` writer"]
pub type W = crate::W<Out0Spec>;
#[doc = "Field `VALUE_31_0` reader - 31:0\\]
LSW of 64- bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
pub type Value31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
LSW of 64- bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
    #[inline(always)]
    pub fn value_31_0(&self) -> Value31_0R {
        Value31_0R::new(self.bits)
    }
}
impl W {}
#[doc = "Random Number Lower Word Readout Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Out0Spec;
impl crate::RegisterSpec for Out0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out0::R`](R) reader structure"]
impl crate::Readable for Out0Spec {}
#[doc = "`write(|w| ..)` method takes [`out0::W`](W) writer structure"]
impl crate::Writable for Out0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT0 to value 0"]
impl crate::Resettable for Out0Spec {
    const RESET_VALUE: u32 = 0;
}

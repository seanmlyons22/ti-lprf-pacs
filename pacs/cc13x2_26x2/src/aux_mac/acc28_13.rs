#[doc = "Register `ACC28_13` reader"]
pub type R = crate::R<Acc28_13Spec>;
#[doc = "Register `ACC28_13` writer"]
pub type W = crate::W<Acc28_13Spec>;
#[doc = "Field `VALUE` reader - 15:0\\]
Value of the accumulator, bits 28:13."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Value of the accumulator, bits 28:13."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Accumulator Bits 28:13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc28_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc28_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Acc28_13Spec;
impl crate::RegisterSpec for Acc28_13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acc28_13::R`](R) reader structure"]
impl crate::Readable for Acc28_13Spec {}
#[doc = "`write(|w| ..)` method takes [`acc28_13::W`](W) writer structure"]
impl crate::Writable for Acc28_13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACC28_13 to value 0"]
impl crate::Resettable for Acc28_13Spec {
    const RESET_VALUE: u32 = 0;
}

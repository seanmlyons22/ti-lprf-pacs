#[doc = "Register `ACC37_22` reader"]
pub type R = crate::R<Acc37_22Spec>;
#[doc = "Register `ACC37_22` writer"]
pub type W = crate::W<Acc37_22Spec>;
#[doc = "Field `VALUE` reader - 15:0\\]
Value of the accumulator, bits 37:22."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - 15:0\\]
Value of the accumulator, bits 37:22."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Value of the accumulator, bits 37:22."]
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
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Value of the accumulator, bits 37:22."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<Acc37_22Spec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Acc37_22Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Accumulator Bits 37:22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc37_22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc37_22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Acc37_22Spec;
impl crate::RegisterSpec for Acc37_22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acc37_22::R`](R) reader structure"]
impl crate::Readable for Acc37_22Spec {}
#[doc = "`write(|w| ..)` method takes [`acc37_22::W`](W) writer structure"]
impl crate::Writable for Acc37_22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACC37_22 to value 0"]
impl crate::Resettable for Acc37_22Spec {
    const RESET_VALUE: u32 = 0;
}

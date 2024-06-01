#[doc = "Register `ACC16_1` reader"]
pub type R = crate::R<Acc16_1Spec>;
#[doc = "Register `ACC16_1` writer"]
pub type W = crate::W<Acc16_1Spec>;
#[doc = "Field `VALUE` reader - 15:0\\]
Value of the accumulator, bits 16:1."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - 15:0\\]
Value of the accumulator, bits 16:1."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Value of the accumulator, bits 16:1."]
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
Value of the accumulator, bits 16:1."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<Acc16_1Spec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Acc16_1Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Accumulator Bits 16:1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc16_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc16_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Acc16_1Spec;
impl crate::RegisterSpec for Acc16_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acc16_1::R`](R) reader structure"]
impl crate::Readable for Acc16_1Spec {}
#[doc = "`write(|w| ..)` method takes [`acc16_1::W`](W) writer structure"]
impl crate::Writable for Acc16_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACC16_1 to value 0"]
impl crate::Resettable for Acc16_1Spec {
    const RESET_VALUE: u32 = 0;
}

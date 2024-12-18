#[doc = "Register `OP0U` reader"]
pub type R = crate::R<Op0uSpec>;
#[doc = "Register `OP0U` writer"]
pub type W = crate::W<Op0uSpec>;
#[doc = "Field `OP0_VALUE` writer - 15:0\\]
Unsigned operand 0. Operand for multiply, multiply-and-accumulate, or 32-bit add operations."]
pub type Op0ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Unsigned operand 0. Operand for multiply, multiply-and-accumulate, or 32-bit add operations."]
    #[inline(always)]
    #[must_use]
    pub fn op0_value(&mut self) -> Op0ValueW<Op0uSpec> {
        Op0ValueW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Op0uSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Unsigned Operand 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op0u::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op0u::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Op0uSpec;
impl crate::RegisterSpec for Op0uSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`op0u::R`](R) reader structure"]
impl crate::Readable for Op0uSpec {}
#[doc = "`write(|w| ..)` method takes [`op0u::W`](W) writer structure"]
impl crate::Writable for Op0uSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OP0U to value 0"]
impl crate::Resettable for Op0uSpec {
    const RESET_VALUE: u32 = 0;
}

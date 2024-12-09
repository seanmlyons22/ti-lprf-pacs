#[doc = "Register `OP0S` reader"]
pub type R = crate::R<Op0sSpec>;
#[doc = "Register `OP0S` writer"]
pub type W = crate::W<Op0sSpec>;
#[doc = "Field `OP0_VALUE` writer - 15:0\\]
Signed operand 0. Operand for multiply, multiply-and-accumulate, or 32-bit add operations."]
pub type Op0ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Signed operand 0. Operand for multiply, multiply-and-accumulate, or 32-bit add operations."]
    #[inline(always)]
    #[must_use]
    pub fn op0_value(&mut self) -> Op0ValueW<Op0sSpec> {
        Op0ValueW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Op0sSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Signed Operand 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op0s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op0s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Op0sSpec;
impl crate::RegisterSpec for Op0sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`op0s::R`](R) reader structure"]
impl crate::Readable for Op0sSpec {}
#[doc = "`write(|w| ..)` method takes [`op0s::W`](W) writer structure"]
impl crate::Writable for Op0sSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OP0S to value 0"]
impl crate::Resettable for Op0sSpec {
    const RESET_VALUE: u32 = 0;
}

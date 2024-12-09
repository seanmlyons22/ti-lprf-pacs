#[doc = "Register `OP1SMAC` reader"]
pub type R = crate::R<Op1smacSpec>;
#[doc = "Register `OP1SMAC` writer"]
pub type W = crate::W<Op1smacSpec>;
#[doc = "Field `OP1_VALUE` writer - 15:0\\]
Signed operand 1 and multiply-accumulation trigger. Write OP1_VALUE to set signed operand 1 and trigger the following operation: When operand 0 was written to OP0S.OP0_VALUE: ACC = ACC + ( OP1_VALUE * OP0S.OP0_VALUE ). When operand 0 was written to OP0U.OP0_VALUE: ACC = ACC + ( OP1_VALUE * OP0U.OP0_VALUE )."]
pub type Op1ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Signed operand 1 and multiply-accumulation trigger. Write OP1_VALUE to set signed operand 1 and trigger the following operation: When operand 0 was written to OP0S.OP0_VALUE: ACC = ACC + ( OP1_VALUE * OP0S.OP0_VALUE ). When operand 0 was written to OP0U.OP0_VALUE: ACC = ACC + ( OP1_VALUE * OP0U.OP0_VALUE )."]
    #[inline(always)]
    #[must_use]
    pub fn op1_value(&mut self) -> Op1ValueW<Op1smacSpec> {
        Op1ValueW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Op1smacSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Signed Operand 1 and Multiply-Accumulate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op1smac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op1smac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Op1smacSpec;
impl crate::RegisterSpec for Op1smacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`op1smac::R`](R) reader structure"]
impl crate::Readable for Op1smacSpec {}
#[doc = "`write(|w| ..)` method takes [`op1smac::W`](W) writer structure"]
impl crate::Writable for Op1smacSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OP1SMAC to value 0"]
impl crate::Resettable for Op1smacSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `OP1UADD32` reader"]
pub type R = crate::R<Op1uadd32Spec>;
#[doc = "Register `OP1UADD32` writer"]
pub type W = crate::W<Op1uadd32Spec>;
#[doc = "Field `OP1_VALUE` writer - 15:0\\]
Upper half of unsigned 32-bit operand and addition trigger. Write OP1_VALUE to set upper half of unsigned 32-bit operand and trigger the following operation: When lower half of 32-bit operand was written to OP0S.OP0_VALUE: ACC = ACC + (( OP1_VALUE &lt;&lt; 16) | OP0S.OP0_VALUE ). When lower half of 32-bit operand was written to OP0U.OP0_VALUE: ACC = ACC + (( OP1_VALUE &lt;&lt; 16) | OP0U.OP0_VALUE )."]
pub type Op1ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Upper half of unsigned 32-bit operand and addition trigger. Write OP1_VALUE to set upper half of unsigned 32-bit operand and trigger the following operation: When lower half of 32-bit operand was written to OP0S.OP0_VALUE: ACC = ACC + (( OP1_VALUE &lt;&lt; 16) | OP0S.OP0_VALUE ). When lower half of 32-bit operand was written to OP0U.OP0_VALUE: ACC = ACC + (( OP1_VALUE &lt;&lt; 16) | OP0U.OP0_VALUE )."]
    #[inline(always)]
    #[must_use]
    pub fn op1_value(&mut self) -> Op1ValueW<Op1uadd32Spec> {
        Op1ValueW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Op1uadd32Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Unsigned Operand 1 and 32-bit Addition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op1uadd32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op1uadd32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Op1uadd32Spec;
impl crate::RegisterSpec for Op1uadd32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`op1uadd32::R`](R) reader structure"]
impl crate::Readable for Op1uadd32Spec {}
#[doc = "`write(|w| ..)` method takes [`op1uadd32::W`](W) writer structure"]
impl crate::Writable for Op1uadd32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OP1UADD32 to value 0"]
impl crate::Resettable for Op1uadd32Spec {
    const RESET_VALUE: u32 = 0;
}

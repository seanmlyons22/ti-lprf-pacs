#[doc = "Register `C0CC` reader"]
pub type R = crate::R<C0ccSpec>;
#[doc = "Register `C0CC` writer"]
pub type W = crate::W<C0ccSpec>;
#[doc = "Field `VAL` reader - 15:0\\]
Capture Compare value. User defined compare value or channel-updated capture value. A read or write to this register will clear the RIS.C0CC interrupt. Compare mode: VAL is compared against CNTR.VAL and an event is generated as specified by C0CFG.CCACT when these are equal. Capture mode: The current counter value is stored in VAL when a capture event occurs. C0CFG.CCACT determines if VAL is a signal period or a regular capture value."]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - 15:0\\]
Capture Compare value. User defined compare value or channel-updated capture value. A read or write to this register will clear the RIS.C0CC interrupt. Compare mode: VAL is compared against CNTR.VAL and an event is generated as specified by C0CFG.CCACT when these are equal. Capture mode: The current counter value is stored in VAL when a capture event occurs. C0CFG.CCACT determines if VAL is a signal period or a regular capture value."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Capture Compare value. User defined compare value or channel-updated capture value. A read or write to this register will clear the RIS.C0CC interrupt. Compare mode: VAL is compared against CNTR.VAL and an event is generated as specified by C0CFG.CCACT when these are equal. Capture mode: The current counter value is stored in VAL when a capture event occurs. C0CFG.CCACT determines if VAL is a signal period or a regular capture value."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
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
Capture Compare value. User defined compare value or channel-updated capture value. A read or write to this register will clear the RIS.C0CC interrupt. Compare mode: VAL is compared against CNTR.VAL and an event is generated as specified by C0CFG.CCACT when these are equal. Capture mode: The current counter value is stored in VAL when a capture event occurs. C0CFG.CCACT determines if VAL is a signal period or a regular capture value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<C0ccSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<C0ccSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Channel 0 Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c0cc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c0cc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C0ccSpec;
impl crate::RegisterSpec for C0ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c0cc::R`](R) reader structure"]
impl crate::Readable for C0ccSpec {}
#[doc = "`write(|w| ..)` method takes [`c0cc::W`](W) writer structure"]
impl crate::Writable for C0ccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C0CC to value 0"]
impl crate::Resettable for C0ccSpec {
    const RESET_VALUE: u32 = 0;
}

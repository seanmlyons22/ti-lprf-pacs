#[doc = "Register `C2CC` reader"]
pub type R = crate::R<C2ccSpec>;
#[doc = "Register `C2CC` writer"]
pub type W = crate::W<C2ccSpec>;
#[doc = "Field `VAL` reader - 23:0\\]
Capture Compare value. User defined compare value or channel-updated capture value. A read or write to this register will clear the RIS.C2CC interrupt. Compare mode: VAL is compared against CNTR.VAL and an event is generated as specified by C2CFG.CCACT when these are equal. Capture mode: The current counter value is stored in VAL when a capture event occurs. C2CFG.CCACT determines if VAL is a signal period or a regular capture value."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 23:0\\]
Capture Compare value. User defined compare value or channel-updated capture value. A read or write to this register will clear the RIS.C2CC interrupt. Compare mode: VAL is compared against CNTR.VAL and an event is generated as specified by C2CFG.CCACT when these are equal. Capture mode: The current counter value is stored in VAL when a capture event occurs. C2CFG.CCACT determines if VAL is a signal period or a regular capture value."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Capture Compare value. User defined compare value or channel-updated capture value. A read or write to this register will clear the RIS.C2CC interrupt. Compare mode: VAL is compared against CNTR.VAL and an event is generated as specified by C2CFG.CCACT when these are equal. Capture mode: The current counter value is stored in VAL when a capture event occurs. C2CFG.CCACT determines if VAL is a signal period or a regular capture value."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Capture Compare value. User defined compare value or channel-updated capture value. A read or write to this register will clear the RIS.C2CC interrupt. Compare mode: VAL is compared against CNTR.VAL and an event is generated as specified by C2CFG.CCACT when these are equal. Capture mode: The current counter value is stored in VAL when a capture event occurs. C2CFG.CCACT determines if VAL is a signal period or a regular capture value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<C2ccSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Channel 2 Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2cc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2cc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2ccSpec;
impl crate::RegisterSpec for C2ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2cc::R`](R) reader structure"]
impl crate::Readable for C2ccSpec {}
#[doc = "`write(|w| ..)` method takes [`c2cc::W`](W) writer structure"]
impl crate::Writable for C2ccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2CC to value 0"]
impl crate::Resettable for C2ccSpec {
    const RESET_VALUE: u32 = 0;
}

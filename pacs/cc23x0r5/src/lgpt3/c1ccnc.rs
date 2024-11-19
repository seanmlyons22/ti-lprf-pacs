#[doc = "Register `C1CCNC` reader"]
pub type R = crate::R<C1ccncSpec>;
#[doc = "Register `C1CCNC` writer"]
pub type W = crate::W<C1ccncSpec>;
#[doc = "Field `VAL` reader - 23:0\\]
Capture Compare value. User defined compare value or channel-updated capture value. A read or write to this register will not clear the RIS.C1CC interrupt. Compare mode: VAL is compared against CNTR.VAL and an event is generated as specified by C1CFG.CCACT when these are equal. Capture mode: The current counter value is stored in VAL when a capture event occurs. C1CFG.CCACT determines if VAL is a signal period or a regular capture value."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 23:0\\]
Capture Compare value. User defined compare value or channel-updated capture value. A read or write to this register will not clear the RIS.C1CC interrupt. Compare mode: VAL is compared against CNTR.VAL and an event is generated as specified by C1CFG.CCACT when these are equal. Capture mode: The current counter value is stored in VAL when a capture event occurs. C1CFG.CCACT determines if VAL is a signal period or a regular capture value."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Capture Compare value. User defined compare value or channel-updated capture value. A read or write to this register will not clear the RIS.C1CC interrupt. Compare mode: VAL is compared against CNTR.VAL and an event is generated as specified by C1CFG.CCACT when these are equal. Capture mode: The current counter value is stored in VAL when a capture event occurs. C1CFG.CCACT determines if VAL is a signal period or a regular capture value."]
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
Capture Compare value. User defined compare value or channel-updated capture value. A read or write to this register will not clear the RIS.C1CC interrupt. Compare mode: VAL is compared against CNTR.VAL and an event is generated as specified by C1CFG.CCACT when these are equal. Capture mode: The current counter value is stored in VAL when a capture event occurs. C1CFG.CCACT determines if VAL is a signal period or a regular capture value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<C1ccncSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<C1ccncSpec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "Channel 1 Capture Compare No Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1ccnc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1ccnc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1ccncSpec;
impl crate::RegisterSpec for C1ccncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1ccnc::R`](R) reader structure"]
impl crate::Readable for C1ccncSpec {}
#[doc = "`write(|w| ..)` method takes [`c1ccnc::W`](W) writer structure"]
impl crate::Writable for C1ccncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1CCNC to value 0"]
impl crate::Resettable for C1ccncSpec {
    const RESET_VALUE: u32 = 0;
}
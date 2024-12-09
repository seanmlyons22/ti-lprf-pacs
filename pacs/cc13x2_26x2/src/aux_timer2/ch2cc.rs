#[doc = "Register `CH2CC` reader"]
pub type R = crate::R<Ch2ccSpec>;
#[doc = "Register `CH2CC` writer"]
pub type W = crate::W<Ch2ccSpec>;
#[doc = "Field `VALUE` reader - 15:0\\]
Capture Compare value. 16-bit user defined compare value or channel-updated capture value. Compare mode: VALUE is compared against CNTR.VALUE and an event is generated as specified by CH2EVCFG.CCACT when these are equal. Capture mode: The current counter value is stored in VALUE when a capture event occurs. CH2EVCFG.CCACT determines if VALUE is a signal period or a regular capture value."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - 15:0\\]
Capture Compare value. 16-bit user defined compare value or channel-updated capture value. Compare mode: VALUE is compared against CNTR.VALUE and an event is generated as specified by CH2EVCFG.CCACT when these are equal. Capture mode: The current counter value is stored in VALUE when a capture event occurs. CH2EVCFG.CCACT determines if VALUE is a signal period or a regular capture value."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Capture Compare value. 16-bit user defined compare value or channel-updated capture value. Compare mode: VALUE is compared against CNTR.VALUE and an event is generated as specified by CH2EVCFG.CCACT when these are equal. Capture mode: The current counter value is stored in VALUE when a capture event occurs. CH2EVCFG.CCACT determines if VALUE is a signal period or a regular capture value."]
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
Capture Compare value. 16-bit user defined compare value or channel-updated capture value. Compare mode: VALUE is compared against CNTR.VALUE and an event is generated as specified by CH2EVCFG.CCACT when these are equal. Capture mode: The current counter value is stored in VALUE when a capture event occurs. CH2EVCFG.CCACT determines if VALUE is a signal period or a regular capture value."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<Ch2ccSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Channel 2 Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2ccSpec;
impl crate::RegisterSpec for Ch2ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2cc::R`](R) reader structure"]
impl crate::Readable for Ch2ccSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2cc::W`](W) writer structure"]
impl crate::Writable for Ch2ccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2CC to value 0"]
impl crate::Resettable for Ch2ccSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `STMPINTRIG` reader"]
pub type R = crate::R<StmpintrigSpec>;
#[doc = "Register `STMPINTRIG` writer"]
pub type W = crate::W<StmpintrigSpec>;
#[doc = "Field `IN_START_WCNT` reader - 15:0\\]
Compare value used to start the incoming audio streams. This bit field shall equal the WCLK counter value during the WCLK period in which the first input word(s) are sampled and stored to memory (that is the sample at the start of the very first DMA input buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as inputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and at least 32 BCLK cycle ticks have happened. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
pub type InStartWcntR = crate::FieldReader<u16>;
#[doc = "Field `IN_START_WCNT` writer - 15:0\\]
Compare value used to start the incoming audio streams. This bit field shall equal the WCLK counter value during the WCLK period in which the first input word(s) are sampled and stored to memory (that is the sample at the start of the very first DMA input buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as inputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and at least 32 BCLK cycle ticks have happened. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
pub type InStartWcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Compare value used to start the incoming audio streams. This bit field shall equal the WCLK counter value during the WCLK period in which the first input word(s) are sampled and stored to memory (that is the sample at the start of the very first DMA input buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as inputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and at least 32 BCLK cycle ticks have happened. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[inline(always)]
    pub fn in_start_wcnt(&self) -> InStartWcntR {
        InStartWcntR::new((self.bits & 0xffff) as u16)
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
Compare value used to start the incoming audio streams. This bit field shall equal the WCLK counter value during the WCLK period in which the first input word(s) are sampled and stored to memory (that is the sample at the start of the very first DMA input buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as inputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and at least 32 BCLK cycle ticks have happened. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[inline(always)]
    #[must_use]
    pub fn in_start_wcnt(&mut self) -> InStartWcntW<StmpintrigSpec> {
        InStartWcntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<StmpintrigSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "WCLK Counter Trigger Value for Input Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpintrig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpintrig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StmpintrigSpec;
impl crate::RegisterSpec for StmpintrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmpintrig::R`](R) reader structure"]
impl crate::Readable for StmpintrigSpec {}
#[doc = "`write(|w| ..)` method takes [`stmpintrig::W`](W) writer structure"]
impl crate::Writable for StmpintrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMPINTRIG to value 0"]
impl crate::Resettable for StmpintrigSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `STMPOUTTRIG` reader"]
pub type R = crate::R<StmpouttrigSpec>;
#[doc = "Register `STMPOUTTRIG` writer"]
pub type W = crate::W<StmpouttrigSpec>;
#[doc = "Field `OUT_START_WCNT` reader - 15:0\\]
Compare value used to start the outgoing audio streams. This bit field must equal the WCLK counter value during the WCLK period in which the first output word(s) read from memory are clocked out (that is the sample at the start of the very first DMA output buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as outputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and 32 BCLK cycle ticks have happened. - 2 samples have been preloaded from memory (examine the AIFOUTPTR register if necessary). Note: The memory read access is only performed when required, that is channels 0/1 must be selected in AIFWMASK0/AIFWMASK1. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
pub type OutStartWcntR = crate::FieldReader<u16>;
#[doc = "Field `OUT_START_WCNT` writer - 15:0\\]
Compare value used to start the outgoing audio streams. This bit field must equal the WCLK counter value during the WCLK period in which the first output word(s) read from memory are clocked out (that is the sample at the start of the very first DMA output buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as outputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and 32 BCLK cycle ticks have happened. - 2 samples have been preloaded from memory (examine the AIFOUTPTR register if necessary). Note: The memory read access is only performed when required, that is channels 0/1 must be selected in AIFWMASK0/AIFWMASK1. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
pub type OutStartWcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Compare value used to start the outgoing audio streams. This bit field must equal the WCLK counter value during the WCLK period in which the first output word(s) read from memory are clocked out (that is the sample at the start of the very first DMA output buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as outputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and 32 BCLK cycle ticks have happened. - 2 samples have been preloaded from memory (examine the AIFOUTPTR register if necessary). Note: The memory read access is only performed when required, that is channels 0/1 must be selected in AIFWMASK0/AIFWMASK1. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[inline(always)]
    pub fn out_start_wcnt(&self) -> OutStartWcntR {
        OutStartWcntR::new((self.bits & 0xffff) as u16)
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
Compare value used to start the outgoing audio streams. This bit field must equal the WCLK counter value during the WCLK period in which the first output word(s) read from memory are clocked out (that is the sample at the start of the very first DMA output buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as outputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and 32 BCLK cycle ticks have happened. - 2 samples have been preloaded from memory (examine the AIFOUTPTR register if necessary). Note: The memory read access is only performed when required, that is channels 0/1 must be selected in AIFWMASK0/AIFWMASK1. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[inline(always)]
    #[must_use]
    pub fn out_start_wcnt(&mut self) -> OutStartWcntW<StmpouttrigSpec> {
        OutStartWcntW::new(self, 0)
    }
}
#[doc = "WCLK Counter Trigger Value for Output Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpouttrig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpouttrig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StmpouttrigSpec;
impl crate::RegisterSpec for StmpouttrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmpouttrig::R`](R) reader structure"]
impl crate::Readable for StmpouttrigSpec {}
#[doc = "`write(|w| ..)` method takes [`stmpouttrig::W`](W) writer structure"]
impl crate::Writable for StmpouttrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMPOUTTRIG to value 0"]
impl crate::Resettable for StmpouttrigSpec {
    const RESET_VALUE: u32 = 0;
}

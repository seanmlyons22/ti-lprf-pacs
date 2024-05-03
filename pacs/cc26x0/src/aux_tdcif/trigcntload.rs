#[doc = "Register `TRIGCNTLOAD` reader"]
pub type R = crate::R<TrigcntloadSpec>;
#[doc = "Register `TRIGCNTLOAD` writer"]
pub type W = crate::W<TrigcntloadSpec>;
#[doc = "Field `CNT` reader - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
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
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<TrigcntloadSpec> {
        CntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<TrigcntloadSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Trigger Counter Load Stop-counter load.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigcntload::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigcntload::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrigcntloadSpec;
impl crate::RegisterSpec for TrigcntloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigcntload::R`](R) reader structure"]
impl crate::Readable for TrigcntloadSpec {}
#[doc = "`write(|w| ..)` method takes [`trigcntload::W`](W) writer structure"]
impl crate::Writable for TrigcntloadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIGCNTLOAD to value 0"]
impl crate::Resettable for TrigcntloadSpec {
    const RESET_VALUE: u32 = 0;
}

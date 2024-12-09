#[doc = "Register `TRIGCNT` reader"]
pub type R = crate::R<TrigcntSpec>;
#[doc = "Register `TRIGCNT` writer"]
pub type W = crate::W<TrigcntSpec>;
#[doc = "Field `CNT` reader - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
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
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<TrigcntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Trigger Counter Stop-counter control and status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrigcntSpec;
impl crate::RegisterSpec for TrigcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigcnt::R`](R) reader structure"]
impl crate::Readable for TrigcntSpec {}
#[doc = "`write(|w| ..)` method takes [`trigcnt::W`](W) writer structure"]
impl crate::Writable for TrigcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIGCNT to value 0"]
impl crate::Resettable for TrigcntSpec {
    const RESET_VALUE: u32 = 0;
}

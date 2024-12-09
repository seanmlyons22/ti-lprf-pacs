#[doc = "Register `PTGTNC` reader"]
pub type R = crate::R<PtgtncSpec>;
#[doc = "Register `PTGTNC` writer"]
pub type W = crate::W<PtgtncSpec>;
#[doc = "Field `VAL` reader - 15:0\\]
A read or write to this register will not clear the RIS.TGT interrupt. If CTL.MODE != QDEC. Target value for next counter period. The timer copies VAL to TGT.VAL when CNTR.VAL becomes 0. The copy does not happen when restarting the timer. This is useful to avoid period jitter in PWM applications with time-varying period, sometimes referenced as phase corrected PWM. If CTL.MODE = QDEC. The CNTR.VAL is updated with VAL on IDX. VAL is not loaded into TGT.VAL when CNTR.VAL becomes 0."]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - 15:0\\]
A read or write to this register will not clear the RIS.TGT interrupt. If CTL.MODE != QDEC. Target value for next counter period. The timer copies VAL to TGT.VAL when CNTR.VAL becomes 0. The copy does not happen when restarting the timer. This is useful to avoid period jitter in PWM applications with time-varying period, sometimes referenced as phase corrected PWM. If CTL.MODE = QDEC. The CNTR.VAL is updated with VAL on IDX. VAL is not loaded into TGT.VAL when CNTR.VAL becomes 0."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
A read or write to this register will not clear the RIS.TGT interrupt. If CTL.MODE != QDEC. Target value for next counter period. The timer copies VAL to TGT.VAL when CNTR.VAL becomes 0. The copy does not happen when restarting the timer. This is useful to avoid period jitter in PWM applications with time-varying period, sometimes referenced as phase corrected PWM. If CTL.MODE = QDEC. The CNTR.VAL is updated with VAL on IDX. VAL is not loaded into TGT.VAL when CNTR.VAL becomes 0."]
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
A read or write to this register will not clear the RIS.TGT interrupt. If CTL.MODE != QDEC. Target value for next counter period. The timer copies VAL to TGT.VAL when CNTR.VAL becomes 0. The copy does not happen when restarting the timer. This is useful to avoid period jitter in PWM applications with time-varying period, sometimes referenced as phase corrected PWM. If CTL.MODE = QDEC. The CNTR.VAL is updated with VAL on IDX. VAL is not loaded into TGT.VAL when CNTR.VAL becomes 0."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<PtgtncSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Pipeline Target No Clear Use this register to read or write to PTGT without clearing the RIS.ZERO and RIS.TGT interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptgtnc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptgtnc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtgtncSpec;
impl crate::RegisterSpec for PtgtncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptgtnc::R`](R) reader structure"]
impl crate::Readable for PtgtncSpec {}
#[doc = "`write(|w| ..)` method takes [`ptgtnc::W`](W) writer structure"]
impl crate::Writable for PtgtncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTGTNC to value 0"]
impl crate::Resettable for PtgtncSpec {
    const RESET_VALUE: u32 = 0;
}

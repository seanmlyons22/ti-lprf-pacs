#[doc = "Register `PTGT` reader"]
pub type R = crate::R<PtgtSpec>;
#[doc = "Register `PTGT` writer"]
pub type W = crate::W<PtgtSpec>;
#[doc = "Field `VAL` reader - 15:0\\]
The pipleline target value."]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - 15:0\\]
The pipleline target value."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The pipleline target value."]
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
The pipleline target value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<PtgtSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<PtgtSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Pipeline Target A read or write to this register will clear the RIS.ZERO and RIS.TGT interrupt. If CTL.MODE != QDEC. Target value for next counter period. The timer will copy PTGT.VAL to TGT.VAL on the upcoming CNTR zero crossing only if PTGT.VAL has been written. The copy does not happen when restarting the timer. This is useful to avoid period jitter in PWM applications with time-varying period, sometimes referenced as phase corrected PWM. If CTL.MODE = QDEC The CNTR value is updated with VALUE on IDX if the counter is counting down. If the counter is counting up, CNTR is loaded with zero on IDX. In this mode the VALUE is not loaded into TGT on zero crossing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptgt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptgt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtgtSpec;
impl crate::RegisterSpec for PtgtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptgt::R`](R) reader structure"]
impl crate::Readable for PtgtSpec {}
#[doc = "`write(|w| ..)` method takes [`ptgt::W`](W) writer structure"]
impl crate::Writable for PtgtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTGT to value 0"]
impl crate::Resettable for PtgtSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `PC2CCNC` reader"]
pub type R = crate::R<Pc2ccncSpec>;
#[doc = "Register `PC2CCNC` writer"]
pub type W = crate::W<Pc2ccncSpec>;
#[doc = "Field `VAL` reader - 23:0\\]
Pipeline Capture Compare value. User defined pipeline compare value or channel-updated capture value. A read or write to this register will not clear the RIS.C2CC interrupt. Compare mode: An update of VAL will be transferred to C2CC.VAL when the next CNTR.VAL is zero and CTL.MODE is different from DIS. This is useful for PWM generation and prevents jitter on the edges of the generated signal. Capture mode: When C2CFG.CCACT equals PER_PULSE_WIDTH_MEAS then VAL contains the width of the low or high phase of the selected signal. This is specified by C2CFG.EDGE."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 23:0\\]
Pipeline Capture Compare value. User defined pipeline compare value or channel-updated capture value. A read or write to this register will not clear the RIS.C2CC interrupt. Compare mode: An update of VAL will be transferred to C2CC.VAL when the next CNTR.VAL is zero and CTL.MODE is different from DIS. This is useful for PWM generation and prevents jitter on the edges of the generated signal. Capture mode: When C2CFG.CCACT equals PER_PULSE_WIDTH_MEAS then VAL contains the width of the low or high phase of the selected signal. This is specified by C2CFG.EDGE."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Pipeline Capture Compare value. User defined pipeline compare value or channel-updated capture value. A read or write to this register will not clear the RIS.C2CC interrupt. Compare mode: An update of VAL will be transferred to C2CC.VAL when the next CNTR.VAL is zero and CTL.MODE is different from DIS. This is useful for PWM generation and prevents jitter on the edges of the generated signal. Capture mode: When C2CFG.CCACT equals PER_PULSE_WIDTH_MEAS then VAL contains the width of the low or high phase of the selected signal. This is specified by C2CFG.EDGE."]
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
Pipeline Capture Compare value. User defined pipeline compare value or channel-updated capture value. A read or write to this register will not clear the RIS.C2CC interrupt. Compare mode: An update of VAL will be transferred to C2CC.VAL when the next CNTR.VAL is zero and CTL.MODE is different from DIS. This is useful for PWM generation and prevents jitter on the edges of the generated signal. Capture mode: When C2CFG.CCACT equals PER_PULSE_WIDTH_MEAS then VAL contains the width of the low or high phase of the selected signal. This is specified by C2CFG.EDGE."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Pc2ccncSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<Pc2ccncSpec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "Pipeline Channel 2 Capture Compare No Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc2ccnc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc2ccnc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc2ccncSpec;
impl crate::RegisterSpec for Pc2ccncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc2ccnc::R`](R) reader structure"]
impl crate::Readable for Pc2ccncSpec {}
#[doc = "`write(|w| ..)` method takes [`pc2ccnc::W`](W) writer structure"]
impl crate::Writable for Pc2ccncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC2CCNC to value 0"]
impl crate::Resettable for Pc2ccncSpec {
    const RESET_VALUE: u32 = 0;
}

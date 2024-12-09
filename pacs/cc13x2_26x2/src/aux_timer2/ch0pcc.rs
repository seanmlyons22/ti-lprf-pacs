#[doc = "Register `CH0PCC` reader"]
pub type R = crate::R<Ch0pccSpec>;
#[doc = "Register `CH0PCC` writer"]
pub type W = crate::W<Ch0pccSpec>;
#[doc = "Field `VALUE` reader - 15:0\\]
Pipeline Capture Compare value. 16-bit user defined pipeline compare value or channel-updated capture value. Compare mode: An update of VALUE will be transferred to CH0CC.VALUE when the next CNTR.VALUE is zero and CTL.MODE is different from DIS. This is useful for PWM generation and prevents jitter on the edges of the generated signal. Capture mode: When CH0EVCFG.CCACT equals PER_PULSE_WIDTH_MEAS then VALUE contains the width of the low or high phase of the selected signal. This is specified by CH0CCFG.EDGE and CH0CCFG.CAPT_SRC."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - 15:0\\]
Pipeline Capture Compare value. 16-bit user defined pipeline compare value or channel-updated capture value. Compare mode: An update of VALUE will be transferred to CH0CC.VALUE when the next CNTR.VALUE is zero and CTL.MODE is different from DIS. This is useful for PWM generation and prevents jitter on the edges of the generated signal. Capture mode: When CH0EVCFG.CCACT equals PER_PULSE_WIDTH_MEAS then VALUE contains the width of the low or high phase of the selected signal. This is specified by CH0CCFG.EDGE and CH0CCFG.CAPT_SRC."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Pipeline Capture Compare value. 16-bit user defined pipeline compare value or channel-updated capture value. Compare mode: An update of VALUE will be transferred to CH0CC.VALUE when the next CNTR.VALUE is zero and CTL.MODE is different from DIS. This is useful for PWM generation and prevents jitter on the edges of the generated signal. Capture mode: When CH0EVCFG.CCACT equals PER_PULSE_WIDTH_MEAS then VALUE contains the width of the low or high phase of the selected signal. This is specified by CH0CCFG.EDGE and CH0CCFG.CAPT_SRC."]
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
Pipeline Capture Compare value. 16-bit user defined pipeline compare value or channel-updated capture value. Compare mode: An update of VALUE will be transferred to CH0CC.VALUE when the next CNTR.VALUE is zero and CTL.MODE is different from DIS. This is useful for PWM generation and prevents jitter on the edges of the generated signal. Capture mode: When CH0EVCFG.CCACT equals PER_PULSE_WIDTH_MEAS then VALUE contains the width of the low or high phase of the selected signal. This is specified by CH0CCFG.EDGE and CH0CCFG.CAPT_SRC."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<Ch0pccSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Channel 0 Pipeline Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0pcc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0pcc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0pccSpec;
impl crate::RegisterSpec for Ch0pccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0pcc::R`](R) reader structure"]
impl crate::Readable for Ch0pccSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0pcc::W`](W) writer structure"]
impl crate::Writable for Ch0pccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH0PCC to value 0"]
impl crate::Resettable for Ch0pccSpec {
    const RESET_VALUE: u32 = 0;
}
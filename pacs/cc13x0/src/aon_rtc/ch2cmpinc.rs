#[doc = "Register `CH2CMPINC` reader"]
pub type R = crate::R<Ch2cmpincSpec>;
#[doc = "Register `CH2CMPINC` writer"]
pub type W = crate::W<Ch2cmpincSpec>;
#[doc = "Field `VALUE` reader - 31:0\\]
If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - 31:0\\]
If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<Ch2cmpincSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\]
event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cmpinc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cmpinc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2cmpincSpec;
impl crate::RegisterSpec for Ch2cmpincSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2cmpinc::R`](R) reader structure"]
impl crate::Readable for Ch2cmpincSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2cmpinc::W`](W) writer structure"]
impl crate::Writable for Ch2cmpincSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2CMPINC to value 0"]
impl crate::Resettable for Ch2cmpincSpec {
    const RESET_VALUE: u32 = 0;
}

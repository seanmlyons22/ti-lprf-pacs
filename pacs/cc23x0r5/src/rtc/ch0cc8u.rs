#[doc = "Register `CH0CC8U` reader"]
pub type R = crate::R<Ch0cc8uSpec>;
#[doc = "Register `CH0CC8U` writer"]
pub type W = crate::W<Ch0cc8uSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
RTC Channel 0 compare value. This value is compared against TIME8U.VAL. A Channel 0 event is generated when TIME8U.VAL value reaches or exceeds this compare value."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
RTC Channel 0 compare value. This value is compared against TIME8U.VAL. A Channel 0 event is generated when TIME8U.VAL value reaches or exceeds this compare value."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
RTC Channel 0 compare value. This value is compared against TIME8U.VAL. A Channel 0 event is generated when TIME8U.VAL value reaches or exceeds this compare value."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
RTC Channel 0 compare value. This value is compared against TIME8U.VAL. A Channel 0 event is generated when TIME8U.VAL value reaches or exceeds this compare value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Ch0cc8uSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Channel 0 compare value. A write to this register automatically enables the channel to trigger an event when RTC timer reaches the programmed value or if the programmed value is 1 sec in the past.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cc8u::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cc8u::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0cc8uSpec;
impl crate::RegisterSpec for Ch0cc8uSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0cc8u::R`](R) reader structure"]
impl crate::Readable for Ch0cc8uSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0cc8u::W`](W) writer structure"]
impl crate::Writable for Ch0cc8uSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH0CC8U to value 0"]
impl crate::Resettable for Ch0cc8uSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CH1CCSR` reader"]
pub type R = crate::R<Ch1ccsrSpec>;
#[doc = "Register `CH1CCSR` writer"]
pub type W = crate::W<Ch1ccsrSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
Capture/compare value"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Capture/compare value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Capture/compare value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Capture/compare value"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Ch1ccsrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Save/restore alias registers channel 1. i. A read to CH1CCSR behaves exactly as a read to CH1VAL. A write to this register sets CH0CC.VAL without affecting channel state or configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1ccsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1ccsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1ccsrSpec;
impl crate::RegisterSpec for Ch1ccsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1ccsr::R`](R) reader structure"]
impl crate::Readable for Ch1ccsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1ccsr::W`](W) writer structure"]
impl crate::Writable for Ch1ccsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1CCSR to value 0"]
impl crate::Resettable for Ch1ccsrSpec {
    const RESET_VALUE: u32 = 0;
}

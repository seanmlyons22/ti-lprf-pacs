#[doc = "Register `CH2CC` reader"]
pub type R = crate::R<Ch2ccSpec>;
#[doc = "Register `CH2CC` writer"]
pub type W = crate::W<Ch2ccSpec>;
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
    pub fn val(&mut self) -> ValW<Ch2ccSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "System Timer channel 2 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2ccSpec;
impl crate::RegisterSpec for Ch2ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2cc::R`](R) reader structure"]
impl crate::Readable for Ch2ccSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2cc::W`](W) writer structure"]
impl crate::Writable for Ch2ccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2CC to value 0"]
impl crate::Resettable for Ch2ccSpec {
    const RESET_VALUE: u32 = 0;
}

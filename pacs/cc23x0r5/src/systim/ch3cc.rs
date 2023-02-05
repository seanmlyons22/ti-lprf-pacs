#[doc = "Register `CH3CC` reader"]
pub type R = crate::R<Ch3ccSpec>;
#[doc = "Register `CH3CC` writer"]
pub type W = crate::W<Ch3ccSpec>;
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
    pub fn val(&mut self) -> ValW<Ch3ccSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "System Timer channel 3 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3cc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3cc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3ccSpec;
impl crate::RegisterSpec for Ch3ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3cc::R`](R) reader structure"]
impl crate::Readable for Ch3ccSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3cc::W`](W) writer structure"]
impl crate::Writable for Ch3ccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH3CC to value 0"]
impl crate::Resettable for Ch3ccSpec {
    const RESET_VALUE: u32 = 0;
}

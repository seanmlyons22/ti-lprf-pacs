#[doc = "Register `CH3CCSR` reader"]
pub type R = crate::R<Ch3ccsrSpec>;
#[doc = "Register `CH3CCSR` writer"]
pub type W = crate::W<Ch3ccsrSpec>;
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
    pub fn val(&mut self) -> ValW<Ch3ccsrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Save/restore alias registers channel 3. i. A read to CH3CCSR behaves exactly as a read to CH3VAL. A write to CH3CCSR sets CH3VAL value of register without affecting channel state or configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3ccsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3ccsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3ccsrSpec;
impl crate::RegisterSpec for Ch3ccsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3ccsr::R`](R) reader structure"]
impl crate::Readable for Ch3ccsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3ccsr::W`](W) writer structure"]
impl crate::Writable for Ch3ccsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH3CCSR to value 0"]
impl crate::Resettable for Ch3ccsrSpec {
    const RESET_VALUE: u32 = 0;
}

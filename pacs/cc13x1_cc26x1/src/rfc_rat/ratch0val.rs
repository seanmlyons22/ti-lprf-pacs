#[doc = "Register `RATCH0VAL` reader"]
pub type R = crate::R<Ratch0valSpec>;
#[doc = "Register `RATCH0VAL` writer"]
pub type W = crate::W<Ratch0valSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
Capture/compare value. Only writable when the channel is configured for compare mode. In compare mode, a write to this register will auto-arm the channel."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Capture/compare value. Only writable when the channel is configured for compare mode. In compare mode, a write to this register will auto-arm the channel."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Capture/compare value. Only writable when the channel is configured for compare mode. In compare mode, a write to this register will auto-arm the channel."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Capture/compare value. Only writable when the channel is configured for compare mode. In compare mode, a write to this register will auto-arm the channel."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Ratch0valSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Timer Channel 0 Capture/Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratch0val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratch0val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ratch0valSpec;
impl crate::RegisterSpec for Ratch0valSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ratch0val::R`](R) reader structure"]
impl crate::Readable for Ratch0valSpec {}
#[doc = "`write(|w| ..)` method takes [`ratch0val::W`](W) writer structure"]
impl crate::Writable for Ratch0valSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RATCH0VAL to value 0"]
impl crate::Resettable for Ratch0valSpec {
    const RESET_VALUE: u32 = 0;
}

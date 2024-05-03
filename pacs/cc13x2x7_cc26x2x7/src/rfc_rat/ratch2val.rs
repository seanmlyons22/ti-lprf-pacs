#[doc = "Register `RATCH2VAL` reader"]
pub type R = crate::R<Ratch2valSpec>;
#[doc = "Register `RATCH2VAL` writer"]
pub type W = crate::W<Ratch2valSpec>;
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
    pub fn val(&mut self) -> ValW<Ratch2valSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Timer Channel 2 Capture/Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratch2val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratch2val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ratch2valSpec;
impl crate::RegisterSpec for Ratch2valSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ratch2val::R`](R) reader structure"]
impl crate::Readable for Ratch2valSpec {}
#[doc = "`write(|w| ..)` method takes [`ratch2val::W`](W) writer structure"]
impl crate::Writable for Ratch2valSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RATCH2VAL to value 0"]
impl crate::Resettable for Ratch2valSpec {
    const RESET_VALUE: u32 = 0;
}

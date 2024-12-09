#[doc = "Register `CLZ` reader"]
pub type R = crate::R<ClzSpec>;
#[doc = "Register `CLZ` writer"]
pub type W = crate::W<ClzSpec>;
#[doc = "Field `VALUE` reader - 5:0\\]
Number of leading zero bits in the accumulator: 0x00: 0 leading zeros. 0x01: 1 leading zero. ... 0x28: 40 leading zeros (accumulator value is 0)."]
pub type ValueR = crate::FieldReader;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Number of leading zero bits in the accumulator: 0x00: 0 leading zeros. 0x01: 1 leading zero. ... 0x28: 40 leading zeros (accumulator value is 0)."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {}
#[doc = "Count Leading Zero\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClzSpec;
impl crate::RegisterSpec for ClzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clz::R`](R) reader structure"]
impl crate::Readable for ClzSpec {}
#[doc = "`write(|w| ..)` method takes [`clz::W`](W) writer structure"]
impl crate::Writable for ClzSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLZ to value 0x28"]
impl crate::Resettable for ClzSpec {
    const RESET_VALUE: u32 = 0x28;
}

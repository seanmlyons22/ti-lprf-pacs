#[doc = "Register `CLS` reader"]
pub type R = crate::R<ClsSpec>;
#[doc = "Register `CLS` writer"]
pub type W = crate::W<ClsSpec>;
#[doc = "Field `VALUE` reader - 5:0\\]
Number of leading sign bits in the accumulator. When MSB of accumulator is 0, VALUE is number of leading zeros, MSB included. When MSB of accumulator is 1, VALUE is number of leading ones, MSB included. VALUE range is 1 thru 40."]
pub type ValueR = crate::FieldReader;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Number of leading sign bits in the accumulator. When MSB of accumulator is 0, VALUE is number of leading zeros, MSB included. When MSB of accumulator is 1, VALUE is number of leading ones, MSB included. VALUE range is 1 thru 40."]
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
#[doc = "Count Leading Sign\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cls::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cls::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClsSpec;
impl crate::RegisterSpec for ClsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cls::R`](R) reader structure"]
impl crate::Readable for ClsSpec {}
#[doc = "`write(|w| ..)` method takes [`cls::W`](W) writer structure"]
impl crate::Writable for ClsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLS to value 0x28"]
impl crate::Resettable for ClsSpec {
    const RESET_VALUE: u32 = 0x28;
}

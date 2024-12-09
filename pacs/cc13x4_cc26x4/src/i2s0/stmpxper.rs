#[doc = "Register `STMPXPER` reader"]
pub type R = crate::R<StmpxperSpec>;
#[doc = "Register `STMPXPER` writer"]
pub type W = crate::W<StmpxperSpec>;
#[doc = "Field `VALUE` reader - 15:0\\]
The number of 24 MHz clock cycles in the previous WCLK period (that is - the next value of the XOSC counter at the positive WCLK edge, had it not been reset to 0). The value is cleared when STMPCTL.STMP_EN = 0."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The number of 24 MHz clock cycles in the previous WCLK period (that is - the next value of the XOSC counter at the positive WCLK edge, had it not been reset to 0). The value is cleared when STMPCTL.STMP_EN = 0."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "XOSC Period Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpxper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpxper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StmpxperSpec;
impl crate::RegisterSpec for StmpxperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmpxper::R`](R) reader structure"]
impl crate::Readable for StmpxperSpec {}
#[doc = "`write(|w| ..)` method takes [`stmpxper::W`](W) writer structure"]
impl crate::Writable for StmpxperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMPXPER to value 0"]
impl crate::Resettable for StmpxperSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `STMPXPERMIN` reader"]
pub type R = crate::R<StmpxperminSpec>;
#[doc = "Register `STMPXPERMIN` writer"]
pub type W = crate::W<StmpxperminSpec>;
#[doc = "Field `VALUE` reader - 15:0\\]
Each time STMPXPER is updated, the value is also loaded into this register, provided that the value is smaller than the current value in this register. When written, the register is reset to 0xFFFF (65535), regardless of the value written. The minimum value can be used to detect extra WCLK pulses (this registers value will be significantly smaller than STMPXPER.VALUE)."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - 15:0\\]
Each time STMPXPER is updated, the value is also loaded into this register, provided that the value is smaller than the current value in this register. When written, the register is reset to 0xFFFF (65535), regardless of the value written. The minimum value can be used to detect extra WCLK pulses (this registers value will be significantly smaller than STMPXPER.VALUE)."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Each time STMPXPER is updated, the value is also loaded into this register, provided that the value is smaller than the current value in this register. When written, the register is reset to 0xFFFF (65535), regardless of the value written. The minimum value can be used to detect extra WCLK pulses (this registers value will be significantly smaller than STMPXPER.VALUE)."]
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
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Each time STMPXPER is updated, the value is also loaded into this register, provided that the value is smaller than the current value in this register. When written, the register is reset to 0xFFFF (65535), regardless of the value written. The minimum value can be used to detect extra WCLK pulses (this registers value will be significantly smaller than STMPXPER.VALUE)."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<StmpxperminSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "XOSC Minimum Period Value Minimum Value of STMPXPER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpxpermin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpxpermin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StmpxperminSpec;
impl crate::RegisterSpec for StmpxperminSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmpxpermin::R`](R) reader structure"]
impl crate::Readable for StmpxperminSpec {}
#[doc = "`write(|w| ..)` method takes [`stmpxpermin::W`](W) writer structure"]
impl crate::Writable for StmpxperminSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMPXPERMIN to value 0xffff"]
impl crate::Resettable for StmpxperminSpec {
    const RESET_VALUE: u32 = 0xffff;
}

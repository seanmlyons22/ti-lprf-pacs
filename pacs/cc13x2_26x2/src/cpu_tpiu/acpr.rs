#[doc = "Register `ACPR` reader"]
pub type R = crate::R<AcprSpec>;
#[doc = "Register `ACPR` writer"]
pub type W = crate::W<AcprSpec>;
#[doc = "Field `PRESCALER` reader - 12:0\\]
Divisor for input trace clock is (PRESCALER + 1)."]
pub type PrescalerR = crate::FieldReader<u16>;
#[doc = "Field `PRESCALER` writer - 12:0\\]
Divisor for input trace clock is (PRESCALER + 1)."]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `RESERVED13` reader - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED13` writer - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
Divisor for input trace clock is (PRESCALER + 1)."]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:12 - 12:0\\]
Divisor for input trace clock is (PRESCALER + 1)."]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PrescalerW<AcprSpec> {
        PrescalerW::new(self, 0)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> Reserved13W<AcprSpec> {
        Reserved13W::new(self, 13)
    }
}
#[doc = "Async Clock Prescaler This register scales the baud rate of the asynchronous output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcprSpec;
impl crate::RegisterSpec for AcprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acpr::R`](R) reader structure"]
impl crate::Readable for AcprSpec {}
#[doc = "`write(|w| ..)` method takes [`acpr::W`](W) writer structure"]
impl crate::Writable for AcprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACPR to value 0"]
impl crate::Resettable for AcprSpec {
    const RESET_VALUE: u32 = 0;
}

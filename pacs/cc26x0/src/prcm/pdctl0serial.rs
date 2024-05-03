#[doc = "Register `PDCTL0SERIAL` reader"]
pub type R = crate::R<Pdctl0serialSpec>;
#[doc = "Register `PDCTL0SERIAL` writer"]
pub type W = crate::W<Pdctl0serialSpec>;
#[doc = "Field `ON` reader - 0:0\\]
Alias for PDCTL0.SERIAL_ON"]
pub type OnR = crate::BitReader;
#[doc = "Field `ON` writer - 0:0\\]
Alias for PDCTL0.SERIAL_ON"]
pub type OnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Alias for PDCTL0.SERIAL_ON"]
    #[inline(always)]
    pub fn on(&self) -> OnR {
        OnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Alias for PDCTL0.SERIAL_ON"]
    #[inline(always)]
    #[must_use]
    pub fn on(&mut self) -> OnW<Pdctl0serialSpec> {
        OnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Pdctl0serialSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "SERIAL Power Domain Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdctl0serial::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdctl0serial::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdctl0serialSpec;
impl crate::RegisterSpec for Pdctl0serialSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdctl0serial::R`](R) reader structure"]
impl crate::Readable for Pdctl0serialSpec {}
#[doc = "`write(|w| ..)` method takes [`pdctl0serial::W`](W) writer structure"]
impl crate::Writable for Pdctl0serialSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCTL0SERIAL to value 0"]
impl crate::Resettable for Pdctl0serialSpec {
    const RESET_VALUE: u32 = 0;
}

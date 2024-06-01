#[doc = "Register `PDCTL0` reader"]
pub type R = crate::R<Pdctl0Spec>;
#[doc = "Register `PDCTL0` writer"]
pub type W = crate::W<Pdctl0Spec>;
#[doc = "Field `RFC_ON` reader - 0:0\\]
0: RFC power domain powered off if also PDCTL1.RFC_ON = 0 1: RFC power domain powered on"]
pub type RfcOnR = crate::BitReader;
#[doc = "Field `RFC_ON` writer - 0:0\\]
0: RFC power domain powered off if also PDCTL1.RFC_ON = 0 1: RFC power domain powered on"]
pub type RfcOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_ON` reader - 1:1\\]
SERIAL Power domain. 0: SERIAL power domain is powered down 1: SERIAL power domain is powered up"]
pub type SerialOnR = crate::BitReader;
#[doc = "Field `SERIAL_ON` writer - 1:1\\]
SERIAL Power domain. 0: SERIAL power domain is powered down 1: SERIAL power domain is powered up"]
pub type SerialOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIPH_ON` reader - 2:2\\]
PERIPH Power domain. 0: PERIPH power domain is powered down 1: PERIPH power domain is powered up"]
pub type PeriphOnR = crate::BitReader;
#[doc = "Field `PERIPH_ON` writer - 2:2\\]
PERIPH Power domain. 0: PERIPH power domain is powered down 1: PERIPH power domain is powered up"]
pub type PeriphOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: RFC power domain powered off if also PDCTL1.RFC_ON = 0 1: RFC power domain powered on"]
    #[inline(always)]
    pub fn rfc_on(&self) -> RfcOnR {
        RfcOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SERIAL Power domain. 0: SERIAL power domain is powered down 1: SERIAL power domain is powered up"]
    #[inline(always)]
    pub fn serial_on(&self) -> SerialOnR {
        SerialOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
PERIPH Power domain. 0: PERIPH power domain is powered down 1: PERIPH power domain is powered up"]
    #[inline(always)]
    pub fn periph_on(&self) -> PeriphOnR {
        PeriphOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: RFC power domain powered off if also PDCTL1.RFC_ON = 0 1: RFC power domain powered on"]
    #[inline(always)]
    #[must_use]
    pub fn rfc_on(&mut self) -> RfcOnW<Pdctl0Spec> {
        RfcOnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SERIAL Power domain. 0: SERIAL power domain is powered down 1: SERIAL power domain is powered up"]
    #[inline(always)]
    #[must_use]
    pub fn serial_on(&mut self) -> SerialOnW<Pdctl0Spec> {
        SerialOnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
PERIPH Power domain. 0: PERIPH power domain is powered down 1: PERIPH power domain is powered up"]
    #[inline(always)]
    #[must_use]
    pub fn periph_on(&mut self) -> PeriphOnW<Pdctl0Spec> {
        PeriphOnW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Pdctl0Spec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Power Domain Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdctl0Spec;
impl crate::RegisterSpec for Pdctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdctl0::R`](R) reader structure"]
impl crate::Readable for Pdctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pdctl0::W`](W) writer structure"]
impl crate::Writable for Pdctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCTL0 to value 0"]
impl crate::Resettable for Pdctl0Spec {
    const RESET_VALUE: u32 = 0;
}

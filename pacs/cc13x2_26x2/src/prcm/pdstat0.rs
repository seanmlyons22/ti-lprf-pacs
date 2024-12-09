#[doc = "Register `PDSTAT0` reader"]
pub type R = crate::R<Pdstat0Spec>;
#[doc = "Register `PDSTAT0` writer"]
pub type W = crate::W<Pdstat0Spec>;
#[doc = "Field `RFC_ON` reader - 0:0\\]
RFC Power domain 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
pub type RfcOnR = crate::BitReader;
#[doc = "Field `SERIAL_ON` reader - 1:1\\]
SERIAL Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
pub type SerialOnR = crate::BitReader;
#[doc = "Field `PERIPH_ON` reader - 2:2\\]
PERIPH Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
pub type PeriphOnR = crate::BitReader;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
RFC Power domain 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
    #[inline(always)]
    pub fn rfc_on(&self) -> RfcOnR {
        RfcOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SERIAL Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
    #[inline(always)]
    pub fn serial_on(&self) -> SerialOnR {
        SerialOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
PERIPH Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
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
impl W {}
#[doc = "Power Domain Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdstat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdstat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdstat0Spec;
impl crate::RegisterSpec for Pdstat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdstat0::R`](R) reader structure"]
impl crate::Readable for Pdstat0Spec {}
#[doc = "`write(|w| ..)` method takes [`pdstat0::W`](W) writer structure"]
impl crate::Writable for Pdstat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDSTAT0 to value 0"]
impl crate::Resettable for Pdstat0Spec {
    const RESET_VALUE: u32 = 0;
}

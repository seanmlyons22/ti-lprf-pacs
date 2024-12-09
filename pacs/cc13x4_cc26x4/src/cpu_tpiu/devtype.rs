#[doc = "Register `DEVTYPE` reader"]
pub type R = crate::R<DevtypeSpec>;
#[doc = "Register `DEVTYPE` writer"]
pub type W = crate::W<DevtypeSpec>;
#[doc = "Field `MAJORTYPE` reader - 3:0\\]
Indicates this device is a trace sink and specifically a TPIU"]
pub type MajortypeR = crate::FieldReader;
#[doc = "Field `SUBTYPE` reader - 7:4\\]
Identifies the classification of the debug component"]
pub type SubtypeR = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates this device is a trace sink and specifically a TPIU"]
    #[inline(always)]
    pub fn majortype(&self) -> MajortypeR {
        MajortypeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Identifies the classification of the debug component"]
    #[inline(always)]
    pub fn subtype(&self) -> SubtypeR {
        SubtypeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "The Device Type Identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devtype::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devtype::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevtypeSpec;
impl crate::RegisterSpec for DevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devtype::R`](R) reader structure"]
impl crate::Readable for DevtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`devtype::W`](W) writer structure"]
impl crate::Writable for DevtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVTYPE to value 0x11"]
impl crate::Resettable for DevtypeSpec {
    const RESET_VALUE: u32 = 0x11;
}

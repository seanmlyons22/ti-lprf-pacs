#[doc = "Register `PEEK15` reader"]
pub type R = crate::R<Peek15Spec>;
#[doc = "Register `PEEK15` writer"]
pub type W = crate::W<Peek15Spec>;
#[doc = "Field `STAT` reader - 0:0\\]
Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
pub type StatR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {}
#[doc = "MCU SEMAPHORE 15 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Peek15Spec;
impl crate::RegisterSpec for Peek15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peek15::R`](R) reader structure"]
impl crate::Readable for Peek15Spec {}
#[doc = "`write(|w| ..)` method takes [`peek15::W`](W) writer structure"]
impl crate::Writable for Peek15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PEEK15 to value 0x01"]
impl crate::Resettable for Peek15Spec {
    const RESET_VALUE: u32 = 0x01;
}
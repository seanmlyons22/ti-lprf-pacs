#[doc = "Register `PEEK28` reader"]
pub type R = crate::R<Peek28Spec>;
#[doc = "Register `PEEK28` writer"]
pub type W = crate::W<Peek28Spec>;
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
#[doc = "MCU SEMAPHORE 28 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Peek28Spec;
impl crate::RegisterSpec for Peek28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peek28::R`](R) reader structure"]
impl crate::Readable for Peek28Spec {}
#[doc = "`write(|w| ..)` method takes [`peek28::W`](W) writer structure"]
impl crate::Writable for Peek28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PEEK28 to value 0x01"]
impl crate::Resettable for Peek28Spec {
    const RESET_VALUE: u32 = 0x01;
}

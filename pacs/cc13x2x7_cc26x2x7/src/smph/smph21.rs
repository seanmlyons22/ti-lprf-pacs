#[doc = "Register `SMPH21` reader"]
pub type R = crate::R<Smph21Spec>;
#[doc = "Register `SMPH21` writer"]
pub type W = crate::W<Smph21Spec>;
#[doc = "Field `STAT` reader - 0:0\\]
Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
pub type StatR = crate::BitReader;
#[doc = "Field `STAT` writer - 0:0\\]
Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
pub type StatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<Smph21Spec> {
        StatW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Smph21Spec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "MCU SEMAPHORE 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smph21Spec;
impl crate::RegisterSpec for Smph21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smph21::R`](R) reader structure"]
impl crate::Readable for Smph21Spec {}
#[doc = "`write(|w| ..)` method takes [`smph21::W`](W) writer structure"]
impl crate::Writable for Smph21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPH21 to value 0x01"]
impl crate::Resettable for Smph21Spec {
    const RESET_VALUE: u32 = 0x01;
}

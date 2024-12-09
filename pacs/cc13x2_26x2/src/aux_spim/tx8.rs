#[doc = "Register `TX8` reader"]
pub type R = crate::R<Tx8Spec>;
#[doc = "Register `TX8` writer"]
pub type W = crate::W<Tx8Spec>;
#[doc = "Field `DATA` writer - 7:0\\]
8 bit data transfer. Write DATA to start transfer, MSB first. When transfer completes, MOSI stays at the value of LSB."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
8 bit data transfer. Write DATA to start transfer, MSB first. When transfer completes, MOSI stays at the value of LSB."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Tx8Spec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Tx8Spec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Transmit 8 Bit Write operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx8Spec;
impl crate::RegisterSpec for Tx8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx8::R`](R) reader structure"]
impl crate::Readable for Tx8Spec {}
#[doc = "`write(|w| ..)` method takes [`tx8::W`](W) writer structure"]
impl crate::Writable for Tx8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX8 to value 0"]
impl crate::Resettable for Tx8Spec {
    const RESET_VALUE: u32 = 0;
}

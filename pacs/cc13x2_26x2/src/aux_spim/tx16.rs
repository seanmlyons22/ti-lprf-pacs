#[doc = "Register `TX16` reader"]
pub type R = crate::R<Tx16Spec>;
#[doc = "Register `TX16` writer"]
pub type W = crate::W<Tx16Spec>;
#[doc = "Field `DATA` writer - 15:0\\]
16 bit data transfer. Write DATA to start transfer, MSB first. When transfer completes, MOSI stays at the value of LSB."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
16 bit data transfer. Write DATA to start transfer, MSB first. When transfer completes, MOSI stays at the value of LSB."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Tx16Spec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Tx16Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Transmit 16 Bit Write operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx16Spec;
impl crate::RegisterSpec for Tx16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx16::R`](R) reader structure"]
impl crate::Readable for Tx16Spec {}
#[doc = "`write(|w| ..)` method takes [`tx16::W`](W) writer structure"]
impl crate::Writable for Tx16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX16 to value 0"]
impl crate::Resettable for Tx16Spec {
    const RESET_VALUE: u32 = 0;
}

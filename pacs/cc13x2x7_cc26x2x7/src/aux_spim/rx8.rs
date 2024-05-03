#[doc = "Register `RX8` reader"]
pub type R = crate::R<Rx8Spec>;
#[doc = "Register `RX8` writer"]
pub type W = crate::W<Rx8Spec>;
#[doc = "Field `DATA` reader - 7:0\\]
Latest 8 bits received on MISO."]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - 7:0\\]
Latest 8 bits received on MISO."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Latest 8 bits received on MISO."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Latest 8 bits received on MISO."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Rx8Spec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Rx8Spec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Receive 8 Bit Read operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx8Spec;
impl crate::RegisterSpec for Rx8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx8::R`](R) reader structure"]
impl crate::Readable for Rx8Spec {}
#[doc = "`write(|w| ..)` method takes [`rx8::W`](W) writer structure"]
impl crate::Writable for Rx8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX8 to value 0"]
impl crate::Resettable for Rx8Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `FLASH_NUMBER` reader"]
pub type R = crate::R<FlashNumberSpec>;
#[doc = "Register `FLASH_NUMBER` writer"]
pub type W = crate::W<FlashNumberSpec>;
#[doc = "Field `LOT_NUMBER` reader - 31:0\\]
Number of the manufacturing lot that produced this unit."]
pub type LotNumberR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Number of the manufacturing lot that produced this unit."]
    #[inline(always)]
    pub fn lot_number(&self) -> LotNumberR {
        LotNumberR::new(self.bits)
    }
}
impl W {}
#[doc = "FLASH_NUMBER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_number::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_number::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashNumberSpec;
impl crate::RegisterSpec for FlashNumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_number::R`](R) reader structure"]
impl crate::Readable for FlashNumberSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_number::W`](W) writer structure"]
impl crate::Writable for FlashNumberSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_NUMBER to value 0"]
impl crate::Resettable for FlashNumberSpec {
    const RESET_VALUE: u32 = 0;
}

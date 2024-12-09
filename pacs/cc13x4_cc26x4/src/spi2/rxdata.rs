#[doc = "Register `RXDATA` reader"]
pub type R = crate::R<RxdataSpec>;
#[doc = "Register `RXDATA` writer"]
pub type W = crate::W<RxdataSpec>;
#[doc = "Field `DATA` reader - 31:0\\]
Received Data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Received Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {}
#[doc = "RXDATA Register. Reading this register returns value in the RX FIFO pointed by the current FIFO read pointer. If the RX FIFO is empty, the last read value is returned. Writing has not effect and is ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdataSpec;
impl crate::RegisterSpec for RxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata::R`](R) reader structure"]
impl crate::Readable for RxdataSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdata::W`](W) writer structure"]
impl crate::Writable for RxdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXDATA to value 0"]
impl crate::Resettable for RxdataSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `RXD` reader"]
pub type R = crate::R<RxdSpec>;
#[doc = "Register `RXD` writer"]
pub type W = crate::W<RxdSpec>;
#[doc = "Field `DATA` reader - 31:0\\]
RX FIFO data. When written the register data is pushed to the RX FIFO. When read, data is popped from the RX FIFO. When writing or reading this register the access size will determine how many bytes are pushed to or popped from the FIFO. It is possible to push or pop 1,2 or 4 bytes depending on the access being done."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - 31:0\\]
RX FIFO data. When written the register data is pushed to the RX FIFO. When read, data is popped from the RX FIFO. When writing or reading this register the access size will determine how many bytes are pushed to or popped from the FIFO. It is possible to push or pop 1,2 or 4 bytes depending on the access being done."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
RX FIFO data. When written the register data is pushed to the RX FIFO. When read, data is popped from the RX FIFO. When writing or reading this register the access size will determine how many bytes are pushed to or popped from the FIFO. It is possible to push or pop 1,2 or 4 bytes depending on the access being done."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
RX FIFO data. When written the register data is pushed to the RX FIFO. When read, data is popped from the RX FIFO. When writing or reading this register the access size will determine how many bytes are pushed to or popped from the FIFO. It is possible to push or pop 1,2 or 4 bytes depending on the access being done."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<RxdSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "RX FIFO data. When written the register data is pushed to the RX FIFO. When read, data is popped from the RX FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdSpec;
impl crate::RegisterSpec for RxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxd::R`](R) reader structure"]
impl crate::Readable for RxdSpec {}
#[doc = "`write(|w| ..)` method takes [`rxd::W`](W) writer structure"]
impl crate::Writable for RxdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXD to value 0"]
impl crate::Resettable for RxdSpec {
    const RESET_VALUE: u32 = 0;
}

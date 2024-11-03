#[doc = "Register `RXDATA` reader"]
pub type R = crate::R<RxdataSpec>;
#[doc = "Register `RXDATA` writer"]
pub type W = crate::W<RxdataSpec>;
#[doc = "Field `DATA` reader - 15:0\\]
Received Data. When read, the entry in the receive FIFO, pointed to by the current FIFO read pointer is accessed. As data values are read by the receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current RX FIFO write pointer. Received data less than 16 bits is automatically right-justified in the receive buffer."]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - 15:0\\]
Received Data. When read, the entry in the receive FIFO, pointed to by the current FIFO read pointer is accessed. As data values are read by the receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current RX FIFO write pointer. Received data less than 16 bits is automatically right-justified in the receive buffer."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Received Data. When read, the entry in the receive FIFO, pointed to by the current FIFO read pointer is accessed. As data values are read by the receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current RX FIFO write pointer. Received data less than 16 bits is automatically right-justified in the receive buffer."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Received Data. When read, the entry in the receive FIFO, pointed to by the current FIFO read pointer is accessed. As data values are read by the receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current RX FIFO write pointer. Received data less than 16 bits is automatically right-justified in the receive buffer."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<RxdataSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<RxdataSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "RXDATA Register. Reading this register returns first value in the RX FIFO. If the FIFO is empty the last read value is returned. Writing has no effect and is ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

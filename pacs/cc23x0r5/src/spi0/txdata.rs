#[doc = "Register `TXDATA` reader"]
pub type R = crate::R<TxdataSpec>;
#[doc = "Register `TXDATA` writer"]
pub type W = crate::W<TxdataSpec>;
#[doc = "Field `DATA` reader - 15:0\\]
Transmit Data. When read, the last entry in the transmit FIFO, pointed to by the current FIFO write pointer is accessed. When written, the entry in the TX FIFO pointed to by the write pointer, is written to. Data values are read from the transmit FIFO by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the output pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits."]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - 15:0\\]
Transmit Data. When read, the last entry in the transmit FIFO, pointed to by the current FIFO write pointer is accessed. When written, the entry in the TX FIFO pointed to by the write pointer, is written to. Data values are read from the transmit FIFO by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the output pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Transmit Data. When read, the last entry in the transmit FIFO, pointed to by the current FIFO write pointer is accessed. When written, the entry in the TX FIFO pointed to by the write pointer, is written to. Data values are read from the transmit FIFO by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the output pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits."]
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
Transmit Data. When read, the last entry in the transmit FIFO, pointed to by the current FIFO write pointer is accessed. When written, the entry in the TX FIFO pointed to by the write pointer, is written to. Data values are read from the transmit FIFO by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the output pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<TxdataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "TXDATA Register. Writing a value in this register puts the data into the TX FIFO. Reading this register returns the last writen value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdataSpec;
impl crate::RegisterSpec for TxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata::R`](R) reader structure"]
impl crate::Readable for TxdataSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata::W`](W) writer structure"]
impl crate::Writable for TxdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDATA to value 0"]
impl crate::Resettable for TxdataSpec {
    const RESET_VALUE: u32 = 0;
}

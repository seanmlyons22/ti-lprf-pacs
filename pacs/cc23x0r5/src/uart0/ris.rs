#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RisSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSRMIS` reader - 1:1\\]
Clear to Send (CTS) modem interrupt status: This field returns the raw interrupt state of UART's clear to send interrupt."]
pub type CtsrmisR = crate::BitReader;
#[doc = "Field `CTSRMIS` writer - 1:1\\]
Clear to Send (CTS) modem interrupt status: This field returns the raw interrupt state of UART's clear to send interrupt."]
pub type CtsrmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 3:2\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXRIS` reader - 4:4\\]
Receive interrupt status: This field returns the raw interrupt state of UART's receive interrupt. When FIFOs are enabled (LCRH.FEN = 1), the receive interrupt is asserted if the receive FIFO reaches the programmed trigger level (IFLS.RXSEL). The receive interrupt is cleared by reading data from the receive FIFO until it becomes less than the trigger level, or by clearing the interrupt through ICR.RXIC. When FIFOs are disabled (LCRH.FEN = 0), that is they have a depth of one location, the receive interrupt is asserted if data is received thereby filling the location. The receive interrupt is cleared by performing a single read of the receive FIFO, or by clearing the interrupt through ICR.RXIC."]
pub type RxrisR = crate::BitReader;
#[doc = "Field `RXRIS` writer - 4:4\\]
Receive interrupt status: This field returns the raw interrupt state of UART's receive interrupt. When FIFOs are enabled (LCRH.FEN = 1), the receive interrupt is asserted if the receive FIFO reaches the programmed trigger level (IFLS.RXSEL). The receive interrupt is cleared by reading data from the receive FIFO until it becomes less than the trigger level, or by clearing the interrupt through ICR.RXIC. When FIFOs are disabled (LCRH.FEN = 0), that is they have a depth of one location, the receive interrupt is asserted if data is received thereby filling the location. The receive interrupt is cleared by performing a single read of the receive FIFO, or by clearing the interrupt through ICR.RXIC."]
pub type RxrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRIS` reader - 5:5\\]
Transmit interrupt status: This field returns the raw interrupt state of UART's transmit interrupt. When FIFOs are enabled (LCRH.FEN = 1), the transmit interrupt is asserted if the number of bytes in transmit FIFO is equal to or lower than the programmed trigger level (IFLS.TXSEL). The transmit interrupt is cleared by writing data to the transmit FIFO until it becomes greater than the trigger level, or by clearing the interrupt through ICR.TXIC. When FIFOs are disabled (LCRH.FEN = 0), that is they have a depth of one location, the transmit interrupt is asserted if there is no data present in the transmitters single location. It is cleared by performing a single write to the transmit FIFO, or by clearing the interrupt through ICR.TXIC."]
pub type TxrisR = crate::BitReader;
#[doc = "Field `TXRIS` writer - 5:5\\]
Transmit interrupt status: This field returns the raw interrupt state of UART's transmit interrupt. When FIFOs are enabled (LCRH.FEN = 1), the transmit interrupt is asserted if the number of bytes in transmit FIFO is equal to or lower than the programmed trigger level (IFLS.TXSEL). The transmit interrupt is cleared by writing data to the transmit FIFO until it becomes greater than the trigger level, or by clearing the interrupt through ICR.TXIC. When FIFOs are disabled (LCRH.FEN = 0), that is they have a depth of one location, the transmit interrupt is asserted if there is no data present in the transmitters single location. It is cleared by performing a single write to the transmit FIFO, or by clearing the interrupt through ICR.TXIC."]
pub type TxrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRIS` reader - 6:6\\]
Receive timeout interrupt status: This field returns the raw interrupt state of UART's receive timeout interrupt. The receive timeout interrupt is asserted when the receive FIFO is not empty, and no more data is received during a 32-bit period. The receive timeout interrupt is cleared either when the FIFO becomes empty through reading all the data, or when a 1 is written to ICR.RTIC. The raw interrupt for receive timeout cannot be set unless the mask is set (IMSC.RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RTRIS."]
pub type RtrisR = crate::BitReader;
#[doc = "Field `RTRIS` writer - 6:6\\]
Receive timeout interrupt status: This field returns the raw interrupt state of UART's receive timeout interrupt. The receive timeout interrupt is asserted when the receive FIFO is not empty, and no more data is received during a 32-bit period. The receive timeout interrupt is cleared either when the FIFO becomes empty through reading all the data, or when a 1 is written to ICR.RTIC. The raw interrupt for receive timeout cannot be set unless the mask is set (IMSC.RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RTRIS."]
pub type RtrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERIS` reader - 7:7\\]
Framing error interrupt status: This field returns the raw interrupt state of UART's framing error interrupt. Framing error is set if the received character does not have a valid stop bit (a valid stop bit is 1)."]
pub type FerisR = crate::BitReader;
#[doc = "Field `FERIS` writer - 7:7\\]
Framing error interrupt status: This field returns the raw interrupt state of UART's framing error interrupt. Framing error is set if the received character does not have a valid stop bit (a valid stop bit is 1)."]
pub type FerisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIS` reader - 8:8\\]
Parity error interrupt status: This field returns the raw interrupt state of UART's parity error interrupt. Parity error is set if the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select."]
pub type PerisR = crate::BitReader;
#[doc = "Field `PERIS` writer - 8:8\\]
Parity error interrupt status: This field returns the raw interrupt state of UART's parity error interrupt. Parity error is set if the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select."]
pub type PerisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERIS` reader - 9:9\\]
Break error interrupt status: This field returns the raw interrupt state of UART's break error interrupt. Break error is set when a break condition is detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits)."]
pub type BerisR = crate::BitReader;
#[doc = "Field `BERIS` writer - 9:9\\]
Break error interrupt status: This field returns the raw interrupt state of UART's break error interrupt. Break error is set when a break condition is detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits)."]
pub type BerisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OERIS` reader - 10:10\\]
Overrun error interrupt status: This field returns the raw interrupt state of UART's overrun error interrupt. Overrun error occurs if data is received and the receive FIFO is full."]
pub type OerisR = crate::BitReader;
#[doc = "Field `OERIS` writer - 10:10\\]
Overrun error interrupt status: This field returns the raw interrupt state of UART's overrun error interrupt. Overrun error occurs if data is received and the receive FIFO is full."]
pub type OerisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOTRIS` reader - 11:11\\]
End of Transmission interrupt status: This field returns the raw interrupt state of UART's end of transmission interrupt. End of transmission flag is set when all the Transmit data in the FIFO and on the TX Line is tranmitted."]
pub type EotrisR = crate::BitReader;
#[doc = "Field `EOTRIS` writer - 11:11\\]
End of Transmission interrupt status: This field returns the raw interrupt state of UART's end of transmission interrupt. End of transmission flag is set when all the Transmit data in the FIFO and on the TX Line is tranmitted."]
pub type EotrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMADONERIS` reader - 12:12\\]
Tx DMA done interrupt status: This field returns the raw interrupt state of UART's tx dma done interrupt. TX DMA done flag is set when you recieve tx dma done status from dma module."]
pub type TxdmadonerisR = crate::BitReader;
#[doc = "Field `TXDMADONERIS` writer - 12:12\\]
Tx DMA done interrupt status: This field returns the raw interrupt state of UART's tx dma done interrupt. TX DMA done flag is set when you recieve tx dma done status from dma module."]
pub type TxdmadonerisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMADONERIS` reader - 13:13\\]
Rx DMA done interrupt status: This field returns the raw interrupt state of UART's rx dma done interrupt. RX DMA done flag is set when you recieve rx dma done status from dma module."]
pub type RxdmadonerisR = crate::BitReader;
#[doc = "Field `RXDMADONERIS` writer - 13:13\\]
Rx DMA done interrupt status: This field returns the raw interrupt state of UART's rx dma done interrupt. RX DMA done flag is set when you recieve rx dma done status from dma module."]
pub type RxdmadonerisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved14R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem interrupt status: This field returns the raw interrupt state of UART's clear to send interrupt."]
    #[inline(always)]
    pub fn ctsrmis(&self) -> CtsrmisR {
        CtsrmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Receive interrupt status: This field returns the raw interrupt state of UART's receive interrupt. When FIFOs are enabled (LCRH.FEN = 1), the receive interrupt is asserted if the receive FIFO reaches the programmed trigger level (IFLS.RXSEL). The receive interrupt is cleared by reading data from the receive FIFO until it becomes less than the trigger level, or by clearing the interrupt through ICR.RXIC. When FIFOs are disabled (LCRH.FEN = 0), that is they have a depth of one location, the receive interrupt is asserted if data is received thereby filling the location. The receive interrupt is cleared by performing a single read of the receive FIFO, or by clearing the interrupt through ICR.RXIC."]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit interrupt status: This field returns the raw interrupt state of UART's transmit interrupt. When FIFOs are enabled (LCRH.FEN = 1), the transmit interrupt is asserted if the number of bytes in transmit FIFO is equal to or lower than the programmed trigger level (IFLS.TXSEL). The transmit interrupt is cleared by writing data to the transmit FIFO until it becomes greater than the trigger level, or by clearing the interrupt through ICR.TXIC. When FIFOs are disabled (LCRH.FEN = 0), that is they have a depth of one location, the transmit interrupt is asserted if there is no data present in the transmitters single location. It is cleared by performing a single write to the transmit FIFO, or by clearing the interrupt through ICR.TXIC."]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout interrupt status: This field returns the raw interrupt state of UART's receive timeout interrupt. The receive timeout interrupt is asserted when the receive FIFO is not empty, and no more data is received during a 32-bit period. The receive timeout interrupt is cleared either when the FIFO becomes empty through reading all the data, or when a 1 is written to ICR.RTIC. The raw interrupt for receive timeout cannot be set unless the mask is set (IMSC.RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RTRIS."]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error interrupt status: This field returns the raw interrupt state of UART's framing error interrupt. Framing error is set if the received character does not have a valid stop bit (a valid stop bit is 1)."]
    #[inline(always)]
    pub fn feris(&self) -> FerisR {
        FerisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error interrupt status: This field returns the raw interrupt state of UART's parity error interrupt. Parity error is set if the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select."]
    #[inline(always)]
    pub fn peris(&self) -> PerisR {
        PerisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Break error interrupt status: This field returns the raw interrupt state of UART's break error interrupt. Break error is set when a break condition is detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits)."]
    #[inline(always)]
    pub fn beris(&self) -> BerisR {
        BerisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error interrupt status: This field returns the raw interrupt state of UART's overrun error interrupt. Overrun error occurs if data is received and the receive FIFO is full."]
    #[inline(always)]
    pub fn oeris(&self) -> OerisR {
        OerisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
End of Transmission interrupt status: This field returns the raw interrupt state of UART's end of transmission interrupt. End of transmission flag is set when all the Transmit data in the FIFO and on the TX Line is tranmitted."]
    #[inline(always)]
    pub fn eotris(&self) -> EotrisR {
        EotrisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Tx DMA done interrupt status: This field returns the raw interrupt state of UART's tx dma done interrupt. TX DMA done flag is set when you recieve tx dma done status from dma module."]
    #[inline(always)]
    pub fn txdmadoneris(&self) -> TxdmadonerisR {
        TxdmadonerisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Rx DMA done interrupt status: This field returns the raw interrupt state of UART's rx dma done interrupt. RX DMA done flag is set when you recieve rx dma done status from dma module."]
    #[inline(always)]
    pub fn rxdmadoneris(&self) -> RxdmadonerisR {
        RxdmadonerisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<RisSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem interrupt status: This field returns the raw interrupt state of UART's clear to send interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ctsrmis(&mut self) -> CtsrmisW<RisSpec> {
        CtsrmisW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<RisSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Receive interrupt status: This field returns the raw interrupt state of UART's receive interrupt. When FIFOs are enabled (LCRH.FEN = 1), the receive interrupt is asserted if the receive FIFO reaches the programmed trigger level (IFLS.RXSEL). The receive interrupt is cleared by reading data from the receive FIFO until it becomes less than the trigger level, or by clearing the interrupt through ICR.RXIC. When FIFOs are disabled (LCRH.FEN = 0), that is they have a depth of one location, the receive interrupt is asserted if data is received thereby filling the location. The receive interrupt is cleared by performing a single read of the receive FIFO, or by clearing the interrupt through ICR.RXIC."]
    #[inline(always)]
    #[must_use]
    pub fn rxris(&mut self) -> RxrisW<RisSpec> {
        RxrisW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit interrupt status: This field returns the raw interrupt state of UART's transmit interrupt. When FIFOs are enabled (LCRH.FEN = 1), the transmit interrupt is asserted if the number of bytes in transmit FIFO is equal to or lower than the programmed trigger level (IFLS.TXSEL). The transmit interrupt is cleared by writing data to the transmit FIFO until it becomes greater than the trigger level, or by clearing the interrupt through ICR.TXIC. When FIFOs are disabled (LCRH.FEN = 0), that is they have a depth of one location, the transmit interrupt is asserted if there is no data present in the transmitters single location. It is cleared by performing a single write to the transmit FIFO, or by clearing the interrupt through ICR.TXIC."]
    #[inline(always)]
    #[must_use]
    pub fn txris(&mut self) -> TxrisW<RisSpec> {
        TxrisW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout interrupt status: This field returns the raw interrupt state of UART's receive timeout interrupt. The receive timeout interrupt is asserted when the receive FIFO is not empty, and no more data is received during a 32-bit period. The receive timeout interrupt is cleared either when the FIFO becomes empty through reading all the data, or when a 1 is written to ICR.RTIC. The raw interrupt for receive timeout cannot be set unless the mask is set (IMSC.RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RTRIS."]
    #[inline(always)]
    #[must_use]
    pub fn rtris(&mut self) -> RtrisW<RisSpec> {
        RtrisW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error interrupt status: This field returns the raw interrupt state of UART's framing error interrupt. Framing error is set if the received character does not have a valid stop bit (a valid stop bit is 1)."]
    #[inline(always)]
    #[must_use]
    pub fn feris(&mut self) -> FerisW<RisSpec> {
        FerisW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error interrupt status: This field returns the raw interrupt state of UART's parity error interrupt. Parity error is set if the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select."]
    #[inline(always)]
    #[must_use]
    pub fn peris(&mut self) -> PerisW<RisSpec> {
        PerisW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Break error interrupt status: This field returns the raw interrupt state of UART's break error interrupt. Break error is set when a break condition is detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits)."]
    #[inline(always)]
    #[must_use]
    pub fn beris(&mut self) -> BerisW<RisSpec> {
        BerisW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error interrupt status: This field returns the raw interrupt state of UART's overrun error interrupt. Overrun error occurs if data is received and the receive FIFO is full."]
    #[inline(always)]
    #[must_use]
    pub fn oeris(&mut self) -> OerisW<RisSpec> {
        OerisW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
End of Transmission interrupt status: This field returns the raw interrupt state of UART's end of transmission interrupt. End of transmission flag is set when all the Transmit data in the FIFO and on the TX Line is tranmitted."]
    #[inline(always)]
    #[must_use]
    pub fn eotris(&mut self) -> EotrisW<RisSpec> {
        EotrisW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Tx DMA done interrupt status: This field returns the raw interrupt state of UART's tx dma done interrupt. TX DMA done flag is set when you recieve tx dma done status from dma module."]
    #[inline(always)]
    #[must_use]
    pub fn txdmadoneris(&mut self) -> TxdmadonerisW<RisSpec> {
        TxdmadonerisW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Rx DMA done interrupt status: This field returns the raw interrupt state of UART's rx dma done interrupt. RX DMA done flag is set when you recieve rx dma done status from dma module."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmadoneris(&mut self) -> RxdmadonerisW<RisSpec> {
        RxdmadonerisW::new(self, 13)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<RisSpec> {
        Reserved14W::new(self, 14)
    }
}
#[doc = "Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RIS to value 0x0d"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0x0d;
}

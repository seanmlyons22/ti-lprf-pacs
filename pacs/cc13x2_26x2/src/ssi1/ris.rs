#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RisSpec>;
#[doc = "Field `RORRIS` reader - 0:0\\]
Raw interrupt state of receive overrun interrupt: The receive overrun interrupt is asserted when the FIFO is already full and an additional data frame is received, causing an overrun of the FIFO. Data is over-written in the receive shift register, but not the FIFO so the FIFO contents stay valid. It can also be cleared by writing to ICR.RORIC."]
pub type RorrisR = crate::BitReader;
#[doc = "Field `RORRIS` writer - 0:0\\]
Raw interrupt state of receive overrun interrupt: The receive overrun interrupt is asserted when the FIFO is already full and an additional data frame is received, causing an overrun of the FIFO. Data is over-written in the receive shift register, but not the FIFO so the FIFO contents stay valid. It can also be cleared by writing to ICR.RORIC."]
pub type RorrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRIS` reader - 1:1\\]
Raw interrupt state of receive timeout interrupt: The receive timeout interrupt is asserted when the receive FIFO is not empty and SSI has remained idle for a fixed 32 bit period. This mechanism can be used to notify the user that data is still present in the receive FIFO and requires servicing. This interrupt is deasserted if the receive FIFO becomes empty by subsequent reads, or if new data is received on RXD. It can also be cleared by writing to ICR.RTIC."]
pub type RtrisR = crate::BitReader;
#[doc = "Field `RTRIS` writer - 1:1\\]
Raw interrupt state of receive timeout interrupt: The receive timeout interrupt is asserted when the receive FIFO is not empty and SSI has remained idle for a fixed 32 bit period. This mechanism can be used to notify the user that data is still present in the receive FIFO and requires servicing. This interrupt is deasserted if the receive FIFO becomes empty by subsequent reads, or if new data is received on RXD. It can also be cleared by writing to ICR.RTIC."]
pub type RtrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRIS` reader - 2:2\\]
Raw interrupt state of receive FIFO interrupt: The receive interrupt is asserted when there are four or more valid entries in the receive FIFO."]
pub type RxrisR = crate::BitReader;
#[doc = "Field `RXRIS` writer - 2:2\\]
Raw interrupt state of receive FIFO interrupt: The receive interrupt is asserted when there are four or more valid entries in the receive FIFO."]
pub type RxrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRIS` reader - 3:3\\]
Raw transmit FIFO interrupt status: The transmit interrupt is asserted when there are four or fewer valid entries in the transmit FIFO. The transmit interrupt is not qualified with the SSI enable signal. Therefore one of the following ways can be used: - data can be written to the transmit FIFO prior to enabling the SSI and the interrupts. - SSI and interrupts can be enabled so that data can be written to the transmit FIFO by an interrupt service routine."]
pub type TxrisR = crate::BitReader;
#[doc = "Field `TXRIS` writer - 3:3\\]
Raw transmit FIFO interrupt status: The transmit interrupt is asserted when there are four or fewer valid entries in the transmit FIFO. The transmit interrupt is not qualified with the SSI enable signal. Therefore one of the following ways can be used: - data can be written to the transmit FIFO prior to enabling the SSI and the interrupts. - SSI and interrupts can be enabled so that data can be written to the transmit FIFO by an interrupt service routine."]
pub type TxrisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Raw interrupt state of receive overrun interrupt: The receive overrun interrupt is asserted when the FIFO is already full and an additional data frame is received, causing an overrun of the FIFO. Data is over-written in the receive shift register, but not the FIFO so the FIFO contents stay valid. It can also be cleared by writing to ICR.RORIC."]
    #[inline(always)]
    pub fn rorris(&self) -> RorrisR {
        RorrisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw interrupt state of receive timeout interrupt: The receive timeout interrupt is asserted when the receive FIFO is not empty and SSI has remained idle for a fixed 32 bit period. This mechanism can be used to notify the user that data is still present in the receive FIFO and requires servicing. This interrupt is deasserted if the receive FIFO becomes empty by subsequent reads, or if new data is received on RXD. It can also be cleared by writing to ICR.RTIC."]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw interrupt state of receive FIFO interrupt: The receive interrupt is asserted when there are four or more valid entries in the receive FIFO."]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Raw transmit FIFO interrupt status: The transmit interrupt is asserted when there are four or fewer valid entries in the transmit FIFO. The transmit interrupt is not qualified with the SSI enable signal. Therefore one of the following ways can be used: - data can be written to the transmit FIFO prior to enabling the SSI and the interrupts. - SSI and interrupts can be enabled so that data can be written to the transmit FIFO by an interrupt service routine."]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Raw interrupt state of receive overrun interrupt: The receive overrun interrupt is asserted when the FIFO is already full and an additional data frame is received, causing an overrun of the FIFO. Data is over-written in the receive shift register, but not the FIFO so the FIFO contents stay valid. It can also be cleared by writing to ICR.RORIC."]
    #[inline(always)]
    #[must_use]
    pub fn rorris(&mut self) -> RorrisW<RisSpec> {
        RorrisW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw interrupt state of receive timeout interrupt: The receive timeout interrupt is asserted when the receive FIFO is not empty and SSI has remained idle for a fixed 32 bit period. This mechanism can be used to notify the user that data is still present in the receive FIFO and requires servicing. This interrupt is deasserted if the receive FIFO becomes empty by subsequent reads, or if new data is received on RXD. It can also be cleared by writing to ICR.RTIC."]
    #[inline(always)]
    #[must_use]
    pub fn rtris(&mut self) -> RtrisW<RisSpec> {
        RtrisW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw interrupt state of receive FIFO interrupt: The receive interrupt is asserted when there are four or more valid entries in the receive FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rxris(&mut self) -> RxrisW<RisSpec> {
        RxrisW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Raw transmit FIFO interrupt status: The transmit interrupt is asserted when there are four or fewer valid entries in the transmit FIFO. The transmit interrupt is not qualified with the SSI enable signal. Therefore one of the following ways can be used: - data can be written to the transmit FIFO prior to enabling the SSI and the interrupts. - SSI and interrupts can be enabled so that data can be written to the transmit FIFO by an interrupt service routine."]
    #[inline(always)]
    #[must_use]
    pub fn txris(&mut self) -> TxrisW<RisSpec> {
        TxrisW::new(self, 3)
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
#[doc = "`reset()` method sets RIS to value 0x08"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0x08;
}

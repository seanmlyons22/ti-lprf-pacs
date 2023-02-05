#[doc = "Register `DMACTL` reader"]
pub type R = crate::R<DmactlSpec>;
#[doc = "Register `DMACTL` writer"]
pub type W = crate::W<DmactlSpec>;
#[doc = "Field `RXDMAE` reader - 0:0\\]
Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RxdmaeR = crate::BitReader;
#[doc = "Field `RXDMAE` writer - 0:0\\]
Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAE` reader - 1:1\\]
Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TxdmaeR = crate::BitReader;
#[doc = "Field `TXDMAE` writer - 1:1\\]
Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAONERR` reader - 2:2\\]
DMA on error. If this bit is set to 1, the DMA receive request outputs (for single and burst requests) are disabled when the UART error interrupt is asserted (more specifically if any of the error interrupts RIS.PERIS, RIS.BERIS, RIS.FERIS or RIS.OERIS are asserted)."]
pub type DmaonerrR = crate::BitReader;
#[doc = "Field `DMAONERR` writer - 2:2\\]
DMA on error. If this bit is set to 1, the DMA receive request outputs (for single and burst requests) are disabled when the UART error interrupt is asserted (more specifically if any of the error interrupts RIS.PERIS, RIS.BERIS, RIS.FERIS or RIS.OERIS are asserted)."]
pub type DmaonerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Reads to this field return zero, writes to this field are ignored. Read as zero, do not modify."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Reads to this field return zero, writes to this field are ignored. Read as zero, do not modify."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub fn rxdmae(&self) -> RxdmaeR {
        RxdmaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    pub fn txdmae(&self) -> TxdmaeR {
        TxdmaeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
DMA on error. If this bit is set to 1, the DMA receive request outputs (for single and burst requests) are disabled when the UART error interrupt is asserted (more specifically if any of the error interrupts RIS.PERIS, RIS.BERIS, RIS.FERIS or RIS.OERIS are asserted)."]
    #[inline(always)]
    pub fn dmaonerr(&self) -> DmaonerrR {
        DmaonerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reads to this field return zero, writes to this field are ignored. Read as zero, do not modify."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RxdmaeW<DmactlSpec> {
        RxdmaeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TxdmaeW<DmactlSpec> {
        TxdmaeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
DMA on error. If this bit is set to 1, the DMA receive request outputs (for single and burst requests) are disabled when the UART error interrupt is asserted (more specifically if any of the error interrupts RIS.PERIS, RIS.BERIS, RIS.FERIS or RIS.OERIS are asserted)."]
    #[inline(always)]
    #[must_use]
    pub fn dmaonerr(&mut self) -> DmaonerrW<DmactlSpec> {
        DmaonerrW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reads to this field return zero, writes to this field are ignored. Read as zero, do not modify."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<DmactlSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "DMA Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactlSpec;
impl crate::RegisterSpec for DmactlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactl::R`](R) reader structure"]
impl crate::Readable for DmactlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactl::W`](W) writer structure"]
impl crate::Writable for DmactlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTL to value 0"]
impl crate::Resettable for DmactlSpec {
    const RESET_VALUE: u32 = 0;
}

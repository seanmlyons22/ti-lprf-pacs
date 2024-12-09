#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MisSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Reads to this field return zero, writes to this field are ignored. Write 0."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `CTSMMIS` reader - 1:1\\]
Clear to Send (CTS) modem masked interrupt status: This field returns the masked interrupt state of the clear to send interrupt which is the AND product of raw interrupt state RIS.CTSRMIS and the mask setting IMSC.CTSMIM."]
pub type CtsmmisR = crate::BitReader;
#[doc = "Field `RESERVED2` reader - 3:2\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RXMIS` reader - 4:4\\]
Receive masked interrupt status: This field returns the masked interrupt state of the receive interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
pub type RxmisR = crate::BitReader;
#[doc = "Field `TXMIS` reader - 5:5\\]
Transmit masked interrupt status: This field returns the masked interrupt state of the transmit interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
pub type TxmisR = crate::BitReader;
#[doc = "Field `RTMIS` reader - 6:6\\]
Receive timeout masked interrupt status: Returns the masked interrupt state of the receive timeout interrupt. The raw interrupt for receive timeout cannot be set unless the mask is set (IMSC.RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from RTMIS and RIS.RTRIS."]
pub type RtmisR = crate::BitReader;
#[doc = "Field `FEMIS` reader - 7:7\\]
Framing error masked interrupt status: Returns the masked interrupt state of the framing error interrupt which is the AND product of raw interrupt state RIS.FERIS and the mask setting IMSC.FEIM."]
pub type FemisR = crate::BitReader;
#[doc = "Field `PEMIS` reader - 8:8\\]
Parity error masked interrupt status: This field returns the masked interrupt state of the parity error interrupt which is the AND product of raw interrupt state RIS.PERIS and the mask setting IMSC.PEIM."]
pub type PemisR = crate::BitReader;
#[doc = "Field `BEMIS` reader - 9:9\\]
Break error masked interrupt status: This field returns the masked interrupt state of the break error interrupt which is the AND product of raw interrupt state RIS.BERIS and the mask setting IMSC.BEIM."]
pub type BemisR = crate::BitReader;
#[doc = "Field `OEMIS` reader - 10:10\\]
Overrun error masked interrupt status: This field returns the masked interrupt state of the overrun interrupt which is the AND product of raw interrupt state RIS.OERIS and the mask setting IMSC.OEIM."]
pub type OemisR = crate::BitReader;
#[doc = "Field `EOTMIS` reader - 11:11\\]
End of Transmission interrupt status: This field returns the masked interrupt state of the End of transmission interrupt which is the AND product of raw interrupt state RIS.EOTRIS and the mask setting IMSC.EOTIM."]
pub type EotmisR = crate::BitReader;
#[doc = "Field `TXDMADONEMIS` reader - 12:12\\]
Tx DMA done interrupt status: This field returns the masked interrupt state of the tx dma done interrupt which is the AND product of raw interrupt state RIS.TXDMADONERIS and the mask setting IMSC.TXDMADONEIM."]
pub type TxdmadonemisR = crate::BitReader;
#[doc = "Field `RXDMADONE` reader - 13:13\\]
Rx DMA done interrupt status: This field returns the masked interrupt state of the rx dma done interrupt which is the AND product of raw interrupt state RIS.RXDMADONERIS and the mask setting IMSC.RXDMADONEIM."]
pub type RxdmadoneR = crate::BitReader;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Reads to this field return zero, writes to this field are ignored. Read as zero, do not modify"]
pub type Reserved14R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reads to this field return zero, writes to this field are ignored. Write 0."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem masked interrupt status: This field returns the masked interrupt state of the clear to send interrupt which is the AND product of raw interrupt state RIS.CTSRMIS and the mask setting IMSC.CTSMIM."]
    #[inline(always)]
    pub fn ctsmmis(&self) -> CtsmmisR {
        CtsmmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Receive masked interrupt status: This field returns the masked interrupt state of the receive interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
    #[inline(always)]
    pub fn rxmis(&self) -> RxmisR {
        RxmisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit masked interrupt status: This field returns the masked interrupt state of the transmit interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
    #[inline(always)]
    pub fn txmis(&self) -> TxmisR {
        TxmisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout masked interrupt status: Returns the masked interrupt state of the receive timeout interrupt. The raw interrupt for receive timeout cannot be set unless the mask is set (IMSC.RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from RTMIS and RIS.RTRIS."]
    #[inline(always)]
    pub fn rtmis(&self) -> RtmisR {
        RtmisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error masked interrupt status: Returns the masked interrupt state of the framing error interrupt which is the AND product of raw interrupt state RIS.FERIS and the mask setting IMSC.FEIM."]
    #[inline(always)]
    pub fn femis(&self) -> FemisR {
        FemisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error masked interrupt status: This field returns the masked interrupt state of the parity error interrupt which is the AND product of raw interrupt state RIS.PERIS and the mask setting IMSC.PEIM."]
    #[inline(always)]
    pub fn pemis(&self) -> PemisR {
        PemisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Break error masked interrupt status: This field returns the masked interrupt state of the break error interrupt which is the AND product of raw interrupt state RIS.BERIS and the mask setting IMSC.BEIM."]
    #[inline(always)]
    pub fn bemis(&self) -> BemisR {
        BemisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error masked interrupt status: This field returns the masked interrupt state of the overrun interrupt which is the AND product of raw interrupt state RIS.OERIS and the mask setting IMSC.OEIM."]
    #[inline(always)]
    pub fn oemis(&self) -> OemisR {
        OemisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
End of Transmission interrupt status: This field returns the masked interrupt state of the End of transmission interrupt which is the AND product of raw interrupt state RIS.EOTRIS and the mask setting IMSC.EOTIM."]
    #[inline(always)]
    pub fn eotmis(&self) -> EotmisR {
        EotmisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Tx DMA done interrupt status: This field returns the masked interrupt state of the tx dma done interrupt which is the AND product of raw interrupt state RIS.TXDMADONERIS and the mask setting IMSC.TXDMADONEIM."]
    #[inline(always)]
    pub fn txdmadonemis(&self) -> TxdmadonemisR {
        TxdmadonemisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Rx DMA done interrupt status: This field returns the masked interrupt state of the rx dma done interrupt which is the AND product of raw interrupt state RIS.RXDMADONERIS and the mask setting IMSC.RXDMADONEIM."]
    #[inline(always)]
    pub fn rxdmadone(&self) -> RxdmadoneR {
        RxdmadoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Reads to this field return zero, writes to this field are ignored. Read as zero, do not modify"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {}
#[doc = "Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}

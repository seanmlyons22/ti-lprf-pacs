#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MisSpec>;
#[doc = "Field `RORMIS` reader - 0:0\\]
Masked interrupt state of receive overrun interrupt: This field returns the masked interrupt state of receive overrun interrupt which is the AND product of raw interrupt state RIS.RORRIS and the mask setting IMSC.RORIM."]
pub type RormisR = crate::BitReader;
#[doc = "Field `RTMIS` reader - 1:1\\]
Masked interrupt state of receive timeout interrupt: This field returns the masked interrupt state of receive timeout interrupt which is the AND product of raw interrupt state RIS.RTRIS and the mask setting IMSC.RTIM."]
pub type RtmisR = crate::BitReader;
#[doc = "Field `RXMIS` reader - 2:2\\]
Masked interrupt state of receive FIFO interrupt: This field returns the masked interrupt state of receive FIFO interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
pub type RxmisR = crate::BitReader;
#[doc = "Field `TXMIS` reader - 3:3\\]
Masked interrupt state of transmit FIFO interrupt: This field returns the masked interrupt state of transmit FIFO interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
pub type TxmisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Masked interrupt state of receive overrun interrupt: This field returns the masked interrupt state of receive overrun interrupt which is the AND product of raw interrupt state RIS.RORRIS and the mask setting IMSC.RORIM."]
    #[inline(always)]
    pub fn rormis(&self) -> RormisR {
        RormisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Masked interrupt state of receive timeout interrupt: This field returns the masked interrupt state of receive timeout interrupt which is the AND product of raw interrupt state RIS.RTRIS and the mask setting IMSC.RTIM."]
    #[inline(always)]
    pub fn rtmis(&self) -> RtmisR {
        RtmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Masked interrupt state of receive FIFO interrupt: This field returns the masked interrupt state of receive FIFO interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
    #[inline(always)]
    pub fn rxmis(&self) -> RxmisR {
        RxmisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Masked interrupt state of transmit FIFO interrupt: This field returns the masked interrupt state of transmit FIFO interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
    #[inline(always)]
    pub fn txmis(&self) -> TxmisR {
        TxmisR::new(((self.bits >> 3) & 1) != 0)
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

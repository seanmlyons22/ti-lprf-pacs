#[doc = "Register `UDMACH3BSEL` reader"]
pub type R = crate::R<Udmach3bselSpec>;
#[doc = "Register `UDMACH3BSEL` writer"]
pub type W = crate::W<Udmach3bselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 40"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "40: SPI0 RX DMA burst request , controlled by SPI0:DMACR.RXDMAE"]
    Spi0RxDmabreq = 40,
}
impl From<Ev> for u8 {
    #[inline(always)]
    fn from(variant: Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev {
    type Ux = u8;
}
impl crate::IsEnum for Ev {}
#[doc = "Field `EV` reader - 7:0\\]
Read only selection value"]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            40 => Some(Ev::Spi0RxDmabreq),
            _ => None,
        }
    }
    #[doc = "SPI0 RX DMA burst request , controlled by SPI0:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn is_spi0_rx_dmabreq(&self) -> bool {
        *self == Ev::Spi0RxDmabreq
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Output Selection for DMA Channel 3 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach3bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach3bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach3bselSpec;
impl crate::RegisterSpec for Udmach3bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach3bsel::R`](R) reader structure"]
impl crate::Readable for Udmach3bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach3bsel::W`](W) writer structure"]
impl crate::Writable for Udmach3bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH3BSEL to value 0x28"]
impl crate::Resettable for Udmach3bselSpec {
    const RESET_VALUE: u32 = 0x28;
}

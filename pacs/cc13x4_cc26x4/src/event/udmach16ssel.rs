#[doc = "Register `UDMACH16SSEL` reader"]
pub type R = crate::R<Udmach16sselSpec>;
#[doc = "Register `UDMACH16SSEL` writer"]
pub type W = crate::W<Udmach16sselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 45"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "45: SPI1 RX DMA single request, controlled by SPI0:DMACR.RXDMAE"]
    Spi1RxDmasreq = 45,
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
            45 => Some(Ev::Spi1RxDmasreq),
            _ => None,
        }
    }
    #[doc = "SPI1 RX DMA single request, controlled by SPI0:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn is_spi1_rx_dmasreq(&self) -> bool {
        *self == Ev::Spi1RxDmasreq
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
#[doc = "Output Selection for DMA Channel 16 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach16ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach16ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach16sselSpec;
impl crate::RegisterSpec for Udmach16sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach16ssel::R`](R) reader structure"]
impl crate::Readable for Udmach16sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach16ssel::W`](W) writer structure"]
impl crate::Writable for Udmach16sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH16SSEL to value 0x2d"]
impl crate::Resettable for Udmach16sselSpec {
    const RESET_VALUE: u32 = 0x2d;
}

#[doc = "Register `UDMACH28BSEL` reader"]
pub type R = crate::R<Udmach28bselSpec>;
#[doc = "Register `UDMACH28BSEL` writer"]
pub type W = crate::W<Udmach28bselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 135"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "135: UART2 RX DMA burst request, controlled by UART2:DMACTL.RXDMAE"]
    Uart2RxDmabreq = 135,
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
            135 => Some(Ev::Uart2RxDmabreq),
            _ => None,
        }
    }
    #[doc = "UART2 RX DMA burst request, controlled by UART2:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn is_uart2_rx_dmabreq(&self) -> bool {
        *self == Ev::Uart2RxDmabreq
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
#[doc = "Output Selection for DMA Channel 28 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach28bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach28bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach28bselSpec;
impl crate::RegisterSpec for Udmach28bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach28bsel::R`](R) reader structure"]
impl crate::Readable for Udmach28bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach28bsel::W`](W) writer structure"]
impl crate::Writable for Udmach28bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH28BSEL to value 0x87"]
impl crate::Resettable for Udmach28bselSpec {
    const RESET_VALUE: u32 = 0x87;
}

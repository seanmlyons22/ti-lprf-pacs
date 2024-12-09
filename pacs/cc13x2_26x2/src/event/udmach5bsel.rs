#[doc = "Register `UDMACH5BSEL` reader"]
pub type R = crate::R<Udmach5bselSpec>;
#[doc = "Register `UDMACH5BSEL` writer"]
pub type W = crate::W<Udmach5bselSpec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 52"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "52: UART1 RX DMA burst request, controlled by UART1:DMACTL.RXDMAE"]
    Uart1RxDmabreq = 52,
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
#[doc = "Field `EV` reader - 6:0\\]
Read only selection value"]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            52 => Some(Ev::Uart1RxDmabreq),
            _ => None,
        }
    }
    #[doc = "UART1 RX DMA burst request, controlled by UART1:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn is_uart1_rx_dmabreq(&self) -> bool {
        *self == Ev::Uart1RxDmabreq
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 0x7f) as u8)
    }
}
impl W {}
#[doc = "Output Selection for DMA Channel 5 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach5bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach5bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach5bselSpec;
impl crate::RegisterSpec for Udmach5bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach5bsel::R`](R) reader structure"]
impl crate::Readable for Udmach5bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach5bsel::W`](W) writer structure"]
impl crate::Writable for Udmach5bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH5BSEL to value 0x34"]
impl crate::Resettable for Udmach5bselSpec {
    const RESET_VALUE: u32 = 0x34;
}

#[doc = "Register `UDMACH28SSEL` reader"]
pub type R = crate::R<Udmach28sselSpec>;
#[doc = "Register `UDMACH28SSEL` writer"]
pub type W = crate::W<Udmach28sselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 136"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "136: UART2 RX DMA single request, controlled by UART2:DMACTL.RXDMAE"]
    Uart2RxDmasreq = 136,
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
            136 => Some(Ev::Uart2RxDmasreq),
            _ => None,
        }
    }
    #[doc = "UART2 RX DMA single request, controlled by UART2:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn is_uart2_rx_dmasreq(&self) -> bool {
        *self == Ev::Uart2RxDmasreq
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
#[doc = "Output Selection for DMA Channel 28 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach28ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach28ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach28sselSpec;
impl crate::RegisterSpec for Udmach28sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach28ssel::R`](R) reader structure"]
impl crate::Readable for Udmach28sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach28ssel::W`](W) writer structure"]
impl crate::Writable for Udmach28sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH28SSEL to value 0x88"]
impl crate::Resettable for Udmach28sselSpec {
    const RESET_VALUE: u32 = 0x88;
}

#[doc = "Register `UDMACH30SSEL` reader"]
pub type R = crate::R<Udmach30sselSpec>;
#[doc = "Register `UDMACH30SSEL` writer"]
pub type W = crate::W<Udmach30sselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 140"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "140: UART3 RX DMA single request, controlled by UART3:DMACTL.RXDMAE"]
    Uart3RxDmasreq = 140,
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
            140 => Some(Ev::Uart3RxDmasreq),
            _ => None,
        }
    }
    #[doc = "UART3 RX DMA single request, controlled by UART3:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn is_uart3_rx_dmasreq(&self) -> bool {
        *self == Ev::Uart3RxDmasreq
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
#[doc = "Output Selection for DMA Channel 30 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach30ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach30ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach30sselSpec;
impl crate::RegisterSpec for Udmach30sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach30ssel::R`](R) reader structure"]
impl crate::Readable for Udmach30sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach30ssel::W`](W) writer structure"]
impl crate::Writable for Udmach30sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH30SSEL to value 0x8c"]
impl crate::Resettable for Udmach30sselSpec {
    const RESET_VALUE: u32 = 0x8c;
}

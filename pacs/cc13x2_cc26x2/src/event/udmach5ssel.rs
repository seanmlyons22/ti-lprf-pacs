#[doc = "Register `UDMACH5SSEL` reader"]
pub type R = crate::R<Udmach5sselSpec>;
#[doc = "Register `UDMACH5SSEL` writer"]
pub type W = crate::W<Udmach5sselSpec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 53"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "53: UART1 RX DMA single request, controlled by UART1:DMACTL.RXDMAE"]
    Uart1RxDmasreq = 53,
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
            53 => Some(Ev::Uart1RxDmasreq),
            _ => None,
        }
    }
    #[doc = "UART1 RX DMA single request, controlled by UART1:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn is_uart1_rx_dmasreq(&self) -> bool {
        *self == Ev::Uart1RxDmasreq
    }
}
#[doc = "Field `EV` writer - 6:0\\]
Read only selection value"]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 7, Ev>;
impl<'a, REG> EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UART1 RX DMA single request, controlled by UART1:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn uart1_rx_dmasreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart1RxDmasreq)
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
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read only selection value"]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<Udmach5sselSpec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for DMA Channel 5 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach5ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach5ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach5sselSpec;
impl crate::RegisterSpec for Udmach5sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach5ssel::R`](R) reader structure"]
impl crate::Readable for Udmach5sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach5ssel::W`](W) writer structure"]
impl crate::Writable for Udmach5sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH5SSEL to value 0x35"]
impl crate::Resettable for Udmach5sselSpec {
    const RESET_VALUE: u32 = 0x35;
}

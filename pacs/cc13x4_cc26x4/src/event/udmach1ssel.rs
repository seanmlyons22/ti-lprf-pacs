#[doc = "Register `UDMACH1SSEL` reader"]
pub type R = crate::R<Udmach1sselSpec>;
#[doc = "Register `UDMACH1SSEL` writer"]
pub type W = crate::W<Udmach1sselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 49"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "49: UART0 RX DMA single request, controlled by UART0:DMACTL.RXDMAE"]
    Uart0RxDmasreq = 49,
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
            49 => Some(Ev::Uart0RxDmasreq),
            _ => None,
        }
    }
    #[doc = "UART0 RX DMA single request, controlled by UART0:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn is_uart0_rx_dmasreq(&self) -> bool {
        *self == Ev::Uart0RxDmasreq
    }
}
#[doc = "Field `EV` writer - 7:0\\]
Read only selection value"]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 8, Ev>;
impl<'a, REG> EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UART0 RX DMA single request, controlled by UART0:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn uart0_rx_dmasreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart0RxDmasreq)
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
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read only selection value"]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<Udmach1sselSpec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for DMA Channel 1 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach1ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach1ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach1sselSpec;
impl crate::RegisterSpec for Udmach1sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach1ssel::R`](R) reader structure"]
impl crate::Readable for Udmach1sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach1ssel::W`](W) writer structure"]
impl crate::Writable for Udmach1sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH1SSEL to value 0x31"]
impl crate::Resettable for Udmach1sselSpec {
    const RESET_VALUE: u32 = 0x31;
}

#[doc = "Register `UDMACH30BSEL` reader"]
pub type R = crate::R<Udmach30bselSpec>;
#[doc = "Register `UDMACH30BSEL` writer"]
pub type W = crate::W<Udmach30bselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 139"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "139: UART3 RX DMA burst request, controlled by UART3:DMACTL.RXDMAE"]
    Uart3RxDmabreq = 139,
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
            139 => Some(Ev::Uart3RxDmabreq),
            _ => None,
        }
    }
    #[doc = "UART3 RX DMA burst request, controlled by UART3:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn is_uart3_rx_dmabreq(&self) -> bool {
        *self == Ev::Uart3RxDmabreq
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
    #[doc = "UART3 RX DMA burst request, controlled by UART3:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn uart3_rx_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart3RxDmabreq)
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
    pub fn ev(&mut self) -> EvW<Udmach30bselSpec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for DMA Channel 30 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach30bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach30bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach30bselSpec;
impl crate::RegisterSpec for Udmach30bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach30bsel::R`](R) reader structure"]
impl crate::Readable for Udmach30bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach30bsel::W`](W) writer structure"]
impl crate::Writable for Udmach30bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH30BSEL to value 0x8b"]
impl crate::Resettable for Udmach30bselSpec {
    const RESET_VALUE: u32 = 0x8b;
}

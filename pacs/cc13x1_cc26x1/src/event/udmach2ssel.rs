#[doc = "Register `UDMACH2SSEL` reader"]
pub type R = crate::R<Udmach2sselSpec>;
#[doc = "Register `UDMACH2SSEL` writer"]
pub type W = crate::W<Udmach2sselSpec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 51"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "51: UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE"]
    Uart0TxDmasreq = 51,
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
            51 => Some(Ev::Uart0TxDmasreq),
            _ => None,
        }
    }
    #[doc = "UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE"]
    #[inline(always)]
    pub fn is_uart0_tx_dmasreq(&self) -> bool {
        *self == Ev::Uart0TxDmasreq
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
    #[doc = "UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE"]
    #[inline(always)]
    pub fn uart0_tx_dmasreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart0TxDmasreq)
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
    pub fn ev(&mut self) -> EvW<Udmach2sselSpec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for DMA Channel 2 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach2ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach2ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach2sselSpec;
impl crate::RegisterSpec for Udmach2sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach2ssel::R`](R) reader structure"]
impl crate::Readable for Udmach2sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach2ssel::W`](W) writer structure"]
impl crate::Writable for Udmach2sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH2SSEL to value 0x33"]
impl crate::Resettable for Udmach2sselSpec {
    const RESET_VALUE: u32 = 0x33;
}

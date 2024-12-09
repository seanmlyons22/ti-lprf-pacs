#[doc = "Register `UDMACH6BSEL` reader"]
pub type R = crate::R<Udmach6bselSpec>;
#[doc = "Register `UDMACH6BSEL` writer"]
pub type W = crate::W<Udmach6bselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 54"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "54: UART1 TX DMA burst request, controlled by UART1:DMACTL.TXDMAE"]
    Uart1TxDmabreq = 54,
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
            54 => Some(Ev::Uart1TxDmabreq),
            _ => None,
        }
    }
    #[doc = "UART1 TX DMA burst request, controlled by UART1:DMACTL.TXDMAE"]
    #[inline(always)]
    pub fn is_uart1_tx_dmabreq(&self) -> bool {
        *self == Ev::Uart1TxDmabreq
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
#[doc = "Output Selection for DMA Channel 6 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach6bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach6bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach6bselSpec;
impl crate::RegisterSpec for Udmach6bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach6bsel::R`](R) reader structure"]
impl crate::Readable for Udmach6bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach6bsel::W`](W) writer structure"]
impl crate::Writable for Udmach6bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH6BSEL to value 0x36"]
impl crate::Resettable for Udmach6bselSpec {
    const RESET_VALUE: u32 = 0x36;
}

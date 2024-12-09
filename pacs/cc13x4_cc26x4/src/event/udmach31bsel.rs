#[doc = "Register `UDMACH31BSEL` reader"]
pub type R = crate::R<Udmach31bselSpec>;
#[doc = "Register `UDMACH31BSEL` writer"]
pub type W = crate::W<Udmach31bselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 141"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "141: UART3 TX DMA burst request, controlled by UART3:DMACTL.TXDMAE"]
    Uart3TxDmabreq = 141,
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
            141 => Some(Ev::Uart3TxDmabreq),
            _ => None,
        }
    }
    #[doc = "UART3 TX DMA burst request, controlled by UART3:DMACTL.TXDMAE"]
    #[inline(always)]
    pub fn is_uart3_tx_dmabreq(&self) -> bool {
        *self == Ev::Uart3TxDmabreq
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
#[doc = "Output Selection for DMA Channel 31 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach31bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach31bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach31bselSpec;
impl crate::RegisterSpec for Udmach31bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach31bsel::R`](R) reader structure"]
impl crate::Readable for Udmach31bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach31bsel::W`](W) writer structure"]
impl crate::Writable for Udmach31bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH31BSEL to value 0x8d"]
impl crate::Resettable for Udmach31bselSpec {
    const RESET_VALUE: u32 = 0x8d;
}

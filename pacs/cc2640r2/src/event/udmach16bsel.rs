#[doc = "Register `UDMACH16BSEL` reader"]
pub type R = crate::R<Udmach16bselSpec>;
#[doc = "Register `UDMACH16BSEL` writer"]
pub type W = crate::W<Udmach16bselSpec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 44"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "44: SSI1 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE"]
    Ssi1RxDmabreq = 44,
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
            44 => Some(Ev::Ssi1RxDmabreq),
            _ => None,
        }
    }
    #[doc = "SSI1 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn is_ssi1_rx_dmabreq(&self) -> bool {
        *self == Ev::Ssi1RxDmabreq
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
#[doc = "Output Selection for DMA Channel 16 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach16bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach16bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach16bselSpec;
impl crate::RegisterSpec for Udmach16bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach16bsel::R`](R) reader structure"]
impl crate::Readable for Udmach16bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach16bsel::W`](W) writer structure"]
impl crate::Writable for Udmach16bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH16BSEL to value 0x2c"]
impl crate::Resettable for Udmach16bselSpec {
    const RESET_VALUE: u32 = 0x2c;
}

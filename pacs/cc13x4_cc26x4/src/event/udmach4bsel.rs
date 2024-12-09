#[doc = "Register `UDMACH4BSEL` reader"]
pub type R = crate::R<Udmach4bselSpec>;
#[doc = "Register `UDMACH4BSEL` writer"]
pub type W = crate::W<Udmach4bselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 42"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "42: SPI0 TX DMA burst request , controlled by SPI0:DMACR.TXDMAE"]
    Spi0TxDmabreq = 42,
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
            42 => Some(Ev::Spi0TxDmabreq),
            _ => None,
        }
    }
    #[doc = "SPI0 TX DMA burst request , controlled by SPI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn is_spi0_tx_dmabreq(&self) -> bool {
        *self == Ev::Spi0TxDmabreq
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
#[doc = "Output Selection for DMA Channel 4 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach4bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach4bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach4bselSpec;
impl crate::RegisterSpec for Udmach4bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach4bsel::R`](R) reader structure"]
impl crate::Readable for Udmach4bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach4bsel::W`](W) writer structure"]
impl crate::Writable for Udmach4bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH4BSEL to value 0x2a"]
impl crate::Resettable for Udmach4bselSpec {
    const RESET_VALUE: u32 = 0x2a;
}

#[doc = "Register `UDMACH26BSEL` reader"]
pub type R = crate::R<Udmach26bselSpec>;
#[doc = "Register `UDMACH26BSEL` writer"]
pub type W = crate::W<Udmach26bselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 133"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "133: SPI3 TX DMA burst request , controlled by SPI2:DMACR.TXDMAE"]
    Spi3TxDmabreq = 133,
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
            133 => Some(Ev::Spi3TxDmabreq),
            _ => None,
        }
    }
    #[doc = "SPI3 TX DMA burst request , controlled by SPI2:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn is_spi3_tx_dmabreq(&self) -> bool {
        *self == Ev::Spi3TxDmabreq
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
#[doc = "Output Selection for DMA Channel 26 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach26bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach26bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach26bselSpec;
impl crate::RegisterSpec for Udmach26bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach26bsel::R`](R) reader structure"]
impl crate::Readable for Udmach26bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach26bsel::W`](W) writer structure"]
impl crate::Writable for Udmach26bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH26BSEL to value 0x85"]
impl crate::Resettable for Udmach26bselSpec {
    const RESET_VALUE: u32 = 0x85;
}

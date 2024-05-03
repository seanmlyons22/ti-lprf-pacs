#[doc = "Register `UDMACH19BSEL` reader"]
pub type R = crate::R<Udmach19bselSpec>;
#[doc = "Register `UDMACH19BSEL` writer"]
pub type W = crate::W<Udmach19bselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 127"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "127: SPI2 RX DMA burst request , controlled by SPI2:DMACR.RXDMAE"]
    Spi2RxDmabreq = 127,
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
            127 => Some(Ev::Spi2RxDmabreq),
            _ => None,
        }
    }
    #[doc = "SPI2 RX DMA burst request , controlled by SPI2:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn is_spi2_rx_dmabreq(&self) -> bool {
        *self == Ev::Spi2RxDmabreq
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
    #[doc = "SPI2 RX DMA burst request , controlled by SPI2:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn spi2_rx_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Spi2RxDmabreq)
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
    pub fn ev(&mut self) -> EvW<Udmach19bselSpec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for DMA Channel 19 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach19bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach19bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach19bselSpec;
impl crate::RegisterSpec for Udmach19bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach19bsel::R`](R) reader structure"]
impl crate::Readable for Udmach19bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach19bsel::W`](W) writer structure"]
impl crate::Writable for Udmach19bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH19BSEL to value 0x7f"]
impl crate::Resettable for Udmach19bselSpec {
    const RESET_VALUE: u32 = 0x7f;
}

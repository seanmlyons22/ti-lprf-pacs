#[doc = "Register `UDMACH4SSEL` reader"]
pub type R = crate::R<Udmach4sselSpec>;
#[doc = "Register `UDMACH4SSEL` writer"]
pub type W = crate::W<Udmach4sselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 43"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "43: SPI0 TX DMA single request, controlled by SPI0:DMACR.TXDMAE"]
    Spi0TxDmasreq = 43,
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
            43 => Some(Ev::Spi0TxDmasreq),
            _ => None,
        }
    }
    #[doc = "SPI0 TX DMA single request, controlled by SPI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn is_spi0_tx_dmasreq(&self) -> bool {
        *self == Ev::Spi0TxDmasreq
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
#[doc = "Output Selection for DMA Channel 4 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach4ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach4ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach4sselSpec;
impl crate::RegisterSpec for Udmach4sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach4ssel::R`](R) reader structure"]
impl crate::Readable for Udmach4sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach4ssel::W`](W) writer structure"]
impl crate::Writable for Udmach4sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH4SSEL to value 0x2b"]
impl crate::Resettable for Udmach4sselSpec {
    const RESET_VALUE: u32 = 0x2b;
}

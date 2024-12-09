#[doc = "Register `UDMACH20SSEL` reader"]
pub type R = crate::R<Udmach20sselSpec>;
#[doc = "Register `UDMACH20SSEL` writer"]
pub type W = crate::W<Udmach20sselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 130"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "130: SPI2 TX DMA single request, controlled by SPI2:DMACR.TXDMAE"]
    Spi2TxDmasreq = 130,
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
            130 => Some(Ev::Spi2TxDmasreq),
            _ => None,
        }
    }
    #[doc = "SPI2 TX DMA single request, controlled by SPI2:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn is_spi2_tx_dmasreq(&self) -> bool {
        *self == Ev::Spi2TxDmasreq
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
#[doc = "Output Selection for DMA Channel 20 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach20ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach20ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach20sselSpec;
impl crate::RegisterSpec for Udmach20sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach20ssel::R`](R) reader structure"]
impl crate::Readable for Udmach20sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach20ssel::W`](W) writer structure"]
impl crate::Writable for Udmach20sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH20SSEL to value 0x82"]
impl crate::Resettable for Udmach20sselSpec {
    const RESET_VALUE: u32 = 0x82;
}

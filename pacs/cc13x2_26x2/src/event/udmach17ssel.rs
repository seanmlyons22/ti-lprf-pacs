#[doc = "Register `UDMACH17SSEL` reader"]
pub type R = crate::R<Udmach17sselSpec>;
#[doc = "Register `UDMACH17SSEL` writer"]
pub type W = crate::W<Udmach17sselSpec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 47"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "47: SSI1 TX DMA single request, controlled by SSI0:DMACR.TXDMAE"]
    Ssi1TxDmasreq = 47,
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
            47 => Some(Ev::Ssi1TxDmasreq),
            _ => None,
        }
    }
    #[doc = "SSI1 TX DMA single request, controlled by SSI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn is_ssi1_tx_dmasreq(&self) -> bool {
        *self == Ev::Ssi1TxDmasreq
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
#[doc = "Output Selection for DMA Channel 17 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach17ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach17ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach17sselSpec;
impl crate::RegisterSpec for Udmach17sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach17ssel::R`](R) reader structure"]
impl crate::Readable for Udmach17sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach17ssel::W`](W) writer structure"]
impl crate::Writable for Udmach17sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH17SSEL to value 0x2f"]
impl crate::Resettable for Udmach17sselSpec {
    const RESET_VALUE: u32 = 0x2f;
}

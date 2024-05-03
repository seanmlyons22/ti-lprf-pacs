#[doc = "Register `UDMACH17BSEL` reader"]
pub type R = crate::R<Udmach17bselSpec>;
#[doc = "Register `UDMACH17BSEL` writer"]
pub type W = crate::W<Udmach17bselSpec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 46"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "46: SSI1 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE"]
    Ssi1TxDmabreq = 46,
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
            46 => Some(Ev::Ssi1TxDmabreq),
            _ => None,
        }
    }
    #[doc = "SSI1 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn is_ssi1_tx_dmabreq(&self) -> bool {
        *self == Ev::Ssi1TxDmabreq
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
    #[doc = "SSI1 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn ssi1_tx_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Ssi1TxDmabreq)
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
    pub fn ev(&mut self) -> EvW<Udmach17bselSpec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for DMA Channel 17 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach17bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach17bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach17bselSpec;
impl crate::RegisterSpec for Udmach17bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach17bsel::R`](R) reader structure"]
impl crate::Readable for Udmach17bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach17bsel::W`](W) writer structure"]
impl crate::Writable for Udmach17bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH17BSEL to value 0x2e"]
impl crate::Resettable for Udmach17bselSpec {
    const RESET_VALUE: u32 = 0x2e;
}

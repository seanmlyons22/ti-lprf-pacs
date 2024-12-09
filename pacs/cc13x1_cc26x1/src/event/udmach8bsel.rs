#[doc = "Register `UDMACH8BSEL` reader"]
pub type R = crate::R<Udmach8bselSpec>;
#[doc = "Register `UDMACH8BSEL` writer"]
pub type W = crate::W<Udmach8bselSpec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 116"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "116: DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    AuxSwDmabreq = 116,
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
            116 => Some(Ev::AuxSwDmabreq),
            _ => None,
        }
    }
    #[doc = "DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    #[inline(always)]
    pub fn is_aux_sw_dmabreq(&self) -> bool {
        *self == Ev::AuxSwDmabreq
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
#[doc = "Output Selection for DMA Channel 8 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach8bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach8bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach8bselSpec;
impl crate::RegisterSpec for Udmach8bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach8bsel::R`](R) reader structure"]
impl crate::Readable for Udmach8bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach8bsel::W`](W) writer structure"]
impl crate::Writable for Udmach8bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH8BSEL to value 0x74"]
impl crate::Resettable for Udmach8bselSpec {
    const RESET_VALUE: u32 = 0x74;
}

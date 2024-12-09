#[doc = "Register `UDMACH23BSEL` reader"]
pub type R = crate::R<Udmach23bselSpec>;
#[doc = "Register `UDMACH23BSEL` writer"]
pub type W = crate::W<Udmach23bselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 102"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "102: Software event 2, triggered by SWEV.SWEV2"]
    Swev2 = 102,
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
            102 => Some(Ev::Swev2),
            _ => None,
        }
    }
    #[doc = "Software event 2, triggered by SWEV.SWEV2"]
    #[inline(always)]
    pub fn is_swev2(&self) -> bool {
        *self == Ev::Swev2
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
#[doc = "Output Selection for DMA Channel 23 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach23bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach23bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach23bselSpec;
impl crate::RegisterSpec for Udmach23bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach23bsel::R`](R) reader structure"]
impl crate::Readable for Udmach23bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach23bsel::W`](W) writer structure"]
impl crate::Writable for Udmach23bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH23BSEL to value 0x66"]
impl crate::Resettable for Udmach23bselSpec {
    const RESET_VALUE: u32 = 0x66;
}

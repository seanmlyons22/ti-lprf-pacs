#[doc = "Register `UDMACH21BSEL` reader"]
pub type R = crate::R<Udmach21bselSpec>;
#[doc = "Register `UDMACH21BSEL` writer"]
pub type W = crate::W<Udmach21bselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 100"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "100: Software event 0, triggered by SWEV.SWEV0"]
    Swev0 = 100,
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
            100 => Some(Ev::Swev0),
            _ => None,
        }
    }
    #[doc = "Software event 0, triggered by SWEV.SWEV0"]
    #[inline(always)]
    pub fn is_swev0(&self) -> bool {
        *self == Ev::Swev0
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
#[doc = "Output Selection for DMA Channel 21 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach21bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach21bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach21bselSpec;
impl crate::RegisterSpec for Udmach21bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach21bsel::R`](R) reader structure"]
impl crate::Readable for Udmach21bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach21bsel::W`](W) writer structure"]
impl crate::Writable for Udmach21bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH21BSEL to value 0x64"]
impl crate::Resettable for Udmach21bselSpec {
    const RESET_VALUE: u32 = 0x64;
}

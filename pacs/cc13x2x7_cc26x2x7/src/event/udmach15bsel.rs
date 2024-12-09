#[doc = "Register `UDMACH15BSEL` reader"]
pub type R = crate::R<Udmach15bselSpec>;
#[doc = "Register `UDMACH15BSEL` writer"]
pub type W = crate::W<Udmach15bselSpec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "7: Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    AonRtcComb = 7,
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
            7 => Some(Ev::AonRtcComb),
            _ => None,
        }
    }
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    #[inline(always)]
    pub fn is_aon_rtc_comb(&self) -> bool {
        *self == Ev::AonRtcComb
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
#[doc = "Output Selection for DMA Channel 15 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach15bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach15bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach15bselSpec;
impl crate::RegisterSpec for Udmach15bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach15bsel::R`](R) reader structure"]
impl crate::Readable for Udmach15bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach15bsel::W`](W) writer structure"]
impl crate::Writable for Udmach15bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH15BSEL to value 0x07"]
impl crate::Resettable for Udmach15bselSpec {
    const RESET_VALUE: u32 = 0x07;
}

#[doc = "Register `UDMACH15SSEL` reader"]
pub type R = crate::R<Udmach15sselSpec>;
#[doc = "Register `UDMACH15SSEL` writer"]
pub type W = crate::W<Udmach15sselSpec>;
#[doc = "31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ev {
    #[doc = "7: Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    AonRtcComb = 7,
}
impl From<Ev> for u32 {
    #[inline(always)]
    fn from(variant: Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev {
    type Ux = u32;
}
impl crate::IsEnum for Ev {}
#[doc = "Field `EV` reader - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new(self.bits)
    }
}
impl W {}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach15ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach15ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach15sselSpec;
impl crate::RegisterSpec for Udmach15sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach15ssel::R`](R) reader structure"]
impl crate::Readable for Udmach15sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach15ssel::W`](W) writer structure"]
impl crate::Writable for Udmach15sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH15SSEL to value 0x07"]
impl crate::Resettable for Udmach15sselSpec {
    const RESET_VALUE: u32 = 0x07;
}

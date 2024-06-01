#[doc = "Register `RFCSEL8` reader"]
pub type R = crate::R<Rfcsel8Spec>;
#[doc = "Register `RFCSEL8` writer"]
pub type W = crate::W<Rfcsel8Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 119"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "119: RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    AonRtcUpd = 119,
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
            119 => Some(Ev::AonRtcUpd),
            _ => None,
        }
    }
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    #[inline(always)]
    pub fn is_aon_rtc_upd(&self) -> bool {
        *self == Ev::AonRtcUpd
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
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    #[inline(always)]
    pub fn aon_rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AonRtcUpd)
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
    pub fn ev(&mut self) -> EvW<Rfcsel8Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for RFC Event 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfcsel8Spec;
impl crate::RegisterSpec for Rfcsel8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcsel8::R`](R) reader structure"]
impl crate::Readable for Rfcsel8Spec {}
#[doc = "`write(|w| ..)` method takes [`rfcsel8::W`](W) writer structure"]
impl crate::Writable for Rfcsel8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCSEL8 to value 0x77"]
impl crate::Resettable for Rfcsel8Spec {
    const RESET_VALUE: u32 = 0x77;
}

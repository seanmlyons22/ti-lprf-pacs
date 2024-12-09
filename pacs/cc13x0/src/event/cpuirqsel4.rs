#[doc = "Register `CPUIRQSEL4` reader"]
pub type R = crate::R<Cpuirqsel4Spec>;
#[doc = "Register `CPUIRQSEL4` writer"]
pub type W = crate::W<Cpuirqsel4Spec>;
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
#[doc = "Output Selection for CPU Interrupt 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel4Spec;
impl crate::RegisterSpec for Cpuirqsel4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel4::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel4Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel4::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL4 to value 0x07"]
impl crate::Resettable for Cpuirqsel4Spec {
    const RESET_VALUE: u32 = 0x07;
}

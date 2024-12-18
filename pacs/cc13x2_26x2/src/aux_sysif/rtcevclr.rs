#[doc = "Register `RTCEVCLR` reader"]
pub type R = crate::R<RtcevclrSpec>;
#[doc = "Register `RTCEVCLR` writer"]
pub type W = crate::W<RtcevclrSpec>;
#[doc = "Field `RTC_CH2_EV_CLR` reader - 0:0\\]
Clear events from AON_RTC channel 2. 0: No effect. 1: Clear events from AON_RTC channel 2. Keep RTC_CH2_EV_CLR high until AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY are 0."]
pub type RtcCh2EvClrR = crate::BitReader;
#[doc = "Field `RTC_CH2_EV_CLR` writer - 0:0\\]
Clear events from AON_RTC channel 2. 0: No effect. 1: Clear events from AON_RTC channel 2. Keep RTC_CH2_EV_CLR high until AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY are 0."]
pub type RtcCh2EvClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear events from AON_RTC channel 2. 0: No effect. 1: Clear events from AON_RTC channel 2. Keep RTC_CH2_EV_CLR high until AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY are 0."]
    #[inline(always)]
    pub fn rtc_ch2_ev_clr(&self) -> RtcCh2EvClrR {
        RtcCh2EvClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear events from AON_RTC channel 2. 0: No effect. 1: Clear events from AON_RTC channel 2. Keep RTC_CH2_EV_CLR high until AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY are 0."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ch2_ev_clr(&mut self) -> RtcCh2EvClrW<RtcevclrSpec> {
        RtcCh2EvClrW::new(self, 0)
    }
}
#[doc = "AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcevclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcevclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcevclrSpec;
impl crate::RegisterSpec for RtcevclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcevclr::R`](R) reader structure"]
impl crate::Readable for RtcevclrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcevclr::W`](W) writer structure"]
impl crate::Writable for RtcevclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTCEVCLR to value 0"]
impl crate::Resettable for RtcevclrSpec {
    const RESET_VALUE: u32 = 0;
}

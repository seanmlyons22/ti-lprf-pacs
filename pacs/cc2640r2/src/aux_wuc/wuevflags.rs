#[doc = "Register `WUEVFLAGS` reader"]
pub type R = crate::R<WuevflagsSpec>;
#[doc = "Register `WUEVFLAGS` writer"]
pub type W = crate::W<WuevflagsSpec>;
#[doc = "Field `AON_PROG_WU` reader - 0:0\\]
Indicates pending event triggered by the sources selected in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV and AON_EVENT:AUXWUSEL.WU2_EV."]
pub type AonProgWuR = crate::BitReader;
#[doc = "Field `AON_SW` reader - 1:1\\]
Indicates pending event triggered by system CPU writing a 1 to AON_WUC:AUXCTL.SWEV."]
pub type AonSwR = crate::BitReader;
#[doc = "Field `AON_RTC_CH2` reader - 2:2\\]
Indicates pending event from AON_RTC_CH2 compare. Note that this flag will be set whenever the AON_RTC_CH2 event happens, but that does not mean that this event is a wake-up event. To make the AON_RTC_CH2 a wake-up event for the AUX domain configure it as a wake-up event in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV or AON_EVENT:AUXWUSEL.WU2_EV."]
pub type AonRtcCh2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates pending event triggered by the sources selected in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV and AON_EVENT:AUXWUSEL.WU2_EV."]
    #[inline(always)]
    pub fn aon_prog_wu(&self) -> AonProgWuR {
        AonProgWuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates pending event triggered by system CPU writing a 1 to AON_WUC:AUXCTL.SWEV."]
    #[inline(always)]
    pub fn aon_sw(&self) -> AonSwR {
        AonSwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates pending event from AON_RTC_CH2 compare. Note that this flag will be set whenever the AON_RTC_CH2 event happens, but that does not mean that this event is a wake-up event. To make the AON_RTC_CH2 a wake-up event for the AUX domain configure it as a wake-up event in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV or AON_EVENT:AUXWUSEL.WU2_EV."]
    #[inline(always)]
    pub fn aon_rtc_ch2(&self) -> AonRtcCh2R {
        AonRtcCh2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {}
#[doc = "Wake-up Event Flags Status of wake-up events from the AON domain The event flags are cleared by setting the corresponding bits in WUEVCLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wuevflags::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wuevflags::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WuevflagsSpec;
impl crate::RegisterSpec for WuevflagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wuevflags::R`](R) reader structure"]
impl crate::Readable for WuevflagsSpec {}
#[doc = "`write(|w| ..)` method takes [`wuevflags::W`](W) writer structure"]
impl crate::Writable for WuevflagsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUEVFLAGS to value 0"]
impl crate::Resettable for WuevflagsSpec {
    const RESET_VALUE: u32 = 0;
}

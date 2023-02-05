#[doc = "Register `WUEVFLAGS` reader"]
pub struct R(crate::R<WUEVFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUEVFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUEVFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUEVFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUEVFLAGS` writer"]
pub struct W(crate::W<WUEVFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUEVFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WUEVFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUEVFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AON_PROG_WU` reader - 0:0\\]
Indicates pending event triggered by the sources selected in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV and AON_EVENT:AUXWUSEL.WU2_EV."]
pub type AON_PROG_WU_R = crate::BitReader<bool>;
#[doc = "Field `AON_PROG_WU` writer - 0:0\\]
Indicates pending event triggered by the sources selected in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV and AON_EVENT:AUXWUSEL.WU2_EV."]
pub type AON_PROG_WU_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUEVFLAGS_SPEC, bool, O>;
#[doc = "Field `AON_SW` reader - 1:1\\]
Indicates pending event triggered by system CPU writing a 1 to AON_WUC:AUXCTL.SWEV."]
pub type AON_SW_R = crate::BitReader<bool>;
#[doc = "Field `AON_SW` writer - 1:1\\]
Indicates pending event triggered by system CPU writing a 1 to AON_WUC:AUXCTL.SWEV."]
pub type AON_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUEVFLAGS_SPEC, bool, O>;
#[doc = "Field `AON_RTC_CH2` reader - 2:2\\]
Indicates pending event from AON_RTC_CH2 compare. Note that this flag will be set whenever the AON_RTC_CH2 event happens, but that does not mean that this event is a wake-up event. To make the AON_RTC_CH2 a wake-up event for the AUX domain configure it as a wake-up event in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV or AON_EVENT:AUXWUSEL.WU2_EV."]
pub type AON_RTC_CH2_R = crate::BitReader<bool>;
#[doc = "Field `AON_RTC_CH2` writer - 2:2\\]
Indicates pending event from AON_RTC_CH2 compare. Note that this flag will be set whenever the AON_RTC_CH2 event happens, but that does not mean that this event is a wake-up event. To make the AON_RTC_CH2 a wake-up event for the AUX domain configure it as a wake-up event in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV or AON_EVENT:AUXWUSEL.WU2_EV."]
pub type AON_RTC_CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUEVFLAGS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates pending event triggered by the sources selected in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV and AON_EVENT:AUXWUSEL.WU2_EV."]
    #[inline(always)]
    pub fn aon_prog_wu(&self) -> AON_PROG_WU_R {
        AON_PROG_WU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates pending event triggered by system CPU writing a 1 to AON_WUC:AUXCTL.SWEV."]
    #[inline(always)]
    pub fn aon_sw(&self) -> AON_SW_R {
        AON_SW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates pending event from AON_RTC_CH2 compare. Note that this flag will be set whenever the AON_RTC_CH2 event happens, but that does not mean that this event is a wake-up event. To make the AON_RTC_CH2 a wake-up event for the AUX domain configure it as a wake-up event in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV or AON_EVENT:AUXWUSEL.WU2_EV."]
    #[inline(always)]
    pub fn aon_rtc_ch2(&self) -> AON_RTC_CH2_R {
        AON_RTC_CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates pending event triggered by the sources selected in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV and AON_EVENT:AUXWUSEL.WU2_EV."]
    #[inline(always)]
    #[must_use]
    pub fn aon_prog_wu(&mut self) -> AON_PROG_WU_W<0> {
        AON_PROG_WU_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates pending event triggered by system CPU writing a 1 to AON_WUC:AUXCTL.SWEV."]
    #[inline(always)]
    #[must_use]
    pub fn aon_sw(&mut self) -> AON_SW_W<1> {
        AON_SW_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates pending event from AON_RTC_CH2 compare. Note that this flag will be set whenever the AON_RTC_CH2 event happens, but that does not mean that this event is a wake-up event. To make the AON_RTC_CH2 a wake-up event for the AUX domain configure it as a wake-up event in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV or AON_EVENT:AUXWUSEL.WU2_EV."]
    #[inline(always)]
    #[must_use]
    pub fn aon_rtc_ch2(&mut self) -> AON_RTC_CH2_W<2> {
        AON_RTC_CH2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-up Event Flags Status of wake-up events from the AON domain The event flags are cleared by setting the corresponding bits in WUEVCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wuevflags](index.html) module"]
pub struct WUEVFLAGS_SPEC;
impl crate::RegisterSpec for WUEVFLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wuevflags::R](R) reader structure"]
impl crate::Readable for WUEVFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wuevflags::W](W) writer structure"]
impl crate::Writable for WUEVFLAGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUEVFLAGS to value 0"]
impl crate::Resettable for WUEVFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

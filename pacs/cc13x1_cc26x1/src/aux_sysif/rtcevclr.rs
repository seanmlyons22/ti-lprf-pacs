#[doc = "Register `RTCEVCLR` reader"]
pub struct R(crate::R<RTCEVCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCEVCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCEVCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCEVCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCEVCLR` writer"]
pub struct W(crate::W<RTCEVCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCEVCLR_SPEC>;
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
impl From<crate::W<RTCEVCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCEVCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CH2_EV_CLR` reader - 0:0\\]
Clear events from AON_RTC channel 2. 0: No effect. 1: Clear events from AON_RTC channel 2. Keep RTC_CH2_EV_CLR high until AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY are 0."]
pub type RTC_CH2_EV_CLR_R = crate::BitReader<bool>;
#[doc = "Field `RTC_CH2_EV_CLR` writer - 0:0\\]
Clear events from AON_RTC channel 2. 0: No effect. 1: Clear events from AON_RTC channel 2. Keep RTC_CH2_EV_CLR high until AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY are 0."]
pub type RTC_CH2_EV_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCEVCLR_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCEVCLR_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear events from AON_RTC channel 2. 0: No effect. 1: Clear events from AON_RTC channel 2. Keep RTC_CH2_EV_CLR high until AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY are 0."]
    #[inline(always)]
    pub fn rtc_ch2_ev_clr(&self) -> RTC_CH2_EV_CLR_R {
        RTC_CH2_EV_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear events from AON_RTC channel 2. 0: No effect. 1: Clear events from AON_RTC channel 2. Keep RTC_CH2_EV_CLR high until AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY are 0."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ch2_ev_clr(&mut self) -> RTC_CH2_EV_CLR_W<0> {
        RTC_CH2_EV_CLR_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcevclr](index.html) module"]
pub struct RTCEVCLR_SPEC;
impl crate::RegisterSpec for RTCEVCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcevclr::R](R) reader structure"]
impl crate::Readable for RTCEVCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcevclr::W](W) writer structure"]
impl crate::Writable for RTCEVCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCEVCLR to value 0"]
impl crate::Resettable for RTCEVCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

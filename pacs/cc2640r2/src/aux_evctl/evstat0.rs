#[doc = "Register `EVSTAT0` reader"]
pub struct R(crate::R<EVSTAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVSTAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVSTAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVSTAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVSTAT0` writer"]
pub struct W(crate::W<EVSTAT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVSTAT0_SPEC>;
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
impl From<crate::W<EVSTAT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVSTAT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AON_RTC_CH2` reader - 0:0\\]
AON_RTC:EVFLAGS.CH2"]
pub type AON_RTC_CH2_R = crate::BitReader<bool>;
#[doc = "Field `AON_RTC_CH2` writer - 0:0\\]
AON_RTC:EVFLAGS.CH2"]
pub type AON_RTC_CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUX_COMPA` reader - 1:1\\]
Comparator A output"]
pub type AUX_COMPA_R = crate::BitReader<bool>;
#[doc = "Field `AUX_COMPA` writer - 1:1\\]
Comparator A output"]
pub type AUX_COMPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUX_COMPB` reader - 2:2\\]
Comparator B output"]
pub type AUX_COMPB_R = crate::BitReader<bool>;
#[doc = "Field `AUX_COMPB` writer - 2:2\\]
Comparator B output"]
pub type AUX_COMPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `TDC_DONE` reader - 3:3\\]
AUX_TDC:STAT.DONE"]
pub type TDC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TDC_DONE` writer - 3:3\\]
AUX_TDC:STAT.DONE"]
pub type TDC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `TIMER0_EV` reader - 4:4\\]
AUX_TIMER0_EV event, see AUX_TIMER:T0TARGET for description."]
pub type TIMER0_EV_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0_EV` writer - 4:4\\]
AUX_TIMER0_EV event, see AUX_TIMER:T0TARGET for description."]
pub type TIMER0_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `TIMER1_EV` reader - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER:T1TARGET for description."]
pub type TIMER1_EV_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1_EV` writer - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER:T1TARGET for description."]
pub type TIMER1_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `SMPH_AUTOTAKE_DONE` reader - 6:6\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
pub type SMPH_AUTOTAKE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `SMPH_AUTOTAKE_DONE` writer - 6:6\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
pub type SMPH_AUTOTAKE_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `ADC_DONE` reader - 7:7\\]
AUX_ANAIF ADC conversion done event."]
pub type ADC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `ADC_DONE` writer - 7:7\\]
AUX_ANAIF ADC conversion done event."]
pub type ADC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `ADC_FIFO_ALMOST_FULL` reader - 8:8\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
pub type ADC_FIFO_ALMOST_FULL_R = crate::BitReader<bool>;
#[doc = "Field `ADC_FIFO_ALMOST_FULL` writer - 8:8\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
pub type ADC_FIFO_ALMOST_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `OBSMUX0` reader - 9:9\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
pub type OBSMUX0_R = crate::BitReader<bool>;
#[doc = "Field `OBSMUX0` writer - 9:9\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
pub type OBSMUX0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `OBSMUX1` reader - 10:10\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
pub type OBSMUX1_R = crate::BitReader<bool>;
#[doc = "Field `OBSMUX1` writer - 10:10\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
pub type OBSMUX1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AON_SW` reader - 11:11\\]
AON_WUC:AUXCTL.SWEV"]
pub type AON_SW_R = crate::BitReader<bool>;
#[doc = "Field `AON_SW` writer - 11:11\\]
AON_WUC:AUXCTL.SWEV"]
pub type AON_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AON_PROG_WU` reader - 12:12\\]
AON_EVENT:AUXWUSEL.WU2_EV OR AON_EVENT:AUXWUSEL.WU1_EV OR AON_EVENT:AUXWUSEL.WU0_EV"]
pub type AON_PROG_WU_R = crate::BitReader<bool>;
#[doc = "Field `AON_PROG_WU` writer - 12:12\\]
AON_EVENT:AUXWUSEL.WU2_EV OR AON_EVENT:AUXWUSEL.WU1_EV OR AON_EVENT:AUXWUSEL.WU0_EV"]
pub type AON_PROG_WU_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO0` reader - 13:13\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
pub type AUXIO0_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO0` writer - 13:13\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
pub type AUXIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO1` reader - 14:14\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
pub type AUXIO1_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO1` writer - 14:14\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
pub type AUXIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO2` reader - 15:15\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
pub type AUXIO2_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO2` writer - 15:15\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
pub type AUXIO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
AON_RTC:EVFLAGS.CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(&self) -> AON_RTC_CH2_R {
        AON_RTC_CH2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Comparator A output"]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Comparator B output"]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUX_TDC:STAT.DONE"]
    #[inline(always)]
    pub fn tdc_done(&self) -> TDC_DONE_R {
        TDC_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUX_TIMER0_EV event, see AUX_TIMER:T0TARGET for description."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> TIMER0_EV_R {
        TIMER0_EV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER:T1TARGET for description."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> TIMER1_EV_R {
        TIMER1_EV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline(always)]
    pub fn smph_autotake_done(&self) -> SMPH_AUTOTAKE_DONE_R {
        SMPH_AUTOTAKE_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUX_ANAIF ADC conversion done event."]
    #[inline(always)]
    pub fn adc_done(&self) -> ADC_DONE_R {
        ADC_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
    #[inline(always)]
    pub fn adc_fifo_almost_full(&self) -> ADC_FIFO_ALMOST_FULL_R {
        ADC_FIFO_ALMOST_FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
    #[inline(always)]
    pub fn obsmux0(&self) -> OBSMUX0_R {
        OBSMUX0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
    #[inline(always)]
    pub fn obsmux1(&self) -> OBSMUX1_R {
        OBSMUX1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AON_WUC:AUXCTL.SWEV"]
    #[inline(always)]
    pub fn aon_sw(&self) -> AON_SW_R {
        AON_SW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AON_EVENT:AUXWUSEL.WU2_EV OR AON_EVENT:AUXWUSEL.WU1_EV OR AON_EVENT:AUXWUSEL.WU0_EV"]
    #[inline(always)]
    pub fn aon_prog_wu(&self) -> AON_PROG_WU_R {
        AON_PROG_WU_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio0(&self) -> AUXIO0_R {
        AUXIO0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio1(&self) -> AUXIO1_R {
        AUXIO1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio2(&self) -> AUXIO2_R {
        AUXIO2_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
AON_RTC:EVFLAGS.CH2"]
    #[inline(always)]
    #[must_use]
    pub fn aon_rtc_ch2(&mut self) -> AON_RTC_CH2_W<0> {
        AON_RTC_CH2_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Comparator A output"]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W<1> {
        AUX_COMPA_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Comparator B output"]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W<2> {
        AUX_COMPB_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
AUX_TDC:STAT.DONE"]
    #[inline(always)]
    #[must_use]
    pub fn tdc_done(&mut self) -> TDC_DONE_W<3> {
        TDC_DONE_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
AUX_TIMER0_EV event, see AUX_TIMER:T0TARGET for description."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_ev(&mut self) -> TIMER0_EV_W<4> {
        TIMER0_EV_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER:T1TARGET for description."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ev(&mut self) -> TIMER1_EV_W<5> {
        TIMER1_EV_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline(always)]
    #[must_use]
    pub fn smph_autotake_done(&mut self) -> SMPH_AUTOTAKE_DONE_W<6> {
        SMPH_AUTOTAKE_DONE_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
AUX_ANAIF ADC conversion done event."]
    #[inline(always)]
    #[must_use]
    pub fn adc_done(&mut self) -> ADC_DONE_W<7> {
        ADC_DONE_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
    #[inline(always)]
    #[must_use]
    pub fn adc_fifo_almost_full(&mut self) -> ADC_FIFO_ALMOST_FULL_W<8> {
        ADC_FIFO_ALMOST_FULL_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
    #[inline(always)]
    #[must_use]
    pub fn obsmux0(&mut self) -> OBSMUX0_W<9> {
        OBSMUX0_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
    #[inline(always)]
    #[must_use]
    pub fn obsmux1(&mut self) -> OBSMUX1_W<10> {
        OBSMUX1_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
AON_WUC:AUXCTL.SWEV"]
    #[inline(always)]
    #[must_use]
    pub fn aon_sw(&mut self) -> AON_SW_W<11> {
        AON_SW_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
AON_EVENT:AUXWUSEL.WU2_EV OR AON_EVENT:AUXWUSEL.WU1_EV OR AON_EVENT:AUXWUSEL.WU0_EV"]
    #[inline(always)]
    #[must_use]
    pub fn aon_prog_wu(&mut self) -> AON_PROG_WU_W<12> {
        AON_PROG_WU_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
    #[inline(always)]
    #[must_use]
    pub fn auxio0(&mut self) -> AUXIO0_W<13> {
        AUXIO0_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
    #[inline(always)]
    #[must_use]
    pub fn auxio1(&mut self) -> AUXIO1_W<14> {
        AUXIO1_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
    #[inline(always)]
    #[must_use]
    pub fn auxio2(&mut self) -> AUXIO2_W<15> {
        AUXIO2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Status 0 Register holds events 0 thru 15 of the 32-bit event bus that is synchronous to AUX clock. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat0](index.html) module"]
pub struct EVSTAT0_SPEC;
impl crate::RegisterSpec for EVSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evstat0::R](R) reader structure"]
impl crate::Readable for EVSTAT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evstat0::W](W) writer structure"]
impl crate::Writable for EVSTAT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVSTAT0 to value 0"]
impl crate::Resettable for EVSTAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

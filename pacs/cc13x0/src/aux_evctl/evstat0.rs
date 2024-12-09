#[doc = "Register `EVSTAT0` reader"]
pub type R = crate::R<Evstat0Spec>;
#[doc = "Register `EVSTAT0` writer"]
pub type W = crate::W<Evstat0Spec>;
#[doc = "Field `AON_RTC_CH2` reader - 0:0\\]
AON_RTC:EVFLAGS.CH2"]
pub type AonRtcCh2R = crate::BitReader;
#[doc = "Field `AUX_COMPA` reader - 1:1\\]
Comparator A output"]
pub type AuxCompaR = crate::BitReader;
#[doc = "Field `AUX_COMPB` reader - 2:2\\]
Comparator B output"]
pub type AuxCompbR = crate::BitReader;
#[doc = "Field `TDC_DONE` reader - 3:3\\]
AUX_TDC:STAT.DONE"]
pub type TdcDoneR = crate::BitReader;
#[doc = "Field `TIMER0_EV` reader - 4:4\\]
AUX_TIMER0_EV event, see AUX_TIMER:T0TARGET for description."]
pub type Timer0EvR = crate::BitReader;
#[doc = "Field `TIMER1_EV` reader - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER:T1TARGET for description."]
pub type Timer1EvR = crate::BitReader;
#[doc = "Field `SMPH_AUTOTAKE_DONE` reader - 6:6\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
pub type SmphAutotakeDoneR = crate::BitReader;
#[doc = "Field `ADC_DONE` reader - 7:7\\]
AUX_ANAIF ADC conversion done event."]
pub type AdcDoneR = crate::BitReader;
#[doc = "Field `ADC_FIFO_ALMOST_FULL` reader - 8:8\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
pub type AdcFifoAlmostFullR = crate::BitReader;
#[doc = "Field `OBSMUX0` reader - 9:9\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
pub type Obsmux0R = crate::BitReader;
#[doc = "Field `OBSMUX1` reader - 10:10\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
pub type Obsmux1R = crate::BitReader;
#[doc = "Field `AON_SW` reader - 11:11\\]
AON_WUC:AUXCTL.SWEV"]
pub type AonSwR = crate::BitReader;
#[doc = "Field `AON_PROG_WU` reader - 12:12\\]
AON_EVENT:AUXWUSEL.WU2_EV OR AON_EVENT:AUXWUSEL.WU1_EV OR AON_EVENT:AUXWUSEL.WU0_EV"]
pub type AonProgWuR = crate::BitReader;
#[doc = "Field `AUXIO0` reader - 13:13\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
pub type Auxio0R = crate::BitReader;
#[doc = "Field `AUXIO1` reader - 14:14\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
pub type Auxio1R = crate::BitReader;
#[doc = "Field `AUXIO2` reader - 15:15\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
pub type Auxio2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
AON_RTC:EVFLAGS.CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(&self) -> AonRtcCh2R {
        AonRtcCh2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Comparator A output"]
    #[inline(always)]
    pub fn aux_compa(&self) -> AuxCompaR {
        AuxCompaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Comparator B output"]
    #[inline(always)]
    pub fn aux_compb(&self) -> AuxCompbR {
        AuxCompbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUX_TDC:STAT.DONE"]
    #[inline(always)]
    pub fn tdc_done(&self) -> TdcDoneR {
        TdcDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUX_TIMER0_EV event, see AUX_TIMER:T0TARGET for description."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> Timer0EvR {
        Timer0EvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER:T1TARGET for description."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> Timer1EvR {
        Timer1EvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline(always)]
    pub fn smph_autotake_done(&self) -> SmphAutotakeDoneR {
        SmphAutotakeDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUX_ANAIF ADC conversion done event."]
    #[inline(always)]
    pub fn adc_done(&self) -> AdcDoneR {
        AdcDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
    #[inline(always)]
    pub fn adc_fifo_almost_full(&self) -> AdcFifoAlmostFullR {
        AdcFifoAlmostFullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
    #[inline(always)]
    pub fn obsmux0(&self) -> Obsmux0R {
        Obsmux0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
    #[inline(always)]
    pub fn obsmux1(&self) -> Obsmux1R {
        Obsmux1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AON_WUC:AUXCTL.SWEV"]
    #[inline(always)]
    pub fn aon_sw(&self) -> AonSwR {
        AonSwR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AON_EVENT:AUXWUSEL.WU2_EV OR AON_EVENT:AUXWUSEL.WU1_EV OR AON_EVENT:AUXWUSEL.WU0_EV"]
    #[inline(always)]
    pub fn aon_prog_wu(&self) -> AonProgWuR {
        AonProgWuR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio0(&self) -> Auxio0R {
        Auxio0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio1(&self) -> Auxio1R {
        Auxio1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio2(&self) -> Auxio2R {
        Auxio2R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {}
#[doc = "Event Status 0 Register holds events 0 thru 15 of the 32-bit event bus that is synchronous to AUX clock. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Evstat0Spec;
impl crate::RegisterSpec for Evstat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evstat0::R`](R) reader structure"]
impl crate::Readable for Evstat0Spec {}
#[doc = "`write(|w| ..)` method takes [`evstat0::W`](W) writer structure"]
impl crate::Writable for Evstat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVSTAT0 to value 0"]
impl crate::Resettable for Evstat0Spec {
    const RESET_VALUE: u32 = 0;
}

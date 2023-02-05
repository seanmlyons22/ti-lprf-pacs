#[doc = "Register `MCUWUSEL1` reader"]
pub struct R(crate::R<MCUWUSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUWUSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUWUSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUWUSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUWUSEL1` writer"]
pub struct W(crate::W<MCUWUSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUWUSEL1_SPEC>;
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
impl From<crate::W<MCUWUSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUWUSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WU4_EV` reader - 5:0\\]
MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU4_EV_R = crate::FieldReader<u8, WU4_EV_A>;
#[doc = "5:0\\]
MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WU4_EV_A {
    #[doc = "63: No event, always low"]
    NONE = 63,
    #[doc = "56: Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N = 56,
    #[doc = "55: Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC = 55,
    #[doc = "54: BATMON voltage update event"]
    BATMON_VOLT = 54,
    #[doc = "53: BATMON temperature update event"]
    BATMON_TEMP = 53,
    #[doc = "52: AUX Timer 1 Event"]
    AUX_TIMER1_EV = 52,
    #[doc = "51: AUX Timer 0 Event"]
    AUX_TIMER0_EV = 51,
    #[doc = "50: TDC completed or timed out"]
    AUX_TDC_DONE = 50,
    #[doc = "49: ADC conversion completed"]
    AUX_ADC_DONE = 49,
    #[doc = "48: Comparator B triggered"]
    AUX_COMPB = 48,
    #[doc = "47: Comparator A triggered"]
    AUX_COMPA = 47,
    #[doc = "46: AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2 = 46,
    #[doc = "45: AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1 = 45,
    #[doc = "44: AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0 = 44,
    #[doc = "43: JTAG generated event"]
    JTAG = 43,
    #[doc = "42: RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD = 42,
    #[doc = "41: RTC combined delayed event"]
    RTC_COMB_DLY = 41,
    #[doc = "40: RTC channel 2 - delayed event"]
    RTC_CH2_DLY = 40,
    #[doc = "39: RTC channel 1 - delayed event"]
    RTC_CH1_DLY = 39,
    #[doc = "38: RTC channel 0 - delayed event"]
    RTC_CH0_DLY = 38,
    #[doc = "37: RTC channel 2 event"]
    RTC_CH2 = 37,
    #[doc = "36: RTC channel 1 event"]
    RTC_CH1 = 36,
    #[doc = "35: RTC channel 0 event"]
    RTC_CH0 = 35,
    #[doc = "32: Edge detect on any PAD"]
    PAD = 32,
    #[doc = "9: Combined event from BATMON"]
    BATMON_COMBINED = 9,
    #[doc = "8: BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL = 8,
    #[doc = "7: BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL = 7,
    #[doc = "6: BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL = 6,
    #[doc = "5: BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL = 5,
    #[doc = "4: Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3 = 4,
    #[doc = "3: Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2 = 3,
    #[doc = "2: Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1 = 2,
    #[doc = "1: Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU = 0,
}
impl From<WU4_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: WU4_EV_A) -> Self {
        variant as _
    }
}
impl WU4_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WU4_EV_A> {
        match self.bits {
            63 => Some(WU4_EV_A::NONE),
            56 => Some(WU4_EV_A::AUX_COMPB_ASYNC_N),
            55 => Some(WU4_EV_A::AUX_COMPB_ASYNC),
            54 => Some(WU4_EV_A::BATMON_VOLT),
            53 => Some(WU4_EV_A::BATMON_TEMP),
            52 => Some(WU4_EV_A::AUX_TIMER1_EV),
            51 => Some(WU4_EV_A::AUX_TIMER0_EV),
            50 => Some(WU4_EV_A::AUX_TDC_DONE),
            49 => Some(WU4_EV_A::AUX_ADC_DONE),
            48 => Some(WU4_EV_A::AUX_COMPB),
            47 => Some(WU4_EV_A::AUX_COMPA),
            46 => Some(WU4_EV_A::AUX_SWEV2),
            45 => Some(WU4_EV_A::AUX_SWEV1),
            44 => Some(WU4_EV_A::AUX_SWEV0),
            43 => Some(WU4_EV_A::JTAG),
            42 => Some(WU4_EV_A::RTC_UPD),
            41 => Some(WU4_EV_A::RTC_COMB_DLY),
            40 => Some(WU4_EV_A::RTC_CH2_DLY),
            39 => Some(WU4_EV_A::RTC_CH1_DLY),
            38 => Some(WU4_EV_A::RTC_CH0_DLY),
            37 => Some(WU4_EV_A::RTC_CH2),
            36 => Some(WU4_EV_A::RTC_CH1),
            35 => Some(WU4_EV_A::RTC_CH0),
            32 => Some(WU4_EV_A::PAD),
            9 => Some(WU4_EV_A::BATMON_COMBINED),
            8 => Some(WU4_EV_A::BATMON_TEMP_LL),
            7 => Some(WU4_EV_A::BATMON_TEMP_UL),
            6 => Some(WU4_EV_A::BATMON_BATT_LL),
            5 => Some(WU4_EV_A::BATMON_BATT_UL),
            4 => Some(WU4_EV_A::AUX_TIMER2_EV3),
            3 => Some(WU4_EV_A::AUX_TIMER2_EV2),
            2 => Some(WU4_EV_A::AUX_TIMER2_EV1),
            1 => Some(WU4_EV_A::AUX_TIMER2_EV0),
            0 => Some(WU4_EV_A::IOEV_MCU_WU),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WU4_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU4_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU4_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU4_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU4_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU4_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU4_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU4_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU4_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU4_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU4_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU4_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU4_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU4_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == WU4_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU4_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU4_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU4_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU4_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU4_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU4_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU4_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU4_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == WU4_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU4_EV_A::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU4_EV_A::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU4_EV_A::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU4_EV_A::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU4_EV_A::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU4_EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU4_EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU4_EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU4_EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU4_EV_A::IOEV_MCU_WU
    }
}
#[doc = "Field `WU4_EV` writer - 5:0\\]
MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU4_EV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCUWUSEL1_SPEC, u8, WU4_EV_A, 6, O>;
impl<'a, const O: u8> WU4_EV_W<'a, O> {
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WU4_EV_A::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU4_EV_A::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU4_EV_A::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU4_EV_A::IOEV_MCU_WU)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUWUSEL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `WU5_EV` reader - 13:8\\]
MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU5_EV_R = crate::FieldReader<u8, WU5_EV_A>;
#[doc = "13:8\\]
MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WU5_EV_A {
    #[doc = "63: No event, always low"]
    NONE = 63,
    #[doc = "56: Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N = 56,
    #[doc = "55: Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC = 55,
    #[doc = "54: BATMON voltage update event"]
    BATMON_VOLT = 54,
    #[doc = "53: BATMON temperature update event"]
    BATMON_TEMP = 53,
    #[doc = "52: AUX Timer 1 Event"]
    AUX_TIMER1_EV = 52,
    #[doc = "51: AUX Timer 0 Event"]
    AUX_TIMER0_EV = 51,
    #[doc = "50: TDC completed or timed out"]
    AUX_TDC_DONE = 50,
    #[doc = "49: ADC conversion completed"]
    AUX_ADC_DONE = 49,
    #[doc = "48: Comparator B triggered"]
    AUX_COMPB = 48,
    #[doc = "47: Comparator A triggered"]
    AUX_COMPA = 47,
    #[doc = "46: AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2 = 46,
    #[doc = "45: AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1 = 45,
    #[doc = "44: AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0 = 44,
    #[doc = "43: JTAG generated event"]
    JTAG = 43,
    #[doc = "42: RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD = 42,
    #[doc = "41: RTC combined delayed event"]
    RTC_COMB_DLY = 41,
    #[doc = "40: RTC channel 2 - delayed event"]
    RTC_CH2_DLY = 40,
    #[doc = "39: RTC channel 1 - delayed event"]
    RTC_CH1_DLY = 39,
    #[doc = "38: RTC channel 0 - delayed event"]
    RTC_CH0_DLY = 38,
    #[doc = "37: RTC channel 2 event"]
    RTC_CH2 = 37,
    #[doc = "36: RTC channel 1 event"]
    RTC_CH1 = 36,
    #[doc = "35: RTC channel 0 event"]
    RTC_CH0 = 35,
    #[doc = "32: Edge detect on any PAD"]
    PAD = 32,
    #[doc = "9: Combined event from BATMON"]
    BATMON_COMBINED = 9,
    #[doc = "8: BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL = 8,
    #[doc = "7: BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL = 7,
    #[doc = "6: BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL = 6,
    #[doc = "5: BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL = 5,
    #[doc = "4: Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3 = 4,
    #[doc = "3: Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2 = 3,
    #[doc = "2: Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1 = 2,
    #[doc = "1: Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU = 0,
}
impl From<WU5_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: WU5_EV_A) -> Self {
        variant as _
    }
}
impl WU5_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WU5_EV_A> {
        match self.bits {
            63 => Some(WU5_EV_A::NONE),
            56 => Some(WU5_EV_A::AUX_COMPB_ASYNC_N),
            55 => Some(WU5_EV_A::AUX_COMPB_ASYNC),
            54 => Some(WU5_EV_A::BATMON_VOLT),
            53 => Some(WU5_EV_A::BATMON_TEMP),
            52 => Some(WU5_EV_A::AUX_TIMER1_EV),
            51 => Some(WU5_EV_A::AUX_TIMER0_EV),
            50 => Some(WU5_EV_A::AUX_TDC_DONE),
            49 => Some(WU5_EV_A::AUX_ADC_DONE),
            48 => Some(WU5_EV_A::AUX_COMPB),
            47 => Some(WU5_EV_A::AUX_COMPA),
            46 => Some(WU5_EV_A::AUX_SWEV2),
            45 => Some(WU5_EV_A::AUX_SWEV1),
            44 => Some(WU5_EV_A::AUX_SWEV0),
            43 => Some(WU5_EV_A::JTAG),
            42 => Some(WU5_EV_A::RTC_UPD),
            41 => Some(WU5_EV_A::RTC_COMB_DLY),
            40 => Some(WU5_EV_A::RTC_CH2_DLY),
            39 => Some(WU5_EV_A::RTC_CH1_DLY),
            38 => Some(WU5_EV_A::RTC_CH0_DLY),
            37 => Some(WU5_EV_A::RTC_CH2),
            36 => Some(WU5_EV_A::RTC_CH1),
            35 => Some(WU5_EV_A::RTC_CH0),
            32 => Some(WU5_EV_A::PAD),
            9 => Some(WU5_EV_A::BATMON_COMBINED),
            8 => Some(WU5_EV_A::BATMON_TEMP_LL),
            7 => Some(WU5_EV_A::BATMON_TEMP_UL),
            6 => Some(WU5_EV_A::BATMON_BATT_LL),
            5 => Some(WU5_EV_A::BATMON_BATT_UL),
            4 => Some(WU5_EV_A::AUX_TIMER2_EV3),
            3 => Some(WU5_EV_A::AUX_TIMER2_EV2),
            2 => Some(WU5_EV_A::AUX_TIMER2_EV1),
            1 => Some(WU5_EV_A::AUX_TIMER2_EV0),
            0 => Some(WU5_EV_A::IOEV_MCU_WU),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WU5_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU5_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU5_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU5_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU5_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU5_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU5_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU5_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU5_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU5_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU5_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU5_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU5_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU5_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == WU5_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU5_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU5_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU5_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU5_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU5_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU5_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU5_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU5_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == WU5_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU5_EV_A::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU5_EV_A::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU5_EV_A::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU5_EV_A::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU5_EV_A::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU5_EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU5_EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU5_EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU5_EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU5_EV_A::IOEV_MCU_WU
    }
}
#[doc = "Field `WU5_EV` writer - 13:8\\]
MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU5_EV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCUWUSEL1_SPEC, u8, WU5_EV_A, 6, O>;
impl<'a, const O: u8> WU5_EV_W<'a, O> {
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WU5_EV_A::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU5_EV_A::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU5_EV_A::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU5_EV_A::IOEV_MCU_WU)
    }
}
#[doc = "Field `RESERVED14` reader - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED14` writer - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUWUSEL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `WU6_EV` reader - 21:16\\]
MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU6_EV_R = crate::FieldReader<u8, WU6_EV_A>;
#[doc = "21:16\\]
MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WU6_EV_A {
    #[doc = "63: No event, always low"]
    NONE = 63,
    #[doc = "56: Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N = 56,
    #[doc = "55: Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC = 55,
    #[doc = "54: BATMON voltage update event"]
    BATMON_VOLT = 54,
    #[doc = "53: BATMON temperature update event"]
    BATMON_TEMP = 53,
    #[doc = "52: AUX Timer 1 Event"]
    AUX_TIMER1_EV = 52,
    #[doc = "51: AUX Timer 0 Event"]
    AUX_TIMER0_EV = 51,
    #[doc = "50: TDC completed or timed out"]
    AUX_TDC_DONE = 50,
    #[doc = "49: ADC conversion completed"]
    AUX_ADC_DONE = 49,
    #[doc = "48: Comparator B triggered"]
    AUX_COMPB = 48,
    #[doc = "47: Comparator A triggered"]
    AUX_COMPA = 47,
    #[doc = "46: AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2 = 46,
    #[doc = "45: AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1 = 45,
    #[doc = "44: AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0 = 44,
    #[doc = "43: JTAG generated event"]
    JTAG = 43,
    #[doc = "42: RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD = 42,
    #[doc = "41: RTC combined delayed event"]
    RTC_COMB_DLY = 41,
    #[doc = "40: RTC channel 2 - delayed event"]
    RTC_CH2_DLY = 40,
    #[doc = "39: RTC channel 1 - delayed event"]
    RTC_CH1_DLY = 39,
    #[doc = "38: RTC channel 0 - delayed event"]
    RTC_CH0_DLY = 38,
    #[doc = "37: RTC channel 2 event"]
    RTC_CH2 = 37,
    #[doc = "36: RTC channel 1 event"]
    RTC_CH1 = 36,
    #[doc = "35: RTC channel 0 event"]
    RTC_CH0 = 35,
    #[doc = "32: Edge detect on any PAD"]
    PAD = 32,
    #[doc = "9: Combined event from BATMON"]
    BATMON_COMBINED = 9,
    #[doc = "8: BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL = 8,
    #[doc = "7: BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL = 7,
    #[doc = "6: BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL = 6,
    #[doc = "5: BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL = 5,
    #[doc = "4: Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3 = 4,
    #[doc = "3: Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2 = 3,
    #[doc = "2: Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1 = 2,
    #[doc = "1: Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU = 0,
}
impl From<WU6_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: WU6_EV_A) -> Self {
        variant as _
    }
}
impl WU6_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WU6_EV_A> {
        match self.bits {
            63 => Some(WU6_EV_A::NONE),
            56 => Some(WU6_EV_A::AUX_COMPB_ASYNC_N),
            55 => Some(WU6_EV_A::AUX_COMPB_ASYNC),
            54 => Some(WU6_EV_A::BATMON_VOLT),
            53 => Some(WU6_EV_A::BATMON_TEMP),
            52 => Some(WU6_EV_A::AUX_TIMER1_EV),
            51 => Some(WU6_EV_A::AUX_TIMER0_EV),
            50 => Some(WU6_EV_A::AUX_TDC_DONE),
            49 => Some(WU6_EV_A::AUX_ADC_DONE),
            48 => Some(WU6_EV_A::AUX_COMPB),
            47 => Some(WU6_EV_A::AUX_COMPA),
            46 => Some(WU6_EV_A::AUX_SWEV2),
            45 => Some(WU6_EV_A::AUX_SWEV1),
            44 => Some(WU6_EV_A::AUX_SWEV0),
            43 => Some(WU6_EV_A::JTAG),
            42 => Some(WU6_EV_A::RTC_UPD),
            41 => Some(WU6_EV_A::RTC_COMB_DLY),
            40 => Some(WU6_EV_A::RTC_CH2_DLY),
            39 => Some(WU6_EV_A::RTC_CH1_DLY),
            38 => Some(WU6_EV_A::RTC_CH0_DLY),
            37 => Some(WU6_EV_A::RTC_CH2),
            36 => Some(WU6_EV_A::RTC_CH1),
            35 => Some(WU6_EV_A::RTC_CH0),
            32 => Some(WU6_EV_A::PAD),
            9 => Some(WU6_EV_A::BATMON_COMBINED),
            8 => Some(WU6_EV_A::BATMON_TEMP_LL),
            7 => Some(WU6_EV_A::BATMON_TEMP_UL),
            6 => Some(WU6_EV_A::BATMON_BATT_LL),
            5 => Some(WU6_EV_A::BATMON_BATT_UL),
            4 => Some(WU6_EV_A::AUX_TIMER2_EV3),
            3 => Some(WU6_EV_A::AUX_TIMER2_EV2),
            2 => Some(WU6_EV_A::AUX_TIMER2_EV1),
            1 => Some(WU6_EV_A::AUX_TIMER2_EV0),
            0 => Some(WU6_EV_A::IOEV_MCU_WU),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WU6_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU6_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU6_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU6_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU6_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU6_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU6_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU6_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU6_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU6_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU6_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU6_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU6_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU6_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == WU6_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU6_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU6_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU6_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU6_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU6_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU6_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU6_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU6_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == WU6_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU6_EV_A::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU6_EV_A::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU6_EV_A::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU6_EV_A::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU6_EV_A::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU6_EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU6_EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU6_EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU6_EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU6_EV_A::IOEV_MCU_WU
    }
}
#[doc = "Field `WU6_EV` writer - 21:16\\]
MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU6_EV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCUWUSEL1_SPEC, u8, WU6_EV_A, 6, O>;
impl<'a, const O: u8> WU6_EV_W<'a, O> {
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WU6_EV_A::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU6_EV_A::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU6_EV_A::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU6_EV_A::IOEV_MCU_WU)
    }
}
#[doc = "Field `RESERVED22` reader - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED22` writer - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUWUSEL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `WU7_EV` reader - 29:24\\]
MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU7_EV_R = crate::FieldReader<u8, WU7_EV_A>;
#[doc = "29:24\\]
MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WU7_EV_A {
    #[doc = "63: No event, always low"]
    NONE = 63,
    #[doc = "56: Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N = 56,
    #[doc = "55: Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC = 55,
    #[doc = "54: BATMON voltage update event"]
    BATMON_VOLT = 54,
    #[doc = "53: BATMON temperature update event"]
    BATMON_TEMP = 53,
    #[doc = "52: AUX Timer 1 Event"]
    AUX_TIMER1_EV = 52,
    #[doc = "51: AUX Timer 0 Event"]
    AUX_TIMER0_EV = 51,
    #[doc = "50: TDC completed or timed out"]
    AUX_TDC_DONE = 50,
    #[doc = "49: ADC conversion completed"]
    AUX_ADC_DONE = 49,
    #[doc = "48: Comparator B triggered"]
    AUX_COMPB = 48,
    #[doc = "47: Comparator A triggered"]
    AUX_COMPA = 47,
    #[doc = "46: AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2 = 46,
    #[doc = "45: AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1 = 45,
    #[doc = "44: AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0 = 44,
    #[doc = "43: JTAG generated event"]
    JTAG = 43,
    #[doc = "42: RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD = 42,
    #[doc = "41: RTC combined delayed event"]
    RTC_COMB_DLY = 41,
    #[doc = "40: RTC channel 2 - delayed event"]
    RTC_CH2_DLY = 40,
    #[doc = "39: RTC channel 1 - delayed event"]
    RTC_CH1_DLY = 39,
    #[doc = "38: RTC channel 0 - delayed event"]
    RTC_CH0_DLY = 38,
    #[doc = "37: RTC channel 2 event"]
    RTC_CH2 = 37,
    #[doc = "36: RTC channel 1 event"]
    RTC_CH1 = 36,
    #[doc = "35: RTC channel 0 event"]
    RTC_CH0 = 35,
    #[doc = "32: Edge detect on any PAD"]
    PAD = 32,
    #[doc = "9: Combined event from BATMON"]
    BATMON_COMBINED = 9,
    #[doc = "8: BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL = 8,
    #[doc = "7: BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL = 7,
    #[doc = "6: BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL = 6,
    #[doc = "5: BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL = 5,
    #[doc = "4: Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3 = 4,
    #[doc = "3: Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2 = 3,
    #[doc = "2: Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1 = 2,
    #[doc = "1: Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU = 0,
}
impl From<WU7_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: WU7_EV_A) -> Self {
        variant as _
    }
}
impl WU7_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WU7_EV_A> {
        match self.bits {
            63 => Some(WU7_EV_A::NONE),
            56 => Some(WU7_EV_A::AUX_COMPB_ASYNC_N),
            55 => Some(WU7_EV_A::AUX_COMPB_ASYNC),
            54 => Some(WU7_EV_A::BATMON_VOLT),
            53 => Some(WU7_EV_A::BATMON_TEMP),
            52 => Some(WU7_EV_A::AUX_TIMER1_EV),
            51 => Some(WU7_EV_A::AUX_TIMER0_EV),
            50 => Some(WU7_EV_A::AUX_TDC_DONE),
            49 => Some(WU7_EV_A::AUX_ADC_DONE),
            48 => Some(WU7_EV_A::AUX_COMPB),
            47 => Some(WU7_EV_A::AUX_COMPA),
            46 => Some(WU7_EV_A::AUX_SWEV2),
            45 => Some(WU7_EV_A::AUX_SWEV1),
            44 => Some(WU7_EV_A::AUX_SWEV0),
            43 => Some(WU7_EV_A::JTAG),
            42 => Some(WU7_EV_A::RTC_UPD),
            41 => Some(WU7_EV_A::RTC_COMB_DLY),
            40 => Some(WU7_EV_A::RTC_CH2_DLY),
            39 => Some(WU7_EV_A::RTC_CH1_DLY),
            38 => Some(WU7_EV_A::RTC_CH0_DLY),
            37 => Some(WU7_EV_A::RTC_CH2),
            36 => Some(WU7_EV_A::RTC_CH1),
            35 => Some(WU7_EV_A::RTC_CH0),
            32 => Some(WU7_EV_A::PAD),
            9 => Some(WU7_EV_A::BATMON_COMBINED),
            8 => Some(WU7_EV_A::BATMON_TEMP_LL),
            7 => Some(WU7_EV_A::BATMON_TEMP_UL),
            6 => Some(WU7_EV_A::BATMON_BATT_LL),
            5 => Some(WU7_EV_A::BATMON_BATT_UL),
            4 => Some(WU7_EV_A::AUX_TIMER2_EV3),
            3 => Some(WU7_EV_A::AUX_TIMER2_EV2),
            2 => Some(WU7_EV_A::AUX_TIMER2_EV1),
            1 => Some(WU7_EV_A::AUX_TIMER2_EV0),
            0 => Some(WU7_EV_A::IOEV_MCU_WU),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WU7_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU7_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU7_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU7_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU7_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU7_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU7_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU7_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU7_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU7_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU7_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU7_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU7_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU7_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == WU7_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU7_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU7_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU7_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU7_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU7_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU7_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU7_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU7_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == WU7_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU7_EV_A::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU7_EV_A::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU7_EV_A::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU7_EV_A::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU7_EV_A::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU7_EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU7_EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU7_EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU7_EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU7_EV_A::IOEV_MCU_WU
    }
}
#[doc = "Field `WU7_EV` writer - 29:24\\]
MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU7_EV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCUWUSEL1_SPEC, u8, WU7_EV_A, 6, O>;
impl<'a, const O: u8> WU7_EV_W<'a, O> {
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WU7_EV_A::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU7_EV_A::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU7_EV_A::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU7_EV_A::IOEV_MCU_WU)
    }
}
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUWUSEL1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu4_ev(&self) -> WU4_EV_R {
        WU4_EV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu5_ev(&self) -> WU5_EV_R {
        WU5_EV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu6_ev(&self) -> WU6_EV_R {
        WU6_EV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> RESERVED22_R {
        RESERVED22_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu7_ev(&self) -> WU7_EV_R {
        WU7_EV_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu4_ev(&mut self) -> WU4_EV_W<0> {
        WU4_EV_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bits 8:13 - 13:8\\]
MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu5_ev(&mut self) -> WU5_EV_W<8> {
        WU5_EV_W::new(self)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> RESERVED14_W<14> {
        RESERVED14_W::new(self)
    }
    #[doc = "Bits 16:21 - 21:16\\]
MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu6_ev(&mut self) -> WU6_EV_W<16> {
        WU6_EV_W::new(self)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> RESERVED22_W<22> {
        RESERVED22_W::new(self)
    }
    #[doc = "Bits 24:29 - 29:24\\]
MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu7_ev(&mut self) -> WU7_EV_W<24> {
        WU7_EV_W::new(self)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved30(&mut self) -> RESERVED30_W<30> {
        RESERVED30_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 4 to 7) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcuwusel1](index.html) module"]
pub struct MCUWUSEL1_SPEC;
impl crate::RegisterSpec for MCUWUSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcuwusel1::R](R) reader structure"]
impl crate::Readable for MCUWUSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcuwusel1::W](W) writer structure"]
impl crate::Writable for MCUWUSEL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUWUSEL1 to value 0x3f3f_3f3f"]
impl crate::Resettable for MCUWUSEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f3f_3f3f;
}

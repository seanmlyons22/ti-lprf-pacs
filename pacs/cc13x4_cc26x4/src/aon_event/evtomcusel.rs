#[doc = "Register `EVTOMCUSEL` reader"]
pub struct R(crate::R<EVTOMCUSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTOMCUSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTOMCUSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTOMCUSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVTOMCUSEL` writer"]
pub struct W(crate::W<EVTOMCUSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVTOMCUSEL_SPEC>;
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
impl From<crate::W<EVTOMCUSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVTOMCUSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AON_PROG0_EV` reader - 5:0\\]
Event selector for AON_PROG0 event. AON Event Source id# selecting event routed to EVENT as AON_PROG0 event."]
pub type AON_PROG0_EV_R = crate::FieldReader<u8, AON_PROG0_EV_A>;
#[doc = "5:0\\]
Event selector for AON_PROG0 event. AON Event Source id# selecting event routed to EVENT as AON_PROG0 event.\n\nValue on reset: 43"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AON_PROG0_EV_A {
    #[doc = "63: 0"]
    NONE = 63,
    #[doc = "56: 0"]
    AUX_COMPB_ASYNC_N = 56,
    #[doc = "55: 0"]
    AUX_COMPB_ASYNC = 55,
    #[doc = "54: 0"]
    BATMON_VOLT = 54,
    #[doc = "53: 0"]
    BATMON_TEMP = 53,
    #[doc = "52: 0"]
    AUX_TIMER1_EV = 52,
    #[doc = "51: 0"]
    AUX_TIMER0_EV = 51,
    #[doc = "50: 0"]
    AUX_TDC_DONE = 50,
    #[doc = "49: 0"]
    AUX_ADC_DONE = 49,
    #[doc = "48: 0"]
    AUX_COMPB = 48,
    #[doc = "47: 0"]
    AUX_COMPA = 47,
    #[doc = "46: 0"]
    AUX_SWEV2 = 46,
    #[doc = "45: 0"]
    AUX_SWEV1 = 45,
    #[doc = "44: 0"]
    AUX_SWEV0 = 44,
    #[doc = "43: 0"]
    JTAG = 43,
    #[doc = "42: 0"]
    RTC_UPD = 42,
    #[doc = "41: 0"]
    RTC_COMB_DLY = 41,
    #[doc = "40: 0"]
    RTC_CH2_DLY = 40,
    #[doc = "39: 0"]
    RTC_CH1_DLY = 39,
    #[doc = "38: 0"]
    RTC_CH0_DLY = 38,
    #[doc = "37: 0"]
    RTC_CH2 = 37,
    #[doc = "36: 0"]
    RTC_CH1 = 36,
    #[doc = "35: 0"]
    RTC_CH0 = 35,
    #[doc = "32: 0"]
    PAD = 32,
    #[doc = "9: 0"]
    BATMON_COMBINED = 9,
    #[doc = "8: 0"]
    BATMON_TEMP_LL = 8,
    #[doc = "7: 0"]
    BATMON_TEMP_UL = 7,
    #[doc = "6: 0"]
    BATMON_BATT_LL = 6,
    #[doc = "5: 0"]
    BATMON_BATT_UL = 5,
    #[doc = "4: 0"]
    AUX_TIMER2_EV3 = 4,
    #[doc = "3: 0"]
    AUX_TIMER2_EV2 = 3,
    #[doc = "2: 0"]
    AUX_TIMER2_EV1 = 2,
    #[doc = "1: 0"]
    AUX_TIMER2_EV0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG0 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG0_EN\\]"]
    IOEV_AON_PROG0 = 0,
}
impl From<AON_PROG0_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: AON_PROG0_EV_A) -> Self {
        variant as _
    }
}
impl AON_PROG0_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AON_PROG0_EV_A> {
        match self.bits {
            63 => Some(AON_PROG0_EV_A::NONE),
            56 => Some(AON_PROG0_EV_A::AUX_COMPB_ASYNC_N),
            55 => Some(AON_PROG0_EV_A::AUX_COMPB_ASYNC),
            54 => Some(AON_PROG0_EV_A::BATMON_VOLT),
            53 => Some(AON_PROG0_EV_A::BATMON_TEMP),
            52 => Some(AON_PROG0_EV_A::AUX_TIMER1_EV),
            51 => Some(AON_PROG0_EV_A::AUX_TIMER0_EV),
            50 => Some(AON_PROG0_EV_A::AUX_TDC_DONE),
            49 => Some(AON_PROG0_EV_A::AUX_ADC_DONE),
            48 => Some(AON_PROG0_EV_A::AUX_COMPB),
            47 => Some(AON_PROG0_EV_A::AUX_COMPA),
            46 => Some(AON_PROG0_EV_A::AUX_SWEV2),
            45 => Some(AON_PROG0_EV_A::AUX_SWEV1),
            44 => Some(AON_PROG0_EV_A::AUX_SWEV0),
            43 => Some(AON_PROG0_EV_A::JTAG),
            42 => Some(AON_PROG0_EV_A::RTC_UPD),
            41 => Some(AON_PROG0_EV_A::RTC_COMB_DLY),
            40 => Some(AON_PROG0_EV_A::RTC_CH2_DLY),
            39 => Some(AON_PROG0_EV_A::RTC_CH1_DLY),
            38 => Some(AON_PROG0_EV_A::RTC_CH0_DLY),
            37 => Some(AON_PROG0_EV_A::RTC_CH2),
            36 => Some(AON_PROG0_EV_A::RTC_CH1),
            35 => Some(AON_PROG0_EV_A::RTC_CH0),
            32 => Some(AON_PROG0_EV_A::PAD),
            9 => Some(AON_PROG0_EV_A::BATMON_COMBINED),
            8 => Some(AON_PROG0_EV_A::BATMON_TEMP_LL),
            7 => Some(AON_PROG0_EV_A::BATMON_TEMP_UL),
            6 => Some(AON_PROG0_EV_A::BATMON_BATT_LL),
            5 => Some(AON_PROG0_EV_A::BATMON_BATT_UL),
            4 => Some(AON_PROG0_EV_A::AUX_TIMER2_EV3),
            3 => Some(AON_PROG0_EV_A::AUX_TIMER2_EV2),
            2 => Some(AON_PROG0_EV_A::AUX_TIMER2_EV1),
            1 => Some(AON_PROG0_EV_A::AUX_TIMER2_EV0),
            0 => Some(AON_PROG0_EV_A::IOEV_AON_PROG0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AON_PROG0_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == AON_PROG0_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == AON_PROG0_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == AON_PROG0_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == AON_PROG0_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == AON_PROG0_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == AON_PROG0_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == AON_PROG0_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == AON_PROG0_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == AON_PROG0_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == AON_PROG0_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == AON_PROG0_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == AON_PROG0_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == AON_PROG0_EV_A::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == AON_PROG0_EV_A::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == AON_PROG0_EV_A::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == AON_PROG0_EV_A::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == AON_PROG0_EV_A::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == AON_PROG0_EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_AON_PROG0`"]
    #[inline(always)]
    pub fn is_ioev_aon_prog0(&self) -> bool {
        *self == AON_PROG0_EV_A::IOEV_AON_PROG0
    }
}
#[doc = "Field `AON_PROG0_EV` writer - 5:0\\]
Event selector for AON_PROG0 event. AON Event Source id# selecting event routed to EVENT as AON_PROG0 event."]
pub type AON_PROG0_EV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVTOMCUSEL_SPEC, u8, AON_PROG0_EV_A, 6, O>;
impl<'a, const O: u8> AON_PROG0_EV_W<'a, O> {
    #[doc = "0"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::NONE)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::BATMON_VOLT)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::BATMON_TEMP)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_TDC_DONE)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_ADC_DONE)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_COMPB)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_COMPA)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_SWEV2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_SWEV1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_SWEV0)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::JTAG)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::RTC_UPD)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::RTC_COMB_DLY)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::RTC_CH2_DLY)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::RTC_CH1_DLY)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::RTC_CH0_DLY)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::RTC_CH2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::RTC_CH1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::RTC_CH0)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::PAD)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::BATMON_COMBINED)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::BATMON_TEMP_LL)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::BATMON_TEMP_UL)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::BATMON_BATT_LL)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::BATMON_BATT_UL)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG0 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG0_EN\\]"]
    #[inline(always)]
    pub fn ioev_aon_prog0(self) -> &'a mut W {
        self.variant(AON_PROG0_EV_A::IOEV_AON_PROG0)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVTOMCUSEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `AON_PROG1_EV` reader - 13:8\\]
Event selector for AON_PROG1 event. AON Event Source id# selecting event routed to EVENT as AON_PROG1 event."]
pub type AON_PROG1_EV_R = crate::FieldReader<u8, AON_PROG1_EV_A>;
#[doc = "13:8\\]
Event selector for AON_PROG1 event. AON Event Source id# selecting event routed to EVENT as AON_PROG1 event.\n\nValue on reset: 43"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AON_PROG1_EV_A {
    #[doc = "63: 0"]
    NONE = 63,
    #[doc = "56: 0"]
    AUX_COMPB_ASYNC_N = 56,
    #[doc = "55: 0"]
    AUX_COMPB_ASYNC = 55,
    #[doc = "54: 0"]
    BATMON_VOLT = 54,
    #[doc = "53: 0"]
    BATMON_TEMP = 53,
    #[doc = "52: 0"]
    AUX_TIMER1_EV = 52,
    #[doc = "51: 0"]
    AUX_TIMER0_EV = 51,
    #[doc = "50: 0"]
    AUX_TDC_DONE = 50,
    #[doc = "49: 0"]
    AUX_ADC_DONE = 49,
    #[doc = "48: 0"]
    AUX_COMPB = 48,
    #[doc = "47: 0"]
    AUX_COMPA = 47,
    #[doc = "46: 0"]
    AUX_SWEV2 = 46,
    #[doc = "45: 0"]
    AUX_SWEV1 = 45,
    #[doc = "44: 0"]
    AUX_SWEV0 = 44,
    #[doc = "43: 0"]
    JTAG = 43,
    #[doc = "42: 0"]
    RTC_UPD = 42,
    #[doc = "41: 0"]
    RTC_COMB_DLY = 41,
    #[doc = "40: 0"]
    RTC_CH2_DLY = 40,
    #[doc = "39: 0"]
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
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG1 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG1_EN\\]"]
    IOEV_AON_PROG1 = 0,
}
impl From<AON_PROG1_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: AON_PROG1_EV_A) -> Self {
        variant as _
    }
}
impl AON_PROG1_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AON_PROG1_EV_A> {
        match self.bits {
            63 => Some(AON_PROG1_EV_A::NONE),
            56 => Some(AON_PROG1_EV_A::AUX_COMPB_ASYNC_N),
            55 => Some(AON_PROG1_EV_A::AUX_COMPB_ASYNC),
            54 => Some(AON_PROG1_EV_A::BATMON_VOLT),
            53 => Some(AON_PROG1_EV_A::BATMON_TEMP),
            52 => Some(AON_PROG1_EV_A::AUX_TIMER1_EV),
            51 => Some(AON_PROG1_EV_A::AUX_TIMER0_EV),
            50 => Some(AON_PROG1_EV_A::AUX_TDC_DONE),
            49 => Some(AON_PROG1_EV_A::AUX_ADC_DONE),
            48 => Some(AON_PROG1_EV_A::AUX_COMPB),
            47 => Some(AON_PROG1_EV_A::AUX_COMPA),
            46 => Some(AON_PROG1_EV_A::AUX_SWEV2),
            45 => Some(AON_PROG1_EV_A::AUX_SWEV1),
            44 => Some(AON_PROG1_EV_A::AUX_SWEV0),
            43 => Some(AON_PROG1_EV_A::JTAG),
            42 => Some(AON_PROG1_EV_A::RTC_UPD),
            41 => Some(AON_PROG1_EV_A::RTC_COMB_DLY),
            40 => Some(AON_PROG1_EV_A::RTC_CH2_DLY),
            39 => Some(AON_PROG1_EV_A::RTC_CH1_DLY),
            38 => Some(AON_PROG1_EV_A::RTC_CH0_DLY),
            37 => Some(AON_PROG1_EV_A::RTC_CH2),
            36 => Some(AON_PROG1_EV_A::RTC_CH1),
            35 => Some(AON_PROG1_EV_A::RTC_CH0),
            32 => Some(AON_PROG1_EV_A::PAD),
            9 => Some(AON_PROG1_EV_A::BATMON_COMBINED),
            8 => Some(AON_PROG1_EV_A::BATMON_TEMP_LL),
            7 => Some(AON_PROG1_EV_A::BATMON_TEMP_UL),
            6 => Some(AON_PROG1_EV_A::BATMON_BATT_LL),
            5 => Some(AON_PROG1_EV_A::BATMON_BATT_UL),
            4 => Some(AON_PROG1_EV_A::AUX_TIMER2_EV3),
            3 => Some(AON_PROG1_EV_A::AUX_TIMER2_EV2),
            2 => Some(AON_PROG1_EV_A::AUX_TIMER2_EV1),
            1 => Some(AON_PROG1_EV_A::AUX_TIMER2_EV0),
            0 => Some(AON_PROG1_EV_A::IOEV_AON_PROG1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AON_PROG1_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == AON_PROG1_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == AON_PROG1_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == AON_PROG1_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == AON_PROG1_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == AON_PROG1_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == AON_PROG1_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == AON_PROG1_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == AON_PROG1_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == AON_PROG1_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == AON_PROG1_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == AON_PROG1_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == AON_PROG1_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == AON_PROG1_EV_A::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == AON_PROG1_EV_A::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == AON_PROG1_EV_A::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == AON_PROG1_EV_A::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == AON_PROG1_EV_A::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == AON_PROG1_EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_AON_PROG1`"]
    #[inline(always)]
    pub fn is_ioev_aon_prog1(&self) -> bool {
        *self == AON_PROG1_EV_A::IOEV_AON_PROG1
    }
}
#[doc = "Field `AON_PROG1_EV` writer - 13:8\\]
Event selector for AON_PROG1 event. AON Event Source id# selecting event routed to EVENT as AON_PROG1 event."]
pub type AON_PROG1_EV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVTOMCUSEL_SPEC, u8, AON_PROG1_EV_A, 6, O>;
impl<'a, const O: u8> AON_PROG1_EV_W<'a, O> {
    #[doc = "0"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::NONE)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::BATMON_VOLT)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::BATMON_TEMP)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_TDC_DONE)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_ADC_DONE)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_COMPB)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_COMPA)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_SWEV2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_SWEV1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_SWEV0)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::JTAG)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::RTC_UPD)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::RTC_COMB_DLY)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::RTC_CH2_DLY)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG1 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG1_EN\\]"]
    #[inline(always)]
    pub fn ioev_aon_prog1(self) -> &'a mut W {
        self.variant(AON_PROG1_EV_A::IOEV_AON_PROG1)
    }
}
#[doc = "Field `RESERVED14` reader - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED14` writer - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVTOMCUSEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `AON_PROG2_EV` reader - 21:16\\]
Event selector for AON_PROG2 event. AON Event Source id# selecting event routed to EVENT as AON_PROG2 event."]
pub type AON_PROG2_EV_R = crate::FieldReader<u8, AON_PROG2_EV_A>;
#[doc = "21:16\\]
Event selector for AON_PROG2 event. AON Event Source id# selecting event routed to EVENT as AON_PROG2 event.\n\nValue on reset: 43"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AON_PROG2_EV_A {
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
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG2 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG2_EN\\]"]
    IOEV_AON_PROG2 = 0,
}
impl From<AON_PROG2_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: AON_PROG2_EV_A) -> Self {
        variant as _
    }
}
impl AON_PROG2_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AON_PROG2_EV_A> {
        match self.bits {
            63 => Some(AON_PROG2_EV_A::NONE),
            56 => Some(AON_PROG2_EV_A::AUX_COMPB_ASYNC_N),
            55 => Some(AON_PROG2_EV_A::AUX_COMPB_ASYNC),
            54 => Some(AON_PROG2_EV_A::BATMON_VOLT),
            53 => Some(AON_PROG2_EV_A::BATMON_TEMP),
            52 => Some(AON_PROG2_EV_A::AUX_TIMER1_EV),
            51 => Some(AON_PROG2_EV_A::AUX_TIMER0_EV),
            50 => Some(AON_PROG2_EV_A::AUX_TDC_DONE),
            49 => Some(AON_PROG2_EV_A::AUX_ADC_DONE),
            48 => Some(AON_PROG2_EV_A::AUX_COMPB),
            47 => Some(AON_PROG2_EV_A::AUX_COMPA),
            46 => Some(AON_PROG2_EV_A::AUX_SWEV2),
            45 => Some(AON_PROG2_EV_A::AUX_SWEV1),
            44 => Some(AON_PROG2_EV_A::AUX_SWEV0),
            43 => Some(AON_PROG2_EV_A::JTAG),
            42 => Some(AON_PROG2_EV_A::RTC_UPD),
            41 => Some(AON_PROG2_EV_A::RTC_COMB_DLY),
            40 => Some(AON_PROG2_EV_A::RTC_CH2_DLY),
            39 => Some(AON_PROG2_EV_A::RTC_CH1_DLY),
            38 => Some(AON_PROG2_EV_A::RTC_CH0_DLY),
            37 => Some(AON_PROG2_EV_A::RTC_CH2),
            36 => Some(AON_PROG2_EV_A::RTC_CH1),
            35 => Some(AON_PROG2_EV_A::RTC_CH0),
            32 => Some(AON_PROG2_EV_A::PAD),
            9 => Some(AON_PROG2_EV_A::BATMON_COMBINED),
            8 => Some(AON_PROG2_EV_A::BATMON_TEMP_LL),
            7 => Some(AON_PROG2_EV_A::BATMON_TEMP_UL),
            6 => Some(AON_PROG2_EV_A::BATMON_BATT_LL),
            5 => Some(AON_PROG2_EV_A::BATMON_BATT_UL),
            4 => Some(AON_PROG2_EV_A::AUX_TIMER2_EV3),
            3 => Some(AON_PROG2_EV_A::AUX_TIMER2_EV2),
            2 => Some(AON_PROG2_EV_A::AUX_TIMER2_EV1),
            1 => Some(AON_PROG2_EV_A::AUX_TIMER2_EV0),
            0 => Some(AON_PROG2_EV_A::IOEV_AON_PROG2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AON_PROG2_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == AON_PROG2_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == AON_PROG2_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == AON_PROG2_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == AON_PROG2_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == AON_PROG2_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == AON_PROG2_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == AON_PROG2_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == AON_PROG2_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == AON_PROG2_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == AON_PROG2_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == AON_PROG2_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == AON_PROG2_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == AON_PROG2_EV_A::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == AON_PROG2_EV_A::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == AON_PROG2_EV_A::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == AON_PROG2_EV_A::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == AON_PROG2_EV_A::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == AON_PROG2_EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_AON_PROG2`"]
    #[inline(always)]
    pub fn is_ioev_aon_prog2(&self) -> bool {
        *self == AON_PROG2_EV_A::IOEV_AON_PROG2
    }
}
#[doc = "Field `AON_PROG2_EV` writer - 21:16\\]
Event selector for AON_PROG2 event. AON Event Source id# selecting event routed to EVENT as AON_PROG2 event."]
pub type AON_PROG2_EV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVTOMCUSEL_SPEC, u8, AON_PROG2_EV_A, 6, O>;
impl<'a, const O: u8> AON_PROG2_EV_W<'a, O> {
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG2 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG2_EN\\]"]
    #[inline(always)]
    pub fn ioev_aon_prog2(self) -> &'a mut W {
        self.variant(AON_PROG2_EV_A::IOEV_AON_PROG2)
    }
}
#[doc = "Field `RESERVED22` reader - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED22_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED22` writer - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED22_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVTOMCUSEL_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Event selector for AON_PROG0 event. AON Event Source id# selecting event routed to EVENT as AON_PROG0 event."]
    #[inline(always)]
    pub fn aon_prog0_ev(&self) -> AON_PROG0_EV_R {
        AON_PROG0_EV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Event selector for AON_PROG1 event. AON Event Source id# selecting event routed to EVENT as AON_PROG1 event."]
    #[inline(always)]
    pub fn aon_prog1_ev(&self) -> AON_PROG1_EV_R {
        AON_PROG1_EV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Event selector for AON_PROG2 event. AON Event Source id# selecting event routed to EVENT as AON_PROG2 event."]
    #[inline(always)]
    pub fn aon_prog2_ev(&self) -> AON_PROG2_EV_R {
        AON_PROG2_EV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> RESERVED22_R {
        RESERVED22_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Event selector for AON_PROG0 event. AON Event Source id# selecting event routed to EVENT as AON_PROG0 event."]
    #[inline(always)]
    #[must_use]
    pub fn aon_prog0_ev(&mut self) -> AON_PROG0_EV_W<0> {
        AON_PROG0_EV_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Event selector for AON_PROG1 event. AON Event Source id# selecting event routed to EVENT as AON_PROG1 event."]
    #[inline(always)]
    #[must_use]
    pub fn aon_prog1_ev(&mut self) -> AON_PROG1_EV_W<8> {
        AON_PROG1_EV_W::new(self)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> RESERVED14_W<14> {
        RESERVED14_W::new(self)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Event selector for AON_PROG2 event. AON Event Source id# selecting event routed to EVENT as AON_PROG2 event."]
    #[inline(always)]
    #[must_use]
    pub fn aon_prog2_ev(&mut self) -> AON_PROG2_EV_W<16> {
        AON_PROG2_EV_W::new(self)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> RESERVED22_W<22> {
        RESERVED22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Selector For MCU Event Fabric This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtomcusel](index.html) module"]
pub struct EVTOMCUSEL_SPEC;
impl crate::RegisterSpec for EVTOMCUSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evtomcusel::R](R) reader structure"]
impl crate::Readable for EVTOMCUSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evtomcusel::W](W) writer structure"]
impl crate::Writable for EVTOMCUSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVTOMCUSEL to value 0x002b_2b2b"]
impl crate::Resettable for EVTOMCUSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x002b_2b2b;
}

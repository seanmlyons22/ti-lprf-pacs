#[doc = "Register `RTCSEL` reader"]
pub struct R(crate::R<RTCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCSEL` writer"]
pub struct W(crate::W<RTCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCSEL_SPEC>;
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
impl From<crate::W<RTCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CH1_CAPT_EV` reader - 5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
pub type RTC_CH1_CAPT_EV_R = crate::FieldReader<u8, RTC_CH1_CAPT_EV_A>;
#[doc = "5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTC_CH1_CAPT_EV_A {
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
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_RTC in \\[MCU_IOC:IOCFGx.IOEV_RTC_EN\\]"]
    IOEV_RTC = 0,
}
impl From<RTC_CH1_CAPT_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_CH1_CAPT_EV_A) -> Self {
        variant as _
    }
}
impl RTC_CH1_CAPT_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTC_CH1_CAPT_EV_A> {
        match self.bits {
            63 => Some(RTC_CH1_CAPT_EV_A::NONE),
            56 => Some(RTC_CH1_CAPT_EV_A::AUX_COMPB_ASYNC_N),
            55 => Some(RTC_CH1_CAPT_EV_A::AUX_COMPB_ASYNC),
            54 => Some(RTC_CH1_CAPT_EV_A::BATMON_VOLT),
            53 => Some(RTC_CH1_CAPT_EV_A::BATMON_TEMP),
            52 => Some(RTC_CH1_CAPT_EV_A::AUX_TIMER1_EV),
            51 => Some(RTC_CH1_CAPT_EV_A::AUX_TIMER0_EV),
            50 => Some(RTC_CH1_CAPT_EV_A::AUX_TDC_DONE),
            49 => Some(RTC_CH1_CAPT_EV_A::AUX_ADC_DONE),
            48 => Some(RTC_CH1_CAPT_EV_A::AUX_COMPB),
            47 => Some(RTC_CH1_CAPT_EV_A::AUX_COMPA),
            46 => Some(RTC_CH1_CAPT_EV_A::AUX_SWEV2),
            45 => Some(RTC_CH1_CAPT_EV_A::AUX_SWEV1),
            44 => Some(RTC_CH1_CAPT_EV_A::AUX_SWEV0),
            43 => Some(RTC_CH1_CAPT_EV_A::JTAG),
            42 => Some(RTC_CH1_CAPT_EV_A::RTC_UPD),
            41 => Some(RTC_CH1_CAPT_EV_A::RTC_COMB_DLY),
            40 => Some(RTC_CH1_CAPT_EV_A::RTC_CH2_DLY),
            39 => Some(RTC_CH1_CAPT_EV_A::RTC_CH1_DLY),
            38 => Some(RTC_CH1_CAPT_EV_A::RTC_CH0_DLY),
            37 => Some(RTC_CH1_CAPT_EV_A::RTC_CH2),
            36 => Some(RTC_CH1_CAPT_EV_A::RTC_CH1),
            35 => Some(RTC_CH1_CAPT_EV_A::RTC_CH0),
            32 => Some(RTC_CH1_CAPT_EV_A::PAD),
            9 => Some(RTC_CH1_CAPT_EV_A::BATMON_COMBINED),
            8 => Some(RTC_CH1_CAPT_EV_A::BATMON_TEMP_LL),
            7 => Some(RTC_CH1_CAPT_EV_A::BATMON_TEMP_UL),
            6 => Some(RTC_CH1_CAPT_EV_A::BATMON_BATT_LL),
            5 => Some(RTC_CH1_CAPT_EV_A::BATMON_BATT_UL),
            4 => Some(RTC_CH1_CAPT_EV_A::AUX_TIMER2_EV3),
            3 => Some(RTC_CH1_CAPT_EV_A::AUX_TIMER2_EV2),
            2 => Some(RTC_CH1_CAPT_EV_A::AUX_TIMER2_EV1),
            1 => Some(RTC_CH1_CAPT_EV_A::AUX_TIMER2_EV0),
            0 => Some(RTC_CH1_CAPT_EV_A::IOEV_RTC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_RTC`"]
    #[inline(always)]
    pub fn is_ioev_rtc(&self) -> bool {
        *self == RTC_CH1_CAPT_EV_A::IOEV_RTC
    }
}
#[doc = "Field `RTC_CH1_CAPT_EV` writer - 5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
pub type RTC_CH1_CAPT_EV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTCSEL_SPEC, u8, RTC_CH1_CAPT_EV_A, 6, O>;
impl<'a, const O: u8> RTC_CH1_CAPT_EV_W<'a, O> {
    #[doc = "0"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::NONE)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::BATMON_VOLT)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::BATMON_TEMP)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_TDC_DONE)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_ADC_DONE)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_COMPB)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_COMPA)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_SWEV2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_SWEV1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_SWEV0)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::JTAG)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::RTC_UPD)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::RTC_COMB_DLY)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::RTC_CH2_DLY)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::RTC_CH1_DLY)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::RTC_CH0_DLY)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::RTC_CH2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::RTC_CH1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::RTC_CH0)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::PAD)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::BATMON_COMBINED)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::BATMON_TEMP_LL)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::BATMON_TEMP_UL)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::BATMON_BATT_LL)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::BATMON_BATT_UL)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_RTC in \\[MCU_IOC:IOCFGx.IOEV_RTC_EN\\]"]
    #[inline(always)]
    pub fn ioev_rtc(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EV_A::IOEV_RTC)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCSEL_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
    #[inline(always)]
    pub fn rtc_ch1_capt_ev(&self) -> RTC_CH1_CAPT_EV_R {
        RTC_CH1_CAPT_EV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ch1_capt_ev(&mut self) -> RTC_CH1_CAPT_EV_W<0> {
        RTC_CH1_CAPT_EV_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsel](index.html) module"]
pub struct RTCSEL_SPEC;
impl crate::RegisterSpec for RTCSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcsel::R](R) reader structure"]
impl crate::Readable for RTCSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcsel::W](W) writer structure"]
impl crate::Writable for RTCSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCSEL to value 0x3f"]
impl crate::Resettable for RTCSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}

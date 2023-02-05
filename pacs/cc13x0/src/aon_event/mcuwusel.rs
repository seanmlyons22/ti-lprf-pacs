#[doc = "Register `MCUWUSEL` reader"]
pub struct R(crate::R<MCUWUSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUWUSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUWUSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUWUSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUWUSEL` writer"]
pub struct W(crate::W<MCUWUSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUWUSEL_SPEC>;
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
impl From<crate::W<MCUWUSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUWUSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WU0_EV` reader - 5:0\\]
MCU Wakeup Source #0 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU0_EV_R = crate::FieldReader<u8, WU0_EV_A>;
#[doc = "5:0\\]
MCU Wakeup Source #0 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WU0_EV_A {
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
    #[doc = "31: Edge detect on PAD31"]
    PAD31 = 31,
    #[doc = "30: Edge detect on PAD30"]
    PAD30 = 30,
    #[doc = "29: Edge detect on PAD29"]
    PAD29 = 29,
    #[doc = "28: Edge detect on PAD28"]
    PAD28 = 28,
    #[doc = "27: Edge detect on PAD27"]
    PAD27 = 27,
    #[doc = "26: Edge detect on PAD26"]
    PAD26 = 26,
    #[doc = "25: Edge detect on PAD25"]
    PAD25 = 25,
    #[doc = "24: Edge detect on PAD24"]
    PAD24 = 24,
    #[doc = "23: Edge detect on PAD23"]
    PAD23 = 23,
    #[doc = "22: Edge detect on PAD22"]
    PAD22 = 22,
    #[doc = "21: Edge detect on PAD21"]
    PAD21 = 21,
    #[doc = "20: Edge detect on PAD20"]
    PAD20 = 20,
    #[doc = "19: Edge detect on PAD19"]
    PAD19 = 19,
    #[doc = "18: Edge detect on PAD18"]
    PAD18 = 18,
    #[doc = "17: Edge detect on PAD17"]
    PAD17 = 17,
    #[doc = "16: Edge detect on PAD16"]
    PAD16 = 16,
    #[doc = "15: Edge detect on PAD15"]
    PAD15 = 15,
    #[doc = "14: Edge detect on PAD14"]
    PAD14 = 14,
    #[doc = "13: Edge detect on PAD13"]
    PAD13 = 13,
    #[doc = "12: Edge detect on PAD12"]
    PAD12 = 12,
    #[doc = "11: Edge detect on PAD11"]
    PAD11 = 11,
    #[doc = "10: Edge detect on PAD10"]
    PAD10 = 10,
    #[doc = "9: Edge detect on PAD9"]
    PAD9 = 9,
    #[doc = "8: Edge detect on PAD8"]
    PAD8 = 8,
    #[doc = "7: Edge detect on PAD7"]
    PAD7 = 7,
    #[doc = "6: Edge detect on PAD6"]
    PAD6 = 6,
    #[doc = "5: Edge detect on PAD5"]
    PAD5 = 5,
    #[doc = "4: Edge detect on PAD4"]
    PAD4 = 4,
    #[doc = "3: Edge detect on PAD3"]
    PAD3 = 3,
    #[doc = "2: Edge detect on PAD2"]
    PAD2 = 2,
    #[doc = "1: Edge detect on PAD1"]
    PAD1 = 1,
    #[doc = "0: Edge detect on PAD0"]
    PAD0 = 0,
}
impl From<WU0_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: WU0_EV_A) -> Self {
        variant as _
    }
}
impl WU0_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WU0_EV_A> {
        match self.bits {
            63 => Some(WU0_EV_A::NONE),
            56 => Some(WU0_EV_A::AUX_COMPB_ASYNC_N),
            55 => Some(WU0_EV_A::AUX_COMPB_ASYNC),
            54 => Some(WU0_EV_A::BATMON_VOLT),
            53 => Some(WU0_EV_A::BATMON_TEMP),
            52 => Some(WU0_EV_A::AUX_TIMER1_EV),
            51 => Some(WU0_EV_A::AUX_TIMER0_EV),
            50 => Some(WU0_EV_A::AUX_TDC_DONE),
            49 => Some(WU0_EV_A::AUX_ADC_DONE),
            48 => Some(WU0_EV_A::AUX_COMPB),
            47 => Some(WU0_EV_A::AUX_COMPA),
            46 => Some(WU0_EV_A::AUX_SWEV2),
            45 => Some(WU0_EV_A::AUX_SWEV1),
            44 => Some(WU0_EV_A::AUX_SWEV0),
            43 => Some(WU0_EV_A::JTAG),
            42 => Some(WU0_EV_A::RTC_UPD),
            41 => Some(WU0_EV_A::RTC_COMB_DLY),
            40 => Some(WU0_EV_A::RTC_CH2_DLY),
            39 => Some(WU0_EV_A::RTC_CH1_DLY),
            38 => Some(WU0_EV_A::RTC_CH0_DLY),
            37 => Some(WU0_EV_A::RTC_CH2),
            36 => Some(WU0_EV_A::RTC_CH1),
            35 => Some(WU0_EV_A::RTC_CH0),
            32 => Some(WU0_EV_A::PAD),
            31 => Some(WU0_EV_A::PAD31),
            30 => Some(WU0_EV_A::PAD30),
            29 => Some(WU0_EV_A::PAD29),
            28 => Some(WU0_EV_A::PAD28),
            27 => Some(WU0_EV_A::PAD27),
            26 => Some(WU0_EV_A::PAD26),
            25 => Some(WU0_EV_A::PAD25),
            24 => Some(WU0_EV_A::PAD24),
            23 => Some(WU0_EV_A::PAD23),
            22 => Some(WU0_EV_A::PAD22),
            21 => Some(WU0_EV_A::PAD21),
            20 => Some(WU0_EV_A::PAD20),
            19 => Some(WU0_EV_A::PAD19),
            18 => Some(WU0_EV_A::PAD18),
            17 => Some(WU0_EV_A::PAD17),
            16 => Some(WU0_EV_A::PAD16),
            15 => Some(WU0_EV_A::PAD15),
            14 => Some(WU0_EV_A::PAD14),
            13 => Some(WU0_EV_A::PAD13),
            12 => Some(WU0_EV_A::PAD12),
            11 => Some(WU0_EV_A::PAD11),
            10 => Some(WU0_EV_A::PAD10),
            9 => Some(WU0_EV_A::PAD9),
            8 => Some(WU0_EV_A::PAD8),
            7 => Some(WU0_EV_A::PAD7),
            6 => Some(WU0_EV_A::PAD6),
            5 => Some(WU0_EV_A::PAD5),
            4 => Some(WU0_EV_A::PAD4),
            3 => Some(WU0_EV_A::PAD3),
            2 => Some(WU0_EV_A::PAD2),
            1 => Some(WU0_EV_A::PAD1),
            0 => Some(WU0_EV_A::PAD0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WU0_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU0_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU0_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU0_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU0_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU0_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU0_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU0_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU0_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU0_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU0_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU0_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU0_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU0_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == WU0_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU0_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU0_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU0_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU0_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU0_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU0_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU0_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU0_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == WU0_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `PAD31`"]
    #[inline(always)]
    pub fn is_pad31(&self) -> bool {
        *self == WU0_EV_A::PAD31
    }
    #[doc = "Checks if the value of the field is `PAD30`"]
    #[inline(always)]
    pub fn is_pad30(&self) -> bool {
        *self == WU0_EV_A::PAD30
    }
    #[doc = "Checks if the value of the field is `PAD29`"]
    #[inline(always)]
    pub fn is_pad29(&self) -> bool {
        *self == WU0_EV_A::PAD29
    }
    #[doc = "Checks if the value of the field is `PAD28`"]
    #[inline(always)]
    pub fn is_pad28(&self) -> bool {
        *self == WU0_EV_A::PAD28
    }
    #[doc = "Checks if the value of the field is `PAD27`"]
    #[inline(always)]
    pub fn is_pad27(&self) -> bool {
        *self == WU0_EV_A::PAD27
    }
    #[doc = "Checks if the value of the field is `PAD26`"]
    #[inline(always)]
    pub fn is_pad26(&self) -> bool {
        *self == WU0_EV_A::PAD26
    }
    #[doc = "Checks if the value of the field is `PAD25`"]
    #[inline(always)]
    pub fn is_pad25(&self) -> bool {
        *self == WU0_EV_A::PAD25
    }
    #[doc = "Checks if the value of the field is `PAD24`"]
    #[inline(always)]
    pub fn is_pad24(&self) -> bool {
        *self == WU0_EV_A::PAD24
    }
    #[doc = "Checks if the value of the field is `PAD23`"]
    #[inline(always)]
    pub fn is_pad23(&self) -> bool {
        *self == WU0_EV_A::PAD23
    }
    #[doc = "Checks if the value of the field is `PAD22`"]
    #[inline(always)]
    pub fn is_pad22(&self) -> bool {
        *self == WU0_EV_A::PAD22
    }
    #[doc = "Checks if the value of the field is `PAD21`"]
    #[inline(always)]
    pub fn is_pad21(&self) -> bool {
        *self == WU0_EV_A::PAD21
    }
    #[doc = "Checks if the value of the field is `PAD20`"]
    #[inline(always)]
    pub fn is_pad20(&self) -> bool {
        *self == WU0_EV_A::PAD20
    }
    #[doc = "Checks if the value of the field is `PAD19`"]
    #[inline(always)]
    pub fn is_pad19(&self) -> bool {
        *self == WU0_EV_A::PAD19
    }
    #[doc = "Checks if the value of the field is `PAD18`"]
    #[inline(always)]
    pub fn is_pad18(&self) -> bool {
        *self == WU0_EV_A::PAD18
    }
    #[doc = "Checks if the value of the field is `PAD17`"]
    #[inline(always)]
    pub fn is_pad17(&self) -> bool {
        *self == WU0_EV_A::PAD17
    }
    #[doc = "Checks if the value of the field is `PAD16`"]
    #[inline(always)]
    pub fn is_pad16(&self) -> bool {
        *self == WU0_EV_A::PAD16
    }
    #[doc = "Checks if the value of the field is `PAD15`"]
    #[inline(always)]
    pub fn is_pad15(&self) -> bool {
        *self == WU0_EV_A::PAD15
    }
    #[doc = "Checks if the value of the field is `PAD14`"]
    #[inline(always)]
    pub fn is_pad14(&self) -> bool {
        *self == WU0_EV_A::PAD14
    }
    #[doc = "Checks if the value of the field is `PAD13`"]
    #[inline(always)]
    pub fn is_pad13(&self) -> bool {
        *self == WU0_EV_A::PAD13
    }
    #[doc = "Checks if the value of the field is `PAD12`"]
    #[inline(always)]
    pub fn is_pad12(&self) -> bool {
        *self == WU0_EV_A::PAD12
    }
    #[doc = "Checks if the value of the field is `PAD11`"]
    #[inline(always)]
    pub fn is_pad11(&self) -> bool {
        *self == WU0_EV_A::PAD11
    }
    #[doc = "Checks if the value of the field is `PAD10`"]
    #[inline(always)]
    pub fn is_pad10(&self) -> bool {
        *self == WU0_EV_A::PAD10
    }
    #[doc = "Checks if the value of the field is `PAD9`"]
    #[inline(always)]
    pub fn is_pad9(&self) -> bool {
        *self == WU0_EV_A::PAD9
    }
    #[doc = "Checks if the value of the field is `PAD8`"]
    #[inline(always)]
    pub fn is_pad8(&self) -> bool {
        *self == WU0_EV_A::PAD8
    }
    #[doc = "Checks if the value of the field is `PAD7`"]
    #[inline(always)]
    pub fn is_pad7(&self) -> bool {
        *self == WU0_EV_A::PAD7
    }
    #[doc = "Checks if the value of the field is `PAD6`"]
    #[inline(always)]
    pub fn is_pad6(&self) -> bool {
        *self == WU0_EV_A::PAD6
    }
    #[doc = "Checks if the value of the field is `PAD5`"]
    #[inline(always)]
    pub fn is_pad5(&self) -> bool {
        *self == WU0_EV_A::PAD5
    }
    #[doc = "Checks if the value of the field is `PAD4`"]
    #[inline(always)]
    pub fn is_pad4(&self) -> bool {
        *self == WU0_EV_A::PAD4
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == WU0_EV_A::PAD3
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == WU0_EV_A::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == WU0_EV_A::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == WU0_EV_A::PAD0
    }
}
#[doc = "Field `WU0_EV` writer - 5:0\\]
MCU Wakeup Source #0 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU0_EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUWUSEL_SPEC, u8, WU0_EV_A, 6, O>;
impl<'a, const O: u8> WU0_EV_W<'a, O> {
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WU0_EV_A::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU0_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU0_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU0_EV_A::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU0_EV_A::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU0_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU0_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU0_EV_A::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU0_EV_A::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU0_EV_A::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU0_EV_A::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU0_EV_A::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU0_EV_A::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU0_EV_A::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU0_EV_A::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU0_EV_A::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU0_EV_A::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU0_EV_A::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU0_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU0_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU0_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU0_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU0_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD)
    }
    #[doc = "Edge detect on PAD31"]
    #[inline(always)]
    pub fn pad31(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD31)
    }
    #[doc = "Edge detect on PAD30"]
    #[inline(always)]
    pub fn pad30(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD30)
    }
    #[doc = "Edge detect on PAD29"]
    #[inline(always)]
    pub fn pad29(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD29)
    }
    #[doc = "Edge detect on PAD28"]
    #[inline(always)]
    pub fn pad28(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD28)
    }
    #[doc = "Edge detect on PAD27"]
    #[inline(always)]
    pub fn pad27(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD27)
    }
    #[doc = "Edge detect on PAD26"]
    #[inline(always)]
    pub fn pad26(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD26)
    }
    #[doc = "Edge detect on PAD25"]
    #[inline(always)]
    pub fn pad25(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD25)
    }
    #[doc = "Edge detect on PAD24"]
    #[inline(always)]
    pub fn pad24(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD24)
    }
    #[doc = "Edge detect on PAD23"]
    #[inline(always)]
    pub fn pad23(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD23)
    }
    #[doc = "Edge detect on PAD22"]
    #[inline(always)]
    pub fn pad22(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD22)
    }
    #[doc = "Edge detect on PAD21"]
    #[inline(always)]
    pub fn pad21(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD21)
    }
    #[doc = "Edge detect on PAD20"]
    #[inline(always)]
    pub fn pad20(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD20)
    }
    #[doc = "Edge detect on PAD19"]
    #[inline(always)]
    pub fn pad19(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD19)
    }
    #[doc = "Edge detect on PAD18"]
    #[inline(always)]
    pub fn pad18(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD18)
    }
    #[doc = "Edge detect on PAD17"]
    #[inline(always)]
    pub fn pad17(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD17)
    }
    #[doc = "Edge detect on PAD16"]
    #[inline(always)]
    pub fn pad16(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD16)
    }
    #[doc = "Edge detect on PAD15"]
    #[inline(always)]
    pub fn pad15(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD15)
    }
    #[doc = "Edge detect on PAD14"]
    #[inline(always)]
    pub fn pad14(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD14)
    }
    #[doc = "Edge detect on PAD13"]
    #[inline(always)]
    pub fn pad13(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD13)
    }
    #[doc = "Edge detect on PAD12"]
    #[inline(always)]
    pub fn pad12(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD12)
    }
    #[doc = "Edge detect on PAD11"]
    #[inline(always)]
    pub fn pad11(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD11)
    }
    #[doc = "Edge detect on PAD10"]
    #[inline(always)]
    pub fn pad10(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD10)
    }
    #[doc = "Edge detect on PAD9"]
    #[inline(always)]
    pub fn pad9(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD9)
    }
    #[doc = "Edge detect on PAD8"]
    #[inline(always)]
    pub fn pad8(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD8)
    }
    #[doc = "Edge detect on PAD7"]
    #[inline(always)]
    pub fn pad7(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD7)
    }
    #[doc = "Edge detect on PAD6"]
    #[inline(always)]
    pub fn pad6(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD6)
    }
    #[doc = "Edge detect on PAD5"]
    #[inline(always)]
    pub fn pad5(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD5)
    }
    #[doc = "Edge detect on PAD4"]
    #[inline(always)]
    pub fn pad4(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD4)
    }
    #[doc = "Edge detect on PAD3"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD3)
    }
    #[doc = "Edge detect on PAD2"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD2)
    }
    #[doc = "Edge detect on PAD1"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD1)
    }
    #[doc = "Edge detect on PAD0"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(WU0_EV_A::PAD0)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUWUSEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `WU1_EV` reader - 13:8\\]
MCU Wakeup Source #1 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU1_EV_R = crate::FieldReader<u8, WU1_EV_A>;
#[doc = "13:8\\]
MCU Wakeup Source #1 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WU1_EV_A {
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
    #[doc = "31: Edge detect on PAD31"]
    PAD31 = 31,
    #[doc = "30: Edge detect on PAD30"]
    PAD30 = 30,
    #[doc = "29: Edge detect on PAD29"]
    PAD29 = 29,
    #[doc = "28: Edge detect on PAD28"]
    PAD28 = 28,
    #[doc = "27: Edge detect on PAD27"]
    PAD27 = 27,
    #[doc = "26: Edge detect on PAD26"]
    PAD26 = 26,
    #[doc = "25: Edge detect on PAD25"]
    PAD25 = 25,
    #[doc = "24: Edge detect on PAD24"]
    PAD24 = 24,
    #[doc = "23: Edge detect on PAD23"]
    PAD23 = 23,
    #[doc = "22: Edge detect on PAD22"]
    PAD22 = 22,
    #[doc = "21: Edge detect on PAD21"]
    PAD21 = 21,
    #[doc = "20: Edge detect on PAD20"]
    PAD20 = 20,
    #[doc = "19: Edge detect on PAD19"]
    PAD19 = 19,
    #[doc = "18: Edge detect on PAD18"]
    PAD18 = 18,
    #[doc = "17: Edge detect on PAD17"]
    PAD17 = 17,
    #[doc = "16: Edge detect on PAD16"]
    PAD16 = 16,
    #[doc = "15: Edge detect on PAD15"]
    PAD15 = 15,
    #[doc = "14: Edge detect on PAD14"]
    PAD14 = 14,
    #[doc = "13: Edge detect on PAD13"]
    PAD13 = 13,
    #[doc = "12: Edge detect on PAD12"]
    PAD12 = 12,
    #[doc = "11: Edge detect on PAD11"]
    PAD11 = 11,
    #[doc = "10: Edge detect on PAD10"]
    PAD10 = 10,
    #[doc = "9: Edge detect on PAD9"]
    PAD9 = 9,
    #[doc = "8: Edge detect on PAD8"]
    PAD8 = 8,
    #[doc = "7: Edge detect on PAD7"]
    PAD7 = 7,
    #[doc = "6: Edge detect on PAD6"]
    PAD6 = 6,
    #[doc = "5: Edge detect on PAD5"]
    PAD5 = 5,
    #[doc = "4: Edge detect on PAD4"]
    PAD4 = 4,
    #[doc = "3: Edge detect on PAD3"]
    PAD3 = 3,
    #[doc = "2: Edge detect on PAD2"]
    PAD2 = 2,
    #[doc = "1: Edge detect on PAD1"]
    PAD1 = 1,
    #[doc = "0: Edge detect on PAD0"]
    PAD0 = 0,
}
impl From<WU1_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: WU1_EV_A) -> Self {
        variant as _
    }
}
impl WU1_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WU1_EV_A> {
        match self.bits {
            63 => Some(WU1_EV_A::NONE),
            56 => Some(WU1_EV_A::AUX_COMPB_ASYNC_N),
            55 => Some(WU1_EV_A::AUX_COMPB_ASYNC),
            54 => Some(WU1_EV_A::BATMON_VOLT),
            53 => Some(WU1_EV_A::BATMON_TEMP),
            52 => Some(WU1_EV_A::AUX_TIMER1_EV),
            51 => Some(WU1_EV_A::AUX_TIMER0_EV),
            50 => Some(WU1_EV_A::AUX_TDC_DONE),
            49 => Some(WU1_EV_A::AUX_ADC_DONE),
            48 => Some(WU1_EV_A::AUX_COMPB),
            47 => Some(WU1_EV_A::AUX_COMPA),
            46 => Some(WU1_EV_A::AUX_SWEV2),
            45 => Some(WU1_EV_A::AUX_SWEV1),
            44 => Some(WU1_EV_A::AUX_SWEV0),
            43 => Some(WU1_EV_A::JTAG),
            42 => Some(WU1_EV_A::RTC_UPD),
            41 => Some(WU1_EV_A::RTC_COMB_DLY),
            40 => Some(WU1_EV_A::RTC_CH2_DLY),
            39 => Some(WU1_EV_A::RTC_CH1_DLY),
            38 => Some(WU1_EV_A::RTC_CH0_DLY),
            37 => Some(WU1_EV_A::RTC_CH2),
            36 => Some(WU1_EV_A::RTC_CH1),
            35 => Some(WU1_EV_A::RTC_CH0),
            32 => Some(WU1_EV_A::PAD),
            31 => Some(WU1_EV_A::PAD31),
            30 => Some(WU1_EV_A::PAD30),
            29 => Some(WU1_EV_A::PAD29),
            28 => Some(WU1_EV_A::PAD28),
            27 => Some(WU1_EV_A::PAD27),
            26 => Some(WU1_EV_A::PAD26),
            25 => Some(WU1_EV_A::PAD25),
            24 => Some(WU1_EV_A::PAD24),
            23 => Some(WU1_EV_A::PAD23),
            22 => Some(WU1_EV_A::PAD22),
            21 => Some(WU1_EV_A::PAD21),
            20 => Some(WU1_EV_A::PAD20),
            19 => Some(WU1_EV_A::PAD19),
            18 => Some(WU1_EV_A::PAD18),
            17 => Some(WU1_EV_A::PAD17),
            16 => Some(WU1_EV_A::PAD16),
            15 => Some(WU1_EV_A::PAD15),
            14 => Some(WU1_EV_A::PAD14),
            13 => Some(WU1_EV_A::PAD13),
            12 => Some(WU1_EV_A::PAD12),
            11 => Some(WU1_EV_A::PAD11),
            10 => Some(WU1_EV_A::PAD10),
            9 => Some(WU1_EV_A::PAD9),
            8 => Some(WU1_EV_A::PAD8),
            7 => Some(WU1_EV_A::PAD7),
            6 => Some(WU1_EV_A::PAD6),
            5 => Some(WU1_EV_A::PAD5),
            4 => Some(WU1_EV_A::PAD4),
            3 => Some(WU1_EV_A::PAD3),
            2 => Some(WU1_EV_A::PAD2),
            1 => Some(WU1_EV_A::PAD1),
            0 => Some(WU1_EV_A::PAD0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WU1_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU1_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU1_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU1_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU1_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU1_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU1_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU1_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU1_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU1_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU1_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU1_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU1_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU1_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == WU1_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU1_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU1_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU1_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU1_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU1_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU1_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU1_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU1_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == WU1_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `PAD31`"]
    #[inline(always)]
    pub fn is_pad31(&self) -> bool {
        *self == WU1_EV_A::PAD31
    }
    #[doc = "Checks if the value of the field is `PAD30`"]
    #[inline(always)]
    pub fn is_pad30(&self) -> bool {
        *self == WU1_EV_A::PAD30
    }
    #[doc = "Checks if the value of the field is `PAD29`"]
    #[inline(always)]
    pub fn is_pad29(&self) -> bool {
        *self == WU1_EV_A::PAD29
    }
    #[doc = "Checks if the value of the field is `PAD28`"]
    #[inline(always)]
    pub fn is_pad28(&self) -> bool {
        *self == WU1_EV_A::PAD28
    }
    #[doc = "Checks if the value of the field is `PAD27`"]
    #[inline(always)]
    pub fn is_pad27(&self) -> bool {
        *self == WU1_EV_A::PAD27
    }
    #[doc = "Checks if the value of the field is `PAD26`"]
    #[inline(always)]
    pub fn is_pad26(&self) -> bool {
        *self == WU1_EV_A::PAD26
    }
    #[doc = "Checks if the value of the field is `PAD25`"]
    #[inline(always)]
    pub fn is_pad25(&self) -> bool {
        *self == WU1_EV_A::PAD25
    }
    #[doc = "Checks if the value of the field is `PAD24`"]
    #[inline(always)]
    pub fn is_pad24(&self) -> bool {
        *self == WU1_EV_A::PAD24
    }
    #[doc = "Checks if the value of the field is `PAD23`"]
    #[inline(always)]
    pub fn is_pad23(&self) -> bool {
        *self == WU1_EV_A::PAD23
    }
    #[doc = "Checks if the value of the field is `PAD22`"]
    #[inline(always)]
    pub fn is_pad22(&self) -> bool {
        *self == WU1_EV_A::PAD22
    }
    #[doc = "Checks if the value of the field is `PAD21`"]
    #[inline(always)]
    pub fn is_pad21(&self) -> bool {
        *self == WU1_EV_A::PAD21
    }
    #[doc = "Checks if the value of the field is `PAD20`"]
    #[inline(always)]
    pub fn is_pad20(&self) -> bool {
        *self == WU1_EV_A::PAD20
    }
    #[doc = "Checks if the value of the field is `PAD19`"]
    #[inline(always)]
    pub fn is_pad19(&self) -> bool {
        *self == WU1_EV_A::PAD19
    }
    #[doc = "Checks if the value of the field is `PAD18`"]
    #[inline(always)]
    pub fn is_pad18(&self) -> bool {
        *self == WU1_EV_A::PAD18
    }
    #[doc = "Checks if the value of the field is `PAD17`"]
    #[inline(always)]
    pub fn is_pad17(&self) -> bool {
        *self == WU1_EV_A::PAD17
    }
    #[doc = "Checks if the value of the field is `PAD16`"]
    #[inline(always)]
    pub fn is_pad16(&self) -> bool {
        *self == WU1_EV_A::PAD16
    }
    #[doc = "Checks if the value of the field is `PAD15`"]
    #[inline(always)]
    pub fn is_pad15(&self) -> bool {
        *self == WU1_EV_A::PAD15
    }
    #[doc = "Checks if the value of the field is `PAD14`"]
    #[inline(always)]
    pub fn is_pad14(&self) -> bool {
        *self == WU1_EV_A::PAD14
    }
    #[doc = "Checks if the value of the field is `PAD13`"]
    #[inline(always)]
    pub fn is_pad13(&self) -> bool {
        *self == WU1_EV_A::PAD13
    }
    #[doc = "Checks if the value of the field is `PAD12`"]
    #[inline(always)]
    pub fn is_pad12(&self) -> bool {
        *self == WU1_EV_A::PAD12
    }
    #[doc = "Checks if the value of the field is `PAD11`"]
    #[inline(always)]
    pub fn is_pad11(&self) -> bool {
        *self == WU1_EV_A::PAD11
    }
    #[doc = "Checks if the value of the field is `PAD10`"]
    #[inline(always)]
    pub fn is_pad10(&self) -> bool {
        *self == WU1_EV_A::PAD10
    }
    #[doc = "Checks if the value of the field is `PAD9`"]
    #[inline(always)]
    pub fn is_pad9(&self) -> bool {
        *self == WU1_EV_A::PAD9
    }
    #[doc = "Checks if the value of the field is `PAD8`"]
    #[inline(always)]
    pub fn is_pad8(&self) -> bool {
        *self == WU1_EV_A::PAD8
    }
    #[doc = "Checks if the value of the field is `PAD7`"]
    #[inline(always)]
    pub fn is_pad7(&self) -> bool {
        *self == WU1_EV_A::PAD7
    }
    #[doc = "Checks if the value of the field is `PAD6`"]
    #[inline(always)]
    pub fn is_pad6(&self) -> bool {
        *self == WU1_EV_A::PAD6
    }
    #[doc = "Checks if the value of the field is `PAD5`"]
    #[inline(always)]
    pub fn is_pad5(&self) -> bool {
        *self == WU1_EV_A::PAD5
    }
    #[doc = "Checks if the value of the field is `PAD4`"]
    #[inline(always)]
    pub fn is_pad4(&self) -> bool {
        *self == WU1_EV_A::PAD4
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == WU1_EV_A::PAD3
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == WU1_EV_A::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == WU1_EV_A::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == WU1_EV_A::PAD0
    }
}
#[doc = "Field `WU1_EV` writer - 13:8\\]
MCU Wakeup Source #1 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU1_EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUWUSEL_SPEC, u8, WU1_EV_A, 6, O>;
impl<'a, const O: u8> WU1_EV_W<'a, O> {
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WU1_EV_A::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU1_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU1_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU1_EV_A::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU1_EV_A::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU1_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU1_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU1_EV_A::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU1_EV_A::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU1_EV_A::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU1_EV_A::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU1_EV_A::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU1_EV_A::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU1_EV_A::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU1_EV_A::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU1_EV_A::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU1_EV_A::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU1_EV_A::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU1_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU1_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU1_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU1_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU1_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD)
    }
    #[doc = "Edge detect on PAD31"]
    #[inline(always)]
    pub fn pad31(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD31)
    }
    #[doc = "Edge detect on PAD30"]
    #[inline(always)]
    pub fn pad30(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD30)
    }
    #[doc = "Edge detect on PAD29"]
    #[inline(always)]
    pub fn pad29(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD29)
    }
    #[doc = "Edge detect on PAD28"]
    #[inline(always)]
    pub fn pad28(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD28)
    }
    #[doc = "Edge detect on PAD27"]
    #[inline(always)]
    pub fn pad27(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD27)
    }
    #[doc = "Edge detect on PAD26"]
    #[inline(always)]
    pub fn pad26(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD26)
    }
    #[doc = "Edge detect on PAD25"]
    #[inline(always)]
    pub fn pad25(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD25)
    }
    #[doc = "Edge detect on PAD24"]
    #[inline(always)]
    pub fn pad24(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD24)
    }
    #[doc = "Edge detect on PAD23"]
    #[inline(always)]
    pub fn pad23(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD23)
    }
    #[doc = "Edge detect on PAD22"]
    #[inline(always)]
    pub fn pad22(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD22)
    }
    #[doc = "Edge detect on PAD21"]
    #[inline(always)]
    pub fn pad21(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD21)
    }
    #[doc = "Edge detect on PAD20"]
    #[inline(always)]
    pub fn pad20(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD20)
    }
    #[doc = "Edge detect on PAD19"]
    #[inline(always)]
    pub fn pad19(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD19)
    }
    #[doc = "Edge detect on PAD18"]
    #[inline(always)]
    pub fn pad18(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD18)
    }
    #[doc = "Edge detect on PAD17"]
    #[inline(always)]
    pub fn pad17(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD17)
    }
    #[doc = "Edge detect on PAD16"]
    #[inline(always)]
    pub fn pad16(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD16)
    }
    #[doc = "Edge detect on PAD15"]
    #[inline(always)]
    pub fn pad15(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD15)
    }
    #[doc = "Edge detect on PAD14"]
    #[inline(always)]
    pub fn pad14(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD14)
    }
    #[doc = "Edge detect on PAD13"]
    #[inline(always)]
    pub fn pad13(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD13)
    }
    #[doc = "Edge detect on PAD12"]
    #[inline(always)]
    pub fn pad12(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD12)
    }
    #[doc = "Edge detect on PAD11"]
    #[inline(always)]
    pub fn pad11(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD11)
    }
    #[doc = "Edge detect on PAD10"]
    #[inline(always)]
    pub fn pad10(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD10)
    }
    #[doc = "Edge detect on PAD9"]
    #[inline(always)]
    pub fn pad9(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD9)
    }
    #[doc = "Edge detect on PAD8"]
    #[inline(always)]
    pub fn pad8(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD8)
    }
    #[doc = "Edge detect on PAD7"]
    #[inline(always)]
    pub fn pad7(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD7)
    }
    #[doc = "Edge detect on PAD6"]
    #[inline(always)]
    pub fn pad6(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD6)
    }
    #[doc = "Edge detect on PAD5"]
    #[inline(always)]
    pub fn pad5(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD5)
    }
    #[doc = "Edge detect on PAD4"]
    #[inline(always)]
    pub fn pad4(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD4)
    }
    #[doc = "Edge detect on PAD3"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD3)
    }
    #[doc = "Edge detect on PAD2"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD2)
    }
    #[doc = "Edge detect on PAD1"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD1)
    }
    #[doc = "Edge detect on PAD0"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(WU1_EV_A::PAD0)
    }
}
#[doc = "Field `RESERVED14` reader - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED14` writer - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUWUSEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `WU2_EV` reader - 21:16\\]
MCU Wakeup Source #2 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU2_EV_R = crate::FieldReader<u8, WU2_EV_A>;
#[doc = "21:16\\]
MCU Wakeup Source #2 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WU2_EV_A {
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
    #[doc = "31: Edge detect on PAD31"]
    PAD31 = 31,
    #[doc = "30: Edge detect on PAD30"]
    PAD30 = 30,
    #[doc = "29: Edge detect on PAD29"]
    PAD29 = 29,
    #[doc = "28: Edge detect on PAD28"]
    PAD28 = 28,
    #[doc = "27: Edge detect on PAD27"]
    PAD27 = 27,
    #[doc = "26: Edge detect on PAD26"]
    PAD26 = 26,
    #[doc = "25: Edge detect on PAD25"]
    PAD25 = 25,
    #[doc = "24: Edge detect on PAD24"]
    PAD24 = 24,
    #[doc = "23: Edge detect on PAD23"]
    PAD23 = 23,
    #[doc = "22: Edge detect on PAD22"]
    PAD22 = 22,
    #[doc = "21: Edge detect on PAD21"]
    PAD21 = 21,
    #[doc = "20: Edge detect on PAD20"]
    PAD20 = 20,
    #[doc = "19: Edge detect on PAD19"]
    PAD19 = 19,
    #[doc = "18: Edge detect on PAD18"]
    PAD18 = 18,
    #[doc = "17: Edge detect on PAD17"]
    PAD17 = 17,
    #[doc = "16: Edge detect on PAD16"]
    PAD16 = 16,
    #[doc = "15: Edge detect on PAD15"]
    PAD15 = 15,
    #[doc = "14: Edge detect on PAD14"]
    PAD14 = 14,
    #[doc = "13: Edge detect on PAD13"]
    PAD13 = 13,
    #[doc = "12: Edge detect on PAD12"]
    PAD12 = 12,
    #[doc = "11: Edge detect on PAD11"]
    PAD11 = 11,
    #[doc = "10: Edge detect on PAD10"]
    PAD10 = 10,
    #[doc = "9: Edge detect on PAD9"]
    PAD9 = 9,
    #[doc = "8: Edge detect on PAD8"]
    PAD8 = 8,
    #[doc = "7: Edge detect on PAD7"]
    PAD7 = 7,
    #[doc = "6: Edge detect on PAD6"]
    PAD6 = 6,
    #[doc = "5: Edge detect on PAD5"]
    PAD5 = 5,
    #[doc = "4: Edge detect on PAD4"]
    PAD4 = 4,
    #[doc = "3: Edge detect on PAD3"]
    PAD3 = 3,
    #[doc = "2: Edge detect on PAD2"]
    PAD2 = 2,
    #[doc = "1: Edge detect on PAD1"]
    PAD1 = 1,
    #[doc = "0: Edge detect on PAD0"]
    PAD0 = 0,
}
impl From<WU2_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: WU2_EV_A) -> Self {
        variant as _
    }
}
impl WU2_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WU2_EV_A> {
        match self.bits {
            63 => Some(WU2_EV_A::NONE),
            56 => Some(WU2_EV_A::AUX_COMPB_ASYNC_N),
            55 => Some(WU2_EV_A::AUX_COMPB_ASYNC),
            54 => Some(WU2_EV_A::BATMON_VOLT),
            53 => Some(WU2_EV_A::BATMON_TEMP),
            52 => Some(WU2_EV_A::AUX_TIMER1_EV),
            51 => Some(WU2_EV_A::AUX_TIMER0_EV),
            50 => Some(WU2_EV_A::AUX_TDC_DONE),
            49 => Some(WU2_EV_A::AUX_ADC_DONE),
            48 => Some(WU2_EV_A::AUX_COMPB),
            47 => Some(WU2_EV_A::AUX_COMPA),
            46 => Some(WU2_EV_A::AUX_SWEV2),
            45 => Some(WU2_EV_A::AUX_SWEV1),
            44 => Some(WU2_EV_A::AUX_SWEV0),
            43 => Some(WU2_EV_A::JTAG),
            42 => Some(WU2_EV_A::RTC_UPD),
            41 => Some(WU2_EV_A::RTC_COMB_DLY),
            40 => Some(WU2_EV_A::RTC_CH2_DLY),
            39 => Some(WU2_EV_A::RTC_CH1_DLY),
            38 => Some(WU2_EV_A::RTC_CH0_DLY),
            37 => Some(WU2_EV_A::RTC_CH2),
            36 => Some(WU2_EV_A::RTC_CH1),
            35 => Some(WU2_EV_A::RTC_CH0),
            32 => Some(WU2_EV_A::PAD),
            31 => Some(WU2_EV_A::PAD31),
            30 => Some(WU2_EV_A::PAD30),
            29 => Some(WU2_EV_A::PAD29),
            28 => Some(WU2_EV_A::PAD28),
            27 => Some(WU2_EV_A::PAD27),
            26 => Some(WU2_EV_A::PAD26),
            25 => Some(WU2_EV_A::PAD25),
            24 => Some(WU2_EV_A::PAD24),
            23 => Some(WU2_EV_A::PAD23),
            22 => Some(WU2_EV_A::PAD22),
            21 => Some(WU2_EV_A::PAD21),
            20 => Some(WU2_EV_A::PAD20),
            19 => Some(WU2_EV_A::PAD19),
            18 => Some(WU2_EV_A::PAD18),
            17 => Some(WU2_EV_A::PAD17),
            16 => Some(WU2_EV_A::PAD16),
            15 => Some(WU2_EV_A::PAD15),
            14 => Some(WU2_EV_A::PAD14),
            13 => Some(WU2_EV_A::PAD13),
            12 => Some(WU2_EV_A::PAD12),
            11 => Some(WU2_EV_A::PAD11),
            10 => Some(WU2_EV_A::PAD10),
            9 => Some(WU2_EV_A::PAD9),
            8 => Some(WU2_EV_A::PAD8),
            7 => Some(WU2_EV_A::PAD7),
            6 => Some(WU2_EV_A::PAD6),
            5 => Some(WU2_EV_A::PAD5),
            4 => Some(WU2_EV_A::PAD4),
            3 => Some(WU2_EV_A::PAD3),
            2 => Some(WU2_EV_A::PAD2),
            1 => Some(WU2_EV_A::PAD1),
            0 => Some(WU2_EV_A::PAD0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WU2_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU2_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU2_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU2_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU2_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU2_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU2_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU2_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU2_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU2_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU2_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU2_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU2_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU2_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == WU2_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU2_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU2_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU2_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU2_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU2_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU2_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU2_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU2_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == WU2_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `PAD31`"]
    #[inline(always)]
    pub fn is_pad31(&self) -> bool {
        *self == WU2_EV_A::PAD31
    }
    #[doc = "Checks if the value of the field is `PAD30`"]
    #[inline(always)]
    pub fn is_pad30(&self) -> bool {
        *self == WU2_EV_A::PAD30
    }
    #[doc = "Checks if the value of the field is `PAD29`"]
    #[inline(always)]
    pub fn is_pad29(&self) -> bool {
        *self == WU2_EV_A::PAD29
    }
    #[doc = "Checks if the value of the field is `PAD28`"]
    #[inline(always)]
    pub fn is_pad28(&self) -> bool {
        *self == WU2_EV_A::PAD28
    }
    #[doc = "Checks if the value of the field is `PAD27`"]
    #[inline(always)]
    pub fn is_pad27(&self) -> bool {
        *self == WU2_EV_A::PAD27
    }
    #[doc = "Checks if the value of the field is `PAD26`"]
    #[inline(always)]
    pub fn is_pad26(&self) -> bool {
        *self == WU2_EV_A::PAD26
    }
    #[doc = "Checks if the value of the field is `PAD25`"]
    #[inline(always)]
    pub fn is_pad25(&self) -> bool {
        *self == WU2_EV_A::PAD25
    }
    #[doc = "Checks if the value of the field is `PAD24`"]
    #[inline(always)]
    pub fn is_pad24(&self) -> bool {
        *self == WU2_EV_A::PAD24
    }
    #[doc = "Checks if the value of the field is `PAD23`"]
    #[inline(always)]
    pub fn is_pad23(&self) -> bool {
        *self == WU2_EV_A::PAD23
    }
    #[doc = "Checks if the value of the field is `PAD22`"]
    #[inline(always)]
    pub fn is_pad22(&self) -> bool {
        *self == WU2_EV_A::PAD22
    }
    #[doc = "Checks if the value of the field is `PAD21`"]
    #[inline(always)]
    pub fn is_pad21(&self) -> bool {
        *self == WU2_EV_A::PAD21
    }
    #[doc = "Checks if the value of the field is `PAD20`"]
    #[inline(always)]
    pub fn is_pad20(&self) -> bool {
        *self == WU2_EV_A::PAD20
    }
    #[doc = "Checks if the value of the field is `PAD19`"]
    #[inline(always)]
    pub fn is_pad19(&self) -> bool {
        *self == WU2_EV_A::PAD19
    }
    #[doc = "Checks if the value of the field is `PAD18`"]
    #[inline(always)]
    pub fn is_pad18(&self) -> bool {
        *self == WU2_EV_A::PAD18
    }
    #[doc = "Checks if the value of the field is `PAD17`"]
    #[inline(always)]
    pub fn is_pad17(&self) -> bool {
        *self == WU2_EV_A::PAD17
    }
    #[doc = "Checks if the value of the field is `PAD16`"]
    #[inline(always)]
    pub fn is_pad16(&self) -> bool {
        *self == WU2_EV_A::PAD16
    }
    #[doc = "Checks if the value of the field is `PAD15`"]
    #[inline(always)]
    pub fn is_pad15(&self) -> bool {
        *self == WU2_EV_A::PAD15
    }
    #[doc = "Checks if the value of the field is `PAD14`"]
    #[inline(always)]
    pub fn is_pad14(&self) -> bool {
        *self == WU2_EV_A::PAD14
    }
    #[doc = "Checks if the value of the field is `PAD13`"]
    #[inline(always)]
    pub fn is_pad13(&self) -> bool {
        *self == WU2_EV_A::PAD13
    }
    #[doc = "Checks if the value of the field is `PAD12`"]
    #[inline(always)]
    pub fn is_pad12(&self) -> bool {
        *self == WU2_EV_A::PAD12
    }
    #[doc = "Checks if the value of the field is `PAD11`"]
    #[inline(always)]
    pub fn is_pad11(&self) -> bool {
        *self == WU2_EV_A::PAD11
    }
    #[doc = "Checks if the value of the field is `PAD10`"]
    #[inline(always)]
    pub fn is_pad10(&self) -> bool {
        *self == WU2_EV_A::PAD10
    }
    #[doc = "Checks if the value of the field is `PAD9`"]
    #[inline(always)]
    pub fn is_pad9(&self) -> bool {
        *self == WU2_EV_A::PAD9
    }
    #[doc = "Checks if the value of the field is `PAD8`"]
    #[inline(always)]
    pub fn is_pad8(&self) -> bool {
        *self == WU2_EV_A::PAD8
    }
    #[doc = "Checks if the value of the field is `PAD7`"]
    #[inline(always)]
    pub fn is_pad7(&self) -> bool {
        *self == WU2_EV_A::PAD7
    }
    #[doc = "Checks if the value of the field is `PAD6`"]
    #[inline(always)]
    pub fn is_pad6(&self) -> bool {
        *self == WU2_EV_A::PAD6
    }
    #[doc = "Checks if the value of the field is `PAD5`"]
    #[inline(always)]
    pub fn is_pad5(&self) -> bool {
        *self == WU2_EV_A::PAD5
    }
    #[doc = "Checks if the value of the field is `PAD4`"]
    #[inline(always)]
    pub fn is_pad4(&self) -> bool {
        *self == WU2_EV_A::PAD4
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == WU2_EV_A::PAD3
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == WU2_EV_A::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == WU2_EV_A::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == WU2_EV_A::PAD0
    }
}
#[doc = "Field `WU2_EV` writer - 21:16\\]
MCU Wakeup Source #2 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU2_EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUWUSEL_SPEC, u8, WU2_EV_A, 6, O>;
impl<'a, const O: u8> WU2_EV_W<'a, O> {
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WU2_EV_A::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU2_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU2_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU2_EV_A::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU2_EV_A::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU2_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU2_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU2_EV_A::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU2_EV_A::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU2_EV_A::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU2_EV_A::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU2_EV_A::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU2_EV_A::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU2_EV_A::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU2_EV_A::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU2_EV_A::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU2_EV_A::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU2_EV_A::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU2_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU2_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU2_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU2_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU2_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD)
    }
    #[doc = "Edge detect on PAD31"]
    #[inline(always)]
    pub fn pad31(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD31)
    }
    #[doc = "Edge detect on PAD30"]
    #[inline(always)]
    pub fn pad30(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD30)
    }
    #[doc = "Edge detect on PAD29"]
    #[inline(always)]
    pub fn pad29(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD29)
    }
    #[doc = "Edge detect on PAD28"]
    #[inline(always)]
    pub fn pad28(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD28)
    }
    #[doc = "Edge detect on PAD27"]
    #[inline(always)]
    pub fn pad27(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD27)
    }
    #[doc = "Edge detect on PAD26"]
    #[inline(always)]
    pub fn pad26(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD26)
    }
    #[doc = "Edge detect on PAD25"]
    #[inline(always)]
    pub fn pad25(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD25)
    }
    #[doc = "Edge detect on PAD24"]
    #[inline(always)]
    pub fn pad24(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD24)
    }
    #[doc = "Edge detect on PAD23"]
    #[inline(always)]
    pub fn pad23(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD23)
    }
    #[doc = "Edge detect on PAD22"]
    #[inline(always)]
    pub fn pad22(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD22)
    }
    #[doc = "Edge detect on PAD21"]
    #[inline(always)]
    pub fn pad21(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD21)
    }
    #[doc = "Edge detect on PAD20"]
    #[inline(always)]
    pub fn pad20(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD20)
    }
    #[doc = "Edge detect on PAD19"]
    #[inline(always)]
    pub fn pad19(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD19)
    }
    #[doc = "Edge detect on PAD18"]
    #[inline(always)]
    pub fn pad18(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD18)
    }
    #[doc = "Edge detect on PAD17"]
    #[inline(always)]
    pub fn pad17(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD17)
    }
    #[doc = "Edge detect on PAD16"]
    #[inline(always)]
    pub fn pad16(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD16)
    }
    #[doc = "Edge detect on PAD15"]
    #[inline(always)]
    pub fn pad15(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD15)
    }
    #[doc = "Edge detect on PAD14"]
    #[inline(always)]
    pub fn pad14(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD14)
    }
    #[doc = "Edge detect on PAD13"]
    #[inline(always)]
    pub fn pad13(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD13)
    }
    #[doc = "Edge detect on PAD12"]
    #[inline(always)]
    pub fn pad12(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD12)
    }
    #[doc = "Edge detect on PAD11"]
    #[inline(always)]
    pub fn pad11(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD11)
    }
    #[doc = "Edge detect on PAD10"]
    #[inline(always)]
    pub fn pad10(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD10)
    }
    #[doc = "Edge detect on PAD9"]
    #[inline(always)]
    pub fn pad9(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD9)
    }
    #[doc = "Edge detect on PAD8"]
    #[inline(always)]
    pub fn pad8(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD8)
    }
    #[doc = "Edge detect on PAD7"]
    #[inline(always)]
    pub fn pad7(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD7)
    }
    #[doc = "Edge detect on PAD6"]
    #[inline(always)]
    pub fn pad6(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD6)
    }
    #[doc = "Edge detect on PAD5"]
    #[inline(always)]
    pub fn pad5(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD5)
    }
    #[doc = "Edge detect on PAD4"]
    #[inline(always)]
    pub fn pad4(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD4)
    }
    #[doc = "Edge detect on PAD3"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD3)
    }
    #[doc = "Edge detect on PAD2"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD2)
    }
    #[doc = "Edge detect on PAD1"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD1)
    }
    #[doc = "Edge detect on PAD0"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(WU2_EV_A::PAD0)
    }
}
#[doc = "Field `RESERVED22` reader - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED22` writer - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUWUSEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `WU3_EV` reader - 29:24\\]
MCU Wakeup Source #3 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU3_EV_R = crate::FieldReader<u8, WU3_EV_A>;
#[doc = "29:24\\]
MCU Wakeup Source #3 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WU3_EV_A {
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
    #[doc = "31: Edge detect on PAD31"]
    PAD31 = 31,
    #[doc = "30: Edge detect on PAD30"]
    PAD30 = 30,
    #[doc = "29: Edge detect on PAD29"]
    PAD29 = 29,
    #[doc = "28: Edge detect on PAD28"]
    PAD28 = 28,
    #[doc = "27: Edge detect on PAD27"]
    PAD27 = 27,
    #[doc = "26: Edge detect on PAD26"]
    PAD26 = 26,
    #[doc = "25: Edge detect on PAD25"]
    PAD25 = 25,
    #[doc = "24: Edge detect on PAD24"]
    PAD24 = 24,
    #[doc = "23: Edge detect on PAD23"]
    PAD23 = 23,
    #[doc = "22: Edge detect on PAD22"]
    PAD22 = 22,
    #[doc = "21: Edge detect on PAD21"]
    PAD21 = 21,
    #[doc = "20: Edge detect on PAD20"]
    PAD20 = 20,
    #[doc = "19: Edge detect on PAD19"]
    PAD19 = 19,
    #[doc = "18: Edge detect on PAD18"]
    PAD18 = 18,
    #[doc = "17: Edge detect on PAD17"]
    PAD17 = 17,
    #[doc = "16: Edge detect on PAD16"]
    PAD16 = 16,
    #[doc = "15: Edge detect on PAD15"]
    PAD15 = 15,
    #[doc = "14: Edge detect on PAD14"]
    PAD14 = 14,
    #[doc = "13: Edge detect on PAD13"]
    PAD13 = 13,
    #[doc = "12: Edge detect on PAD12"]
    PAD12 = 12,
    #[doc = "11: Edge detect on PAD11"]
    PAD11 = 11,
    #[doc = "10: Edge detect on PAD10"]
    PAD10 = 10,
    #[doc = "9: Edge detect on PAD9"]
    PAD9 = 9,
    #[doc = "8: Edge detect on PAD8"]
    PAD8 = 8,
    #[doc = "7: Edge detect on PAD7"]
    PAD7 = 7,
    #[doc = "6: Edge detect on PAD6"]
    PAD6 = 6,
    #[doc = "5: Edge detect on PAD5"]
    PAD5 = 5,
    #[doc = "4: Edge detect on PAD4"]
    PAD4 = 4,
    #[doc = "3: Edge detect on PAD3"]
    PAD3 = 3,
    #[doc = "2: Edge detect on PAD2"]
    PAD2 = 2,
    #[doc = "1: Edge detect on PAD1"]
    PAD1 = 1,
    #[doc = "0: Edge detect on PAD0"]
    PAD0 = 0,
}
impl From<WU3_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: WU3_EV_A) -> Self {
        variant as _
    }
}
impl WU3_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WU3_EV_A> {
        match self.bits {
            63 => Some(WU3_EV_A::NONE),
            56 => Some(WU3_EV_A::AUX_COMPB_ASYNC_N),
            55 => Some(WU3_EV_A::AUX_COMPB_ASYNC),
            54 => Some(WU3_EV_A::BATMON_VOLT),
            53 => Some(WU3_EV_A::BATMON_TEMP),
            52 => Some(WU3_EV_A::AUX_TIMER1_EV),
            51 => Some(WU3_EV_A::AUX_TIMER0_EV),
            50 => Some(WU3_EV_A::AUX_TDC_DONE),
            49 => Some(WU3_EV_A::AUX_ADC_DONE),
            48 => Some(WU3_EV_A::AUX_COMPB),
            47 => Some(WU3_EV_A::AUX_COMPA),
            46 => Some(WU3_EV_A::AUX_SWEV2),
            45 => Some(WU3_EV_A::AUX_SWEV1),
            44 => Some(WU3_EV_A::AUX_SWEV0),
            43 => Some(WU3_EV_A::JTAG),
            42 => Some(WU3_EV_A::RTC_UPD),
            41 => Some(WU3_EV_A::RTC_COMB_DLY),
            40 => Some(WU3_EV_A::RTC_CH2_DLY),
            39 => Some(WU3_EV_A::RTC_CH1_DLY),
            38 => Some(WU3_EV_A::RTC_CH0_DLY),
            37 => Some(WU3_EV_A::RTC_CH2),
            36 => Some(WU3_EV_A::RTC_CH1),
            35 => Some(WU3_EV_A::RTC_CH0),
            32 => Some(WU3_EV_A::PAD),
            31 => Some(WU3_EV_A::PAD31),
            30 => Some(WU3_EV_A::PAD30),
            29 => Some(WU3_EV_A::PAD29),
            28 => Some(WU3_EV_A::PAD28),
            27 => Some(WU3_EV_A::PAD27),
            26 => Some(WU3_EV_A::PAD26),
            25 => Some(WU3_EV_A::PAD25),
            24 => Some(WU3_EV_A::PAD24),
            23 => Some(WU3_EV_A::PAD23),
            22 => Some(WU3_EV_A::PAD22),
            21 => Some(WU3_EV_A::PAD21),
            20 => Some(WU3_EV_A::PAD20),
            19 => Some(WU3_EV_A::PAD19),
            18 => Some(WU3_EV_A::PAD18),
            17 => Some(WU3_EV_A::PAD17),
            16 => Some(WU3_EV_A::PAD16),
            15 => Some(WU3_EV_A::PAD15),
            14 => Some(WU3_EV_A::PAD14),
            13 => Some(WU3_EV_A::PAD13),
            12 => Some(WU3_EV_A::PAD12),
            11 => Some(WU3_EV_A::PAD11),
            10 => Some(WU3_EV_A::PAD10),
            9 => Some(WU3_EV_A::PAD9),
            8 => Some(WU3_EV_A::PAD8),
            7 => Some(WU3_EV_A::PAD7),
            6 => Some(WU3_EV_A::PAD6),
            5 => Some(WU3_EV_A::PAD5),
            4 => Some(WU3_EV_A::PAD4),
            3 => Some(WU3_EV_A::PAD3),
            2 => Some(WU3_EV_A::PAD2),
            1 => Some(WU3_EV_A::PAD1),
            0 => Some(WU3_EV_A::PAD0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WU3_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU3_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU3_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU3_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU3_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU3_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU3_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU3_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU3_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU3_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU3_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU3_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU3_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU3_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == WU3_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU3_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU3_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU3_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU3_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU3_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU3_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU3_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU3_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == WU3_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `PAD31`"]
    #[inline(always)]
    pub fn is_pad31(&self) -> bool {
        *self == WU3_EV_A::PAD31
    }
    #[doc = "Checks if the value of the field is `PAD30`"]
    #[inline(always)]
    pub fn is_pad30(&self) -> bool {
        *self == WU3_EV_A::PAD30
    }
    #[doc = "Checks if the value of the field is `PAD29`"]
    #[inline(always)]
    pub fn is_pad29(&self) -> bool {
        *self == WU3_EV_A::PAD29
    }
    #[doc = "Checks if the value of the field is `PAD28`"]
    #[inline(always)]
    pub fn is_pad28(&self) -> bool {
        *self == WU3_EV_A::PAD28
    }
    #[doc = "Checks if the value of the field is `PAD27`"]
    #[inline(always)]
    pub fn is_pad27(&self) -> bool {
        *self == WU3_EV_A::PAD27
    }
    #[doc = "Checks if the value of the field is `PAD26`"]
    #[inline(always)]
    pub fn is_pad26(&self) -> bool {
        *self == WU3_EV_A::PAD26
    }
    #[doc = "Checks if the value of the field is `PAD25`"]
    #[inline(always)]
    pub fn is_pad25(&self) -> bool {
        *self == WU3_EV_A::PAD25
    }
    #[doc = "Checks if the value of the field is `PAD24`"]
    #[inline(always)]
    pub fn is_pad24(&self) -> bool {
        *self == WU3_EV_A::PAD24
    }
    #[doc = "Checks if the value of the field is `PAD23`"]
    #[inline(always)]
    pub fn is_pad23(&self) -> bool {
        *self == WU3_EV_A::PAD23
    }
    #[doc = "Checks if the value of the field is `PAD22`"]
    #[inline(always)]
    pub fn is_pad22(&self) -> bool {
        *self == WU3_EV_A::PAD22
    }
    #[doc = "Checks if the value of the field is `PAD21`"]
    #[inline(always)]
    pub fn is_pad21(&self) -> bool {
        *self == WU3_EV_A::PAD21
    }
    #[doc = "Checks if the value of the field is `PAD20`"]
    #[inline(always)]
    pub fn is_pad20(&self) -> bool {
        *self == WU3_EV_A::PAD20
    }
    #[doc = "Checks if the value of the field is `PAD19`"]
    #[inline(always)]
    pub fn is_pad19(&self) -> bool {
        *self == WU3_EV_A::PAD19
    }
    #[doc = "Checks if the value of the field is `PAD18`"]
    #[inline(always)]
    pub fn is_pad18(&self) -> bool {
        *self == WU3_EV_A::PAD18
    }
    #[doc = "Checks if the value of the field is `PAD17`"]
    #[inline(always)]
    pub fn is_pad17(&self) -> bool {
        *self == WU3_EV_A::PAD17
    }
    #[doc = "Checks if the value of the field is `PAD16`"]
    #[inline(always)]
    pub fn is_pad16(&self) -> bool {
        *self == WU3_EV_A::PAD16
    }
    #[doc = "Checks if the value of the field is `PAD15`"]
    #[inline(always)]
    pub fn is_pad15(&self) -> bool {
        *self == WU3_EV_A::PAD15
    }
    #[doc = "Checks if the value of the field is `PAD14`"]
    #[inline(always)]
    pub fn is_pad14(&self) -> bool {
        *self == WU3_EV_A::PAD14
    }
    #[doc = "Checks if the value of the field is `PAD13`"]
    #[inline(always)]
    pub fn is_pad13(&self) -> bool {
        *self == WU3_EV_A::PAD13
    }
    #[doc = "Checks if the value of the field is `PAD12`"]
    #[inline(always)]
    pub fn is_pad12(&self) -> bool {
        *self == WU3_EV_A::PAD12
    }
    #[doc = "Checks if the value of the field is `PAD11`"]
    #[inline(always)]
    pub fn is_pad11(&self) -> bool {
        *self == WU3_EV_A::PAD11
    }
    #[doc = "Checks if the value of the field is `PAD10`"]
    #[inline(always)]
    pub fn is_pad10(&self) -> bool {
        *self == WU3_EV_A::PAD10
    }
    #[doc = "Checks if the value of the field is `PAD9`"]
    #[inline(always)]
    pub fn is_pad9(&self) -> bool {
        *self == WU3_EV_A::PAD9
    }
    #[doc = "Checks if the value of the field is `PAD8`"]
    #[inline(always)]
    pub fn is_pad8(&self) -> bool {
        *self == WU3_EV_A::PAD8
    }
    #[doc = "Checks if the value of the field is `PAD7`"]
    #[inline(always)]
    pub fn is_pad7(&self) -> bool {
        *self == WU3_EV_A::PAD7
    }
    #[doc = "Checks if the value of the field is `PAD6`"]
    #[inline(always)]
    pub fn is_pad6(&self) -> bool {
        *self == WU3_EV_A::PAD6
    }
    #[doc = "Checks if the value of the field is `PAD5`"]
    #[inline(always)]
    pub fn is_pad5(&self) -> bool {
        *self == WU3_EV_A::PAD5
    }
    #[doc = "Checks if the value of the field is `PAD4`"]
    #[inline(always)]
    pub fn is_pad4(&self) -> bool {
        *self == WU3_EV_A::PAD4
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == WU3_EV_A::PAD3
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == WU3_EV_A::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == WU3_EV_A::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == WU3_EV_A::PAD0
    }
}
#[doc = "Field `WU3_EV` writer - 29:24\\]
MCU Wakeup Source #3 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type WU3_EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUWUSEL_SPEC, u8, WU3_EV_A, 6, O>;
impl<'a, const O: u8> WU3_EV_W<'a, O> {
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WU3_EV_A::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU3_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU3_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU3_EV_A::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU3_EV_A::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU3_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU3_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU3_EV_A::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU3_EV_A::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU3_EV_A::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU3_EV_A::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU3_EV_A::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU3_EV_A::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU3_EV_A::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU3_EV_A::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU3_EV_A::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU3_EV_A::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU3_EV_A::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU3_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU3_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU3_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU3_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU3_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD)
    }
    #[doc = "Edge detect on PAD31"]
    #[inline(always)]
    pub fn pad31(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD31)
    }
    #[doc = "Edge detect on PAD30"]
    #[inline(always)]
    pub fn pad30(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD30)
    }
    #[doc = "Edge detect on PAD29"]
    #[inline(always)]
    pub fn pad29(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD29)
    }
    #[doc = "Edge detect on PAD28"]
    #[inline(always)]
    pub fn pad28(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD28)
    }
    #[doc = "Edge detect on PAD27"]
    #[inline(always)]
    pub fn pad27(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD27)
    }
    #[doc = "Edge detect on PAD26"]
    #[inline(always)]
    pub fn pad26(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD26)
    }
    #[doc = "Edge detect on PAD25"]
    #[inline(always)]
    pub fn pad25(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD25)
    }
    #[doc = "Edge detect on PAD24"]
    #[inline(always)]
    pub fn pad24(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD24)
    }
    #[doc = "Edge detect on PAD23"]
    #[inline(always)]
    pub fn pad23(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD23)
    }
    #[doc = "Edge detect on PAD22"]
    #[inline(always)]
    pub fn pad22(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD22)
    }
    #[doc = "Edge detect on PAD21"]
    #[inline(always)]
    pub fn pad21(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD21)
    }
    #[doc = "Edge detect on PAD20"]
    #[inline(always)]
    pub fn pad20(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD20)
    }
    #[doc = "Edge detect on PAD19"]
    #[inline(always)]
    pub fn pad19(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD19)
    }
    #[doc = "Edge detect on PAD18"]
    #[inline(always)]
    pub fn pad18(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD18)
    }
    #[doc = "Edge detect on PAD17"]
    #[inline(always)]
    pub fn pad17(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD17)
    }
    #[doc = "Edge detect on PAD16"]
    #[inline(always)]
    pub fn pad16(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD16)
    }
    #[doc = "Edge detect on PAD15"]
    #[inline(always)]
    pub fn pad15(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD15)
    }
    #[doc = "Edge detect on PAD14"]
    #[inline(always)]
    pub fn pad14(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD14)
    }
    #[doc = "Edge detect on PAD13"]
    #[inline(always)]
    pub fn pad13(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD13)
    }
    #[doc = "Edge detect on PAD12"]
    #[inline(always)]
    pub fn pad12(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD12)
    }
    #[doc = "Edge detect on PAD11"]
    #[inline(always)]
    pub fn pad11(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD11)
    }
    #[doc = "Edge detect on PAD10"]
    #[inline(always)]
    pub fn pad10(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD10)
    }
    #[doc = "Edge detect on PAD9"]
    #[inline(always)]
    pub fn pad9(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD9)
    }
    #[doc = "Edge detect on PAD8"]
    #[inline(always)]
    pub fn pad8(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD8)
    }
    #[doc = "Edge detect on PAD7"]
    #[inline(always)]
    pub fn pad7(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD7)
    }
    #[doc = "Edge detect on PAD6"]
    #[inline(always)]
    pub fn pad6(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD6)
    }
    #[doc = "Edge detect on PAD5"]
    #[inline(always)]
    pub fn pad5(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD5)
    }
    #[doc = "Edge detect on PAD4"]
    #[inline(always)]
    pub fn pad4(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD4)
    }
    #[doc = "Edge detect on PAD3"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD3)
    }
    #[doc = "Edge detect on PAD2"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD2)
    }
    #[doc = "Edge detect on PAD1"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD1)
    }
    #[doc = "Edge detect on PAD0"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(WU3_EV_A::PAD0)
    }
}
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUWUSEL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
MCU Wakeup Source #0 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu0_ev(&self) -> WU0_EV_R {
        WU0_EV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
MCU Wakeup Source #1 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu1_ev(&self) -> WU1_EV_R {
        WU1_EV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
MCU Wakeup Source #2 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu2_ev(&self) -> WU2_EV_R {
        WU2_EV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> RESERVED22_R {
        RESERVED22_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
MCU Wakeup Source #3 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu3_ev(&self) -> WU3_EV_R {
        WU3_EV_R::new(((self.bits >> 24) & 0x3f) as u8)
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
MCU Wakeup Source #0 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu0_ev(&mut self) -> WU0_EV_W<0> {
        WU0_EV_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bits 8:13 - 13:8\\]
MCU Wakeup Source #1 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu1_ev(&mut self) -> WU1_EV_W<8> {
        WU1_EV_W::new(self)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> RESERVED14_W<14> {
        RESERVED14_W::new(self)
    }
    #[doc = "Bits 16:21 - 21:16\\]
MCU Wakeup Source #2 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu2_ev(&mut self) -> WU2_EV_W<16> {
        WU2_EV_W::new(self)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> RESERVED22_W<22> {
        RESERVED22_W::new(self)
    }
    #[doc = "Bits 24:29 - 29:24\\]
MCU Wakeup Source #3 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu3_ev(&mut self) -> WU3_EV_W<24> {
        WU3_EV_W::new(self)
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
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 events which are routed to AON_WUC as wakeup sources for MCU. AON_WUC will start a wakeup sequence for the MCU domain when either of the 4 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is recommended ( or required when AON_WUC:MCUCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before MCU is requesting powerdown. ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ) as it will speed up the wakeup procedure.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcuwusel](index.html) module"]
pub struct MCUWUSEL_SPEC;
impl crate::RegisterSpec for MCUWUSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcuwusel::R](R) reader structure"]
impl crate::Readable for MCUWUSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcuwusel::W](W) writer structure"]
impl crate::Writable for MCUWUSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUWUSEL to value 0x3f3f_3f3f"]
impl crate::Resettable for MCUWUSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f3f_3f3f;
}

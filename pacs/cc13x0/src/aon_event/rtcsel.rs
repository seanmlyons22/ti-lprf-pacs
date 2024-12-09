#[doc = "Register `RTCSEL` reader"]
pub type R = crate::R<RtcselSpec>;
#[doc = "Register `RTCSEL` writer"]
pub type W = crate::W<RtcselSpec>;
#[doc = "5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RtcCh1CaptEv {
    #[doc = "63: No event, always low"]
    None = 63,
    #[doc = "56: Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AuxCompbAsyncN = 56,
    #[doc = "55: Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AuxCompbAsync = 55,
    #[doc = "54: BATMON voltage update event"]
    BatmonVolt = 54,
    #[doc = "53: BATMON temperature update event"]
    BatmonTemp = 53,
    #[doc = "52: AUX Timer 1 Event"]
    AuxTimer1Ev = 52,
    #[doc = "51: AUX Timer 0 Event"]
    AuxTimer0Ev = 51,
    #[doc = "50: TDC completed or timed out"]
    AuxTdcDone = 50,
    #[doc = "49: ADC conversion completed"]
    AuxAdcDone = 49,
    #[doc = "48: Comparator B triggered"]
    AuxCompb = 48,
    #[doc = "47: Comparator A triggered"]
    AuxCompa = 47,
    #[doc = "46: AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AuxSwev2 = 46,
    #[doc = "45: AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AuxSwev1 = 45,
    #[doc = "44: AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AuxSwev0 = 44,
    #[doc = "43: JTAG generated event"]
    Jtag = 43,
    #[doc = "42: RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RtcUpd = 42,
    #[doc = "41: RTC combined delayed event"]
    RtcCombDly = 41,
    #[doc = "40: RTC channel 2 - delayed event"]
    RtcCh2Dly = 40,
    #[doc = "39: RTC channel 1 - delayed event"]
    RtcCh1Dly = 39,
    #[doc = "38: RTC channel 0 - delayed event"]
    RtcCh0Dly = 38,
    #[doc = "37: RTC channel 2 event"]
    RtcCh2 = 37,
    #[doc = "36: RTC channel 1 event"]
    RtcCh1 = 36,
    #[doc = "35: RTC channel 0 event"]
    RtcCh0 = 35,
    #[doc = "32: Edge detect on any PAD"]
    Pad = 32,
    #[doc = "31: Edge detect on PAD31"]
    Pad31 = 31,
    #[doc = "30: Edge detect on PAD30"]
    Pad30 = 30,
    #[doc = "29: Edge detect on PAD29"]
    Pad29 = 29,
    #[doc = "28: Edge detect on PAD28"]
    Pad28 = 28,
    #[doc = "27: Edge detect on PAD27"]
    Pad27 = 27,
    #[doc = "26: Edge detect on PAD26"]
    Pad26 = 26,
    #[doc = "25: Edge detect on PAD25"]
    Pad25 = 25,
    #[doc = "24: Edge detect on PAD24"]
    Pad24 = 24,
    #[doc = "23: Edge detect on PAD23"]
    Pad23 = 23,
    #[doc = "22: Edge detect on PAD22"]
    Pad22 = 22,
    #[doc = "21: Edge detect on PAD21"]
    Pad21 = 21,
    #[doc = "20: Edge detect on PAD20"]
    Pad20 = 20,
    #[doc = "19: Edge detect on PAD19"]
    Pad19 = 19,
    #[doc = "18: Edge detect on PAD18"]
    Pad18 = 18,
    #[doc = "17: Edge detect on PAD17"]
    Pad17 = 17,
    #[doc = "16: Edge detect on PAD16"]
    Pad16 = 16,
    #[doc = "15: Edge detect on PAD15"]
    Pad15 = 15,
    #[doc = "14: Edge detect on PAD14"]
    Pad14 = 14,
    #[doc = "13: Edge detect on PAD13"]
    Pad13 = 13,
    #[doc = "12: Edge detect on PAD12"]
    Pad12 = 12,
    #[doc = "11: Edge detect on PAD11"]
    Pad11 = 11,
    #[doc = "10: Edge detect on PAD10"]
    Pad10 = 10,
    #[doc = "9: Edge detect on PAD9"]
    Pad9 = 9,
    #[doc = "8: Edge detect on PAD8"]
    Pad8 = 8,
    #[doc = "7: Edge detect on PAD7"]
    Pad7 = 7,
    #[doc = "6: Edge detect on PAD6"]
    Pad6 = 6,
    #[doc = "5: Edge detect on PAD5"]
    Pad5 = 5,
    #[doc = "4: Edge detect on PAD4"]
    Pad4 = 4,
    #[doc = "3: Edge detect on PAD3"]
    Pad3 = 3,
    #[doc = "2: Edge detect on PAD2"]
    Pad2 = 2,
    #[doc = "1: Edge detect on PAD1"]
    Pad1 = 1,
    #[doc = "0: Edge detect on PAD0"]
    Pad0 = 0,
}
impl From<RtcCh1CaptEv> for u8 {
    #[inline(always)]
    fn from(variant: RtcCh1CaptEv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RtcCh1CaptEv {
    type Ux = u8;
}
impl crate::IsEnum for RtcCh1CaptEv {}
#[doc = "Field `RTC_CH1_CAPT_EV` reader - 5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
pub type RtcCh1CaptEvR = crate::FieldReader<RtcCh1CaptEv>;
impl RtcCh1CaptEvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RtcCh1CaptEv> {
        match self.bits {
            63 => Some(RtcCh1CaptEv::None),
            56 => Some(RtcCh1CaptEv::AuxCompbAsyncN),
            55 => Some(RtcCh1CaptEv::AuxCompbAsync),
            54 => Some(RtcCh1CaptEv::BatmonVolt),
            53 => Some(RtcCh1CaptEv::BatmonTemp),
            52 => Some(RtcCh1CaptEv::AuxTimer1Ev),
            51 => Some(RtcCh1CaptEv::AuxTimer0Ev),
            50 => Some(RtcCh1CaptEv::AuxTdcDone),
            49 => Some(RtcCh1CaptEv::AuxAdcDone),
            48 => Some(RtcCh1CaptEv::AuxCompb),
            47 => Some(RtcCh1CaptEv::AuxCompa),
            46 => Some(RtcCh1CaptEv::AuxSwev2),
            45 => Some(RtcCh1CaptEv::AuxSwev1),
            44 => Some(RtcCh1CaptEv::AuxSwev0),
            43 => Some(RtcCh1CaptEv::Jtag),
            42 => Some(RtcCh1CaptEv::RtcUpd),
            41 => Some(RtcCh1CaptEv::RtcCombDly),
            40 => Some(RtcCh1CaptEv::RtcCh2Dly),
            39 => Some(RtcCh1CaptEv::RtcCh1Dly),
            38 => Some(RtcCh1CaptEv::RtcCh0Dly),
            37 => Some(RtcCh1CaptEv::RtcCh2),
            36 => Some(RtcCh1CaptEv::RtcCh1),
            35 => Some(RtcCh1CaptEv::RtcCh0),
            32 => Some(RtcCh1CaptEv::Pad),
            31 => Some(RtcCh1CaptEv::Pad31),
            30 => Some(RtcCh1CaptEv::Pad30),
            29 => Some(RtcCh1CaptEv::Pad29),
            28 => Some(RtcCh1CaptEv::Pad28),
            27 => Some(RtcCh1CaptEv::Pad27),
            26 => Some(RtcCh1CaptEv::Pad26),
            25 => Some(RtcCh1CaptEv::Pad25),
            24 => Some(RtcCh1CaptEv::Pad24),
            23 => Some(RtcCh1CaptEv::Pad23),
            22 => Some(RtcCh1CaptEv::Pad22),
            21 => Some(RtcCh1CaptEv::Pad21),
            20 => Some(RtcCh1CaptEv::Pad20),
            19 => Some(RtcCh1CaptEv::Pad19),
            18 => Some(RtcCh1CaptEv::Pad18),
            17 => Some(RtcCh1CaptEv::Pad17),
            16 => Some(RtcCh1CaptEv::Pad16),
            15 => Some(RtcCh1CaptEv::Pad15),
            14 => Some(RtcCh1CaptEv::Pad14),
            13 => Some(RtcCh1CaptEv::Pad13),
            12 => Some(RtcCh1CaptEv::Pad12),
            11 => Some(RtcCh1CaptEv::Pad11),
            10 => Some(RtcCh1CaptEv::Pad10),
            9 => Some(RtcCh1CaptEv::Pad9),
            8 => Some(RtcCh1CaptEv::Pad8),
            7 => Some(RtcCh1CaptEv::Pad7),
            6 => Some(RtcCh1CaptEv::Pad6),
            5 => Some(RtcCh1CaptEv::Pad5),
            4 => Some(RtcCh1CaptEv::Pad4),
            3 => Some(RtcCh1CaptEv::Pad3),
            2 => Some(RtcCh1CaptEv::Pad2),
            1 => Some(RtcCh1CaptEv::Pad1),
            0 => Some(RtcCh1CaptEv::Pad0),
            _ => None,
        }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RtcCh1CaptEv::None
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == RtcCh1CaptEv::AuxCompbAsyncN
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == RtcCh1CaptEv::AuxCompbAsync
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == RtcCh1CaptEv::BatmonVolt
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == RtcCh1CaptEv::BatmonTemp
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == RtcCh1CaptEv::AuxTimer1Ev
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == RtcCh1CaptEv::AuxTimer0Ev
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == RtcCh1CaptEv::AuxTdcDone
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == RtcCh1CaptEv::AuxAdcDone
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == RtcCh1CaptEv::AuxCompb
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == RtcCh1CaptEv::AuxCompa
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == RtcCh1CaptEv::AuxSwev2
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == RtcCh1CaptEv::AuxSwev1
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == RtcCh1CaptEv::AuxSwev0
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == RtcCh1CaptEv::Jtag
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == RtcCh1CaptEv::RtcUpd
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCombDly
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCh2Dly
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCh1Dly
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCh0Dly
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCh2
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCh1
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCh0
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == RtcCh1CaptEv::Pad
    }
    #[doc = "Edge detect on PAD31"]
    #[inline(always)]
    pub fn is_pad31(&self) -> bool {
        *self == RtcCh1CaptEv::Pad31
    }
    #[doc = "Edge detect on PAD30"]
    #[inline(always)]
    pub fn is_pad30(&self) -> bool {
        *self == RtcCh1CaptEv::Pad30
    }
    #[doc = "Edge detect on PAD29"]
    #[inline(always)]
    pub fn is_pad29(&self) -> bool {
        *self == RtcCh1CaptEv::Pad29
    }
    #[doc = "Edge detect on PAD28"]
    #[inline(always)]
    pub fn is_pad28(&self) -> bool {
        *self == RtcCh1CaptEv::Pad28
    }
    #[doc = "Edge detect on PAD27"]
    #[inline(always)]
    pub fn is_pad27(&self) -> bool {
        *self == RtcCh1CaptEv::Pad27
    }
    #[doc = "Edge detect on PAD26"]
    #[inline(always)]
    pub fn is_pad26(&self) -> bool {
        *self == RtcCh1CaptEv::Pad26
    }
    #[doc = "Edge detect on PAD25"]
    #[inline(always)]
    pub fn is_pad25(&self) -> bool {
        *self == RtcCh1CaptEv::Pad25
    }
    #[doc = "Edge detect on PAD24"]
    #[inline(always)]
    pub fn is_pad24(&self) -> bool {
        *self == RtcCh1CaptEv::Pad24
    }
    #[doc = "Edge detect on PAD23"]
    #[inline(always)]
    pub fn is_pad23(&self) -> bool {
        *self == RtcCh1CaptEv::Pad23
    }
    #[doc = "Edge detect on PAD22"]
    #[inline(always)]
    pub fn is_pad22(&self) -> bool {
        *self == RtcCh1CaptEv::Pad22
    }
    #[doc = "Edge detect on PAD21"]
    #[inline(always)]
    pub fn is_pad21(&self) -> bool {
        *self == RtcCh1CaptEv::Pad21
    }
    #[doc = "Edge detect on PAD20"]
    #[inline(always)]
    pub fn is_pad20(&self) -> bool {
        *self == RtcCh1CaptEv::Pad20
    }
    #[doc = "Edge detect on PAD19"]
    #[inline(always)]
    pub fn is_pad19(&self) -> bool {
        *self == RtcCh1CaptEv::Pad19
    }
    #[doc = "Edge detect on PAD18"]
    #[inline(always)]
    pub fn is_pad18(&self) -> bool {
        *self == RtcCh1CaptEv::Pad18
    }
    #[doc = "Edge detect on PAD17"]
    #[inline(always)]
    pub fn is_pad17(&self) -> bool {
        *self == RtcCh1CaptEv::Pad17
    }
    #[doc = "Edge detect on PAD16"]
    #[inline(always)]
    pub fn is_pad16(&self) -> bool {
        *self == RtcCh1CaptEv::Pad16
    }
    #[doc = "Edge detect on PAD15"]
    #[inline(always)]
    pub fn is_pad15(&self) -> bool {
        *self == RtcCh1CaptEv::Pad15
    }
    #[doc = "Edge detect on PAD14"]
    #[inline(always)]
    pub fn is_pad14(&self) -> bool {
        *self == RtcCh1CaptEv::Pad14
    }
    #[doc = "Edge detect on PAD13"]
    #[inline(always)]
    pub fn is_pad13(&self) -> bool {
        *self == RtcCh1CaptEv::Pad13
    }
    #[doc = "Edge detect on PAD12"]
    #[inline(always)]
    pub fn is_pad12(&self) -> bool {
        *self == RtcCh1CaptEv::Pad12
    }
    #[doc = "Edge detect on PAD11"]
    #[inline(always)]
    pub fn is_pad11(&self) -> bool {
        *self == RtcCh1CaptEv::Pad11
    }
    #[doc = "Edge detect on PAD10"]
    #[inline(always)]
    pub fn is_pad10(&self) -> bool {
        *self == RtcCh1CaptEv::Pad10
    }
    #[doc = "Edge detect on PAD9"]
    #[inline(always)]
    pub fn is_pad9(&self) -> bool {
        *self == RtcCh1CaptEv::Pad9
    }
    #[doc = "Edge detect on PAD8"]
    #[inline(always)]
    pub fn is_pad8(&self) -> bool {
        *self == RtcCh1CaptEv::Pad8
    }
    #[doc = "Edge detect on PAD7"]
    #[inline(always)]
    pub fn is_pad7(&self) -> bool {
        *self == RtcCh1CaptEv::Pad7
    }
    #[doc = "Edge detect on PAD6"]
    #[inline(always)]
    pub fn is_pad6(&self) -> bool {
        *self == RtcCh1CaptEv::Pad6
    }
    #[doc = "Edge detect on PAD5"]
    #[inline(always)]
    pub fn is_pad5(&self) -> bool {
        *self == RtcCh1CaptEv::Pad5
    }
    #[doc = "Edge detect on PAD4"]
    #[inline(always)]
    pub fn is_pad4(&self) -> bool {
        *self == RtcCh1CaptEv::Pad4
    }
    #[doc = "Edge detect on PAD3"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == RtcCh1CaptEv::Pad3
    }
    #[doc = "Edge detect on PAD2"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == RtcCh1CaptEv::Pad2
    }
    #[doc = "Edge detect on PAD1"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == RtcCh1CaptEv::Pad1
    }
    #[doc = "Edge detect on PAD0"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == RtcCh1CaptEv::Pad0
    }
}
#[doc = "Field `RTC_CH1_CAPT_EV` writer - 5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
pub type RtcCh1CaptEvW<'a, REG> = crate::FieldWriter<'a, REG, 6, RtcCh1CaptEv>;
impl<'a, REG> RtcCh1CaptEvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::None)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxCompbAsyncN)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxCompbAsync)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::BatmonVolt)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::BatmonTemp)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxTimer1Ev)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxTimer0Ev)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxTdcDone)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxAdcDone)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxCompb)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxCompa)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxSwev2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxSwev1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxSwev0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Jtag)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcUpd)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCombDly)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCh2Dly)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCh1Dly)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCh0Dly)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCh2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCh1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCh0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad)
    }
    #[doc = "Edge detect on PAD31"]
    #[inline(always)]
    pub fn pad31(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad31)
    }
    #[doc = "Edge detect on PAD30"]
    #[inline(always)]
    pub fn pad30(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad30)
    }
    #[doc = "Edge detect on PAD29"]
    #[inline(always)]
    pub fn pad29(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad29)
    }
    #[doc = "Edge detect on PAD28"]
    #[inline(always)]
    pub fn pad28(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad28)
    }
    #[doc = "Edge detect on PAD27"]
    #[inline(always)]
    pub fn pad27(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad27)
    }
    #[doc = "Edge detect on PAD26"]
    #[inline(always)]
    pub fn pad26(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad26)
    }
    #[doc = "Edge detect on PAD25"]
    #[inline(always)]
    pub fn pad25(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad25)
    }
    #[doc = "Edge detect on PAD24"]
    #[inline(always)]
    pub fn pad24(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad24)
    }
    #[doc = "Edge detect on PAD23"]
    #[inline(always)]
    pub fn pad23(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad23)
    }
    #[doc = "Edge detect on PAD22"]
    #[inline(always)]
    pub fn pad22(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad22)
    }
    #[doc = "Edge detect on PAD21"]
    #[inline(always)]
    pub fn pad21(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad21)
    }
    #[doc = "Edge detect on PAD20"]
    #[inline(always)]
    pub fn pad20(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad20)
    }
    #[doc = "Edge detect on PAD19"]
    #[inline(always)]
    pub fn pad19(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad19)
    }
    #[doc = "Edge detect on PAD18"]
    #[inline(always)]
    pub fn pad18(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad18)
    }
    #[doc = "Edge detect on PAD17"]
    #[inline(always)]
    pub fn pad17(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad17)
    }
    #[doc = "Edge detect on PAD16"]
    #[inline(always)]
    pub fn pad16(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad16)
    }
    #[doc = "Edge detect on PAD15"]
    #[inline(always)]
    pub fn pad15(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad15)
    }
    #[doc = "Edge detect on PAD14"]
    #[inline(always)]
    pub fn pad14(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad14)
    }
    #[doc = "Edge detect on PAD13"]
    #[inline(always)]
    pub fn pad13(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad13)
    }
    #[doc = "Edge detect on PAD12"]
    #[inline(always)]
    pub fn pad12(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad12)
    }
    #[doc = "Edge detect on PAD11"]
    #[inline(always)]
    pub fn pad11(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad11)
    }
    #[doc = "Edge detect on PAD10"]
    #[inline(always)]
    pub fn pad10(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad10)
    }
    #[doc = "Edge detect on PAD9"]
    #[inline(always)]
    pub fn pad9(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad9)
    }
    #[doc = "Edge detect on PAD8"]
    #[inline(always)]
    pub fn pad8(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad8)
    }
    #[doc = "Edge detect on PAD7"]
    #[inline(always)]
    pub fn pad7(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad7)
    }
    #[doc = "Edge detect on PAD6"]
    #[inline(always)]
    pub fn pad6(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad6)
    }
    #[doc = "Edge detect on PAD5"]
    #[inline(always)]
    pub fn pad5(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad5)
    }
    #[doc = "Edge detect on PAD4"]
    #[inline(always)]
    pub fn pad4(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad4)
    }
    #[doc = "Edge detect on PAD3"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad3)
    }
    #[doc = "Edge detect on PAD2"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad2)
    }
    #[doc = "Edge detect on PAD1"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad1)
    }
    #[doc = "Edge detect on PAD0"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad0)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
    #[inline(always)]
    pub fn rtc_ch1_capt_ev(&self) -> RtcCh1CaptEvR {
        RtcCh1CaptEvR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ch1_capt_ev(&mut self) -> RtcCh1CaptEvW<RtcselSpec> {
        RtcCh1CaptEvW::new(self, 0)
    }
}
#[doc = "RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcselSpec;
impl crate::RegisterSpec for RtcselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcsel::R`](R) reader structure"]
impl crate::Readable for RtcselSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcsel::W`](W) writer structure"]
impl crate::Writable for RtcselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTCSEL to value 0x3f"]
impl crate::Resettable for RtcselSpec {
    const RESET_VALUE: u32 = 0x3f;
}

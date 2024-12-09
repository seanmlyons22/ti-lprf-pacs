#[doc = "Register `AUXWUSEL` reader"]
pub type R = crate::R<AuxwuselSpec>;
#[doc = "Register `AUXWUSEL` writer"]
pub type W = crate::W<AuxwuselSpec>;
#[doc = "5:0\\]
AUX Wakeup Source #0 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wu0Ev {
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
impl From<Wu0Ev> for u8 {
    #[inline(always)]
    fn from(variant: Wu0Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wu0Ev {
    type Ux = u8;
}
impl crate::IsEnum for Wu0Ev {}
#[doc = "Field `WU0_EV` reader - 5:0\\]
AUX Wakeup Source #0 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
pub type Wu0EvR = crate::FieldReader<Wu0Ev>;
impl Wu0EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wu0Ev> {
        match self.bits {
            63 => Some(Wu0Ev::None),
            56 => Some(Wu0Ev::AuxCompbAsyncN),
            55 => Some(Wu0Ev::AuxCompbAsync),
            54 => Some(Wu0Ev::BatmonVolt),
            53 => Some(Wu0Ev::BatmonTemp),
            52 => Some(Wu0Ev::AuxTimer1Ev),
            51 => Some(Wu0Ev::AuxTimer0Ev),
            50 => Some(Wu0Ev::AuxTdcDone),
            49 => Some(Wu0Ev::AuxAdcDone),
            48 => Some(Wu0Ev::AuxCompb),
            47 => Some(Wu0Ev::AuxCompa),
            46 => Some(Wu0Ev::AuxSwev2),
            45 => Some(Wu0Ev::AuxSwev1),
            44 => Some(Wu0Ev::AuxSwev0),
            43 => Some(Wu0Ev::Jtag),
            42 => Some(Wu0Ev::RtcUpd),
            41 => Some(Wu0Ev::RtcCombDly),
            40 => Some(Wu0Ev::RtcCh2Dly),
            39 => Some(Wu0Ev::RtcCh1Dly),
            38 => Some(Wu0Ev::RtcCh0Dly),
            37 => Some(Wu0Ev::RtcCh2),
            36 => Some(Wu0Ev::RtcCh1),
            35 => Some(Wu0Ev::RtcCh0),
            32 => Some(Wu0Ev::Pad),
            31 => Some(Wu0Ev::Pad31),
            30 => Some(Wu0Ev::Pad30),
            29 => Some(Wu0Ev::Pad29),
            28 => Some(Wu0Ev::Pad28),
            27 => Some(Wu0Ev::Pad27),
            26 => Some(Wu0Ev::Pad26),
            25 => Some(Wu0Ev::Pad25),
            24 => Some(Wu0Ev::Pad24),
            23 => Some(Wu0Ev::Pad23),
            22 => Some(Wu0Ev::Pad22),
            21 => Some(Wu0Ev::Pad21),
            20 => Some(Wu0Ev::Pad20),
            19 => Some(Wu0Ev::Pad19),
            18 => Some(Wu0Ev::Pad18),
            17 => Some(Wu0Ev::Pad17),
            16 => Some(Wu0Ev::Pad16),
            15 => Some(Wu0Ev::Pad15),
            14 => Some(Wu0Ev::Pad14),
            13 => Some(Wu0Ev::Pad13),
            12 => Some(Wu0Ev::Pad12),
            11 => Some(Wu0Ev::Pad11),
            10 => Some(Wu0Ev::Pad10),
            9 => Some(Wu0Ev::Pad9),
            8 => Some(Wu0Ev::Pad8),
            7 => Some(Wu0Ev::Pad7),
            6 => Some(Wu0Ev::Pad6),
            5 => Some(Wu0Ev::Pad5),
            4 => Some(Wu0Ev::Pad4),
            3 => Some(Wu0Ev::Pad3),
            2 => Some(Wu0Ev::Pad2),
            1 => Some(Wu0Ev::Pad1),
            0 => Some(Wu0Ev::Pad0),
            _ => None,
        }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Wu0Ev::None
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == Wu0Ev::AuxCompbAsyncN
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == Wu0Ev::AuxCompbAsync
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == Wu0Ev::BatmonVolt
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == Wu0Ev::BatmonTemp
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == Wu0Ev::AuxTimer1Ev
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == Wu0Ev::AuxTimer0Ev
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == Wu0Ev::AuxTdcDone
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == Wu0Ev::AuxAdcDone
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Wu0Ev::AuxCompb
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Wu0Ev::AuxCompa
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == Wu0Ev::AuxSwev2
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == Wu0Ev::AuxSwev1
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == Wu0Ev::AuxSwev0
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == Wu0Ev::Jtag
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == Wu0Ev::RtcUpd
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == Wu0Ev::RtcCombDly
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == Wu0Ev::RtcCh2Dly
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == Wu0Ev::RtcCh1Dly
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == Wu0Ev::RtcCh0Dly
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == Wu0Ev::RtcCh2
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == Wu0Ev::RtcCh1
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == Wu0Ev::RtcCh0
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == Wu0Ev::Pad
    }
    #[doc = "Edge detect on PAD31"]
    #[inline(always)]
    pub fn is_pad31(&self) -> bool {
        *self == Wu0Ev::Pad31
    }
    #[doc = "Edge detect on PAD30"]
    #[inline(always)]
    pub fn is_pad30(&self) -> bool {
        *self == Wu0Ev::Pad30
    }
    #[doc = "Edge detect on PAD29"]
    #[inline(always)]
    pub fn is_pad29(&self) -> bool {
        *self == Wu0Ev::Pad29
    }
    #[doc = "Edge detect on PAD28"]
    #[inline(always)]
    pub fn is_pad28(&self) -> bool {
        *self == Wu0Ev::Pad28
    }
    #[doc = "Edge detect on PAD27"]
    #[inline(always)]
    pub fn is_pad27(&self) -> bool {
        *self == Wu0Ev::Pad27
    }
    #[doc = "Edge detect on PAD26"]
    #[inline(always)]
    pub fn is_pad26(&self) -> bool {
        *self == Wu0Ev::Pad26
    }
    #[doc = "Edge detect on PAD25"]
    #[inline(always)]
    pub fn is_pad25(&self) -> bool {
        *self == Wu0Ev::Pad25
    }
    #[doc = "Edge detect on PAD24"]
    #[inline(always)]
    pub fn is_pad24(&self) -> bool {
        *self == Wu0Ev::Pad24
    }
    #[doc = "Edge detect on PAD23"]
    #[inline(always)]
    pub fn is_pad23(&self) -> bool {
        *self == Wu0Ev::Pad23
    }
    #[doc = "Edge detect on PAD22"]
    #[inline(always)]
    pub fn is_pad22(&self) -> bool {
        *self == Wu0Ev::Pad22
    }
    #[doc = "Edge detect on PAD21"]
    #[inline(always)]
    pub fn is_pad21(&self) -> bool {
        *self == Wu0Ev::Pad21
    }
    #[doc = "Edge detect on PAD20"]
    #[inline(always)]
    pub fn is_pad20(&self) -> bool {
        *self == Wu0Ev::Pad20
    }
    #[doc = "Edge detect on PAD19"]
    #[inline(always)]
    pub fn is_pad19(&self) -> bool {
        *self == Wu0Ev::Pad19
    }
    #[doc = "Edge detect on PAD18"]
    #[inline(always)]
    pub fn is_pad18(&self) -> bool {
        *self == Wu0Ev::Pad18
    }
    #[doc = "Edge detect on PAD17"]
    #[inline(always)]
    pub fn is_pad17(&self) -> bool {
        *self == Wu0Ev::Pad17
    }
    #[doc = "Edge detect on PAD16"]
    #[inline(always)]
    pub fn is_pad16(&self) -> bool {
        *self == Wu0Ev::Pad16
    }
    #[doc = "Edge detect on PAD15"]
    #[inline(always)]
    pub fn is_pad15(&self) -> bool {
        *self == Wu0Ev::Pad15
    }
    #[doc = "Edge detect on PAD14"]
    #[inline(always)]
    pub fn is_pad14(&self) -> bool {
        *self == Wu0Ev::Pad14
    }
    #[doc = "Edge detect on PAD13"]
    #[inline(always)]
    pub fn is_pad13(&self) -> bool {
        *self == Wu0Ev::Pad13
    }
    #[doc = "Edge detect on PAD12"]
    #[inline(always)]
    pub fn is_pad12(&self) -> bool {
        *self == Wu0Ev::Pad12
    }
    #[doc = "Edge detect on PAD11"]
    #[inline(always)]
    pub fn is_pad11(&self) -> bool {
        *self == Wu0Ev::Pad11
    }
    #[doc = "Edge detect on PAD10"]
    #[inline(always)]
    pub fn is_pad10(&self) -> bool {
        *self == Wu0Ev::Pad10
    }
    #[doc = "Edge detect on PAD9"]
    #[inline(always)]
    pub fn is_pad9(&self) -> bool {
        *self == Wu0Ev::Pad9
    }
    #[doc = "Edge detect on PAD8"]
    #[inline(always)]
    pub fn is_pad8(&self) -> bool {
        *self == Wu0Ev::Pad8
    }
    #[doc = "Edge detect on PAD7"]
    #[inline(always)]
    pub fn is_pad7(&self) -> bool {
        *self == Wu0Ev::Pad7
    }
    #[doc = "Edge detect on PAD6"]
    #[inline(always)]
    pub fn is_pad6(&self) -> bool {
        *self == Wu0Ev::Pad6
    }
    #[doc = "Edge detect on PAD5"]
    #[inline(always)]
    pub fn is_pad5(&self) -> bool {
        *self == Wu0Ev::Pad5
    }
    #[doc = "Edge detect on PAD4"]
    #[inline(always)]
    pub fn is_pad4(&self) -> bool {
        *self == Wu0Ev::Pad4
    }
    #[doc = "Edge detect on PAD3"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == Wu0Ev::Pad3
    }
    #[doc = "Edge detect on PAD2"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == Wu0Ev::Pad2
    }
    #[doc = "Edge detect on PAD1"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == Wu0Ev::Pad1
    }
    #[doc = "Edge detect on PAD0"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == Wu0Ev::Pad0
    }
}
#[doc = "Field `WU0_EV` writer - 5:0\\]
AUX Wakeup Source #0 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
pub type Wu0EvW<'a, REG> = crate::FieldWriter<'a, REG, 6, Wu0Ev>;
impl<'a, REG> Wu0EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::None)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxCompbAsyncN)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxCompbAsync)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::BatmonVolt)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::BatmonTemp)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxTimer1Ev)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxTimer0Ev)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxTdcDone)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxAdcDone)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxCompb)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxCompa)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxSwev2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxSwev1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxSwev0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Jtag)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::RtcUpd)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::RtcCombDly)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::RtcCh2Dly)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::RtcCh1Dly)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::RtcCh0Dly)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::RtcCh2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::RtcCh1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::RtcCh0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad)
    }
    #[doc = "Edge detect on PAD31"]
    #[inline(always)]
    pub fn pad31(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad31)
    }
    #[doc = "Edge detect on PAD30"]
    #[inline(always)]
    pub fn pad30(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad30)
    }
    #[doc = "Edge detect on PAD29"]
    #[inline(always)]
    pub fn pad29(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad29)
    }
    #[doc = "Edge detect on PAD28"]
    #[inline(always)]
    pub fn pad28(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad28)
    }
    #[doc = "Edge detect on PAD27"]
    #[inline(always)]
    pub fn pad27(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad27)
    }
    #[doc = "Edge detect on PAD26"]
    #[inline(always)]
    pub fn pad26(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad26)
    }
    #[doc = "Edge detect on PAD25"]
    #[inline(always)]
    pub fn pad25(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad25)
    }
    #[doc = "Edge detect on PAD24"]
    #[inline(always)]
    pub fn pad24(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad24)
    }
    #[doc = "Edge detect on PAD23"]
    #[inline(always)]
    pub fn pad23(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad23)
    }
    #[doc = "Edge detect on PAD22"]
    #[inline(always)]
    pub fn pad22(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad22)
    }
    #[doc = "Edge detect on PAD21"]
    #[inline(always)]
    pub fn pad21(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad21)
    }
    #[doc = "Edge detect on PAD20"]
    #[inline(always)]
    pub fn pad20(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad20)
    }
    #[doc = "Edge detect on PAD19"]
    #[inline(always)]
    pub fn pad19(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad19)
    }
    #[doc = "Edge detect on PAD18"]
    #[inline(always)]
    pub fn pad18(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad18)
    }
    #[doc = "Edge detect on PAD17"]
    #[inline(always)]
    pub fn pad17(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad17)
    }
    #[doc = "Edge detect on PAD16"]
    #[inline(always)]
    pub fn pad16(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad16)
    }
    #[doc = "Edge detect on PAD15"]
    #[inline(always)]
    pub fn pad15(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad15)
    }
    #[doc = "Edge detect on PAD14"]
    #[inline(always)]
    pub fn pad14(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad14)
    }
    #[doc = "Edge detect on PAD13"]
    #[inline(always)]
    pub fn pad13(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad13)
    }
    #[doc = "Edge detect on PAD12"]
    #[inline(always)]
    pub fn pad12(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad12)
    }
    #[doc = "Edge detect on PAD11"]
    #[inline(always)]
    pub fn pad11(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad11)
    }
    #[doc = "Edge detect on PAD10"]
    #[inline(always)]
    pub fn pad10(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad10)
    }
    #[doc = "Edge detect on PAD9"]
    #[inline(always)]
    pub fn pad9(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad9)
    }
    #[doc = "Edge detect on PAD8"]
    #[inline(always)]
    pub fn pad8(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad8)
    }
    #[doc = "Edge detect on PAD7"]
    #[inline(always)]
    pub fn pad7(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad7)
    }
    #[doc = "Edge detect on PAD6"]
    #[inline(always)]
    pub fn pad6(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad6)
    }
    #[doc = "Edge detect on PAD5"]
    #[inline(always)]
    pub fn pad5(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad5)
    }
    #[doc = "Edge detect on PAD4"]
    #[inline(always)]
    pub fn pad4(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad4)
    }
    #[doc = "Edge detect on PAD3"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad3)
    }
    #[doc = "Edge detect on PAD2"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad2)
    }
    #[doc = "Edge detect on PAD1"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad1)
    }
    #[doc = "Edge detect on PAD0"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::Pad0)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "13:8\\]
AUX Wakeup Source #1 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wu1Ev {
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
impl From<Wu1Ev> for u8 {
    #[inline(always)]
    fn from(variant: Wu1Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wu1Ev {
    type Ux = u8;
}
impl crate::IsEnum for Wu1Ev {}
#[doc = "Field `WU1_EV` reader - 13:8\\]
AUX Wakeup Source #1 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
pub type Wu1EvR = crate::FieldReader<Wu1Ev>;
impl Wu1EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wu1Ev> {
        match self.bits {
            63 => Some(Wu1Ev::None),
            56 => Some(Wu1Ev::AuxCompbAsyncN),
            55 => Some(Wu1Ev::AuxCompbAsync),
            54 => Some(Wu1Ev::BatmonVolt),
            53 => Some(Wu1Ev::BatmonTemp),
            52 => Some(Wu1Ev::AuxTimer1Ev),
            51 => Some(Wu1Ev::AuxTimer0Ev),
            50 => Some(Wu1Ev::AuxTdcDone),
            49 => Some(Wu1Ev::AuxAdcDone),
            48 => Some(Wu1Ev::AuxCompb),
            47 => Some(Wu1Ev::AuxCompa),
            46 => Some(Wu1Ev::AuxSwev2),
            45 => Some(Wu1Ev::AuxSwev1),
            44 => Some(Wu1Ev::AuxSwev0),
            43 => Some(Wu1Ev::Jtag),
            42 => Some(Wu1Ev::RtcUpd),
            41 => Some(Wu1Ev::RtcCombDly),
            40 => Some(Wu1Ev::RtcCh2Dly),
            39 => Some(Wu1Ev::RtcCh1Dly),
            38 => Some(Wu1Ev::RtcCh0Dly),
            37 => Some(Wu1Ev::RtcCh2),
            36 => Some(Wu1Ev::RtcCh1),
            35 => Some(Wu1Ev::RtcCh0),
            32 => Some(Wu1Ev::Pad),
            31 => Some(Wu1Ev::Pad31),
            30 => Some(Wu1Ev::Pad30),
            29 => Some(Wu1Ev::Pad29),
            28 => Some(Wu1Ev::Pad28),
            27 => Some(Wu1Ev::Pad27),
            26 => Some(Wu1Ev::Pad26),
            25 => Some(Wu1Ev::Pad25),
            24 => Some(Wu1Ev::Pad24),
            23 => Some(Wu1Ev::Pad23),
            22 => Some(Wu1Ev::Pad22),
            21 => Some(Wu1Ev::Pad21),
            20 => Some(Wu1Ev::Pad20),
            19 => Some(Wu1Ev::Pad19),
            18 => Some(Wu1Ev::Pad18),
            17 => Some(Wu1Ev::Pad17),
            16 => Some(Wu1Ev::Pad16),
            15 => Some(Wu1Ev::Pad15),
            14 => Some(Wu1Ev::Pad14),
            13 => Some(Wu1Ev::Pad13),
            12 => Some(Wu1Ev::Pad12),
            11 => Some(Wu1Ev::Pad11),
            10 => Some(Wu1Ev::Pad10),
            9 => Some(Wu1Ev::Pad9),
            8 => Some(Wu1Ev::Pad8),
            7 => Some(Wu1Ev::Pad7),
            6 => Some(Wu1Ev::Pad6),
            5 => Some(Wu1Ev::Pad5),
            4 => Some(Wu1Ev::Pad4),
            3 => Some(Wu1Ev::Pad3),
            2 => Some(Wu1Ev::Pad2),
            1 => Some(Wu1Ev::Pad1),
            0 => Some(Wu1Ev::Pad0),
            _ => None,
        }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Wu1Ev::None
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == Wu1Ev::AuxCompbAsyncN
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == Wu1Ev::AuxCompbAsync
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == Wu1Ev::BatmonVolt
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == Wu1Ev::BatmonTemp
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == Wu1Ev::AuxTimer1Ev
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == Wu1Ev::AuxTimer0Ev
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == Wu1Ev::AuxTdcDone
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == Wu1Ev::AuxAdcDone
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Wu1Ev::AuxCompb
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Wu1Ev::AuxCompa
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == Wu1Ev::AuxSwev2
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == Wu1Ev::AuxSwev1
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == Wu1Ev::AuxSwev0
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == Wu1Ev::Jtag
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == Wu1Ev::RtcUpd
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == Wu1Ev::RtcCombDly
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == Wu1Ev::RtcCh2Dly
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == Wu1Ev::RtcCh1Dly
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == Wu1Ev::RtcCh0Dly
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == Wu1Ev::RtcCh2
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == Wu1Ev::RtcCh1
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == Wu1Ev::RtcCh0
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == Wu1Ev::Pad
    }
    #[doc = "Edge detect on PAD31"]
    #[inline(always)]
    pub fn is_pad31(&self) -> bool {
        *self == Wu1Ev::Pad31
    }
    #[doc = "Edge detect on PAD30"]
    #[inline(always)]
    pub fn is_pad30(&self) -> bool {
        *self == Wu1Ev::Pad30
    }
    #[doc = "Edge detect on PAD29"]
    #[inline(always)]
    pub fn is_pad29(&self) -> bool {
        *self == Wu1Ev::Pad29
    }
    #[doc = "Edge detect on PAD28"]
    #[inline(always)]
    pub fn is_pad28(&self) -> bool {
        *self == Wu1Ev::Pad28
    }
    #[doc = "Edge detect on PAD27"]
    #[inline(always)]
    pub fn is_pad27(&self) -> bool {
        *self == Wu1Ev::Pad27
    }
    #[doc = "Edge detect on PAD26"]
    #[inline(always)]
    pub fn is_pad26(&self) -> bool {
        *self == Wu1Ev::Pad26
    }
    #[doc = "Edge detect on PAD25"]
    #[inline(always)]
    pub fn is_pad25(&self) -> bool {
        *self == Wu1Ev::Pad25
    }
    #[doc = "Edge detect on PAD24"]
    #[inline(always)]
    pub fn is_pad24(&self) -> bool {
        *self == Wu1Ev::Pad24
    }
    #[doc = "Edge detect on PAD23"]
    #[inline(always)]
    pub fn is_pad23(&self) -> bool {
        *self == Wu1Ev::Pad23
    }
    #[doc = "Edge detect on PAD22"]
    #[inline(always)]
    pub fn is_pad22(&self) -> bool {
        *self == Wu1Ev::Pad22
    }
    #[doc = "Edge detect on PAD21"]
    #[inline(always)]
    pub fn is_pad21(&self) -> bool {
        *self == Wu1Ev::Pad21
    }
    #[doc = "Edge detect on PAD20"]
    #[inline(always)]
    pub fn is_pad20(&self) -> bool {
        *self == Wu1Ev::Pad20
    }
    #[doc = "Edge detect on PAD19"]
    #[inline(always)]
    pub fn is_pad19(&self) -> bool {
        *self == Wu1Ev::Pad19
    }
    #[doc = "Edge detect on PAD18"]
    #[inline(always)]
    pub fn is_pad18(&self) -> bool {
        *self == Wu1Ev::Pad18
    }
    #[doc = "Edge detect on PAD17"]
    #[inline(always)]
    pub fn is_pad17(&self) -> bool {
        *self == Wu1Ev::Pad17
    }
    #[doc = "Edge detect on PAD16"]
    #[inline(always)]
    pub fn is_pad16(&self) -> bool {
        *self == Wu1Ev::Pad16
    }
    #[doc = "Edge detect on PAD15"]
    #[inline(always)]
    pub fn is_pad15(&self) -> bool {
        *self == Wu1Ev::Pad15
    }
    #[doc = "Edge detect on PAD14"]
    #[inline(always)]
    pub fn is_pad14(&self) -> bool {
        *self == Wu1Ev::Pad14
    }
    #[doc = "Edge detect on PAD13"]
    #[inline(always)]
    pub fn is_pad13(&self) -> bool {
        *self == Wu1Ev::Pad13
    }
    #[doc = "Edge detect on PAD12"]
    #[inline(always)]
    pub fn is_pad12(&self) -> bool {
        *self == Wu1Ev::Pad12
    }
    #[doc = "Edge detect on PAD11"]
    #[inline(always)]
    pub fn is_pad11(&self) -> bool {
        *self == Wu1Ev::Pad11
    }
    #[doc = "Edge detect on PAD10"]
    #[inline(always)]
    pub fn is_pad10(&self) -> bool {
        *self == Wu1Ev::Pad10
    }
    #[doc = "Edge detect on PAD9"]
    #[inline(always)]
    pub fn is_pad9(&self) -> bool {
        *self == Wu1Ev::Pad9
    }
    #[doc = "Edge detect on PAD8"]
    #[inline(always)]
    pub fn is_pad8(&self) -> bool {
        *self == Wu1Ev::Pad8
    }
    #[doc = "Edge detect on PAD7"]
    #[inline(always)]
    pub fn is_pad7(&self) -> bool {
        *self == Wu1Ev::Pad7
    }
    #[doc = "Edge detect on PAD6"]
    #[inline(always)]
    pub fn is_pad6(&self) -> bool {
        *self == Wu1Ev::Pad6
    }
    #[doc = "Edge detect on PAD5"]
    #[inline(always)]
    pub fn is_pad5(&self) -> bool {
        *self == Wu1Ev::Pad5
    }
    #[doc = "Edge detect on PAD4"]
    #[inline(always)]
    pub fn is_pad4(&self) -> bool {
        *self == Wu1Ev::Pad4
    }
    #[doc = "Edge detect on PAD3"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == Wu1Ev::Pad3
    }
    #[doc = "Edge detect on PAD2"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == Wu1Ev::Pad2
    }
    #[doc = "Edge detect on PAD1"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == Wu1Ev::Pad1
    }
    #[doc = "Edge detect on PAD0"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == Wu1Ev::Pad0
    }
}
#[doc = "Field `WU1_EV` writer - 13:8\\]
AUX Wakeup Source #1 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
pub type Wu1EvW<'a, REG> = crate::FieldWriter<'a, REG, 6, Wu1Ev>;
impl<'a, REG> Wu1EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::None)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxCompbAsyncN)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxCompbAsync)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::BatmonVolt)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::BatmonTemp)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxTimer1Ev)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxTimer0Ev)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxTdcDone)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxAdcDone)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxCompb)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxCompa)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxSwev2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxSwev1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxSwev0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Jtag)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::RtcUpd)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::RtcCombDly)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::RtcCh2Dly)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::RtcCh1Dly)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::RtcCh0Dly)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::RtcCh2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::RtcCh1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::RtcCh0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad)
    }
    #[doc = "Edge detect on PAD31"]
    #[inline(always)]
    pub fn pad31(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad31)
    }
    #[doc = "Edge detect on PAD30"]
    #[inline(always)]
    pub fn pad30(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad30)
    }
    #[doc = "Edge detect on PAD29"]
    #[inline(always)]
    pub fn pad29(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad29)
    }
    #[doc = "Edge detect on PAD28"]
    #[inline(always)]
    pub fn pad28(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad28)
    }
    #[doc = "Edge detect on PAD27"]
    #[inline(always)]
    pub fn pad27(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad27)
    }
    #[doc = "Edge detect on PAD26"]
    #[inline(always)]
    pub fn pad26(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad26)
    }
    #[doc = "Edge detect on PAD25"]
    #[inline(always)]
    pub fn pad25(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad25)
    }
    #[doc = "Edge detect on PAD24"]
    #[inline(always)]
    pub fn pad24(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad24)
    }
    #[doc = "Edge detect on PAD23"]
    #[inline(always)]
    pub fn pad23(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad23)
    }
    #[doc = "Edge detect on PAD22"]
    #[inline(always)]
    pub fn pad22(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad22)
    }
    #[doc = "Edge detect on PAD21"]
    #[inline(always)]
    pub fn pad21(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad21)
    }
    #[doc = "Edge detect on PAD20"]
    #[inline(always)]
    pub fn pad20(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad20)
    }
    #[doc = "Edge detect on PAD19"]
    #[inline(always)]
    pub fn pad19(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad19)
    }
    #[doc = "Edge detect on PAD18"]
    #[inline(always)]
    pub fn pad18(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad18)
    }
    #[doc = "Edge detect on PAD17"]
    #[inline(always)]
    pub fn pad17(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad17)
    }
    #[doc = "Edge detect on PAD16"]
    #[inline(always)]
    pub fn pad16(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad16)
    }
    #[doc = "Edge detect on PAD15"]
    #[inline(always)]
    pub fn pad15(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad15)
    }
    #[doc = "Edge detect on PAD14"]
    #[inline(always)]
    pub fn pad14(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad14)
    }
    #[doc = "Edge detect on PAD13"]
    #[inline(always)]
    pub fn pad13(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad13)
    }
    #[doc = "Edge detect on PAD12"]
    #[inline(always)]
    pub fn pad12(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad12)
    }
    #[doc = "Edge detect on PAD11"]
    #[inline(always)]
    pub fn pad11(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad11)
    }
    #[doc = "Edge detect on PAD10"]
    #[inline(always)]
    pub fn pad10(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad10)
    }
    #[doc = "Edge detect on PAD9"]
    #[inline(always)]
    pub fn pad9(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad9)
    }
    #[doc = "Edge detect on PAD8"]
    #[inline(always)]
    pub fn pad8(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad8)
    }
    #[doc = "Edge detect on PAD7"]
    #[inline(always)]
    pub fn pad7(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad7)
    }
    #[doc = "Edge detect on PAD6"]
    #[inline(always)]
    pub fn pad6(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad6)
    }
    #[doc = "Edge detect on PAD5"]
    #[inline(always)]
    pub fn pad5(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad5)
    }
    #[doc = "Edge detect on PAD4"]
    #[inline(always)]
    pub fn pad4(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad4)
    }
    #[doc = "Edge detect on PAD3"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad3)
    }
    #[doc = "Edge detect on PAD2"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad2)
    }
    #[doc = "Edge detect on PAD1"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad1)
    }
    #[doc = "Edge detect on PAD0"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::Pad0)
    }
}
#[doc = "Field `RESERVED14` reader - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader;
#[doc = "21:16\\]
AUX Wakeup Source #2 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wu2Ev {
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
impl From<Wu2Ev> for u8 {
    #[inline(always)]
    fn from(variant: Wu2Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wu2Ev {
    type Ux = u8;
}
impl crate::IsEnum for Wu2Ev {}
#[doc = "Field `WU2_EV` reader - 21:16\\]
AUX Wakeup Source #2 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
pub type Wu2EvR = crate::FieldReader<Wu2Ev>;
impl Wu2EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wu2Ev> {
        match self.bits {
            63 => Some(Wu2Ev::None),
            56 => Some(Wu2Ev::AuxCompbAsyncN),
            55 => Some(Wu2Ev::AuxCompbAsync),
            54 => Some(Wu2Ev::BatmonVolt),
            53 => Some(Wu2Ev::BatmonTemp),
            52 => Some(Wu2Ev::AuxTimer1Ev),
            51 => Some(Wu2Ev::AuxTimer0Ev),
            50 => Some(Wu2Ev::AuxTdcDone),
            49 => Some(Wu2Ev::AuxAdcDone),
            48 => Some(Wu2Ev::AuxCompb),
            47 => Some(Wu2Ev::AuxCompa),
            46 => Some(Wu2Ev::AuxSwev2),
            45 => Some(Wu2Ev::AuxSwev1),
            44 => Some(Wu2Ev::AuxSwev0),
            43 => Some(Wu2Ev::Jtag),
            42 => Some(Wu2Ev::RtcUpd),
            41 => Some(Wu2Ev::RtcCombDly),
            40 => Some(Wu2Ev::RtcCh2Dly),
            39 => Some(Wu2Ev::RtcCh1Dly),
            38 => Some(Wu2Ev::RtcCh0Dly),
            37 => Some(Wu2Ev::RtcCh2),
            36 => Some(Wu2Ev::RtcCh1),
            35 => Some(Wu2Ev::RtcCh0),
            32 => Some(Wu2Ev::Pad),
            31 => Some(Wu2Ev::Pad31),
            30 => Some(Wu2Ev::Pad30),
            29 => Some(Wu2Ev::Pad29),
            28 => Some(Wu2Ev::Pad28),
            27 => Some(Wu2Ev::Pad27),
            26 => Some(Wu2Ev::Pad26),
            25 => Some(Wu2Ev::Pad25),
            24 => Some(Wu2Ev::Pad24),
            23 => Some(Wu2Ev::Pad23),
            22 => Some(Wu2Ev::Pad22),
            21 => Some(Wu2Ev::Pad21),
            20 => Some(Wu2Ev::Pad20),
            19 => Some(Wu2Ev::Pad19),
            18 => Some(Wu2Ev::Pad18),
            17 => Some(Wu2Ev::Pad17),
            16 => Some(Wu2Ev::Pad16),
            15 => Some(Wu2Ev::Pad15),
            14 => Some(Wu2Ev::Pad14),
            13 => Some(Wu2Ev::Pad13),
            12 => Some(Wu2Ev::Pad12),
            11 => Some(Wu2Ev::Pad11),
            10 => Some(Wu2Ev::Pad10),
            9 => Some(Wu2Ev::Pad9),
            8 => Some(Wu2Ev::Pad8),
            7 => Some(Wu2Ev::Pad7),
            6 => Some(Wu2Ev::Pad6),
            5 => Some(Wu2Ev::Pad5),
            4 => Some(Wu2Ev::Pad4),
            3 => Some(Wu2Ev::Pad3),
            2 => Some(Wu2Ev::Pad2),
            1 => Some(Wu2Ev::Pad1),
            0 => Some(Wu2Ev::Pad0),
            _ => None,
        }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Wu2Ev::None
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == Wu2Ev::AuxCompbAsyncN
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == Wu2Ev::AuxCompbAsync
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == Wu2Ev::BatmonVolt
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == Wu2Ev::BatmonTemp
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == Wu2Ev::AuxTimer1Ev
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == Wu2Ev::AuxTimer0Ev
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == Wu2Ev::AuxTdcDone
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == Wu2Ev::AuxAdcDone
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Wu2Ev::AuxCompb
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Wu2Ev::AuxCompa
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == Wu2Ev::AuxSwev2
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == Wu2Ev::AuxSwev1
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == Wu2Ev::AuxSwev0
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == Wu2Ev::Jtag
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == Wu2Ev::RtcUpd
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == Wu2Ev::RtcCombDly
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == Wu2Ev::RtcCh2Dly
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == Wu2Ev::RtcCh1Dly
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == Wu2Ev::RtcCh0Dly
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == Wu2Ev::RtcCh2
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == Wu2Ev::RtcCh1
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == Wu2Ev::RtcCh0
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == Wu2Ev::Pad
    }
    #[doc = "Edge detect on PAD31"]
    #[inline(always)]
    pub fn is_pad31(&self) -> bool {
        *self == Wu2Ev::Pad31
    }
    #[doc = "Edge detect on PAD30"]
    #[inline(always)]
    pub fn is_pad30(&self) -> bool {
        *self == Wu2Ev::Pad30
    }
    #[doc = "Edge detect on PAD29"]
    #[inline(always)]
    pub fn is_pad29(&self) -> bool {
        *self == Wu2Ev::Pad29
    }
    #[doc = "Edge detect on PAD28"]
    #[inline(always)]
    pub fn is_pad28(&self) -> bool {
        *self == Wu2Ev::Pad28
    }
    #[doc = "Edge detect on PAD27"]
    #[inline(always)]
    pub fn is_pad27(&self) -> bool {
        *self == Wu2Ev::Pad27
    }
    #[doc = "Edge detect on PAD26"]
    #[inline(always)]
    pub fn is_pad26(&self) -> bool {
        *self == Wu2Ev::Pad26
    }
    #[doc = "Edge detect on PAD25"]
    #[inline(always)]
    pub fn is_pad25(&self) -> bool {
        *self == Wu2Ev::Pad25
    }
    #[doc = "Edge detect on PAD24"]
    #[inline(always)]
    pub fn is_pad24(&self) -> bool {
        *self == Wu2Ev::Pad24
    }
    #[doc = "Edge detect on PAD23"]
    #[inline(always)]
    pub fn is_pad23(&self) -> bool {
        *self == Wu2Ev::Pad23
    }
    #[doc = "Edge detect on PAD22"]
    #[inline(always)]
    pub fn is_pad22(&self) -> bool {
        *self == Wu2Ev::Pad22
    }
    #[doc = "Edge detect on PAD21"]
    #[inline(always)]
    pub fn is_pad21(&self) -> bool {
        *self == Wu2Ev::Pad21
    }
    #[doc = "Edge detect on PAD20"]
    #[inline(always)]
    pub fn is_pad20(&self) -> bool {
        *self == Wu2Ev::Pad20
    }
    #[doc = "Edge detect on PAD19"]
    #[inline(always)]
    pub fn is_pad19(&self) -> bool {
        *self == Wu2Ev::Pad19
    }
    #[doc = "Edge detect on PAD18"]
    #[inline(always)]
    pub fn is_pad18(&self) -> bool {
        *self == Wu2Ev::Pad18
    }
    #[doc = "Edge detect on PAD17"]
    #[inline(always)]
    pub fn is_pad17(&self) -> bool {
        *self == Wu2Ev::Pad17
    }
    #[doc = "Edge detect on PAD16"]
    #[inline(always)]
    pub fn is_pad16(&self) -> bool {
        *self == Wu2Ev::Pad16
    }
    #[doc = "Edge detect on PAD15"]
    #[inline(always)]
    pub fn is_pad15(&self) -> bool {
        *self == Wu2Ev::Pad15
    }
    #[doc = "Edge detect on PAD14"]
    #[inline(always)]
    pub fn is_pad14(&self) -> bool {
        *self == Wu2Ev::Pad14
    }
    #[doc = "Edge detect on PAD13"]
    #[inline(always)]
    pub fn is_pad13(&self) -> bool {
        *self == Wu2Ev::Pad13
    }
    #[doc = "Edge detect on PAD12"]
    #[inline(always)]
    pub fn is_pad12(&self) -> bool {
        *self == Wu2Ev::Pad12
    }
    #[doc = "Edge detect on PAD11"]
    #[inline(always)]
    pub fn is_pad11(&self) -> bool {
        *self == Wu2Ev::Pad11
    }
    #[doc = "Edge detect on PAD10"]
    #[inline(always)]
    pub fn is_pad10(&self) -> bool {
        *self == Wu2Ev::Pad10
    }
    #[doc = "Edge detect on PAD9"]
    #[inline(always)]
    pub fn is_pad9(&self) -> bool {
        *self == Wu2Ev::Pad9
    }
    #[doc = "Edge detect on PAD8"]
    #[inline(always)]
    pub fn is_pad8(&self) -> bool {
        *self == Wu2Ev::Pad8
    }
    #[doc = "Edge detect on PAD7"]
    #[inline(always)]
    pub fn is_pad7(&self) -> bool {
        *self == Wu2Ev::Pad7
    }
    #[doc = "Edge detect on PAD6"]
    #[inline(always)]
    pub fn is_pad6(&self) -> bool {
        *self == Wu2Ev::Pad6
    }
    #[doc = "Edge detect on PAD5"]
    #[inline(always)]
    pub fn is_pad5(&self) -> bool {
        *self == Wu2Ev::Pad5
    }
    #[doc = "Edge detect on PAD4"]
    #[inline(always)]
    pub fn is_pad4(&self) -> bool {
        *self == Wu2Ev::Pad4
    }
    #[doc = "Edge detect on PAD3"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == Wu2Ev::Pad3
    }
    #[doc = "Edge detect on PAD2"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == Wu2Ev::Pad2
    }
    #[doc = "Edge detect on PAD1"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == Wu2Ev::Pad1
    }
    #[doc = "Edge detect on PAD0"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == Wu2Ev::Pad0
    }
}
#[doc = "Field `WU2_EV` writer - 21:16\\]
AUX Wakeup Source #2 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
pub type Wu2EvW<'a, REG> = crate::FieldWriter<'a, REG, 6, Wu2Ev>;
impl<'a, REG> Wu2EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::None)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxCompbAsyncN)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxCompbAsync)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::BatmonVolt)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::BatmonTemp)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxTimer1Ev)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxTimer0Ev)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxTdcDone)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxAdcDone)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxCompb)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxCompa)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxSwev2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxSwev1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxSwev0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Jtag)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::RtcUpd)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::RtcCombDly)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::RtcCh2Dly)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::RtcCh1Dly)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::RtcCh0Dly)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::RtcCh2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::RtcCh1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::RtcCh0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad)
    }
    #[doc = "Edge detect on PAD31"]
    #[inline(always)]
    pub fn pad31(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad31)
    }
    #[doc = "Edge detect on PAD30"]
    #[inline(always)]
    pub fn pad30(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad30)
    }
    #[doc = "Edge detect on PAD29"]
    #[inline(always)]
    pub fn pad29(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad29)
    }
    #[doc = "Edge detect on PAD28"]
    #[inline(always)]
    pub fn pad28(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad28)
    }
    #[doc = "Edge detect on PAD27"]
    #[inline(always)]
    pub fn pad27(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad27)
    }
    #[doc = "Edge detect on PAD26"]
    #[inline(always)]
    pub fn pad26(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad26)
    }
    #[doc = "Edge detect on PAD25"]
    #[inline(always)]
    pub fn pad25(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad25)
    }
    #[doc = "Edge detect on PAD24"]
    #[inline(always)]
    pub fn pad24(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad24)
    }
    #[doc = "Edge detect on PAD23"]
    #[inline(always)]
    pub fn pad23(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad23)
    }
    #[doc = "Edge detect on PAD22"]
    #[inline(always)]
    pub fn pad22(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad22)
    }
    #[doc = "Edge detect on PAD21"]
    #[inline(always)]
    pub fn pad21(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad21)
    }
    #[doc = "Edge detect on PAD20"]
    #[inline(always)]
    pub fn pad20(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad20)
    }
    #[doc = "Edge detect on PAD19"]
    #[inline(always)]
    pub fn pad19(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad19)
    }
    #[doc = "Edge detect on PAD18"]
    #[inline(always)]
    pub fn pad18(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad18)
    }
    #[doc = "Edge detect on PAD17"]
    #[inline(always)]
    pub fn pad17(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad17)
    }
    #[doc = "Edge detect on PAD16"]
    #[inline(always)]
    pub fn pad16(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad16)
    }
    #[doc = "Edge detect on PAD15"]
    #[inline(always)]
    pub fn pad15(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad15)
    }
    #[doc = "Edge detect on PAD14"]
    #[inline(always)]
    pub fn pad14(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad14)
    }
    #[doc = "Edge detect on PAD13"]
    #[inline(always)]
    pub fn pad13(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad13)
    }
    #[doc = "Edge detect on PAD12"]
    #[inline(always)]
    pub fn pad12(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad12)
    }
    #[doc = "Edge detect on PAD11"]
    #[inline(always)]
    pub fn pad11(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad11)
    }
    #[doc = "Edge detect on PAD10"]
    #[inline(always)]
    pub fn pad10(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad10)
    }
    #[doc = "Edge detect on PAD9"]
    #[inline(always)]
    pub fn pad9(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad9)
    }
    #[doc = "Edge detect on PAD8"]
    #[inline(always)]
    pub fn pad8(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad8)
    }
    #[doc = "Edge detect on PAD7"]
    #[inline(always)]
    pub fn pad7(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad7)
    }
    #[doc = "Edge detect on PAD6"]
    #[inline(always)]
    pub fn pad6(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad6)
    }
    #[doc = "Edge detect on PAD5"]
    #[inline(always)]
    pub fn pad5(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad5)
    }
    #[doc = "Edge detect on PAD4"]
    #[inline(always)]
    pub fn pad4(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad4)
    }
    #[doc = "Edge detect on PAD3"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad3)
    }
    #[doc = "Edge detect on PAD2"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad2)
    }
    #[doc = "Edge detect on PAD1"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad1)
    }
    #[doc = "Edge detect on PAD0"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::Pad0)
    }
}
#[doc = "Field `RESERVED22` reader - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved22R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
AUX Wakeup Source #0 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu0_ev(&self) -> Wu0EvR {
        Wu0EvR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
AUX Wakeup Source #1 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu1_ev(&self) -> Wu1EvR {
        Wu1EvR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
AUX Wakeup Source #2 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu2_ev(&self) -> Wu2EvR {
        Wu2EvR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> Reserved22R {
        Reserved22R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
AUX Wakeup Source #0 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu0_ev(&mut self) -> Wu0EvW<AuxwuselSpec> {
        Wu0EvW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
AUX Wakeup Source #1 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu1_ev(&mut self) -> Wu1EvW<AuxwuselSpec> {
        Wu1EvW::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
AUX Wakeup Source #2 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu2_ev(&mut self) -> Wu2EvW<AuxwuselSpec> {
        Wu2EvW::new(self, 16)
    }
}
#[doc = "Wake-up Selector For AUX This register contains pointers to 3 events which are routed to AON_WUC as wakeup sources for AUX. AON_WUC will start a wakeup sequence for the AUX domain when either of the 3 selected events are asserted. A wakeup sequence will guarantee that the AUX power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for AUX. Note: It is recommended ( or required when AON_WUC:AUXCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before AUX is requesting powerdown. ( AUX_WUC:PWRDWNREQ.REQ is asserted\\]
) as it will speed up the wakeup procedure.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxwusel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxwusel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxwuselSpec;
impl crate::RegisterSpec for AuxwuselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxwusel::R`](R) reader structure"]
impl crate::Readable for AuxwuselSpec {}
#[doc = "`write(|w| ..)` method takes [`auxwusel::W`](W) writer structure"]
impl crate::Writable for AuxwuselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUXWUSEL to value 0x003f_3f3f"]
impl crate::Resettable for AuxwuselSpec {
    const RESET_VALUE: u32 = 0x003f_3f3f;
}

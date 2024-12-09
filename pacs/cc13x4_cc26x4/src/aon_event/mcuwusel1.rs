#[doc = "Register `MCUWUSEL1` reader"]
pub type R = crate::R<Mcuwusel1Spec>;
#[doc = "Register `MCUWUSEL1` writer"]
pub type W = crate::W<Mcuwusel1Spec>;
#[doc = "5:0\\]
MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wu4Ev {
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
    #[doc = "9: Combined event from BATMON"]
    BatmonCombined = 9,
    #[doc = "8: BATMON event: Temperature level below lower limit"]
    BatmonTempLl = 8,
    #[doc = "7: BATMON event: Temperature level above upper limit"]
    BatmonTempUl = 7,
    #[doc = "6: BATMON event: Battery level below lower limit"]
    BatmonBattLl = 6,
    #[doc = "5: BATMON event: Battery level above upper limit"]
    BatmonBattUl = 5,
    #[doc = "4: Event 3 from AUX TImer2"]
    AuxTimer2Ev3 = 4,
    #[doc = "3: Event 2 from AUX TImer2"]
    AuxTimer2Ev2 = 3,
    #[doc = "2: Event 1 from AUX TImer2"]
    AuxTimer2Ev1 = 2,
    #[doc = "1: Event 0 from AUX TImer2"]
    AuxTimer2Ev0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IoevMcuWu = 0,
}
impl From<Wu4Ev> for u8 {
    #[inline(always)]
    fn from(variant: Wu4Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wu4Ev {
    type Ux = u8;
}
impl crate::IsEnum for Wu4Ev {}
#[doc = "Field `WU4_EV` reader - 5:0\\]
MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type Wu4EvR = crate::FieldReader<Wu4Ev>;
impl Wu4EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wu4Ev> {
        match self.bits {
            63 => Some(Wu4Ev::None),
            56 => Some(Wu4Ev::AuxCompbAsyncN),
            55 => Some(Wu4Ev::AuxCompbAsync),
            54 => Some(Wu4Ev::BatmonVolt),
            53 => Some(Wu4Ev::BatmonTemp),
            52 => Some(Wu4Ev::AuxTimer1Ev),
            51 => Some(Wu4Ev::AuxTimer0Ev),
            50 => Some(Wu4Ev::AuxTdcDone),
            49 => Some(Wu4Ev::AuxAdcDone),
            48 => Some(Wu4Ev::AuxCompb),
            47 => Some(Wu4Ev::AuxCompa),
            46 => Some(Wu4Ev::AuxSwev2),
            45 => Some(Wu4Ev::AuxSwev1),
            44 => Some(Wu4Ev::AuxSwev0),
            43 => Some(Wu4Ev::Jtag),
            42 => Some(Wu4Ev::RtcUpd),
            41 => Some(Wu4Ev::RtcCombDly),
            40 => Some(Wu4Ev::RtcCh2Dly),
            39 => Some(Wu4Ev::RtcCh1Dly),
            38 => Some(Wu4Ev::RtcCh0Dly),
            37 => Some(Wu4Ev::RtcCh2),
            36 => Some(Wu4Ev::RtcCh1),
            35 => Some(Wu4Ev::RtcCh0),
            32 => Some(Wu4Ev::Pad),
            9 => Some(Wu4Ev::BatmonCombined),
            8 => Some(Wu4Ev::BatmonTempLl),
            7 => Some(Wu4Ev::BatmonTempUl),
            6 => Some(Wu4Ev::BatmonBattLl),
            5 => Some(Wu4Ev::BatmonBattUl),
            4 => Some(Wu4Ev::AuxTimer2Ev3),
            3 => Some(Wu4Ev::AuxTimer2Ev2),
            2 => Some(Wu4Ev::AuxTimer2Ev1),
            1 => Some(Wu4Ev::AuxTimer2Ev0),
            0 => Some(Wu4Ev::IoevMcuWu),
            _ => None,
        }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Wu4Ev::None
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == Wu4Ev::AuxCompbAsyncN
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == Wu4Ev::AuxCompbAsync
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == Wu4Ev::BatmonVolt
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == Wu4Ev::BatmonTemp
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == Wu4Ev::AuxTimer1Ev
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == Wu4Ev::AuxTimer0Ev
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == Wu4Ev::AuxTdcDone
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == Wu4Ev::AuxAdcDone
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Wu4Ev::AuxCompb
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Wu4Ev::AuxCompa
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == Wu4Ev::AuxSwev2
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == Wu4Ev::AuxSwev1
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == Wu4Ev::AuxSwev0
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == Wu4Ev::Jtag
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == Wu4Ev::RtcUpd
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == Wu4Ev::RtcCombDly
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == Wu4Ev::RtcCh2Dly
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == Wu4Ev::RtcCh1Dly
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == Wu4Ev::RtcCh0Dly
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == Wu4Ev::RtcCh2
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == Wu4Ev::RtcCh1
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == Wu4Ev::RtcCh0
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == Wu4Ev::Pad
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == Wu4Ev::BatmonCombined
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == Wu4Ev::BatmonTempLl
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == Wu4Ev::BatmonTempUl
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == Wu4Ev::BatmonBattLl
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == Wu4Ev::BatmonBattUl
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == Wu4Ev::AuxTimer2Ev3
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == Wu4Ev::AuxTimer2Ev2
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == Wu4Ev::AuxTimer2Ev1
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == Wu4Ev::AuxTimer2Ev0
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == Wu4Ev::IoevMcuWu
    }
}
#[doc = "Field `WU4_EV` writer - 5:0\\]
MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type Wu4EvW<'a, REG> = crate::FieldWriter<'a, REG, 6, Wu4Ev>;
impl<'a, REG> Wu4EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::None)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxCompbAsyncN)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxCompbAsync)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::BatmonVolt)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::BatmonTemp)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxTimer1Ev)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxTimer0Ev)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxTdcDone)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxAdcDone)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxCompb)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxCompa)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxSwev2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxSwev1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxSwev0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::Jtag)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::RtcUpd)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::RtcCombDly)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::RtcCh2Dly)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::RtcCh1Dly)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::RtcCh0Dly)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::RtcCh2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::RtcCh1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::RtcCh0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::Pad)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::BatmonCombined)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::BatmonTempLl)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::BatmonTempUl)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::BatmonBattLl)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::BatmonBattUl)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxTimer2Ev3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxTimer2Ev2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxTimer2Ev1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::AuxTimer2Ev0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut crate::W<REG> {
        self.variant(Wu4Ev::IoevMcuWu)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "13:8\\]
MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wu5Ev {
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
    #[doc = "9: Combined event from BATMON"]
    BatmonCombined = 9,
    #[doc = "8: BATMON event: Temperature level below lower limit"]
    BatmonTempLl = 8,
    #[doc = "7: BATMON event: Temperature level above upper limit"]
    BatmonTempUl = 7,
    #[doc = "6: BATMON event: Battery level below lower limit"]
    BatmonBattLl = 6,
    #[doc = "5: BATMON event: Battery level above upper limit"]
    BatmonBattUl = 5,
    #[doc = "4: Event 3 from AUX TImer2"]
    AuxTimer2Ev3 = 4,
    #[doc = "3: Event 2 from AUX TImer2"]
    AuxTimer2Ev2 = 3,
    #[doc = "2: Event 1 from AUX TImer2"]
    AuxTimer2Ev1 = 2,
    #[doc = "1: Event 0 from AUX TImer2"]
    AuxTimer2Ev0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IoevMcuWu = 0,
}
impl From<Wu5Ev> for u8 {
    #[inline(always)]
    fn from(variant: Wu5Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wu5Ev {
    type Ux = u8;
}
impl crate::IsEnum for Wu5Ev {}
#[doc = "Field `WU5_EV` reader - 13:8\\]
MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type Wu5EvR = crate::FieldReader<Wu5Ev>;
impl Wu5EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wu5Ev> {
        match self.bits {
            63 => Some(Wu5Ev::None),
            56 => Some(Wu5Ev::AuxCompbAsyncN),
            55 => Some(Wu5Ev::AuxCompbAsync),
            54 => Some(Wu5Ev::BatmonVolt),
            53 => Some(Wu5Ev::BatmonTemp),
            52 => Some(Wu5Ev::AuxTimer1Ev),
            51 => Some(Wu5Ev::AuxTimer0Ev),
            50 => Some(Wu5Ev::AuxTdcDone),
            49 => Some(Wu5Ev::AuxAdcDone),
            48 => Some(Wu5Ev::AuxCompb),
            47 => Some(Wu5Ev::AuxCompa),
            46 => Some(Wu5Ev::AuxSwev2),
            45 => Some(Wu5Ev::AuxSwev1),
            44 => Some(Wu5Ev::AuxSwev0),
            43 => Some(Wu5Ev::Jtag),
            42 => Some(Wu5Ev::RtcUpd),
            41 => Some(Wu5Ev::RtcCombDly),
            40 => Some(Wu5Ev::RtcCh2Dly),
            39 => Some(Wu5Ev::RtcCh1Dly),
            38 => Some(Wu5Ev::RtcCh0Dly),
            37 => Some(Wu5Ev::RtcCh2),
            36 => Some(Wu5Ev::RtcCh1),
            35 => Some(Wu5Ev::RtcCh0),
            32 => Some(Wu5Ev::Pad),
            9 => Some(Wu5Ev::BatmonCombined),
            8 => Some(Wu5Ev::BatmonTempLl),
            7 => Some(Wu5Ev::BatmonTempUl),
            6 => Some(Wu5Ev::BatmonBattLl),
            5 => Some(Wu5Ev::BatmonBattUl),
            4 => Some(Wu5Ev::AuxTimer2Ev3),
            3 => Some(Wu5Ev::AuxTimer2Ev2),
            2 => Some(Wu5Ev::AuxTimer2Ev1),
            1 => Some(Wu5Ev::AuxTimer2Ev0),
            0 => Some(Wu5Ev::IoevMcuWu),
            _ => None,
        }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Wu5Ev::None
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == Wu5Ev::AuxCompbAsyncN
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == Wu5Ev::AuxCompbAsync
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == Wu5Ev::BatmonVolt
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == Wu5Ev::BatmonTemp
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == Wu5Ev::AuxTimer1Ev
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == Wu5Ev::AuxTimer0Ev
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == Wu5Ev::AuxTdcDone
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == Wu5Ev::AuxAdcDone
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Wu5Ev::AuxCompb
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Wu5Ev::AuxCompa
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == Wu5Ev::AuxSwev2
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == Wu5Ev::AuxSwev1
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == Wu5Ev::AuxSwev0
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == Wu5Ev::Jtag
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == Wu5Ev::RtcUpd
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == Wu5Ev::RtcCombDly
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == Wu5Ev::RtcCh2Dly
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == Wu5Ev::RtcCh1Dly
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == Wu5Ev::RtcCh0Dly
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == Wu5Ev::RtcCh2
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == Wu5Ev::RtcCh1
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == Wu5Ev::RtcCh0
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == Wu5Ev::Pad
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == Wu5Ev::BatmonCombined
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == Wu5Ev::BatmonTempLl
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == Wu5Ev::BatmonTempUl
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == Wu5Ev::BatmonBattLl
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == Wu5Ev::BatmonBattUl
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == Wu5Ev::AuxTimer2Ev3
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == Wu5Ev::AuxTimer2Ev2
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == Wu5Ev::AuxTimer2Ev1
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == Wu5Ev::AuxTimer2Ev0
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == Wu5Ev::IoevMcuWu
    }
}
#[doc = "Field `WU5_EV` writer - 13:8\\]
MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type Wu5EvW<'a, REG> = crate::FieldWriter<'a, REG, 6, Wu5Ev>;
impl<'a, REG> Wu5EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::None)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxCompbAsyncN)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxCompbAsync)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::BatmonVolt)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::BatmonTemp)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxTimer1Ev)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxTimer0Ev)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxTdcDone)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxAdcDone)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxCompb)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxCompa)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxSwev2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxSwev1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxSwev0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::Jtag)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::RtcUpd)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::RtcCombDly)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::RtcCh2Dly)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::RtcCh1Dly)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::RtcCh0Dly)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::RtcCh2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::RtcCh1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::RtcCh0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::Pad)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::BatmonCombined)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::BatmonTempLl)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::BatmonTempUl)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::BatmonBattLl)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::BatmonBattUl)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxTimer2Ev3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxTimer2Ev2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxTimer2Ev1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::AuxTimer2Ev0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut crate::W<REG> {
        self.variant(Wu5Ev::IoevMcuWu)
    }
}
#[doc = "Field `RESERVED14` reader - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader;
#[doc = "21:16\\]
MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wu6Ev {
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
    #[doc = "9: Combined event from BATMON"]
    BatmonCombined = 9,
    #[doc = "8: BATMON event: Temperature level below lower limit"]
    BatmonTempLl = 8,
    #[doc = "7: BATMON event: Temperature level above upper limit"]
    BatmonTempUl = 7,
    #[doc = "6: BATMON event: Battery level below lower limit"]
    BatmonBattLl = 6,
    #[doc = "5: BATMON event: Battery level above upper limit"]
    BatmonBattUl = 5,
    #[doc = "4: Event 3 from AUX TImer2"]
    AuxTimer2Ev3 = 4,
    #[doc = "3: Event 2 from AUX TImer2"]
    AuxTimer2Ev2 = 3,
    #[doc = "2: Event 1 from AUX TImer2"]
    AuxTimer2Ev1 = 2,
    #[doc = "1: Event 0 from AUX TImer2"]
    AuxTimer2Ev0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IoevMcuWu = 0,
}
impl From<Wu6Ev> for u8 {
    #[inline(always)]
    fn from(variant: Wu6Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wu6Ev {
    type Ux = u8;
}
impl crate::IsEnum for Wu6Ev {}
#[doc = "Field `WU6_EV` reader - 21:16\\]
MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type Wu6EvR = crate::FieldReader<Wu6Ev>;
impl Wu6EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wu6Ev> {
        match self.bits {
            63 => Some(Wu6Ev::None),
            56 => Some(Wu6Ev::AuxCompbAsyncN),
            55 => Some(Wu6Ev::AuxCompbAsync),
            54 => Some(Wu6Ev::BatmonVolt),
            53 => Some(Wu6Ev::BatmonTemp),
            52 => Some(Wu6Ev::AuxTimer1Ev),
            51 => Some(Wu6Ev::AuxTimer0Ev),
            50 => Some(Wu6Ev::AuxTdcDone),
            49 => Some(Wu6Ev::AuxAdcDone),
            48 => Some(Wu6Ev::AuxCompb),
            47 => Some(Wu6Ev::AuxCompa),
            46 => Some(Wu6Ev::AuxSwev2),
            45 => Some(Wu6Ev::AuxSwev1),
            44 => Some(Wu6Ev::AuxSwev0),
            43 => Some(Wu6Ev::Jtag),
            42 => Some(Wu6Ev::RtcUpd),
            41 => Some(Wu6Ev::RtcCombDly),
            40 => Some(Wu6Ev::RtcCh2Dly),
            39 => Some(Wu6Ev::RtcCh1Dly),
            38 => Some(Wu6Ev::RtcCh0Dly),
            37 => Some(Wu6Ev::RtcCh2),
            36 => Some(Wu6Ev::RtcCh1),
            35 => Some(Wu6Ev::RtcCh0),
            32 => Some(Wu6Ev::Pad),
            9 => Some(Wu6Ev::BatmonCombined),
            8 => Some(Wu6Ev::BatmonTempLl),
            7 => Some(Wu6Ev::BatmonTempUl),
            6 => Some(Wu6Ev::BatmonBattLl),
            5 => Some(Wu6Ev::BatmonBattUl),
            4 => Some(Wu6Ev::AuxTimer2Ev3),
            3 => Some(Wu6Ev::AuxTimer2Ev2),
            2 => Some(Wu6Ev::AuxTimer2Ev1),
            1 => Some(Wu6Ev::AuxTimer2Ev0),
            0 => Some(Wu6Ev::IoevMcuWu),
            _ => None,
        }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Wu6Ev::None
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == Wu6Ev::AuxCompbAsyncN
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == Wu6Ev::AuxCompbAsync
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == Wu6Ev::BatmonVolt
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == Wu6Ev::BatmonTemp
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == Wu6Ev::AuxTimer1Ev
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == Wu6Ev::AuxTimer0Ev
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == Wu6Ev::AuxTdcDone
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == Wu6Ev::AuxAdcDone
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Wu6Ev::AuxCompb
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Wu6Ev::AuxCompa
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == Wu6Ev::AuxSwev2
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == Wu6Ev::AuxSwev1
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == Wu6Ev::AuxSwev0
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == Wu6Ev::Jtag
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == Wu6Ev::RtcUpd
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == Wu6Ev::RtcCombDly
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == Wu6Ev::RtcCh2Dly
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == Wu6Ev::RtcCh1Dly
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == Wu6Ev::RtcCh0Dly
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == Wu6Ev::RtcCh2
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == Wu6Ev::RtcCh1
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == Wu6Ev::RtcCh0
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == Wu6Ev::Pad
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == Wu6Ev::BatmonCombined
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == Wu6Ev::BatmonTempLl
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == Wu6Ev::BatmonTempUl
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == Wu6Ev::BatmonBattLl
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == Wu6Ev::BatmonBattUl
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == Wu6Ev::AuxTimer2Ev3
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == Wu6Ev::AuxTimer2Ev2
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == Wu6Ev::AuxTimer2Ev1
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == Wu6Ev::AuxTimer2Ev0
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == Wu6Ev::IoevMcuWu
    }
}
#[doc = "Field `WU6_EV` writer - 21:16\\]
MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type Wu6EvW<'a, REG> = crate::FieldWriter<'a, REG, 6, Wu6Ev>;
impl<'a, REG> Wu6EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::None)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxCompbAsyncN)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxCompbAsync)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::BatmonVolt)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::BatmonTemp)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxTimer1Ev)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxTimer0Ev)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxTdcDone)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxAdcDone)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxCompb)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxCompa)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxSwev2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxSwev1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxSwev0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::Jtag)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::RtcUpd)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::RtcCombDly)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::RtcCh2Dly)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::RtcCh1Dly)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::RtcCh0Dly)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::RtcCh2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::RtcCh1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::RtcCh0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::Pad)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::BatmonCombined)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::BatmonTempLl)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::BatmonTempUl)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::BatmonBattLl)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::BatmonBattUl)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxTimer2Ev3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxTimer2Ev2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxTimer2Ev1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::AuxTimer2Ev0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut crate::W<REG> {
        self.variant(Wu6Ev::IoevMcuWu)
    }
}
#[doc = "Field `RESERVED22` reader - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved22R = crate::FieldReader;
#[doc = "29:24\\]
MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wu7Ev {
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
    #[doc = "9: Combined event from BATMON"]
    BatmonCombined = 9,
    #[doc = "8: BATMON event: Temperature level below lower limit"]
    BatmonTempLl = 8,
    #[doc = "7: BATMON event: Temperature level above upper limit"]
    BatmonTempUl = 7,
    #[doc = "6: BATMON event: Battery level below lower limit"]
    BatmonBattLl = 6,
    #[doc = "5: BATMON event: Battery level above upper limit"]
    BatmonBattUl = 5,
    #[doc = "4: Event 3 from AUX TImer2"]
    AuxTimer2Ev3 = 4,
    #[doc = "3: Event 2 from AUX TImer2"]
    AuxTimer2Ev2 = 3,
    #[doc = "2: Event 1 from AUX TImer2"]
    AuxTimer2Ev1 = 2,
    #[doc = "1: Event 0 from AUX TImer2"]
    AuxTimer2Ev0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IoevMcuWu = 0,
}
impl From<Wu7Ev> for u8 {
    #[inline(always)]
    fn from(variant: Wu7Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wu7Ev {
    type Ux = u8;
}
impl crate::IsEnum for Wu7Ev {}
#[doc = "Field `WU7_EV` reader - 29:24\\]
MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type Wu7EvR = crate::FieldReader<Wu7Ev>;
impl Wu7EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wu7Ev> {
        match self.bits {
            63 => Some(Wu7Ev::None),
            56 => Some(Wu7Ev::AuxCompbAsyncN),
            55 => Some(Wu7Ev::AuxCompbAsync),
            54 => Some(Wu7Ev::BatmonVolt),
            53 => Some(Wu7Ev::BatmonTemp),
            52 => Some(Wu7Ev::AuxTimer1Ev),
            51 => Some(Wu7Ev::AuxTimer0Ev),
            50 => Some(Wu7Ev::AuxTdcDone),
            49 => Some(Wu7Ev::AuxAdcDone),
            48 => Some(Wu7Ev::AuxCompb),
            47 => Some(Wu7Ev::AuxCompa),
            46 => Some(Wu7Ev::AuxSwev2),
            45 => Some(Wu7Ev::AuxSwev1),
            44 => Some(Wu7Ev::AuxSwev0),
            43 => Some(Wu7Ev::Jtag),
            42 => Some(Wu7Ev::RtcUpd),
            41 => Some(Wu7Ev::RtcCombDly),
            40 => Some(Wu7Ev::RtcCh2Dly),
            39 => Some(Wu7Ev::RtcCh1Dly),
            38 => Some(Wu7Ev::RtcCh0Dly),
            37 => Some(Wu7Ev::RtcCh2),
            36 => Some(Wu7Ev::RtcCh1),
            35 => Some(Wu7Ev::RtcCh0),
            32 => Some(Wu7Ev::Pad),
            9 => Some(Wu7Ev::BatmonCombined),
            8 => Some(Wu7Ev::BatmonTempLl),
            7 => Some(Wu7Ev::BatmonTempUl),
            6 => Some(Wu7Ev::BatmonBattLl),
            5 => Some(Wu7Ev::BatmonBattUl),
            4 => Some(Wu7Ev::AuxTimer2Ev3),
            3 => Some(Wu7Ev::AuxTimer2Ev2),
            2 => Some(Wu7Ev::AuxTimer2Ev1),
            1 => Some(Wu7Ev::AuxTimer2Ev0),
            0 => Some(Wu7Ev::IoevMcuWu),
            _ => None,
        }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Wu7Ev::None
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == Wu7Ev::AuxCompbAsyncN
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == Wu7Ev::AuxCompbAsync
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == Wu7Ev::BatmonVolt
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == Wu7Ev::BatmonTemp
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == Wu7Ev::AuxTimer1Ev
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == Wu7Ev::AuxTimer0Ev
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == Wu7Ev::AuxTdcDone
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == Wu7Ev::AuxAdcDone
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Wu7Ev::AuxCompb
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Wu7Ev::AuxCompa
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == Wu7Ev::AuxSwev2
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == Wu7Ev::AuxSwev1
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == Wu7Ev::AuxSwev0
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == Wu7Ev::Jtag
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == Wu7Ev::RtcUpd
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == Wu7Ev::RtcCombDly
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == Wu7Ev::RtcCh2Dly
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == Wu7Ev::RtcCh1Dly
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == Wu7Ev::RtcCh0Dly
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == Wu7Ev::RtcCh2
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == Wu7Ev::RtcCh1
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == Wu7Ev::RtcCh0
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == Wu7Ev::Pad
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == Wu7Ev::BatmonCombined
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == Wu7Ev::BatmonTempLl
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == Wu7Ev::BatmonTempUl
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == Wu7Ev::BatmonBattLl
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == Wu7Ev::BatmonBattUl
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == Wu7Ev::AuxTimer2Ev3
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == Wu7Ev::AuxTimer2Ev2
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == Wu7Ev::AuxTimer2Ev1
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == Wu7Ev::AuxTimer2Ev0
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == Wu7Ev::IoevMcuWu
    }
}
#[doc = "Field `WU7_EV` writer - 29:24\\]
MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type Wu7EvW<'a, REG> = crate::FieldWriter<'a, REG, 6, Wu7Ev>;
impl<'a, REG> Wu7EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::None)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxCompbAsyncN)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxCompbAsync)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::BatmonVolt)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::BatmonTemp)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxTimer1Ev)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxTimer0Ev)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxTdcDone)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxAdcDone)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxCompb)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxCompa)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxSwev2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxSwev1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxSwev0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::Jtag)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::RtcUpd)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::RtcCombDly)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::RtcCh2Dly)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::RtcCh1Dly)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::RtcCh0Dly)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::RtcCh2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::RtcCh1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::RtcCh0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::Pad)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::BatmonCombined)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::BatmonTempLl)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::BatmonTempUl)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::BatmonBattLl)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::BatmonBattUl)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxTimer2Ev3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxTimer2Ev2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxTimer2Ev1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::AuxTimer2Ev0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut crate::W<REG> {
        self.variant(Wu7Ev::IoevMcuWu)
    }
}
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu4_ev(&self) -> Wu4EvR {
        Wu4EvR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu5_ev(&self) -> Wu5EvR {
        Wu5EvR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu6_ev(&self) -> Wu6EvR {
        Wu6EvR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> Reserved22R {
        Reserved22R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu7_ev(&self) -> Wu7EvR {
        Wu7EvR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> Reserved30R {
        Reserved30R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu4_ev(&mut self) -> Wu4EvW<Mcuwusel1Spec> {
        Wu4EvW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu5_ev(&mut self) -> Wu5EvW<Mcuwusel1Spec> {
        Wu5EvW::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu6_ev(&mut self) -> Wu6EvW<Mcuwusel1Spec> {
        Wu6EvW::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu7_ev(&mut self) -> Wu7EvW<Mcuwusel1Spec> {
        Wu7EvW::new(self, 24)
    }
}
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 4 to 7) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcuwusel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcuwusel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcuwusel1Spec;
impl crate::RegisterSpec for Mcuwusel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcuwusel1::R`](R) reader structure"]
impl crate::Readable for Mcuwusel1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcuwusel1::W`](W) writer structure"]
impl crate::Writable for Mcuwusel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCUWUSEL1 to value 0x3f3f_3f3f"]
impl crate::Resettable for Mcuwusel1Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}

#[doc = "Register `MCUWUSEL` reader"]
pub type R = crate::R<McuwuselSpec>;
#[doc = "Register `MCUWUSEL` writer"]
pub type W = crate::W<McuwuselSpec>;
#[doc = "5:0\\]
MCU Wakeup Source #0 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
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
MCU Wakeup Source #0 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
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
            9 => Some(Wu0Ev::BatmonCombined),
            8 => Some(Wu0Ev::BatmonTempLl),
            7 => Some(Wu0Ev::BatmonTempUl),
            6 => Some(Wu0Ev::BatmonBattLl),
            5 => Some(Wu0Ev::BatmonBattUl),
            4 => Some(Wu0Ev::AuxTimer2Ev3),
            3 => Some(Wu0Ev::AuxTimer2Ev2),
            2 => Some(Wu0Ev::AuxTimer2Ev1),
            1 => Some(Wu0Ev::AuxTimer2Ev0),
            0 => Some(Wu0Ev::IoevMcuWu),
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
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == Wu0Ev::BatmonCombined
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == Wu0Ev::BatmonTempLl
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == Wu0Ev::BatmonTempUl
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == Wu0Ev::BatmonBattLl
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == Wu0Ev::BatmonBattUl
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == Wu0Ev::AuxTimer2Ev3
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == Wu0Ev::AuxTimer2Ev2
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == Wu0Ev::AuxTimer2Ev1
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == Wu0Ev::AuxTimer2Ev0
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == Wu0Ev::IoevMcuWu
    }
}
#[doc = "Field `WU0_EV` writer - 5:0\\]
MCU Wakeup Source #0 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
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
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::BatmonCombined)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::BatmonTempLl)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::BatmonTempUl)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::BatmonBattLl)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::BatmonBattUl)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxTimer2Ev3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxTimer2Ev2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxTimer2Ev1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::AuxTimer2Ev0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut crate::W<REG> {
        self.variant(Wu0Ev::IoevMcuWu)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "13:8\\]
MCU Wakeup Source #1 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
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
MCU Wakeup Source #1 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
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
            9 => Some(Wu1Ev::BatmonCombined),
            8 => Some(Wu1Ev::BatmonTempLl),
            7 => Some(Wu1Ev::BatmonTempUl),
            6 => Some(Wu1Ev::BatmonBattLl),
            5 => Some(Wu1Ev::BatmonBattUl),
            4 => Some(Wu1Ev::AuxTimer2Ev3),
            3 => Some(Wu1Ev::AuxTimer2Ev2),
            2 => Some(Wu1Ev::AuxTimer2Ev1),
            1 => Some(Wu1Ev::AuxTimer2Ev0),
            0 => Some(Wu1Ev::IoevMcuWu),
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
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == Wu1Ev::BatmonCombined
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == Wu1Ev::BatmonTempLl
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == Wu1Ev::BatmonTempUl
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == Wu1Ev::BatmonBattLl
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == Wu1Ev::BatmonBattUl
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == Wu1Ev::AuxTimer2Ev3
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == Wu1Ev::AuxTimer2Ev2
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == Wu1Ev::AuxTimer2Ev1
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == Wu1Ev::AuxTimer2Ev0
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == Wu1Ev::IoevMcuWu
    }
}
#[doc = "Field `WU1_EV` writer - 13:8\\]
MCU Wakeup Source #1 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
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
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::BatmonCombined)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::BatmonTempLl)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::BatmonTempUl)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::BatmonBattLl)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::BatmonBattUl)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxTimer2Ev3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxTimer2Ev2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxTimer2Ev1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::AuxTimer2Ev0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut crate::W<REG> {
        self.variant(Wu1Ev::IoevMcuWu)
    }
}
#[doc = "Field `RESERVED14` reader - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader;
#[doc = "Field `RESERVED14` writer - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "21:16\\]
MCU Wakeup Source #2 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
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
MCU Wakeup Source #2 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
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
            9 => Some(Wu2Ev::BatmonCombined),
            8 => Some(Wu2Ev::BatmonTempLl),
            7 => Some(Wu2Ev::BatmonTempUl),
            6 => Some(Wu2Ev::BatmonBattLl),
            5 => Some(Wu2Ev::BatmonBattUl),
            4 => Some(Wu2Ev::AuxTimer2Ev3),
            3 => Some(Wu2Ev::AuxTimer2Ev2),
            2 => Some(Wu2Ev::AuxTimer2Ev1),
            1 => Some(Wu2Ev::AuxTimer2Ev0),
            0 => Some(Wu2Ev::IoevMcuWu),
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
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == Wu2Ev::BatmonCombined
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == Wu2Ev::BatmonTempLl
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == Wu2Ev::BatmonTempUl
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == Wu2Ev::BatmonBattLl
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == Wu2Ev::BatmonBattUl
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == Wu2Ev::AuxTimer2Ev3
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == Wu2Ev::AuxTimer2Ev2
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == Wu2Ev::AuxTimer2Ev1
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == Wu2Ev::AuxTimer2Ev0
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == Wu2Ev::IoevMcuWu
    }
}
#[doc = "Field `WU2_EV` writer - 21:16\\]
MCU Wakeup Source #2 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
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
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::BatmonCombined)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::BatmonTempLl)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::BatmonTempUl)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::BatmonBattLl)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::BatmonBattUl)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxTimer2Ev3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxTimer2Ev2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxTimer2Ev1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::AuxTimer2Ev0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut crate::W<REG> {
        self.variant(Wu2Ev::IoevMcuWu)
    }
}
#[doc = "Field `RESERVED22` reader - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved22R = crate::FieldReader;
#[doc = "Field `RESERVED22` writer - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved22W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "29:24\\]
MCU Wakeup Source #3 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wu3Ev {
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
impl From<Wu3Ev> for u8 {
    #[inline(always)]
    fn from(variant: Wu3Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wu3Ev {
    type Ux = u8;
}
impl crate::IsEnum for Wu3Ev {}
#[doc = "Field `WU3_EV` reader - 29:24\\]
MCU Wakeup Source #3 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type Wu3EvR = crate::FieldReader<Wu3Ev>;
impl Wu3EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wu3Ev> {
        match self.bits {
            63 => Some(Wu3Ev::None),
            56 => Some(Wu3Ev::AuxCompbAsyncN),
            55 => Some(Wu3Ev::AuxCompbAsync),
            54 => Some(Wu3Ev::BatmonVolt),
            53 => Some(Wu3Ev::BatmonTemp),
            52 => Some(Wu3Ev::AuxTimer1Ev),
            51 => Some(Wu3Ev::AuxTimer0Ev),
            50 => Some(Wu3Ev::AuxTdcDone),
            49 => Some(Wu3Ev::AuxAdcDone),
            48 => Some(Wu3Ev::AuxCompb),
            47 => Some(Wu3Ev::AuxCompa),
            46 => Some(Wu3Ev::AuxSwev2),
            45 => Some(Wu3Ev::AuxSwev1),
            44 => Some(Wu3Ev::AuxSwev0),
            43 => Some(Wu3Ev::Jtag),
            42 => Some(Wu3Ev::RtcUpd),
            41 => Some(Wu3Ev::RtcCombDly),
            40 => Some(Wu3Ev::RtcCh2Dly),
            39 => Some(Wu3Ev::RtcCh1Dly),
            38 => Some(Wu3Ev::RtcCh0Dly),
            37 => Some(Wu3Ev::RtcCh2),
            36 => Some(Wu3Ev::RtcCh1),
            35 => Some(Wu3Ev::RtcCh0),
            32 => Some(Wu3Ev::Pad),
            9 => Some(Wu3Ev::BatmonCombined),
            8 => Some(Wu3Ev::BatmonTempLl),
            7 => Some(Wu3Ev::BatmonTempUl),
            6 => Some(Wu3Ev::BatmonBattLl),
            5 => Some(Wu3Ev::BatmonBattUl),
            4 => Some(Wu3Ev::AuxTimer2Ev3),
            3 => Some(Wu3Ev::AuxTimer2Ev2),
            2 => Some(Wu3Ev::AuxTimer2Ev1),
            1 => Some(Wu3Ev::AuxTimer2Ev0),
            0 => Some(Wu3Ev::IoevMcuWu),
            _ => None,
        }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Wu3Ev::None
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == Wu3Ev::AuxCompbAsyncN
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == Wu3Ev::AuxCompbAsync
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == Wu3Ev::BatmonVolt
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == Wu3Ev::BatmonTemp
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == Wu3Ev::AuxTimer1Ev
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == Wu3Ev::AuxTimer0Ev
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == Wu3Ev::AuxTdcDone
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == Wu3Ev::AuxAdcDone
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Wu3Ev::AuxCompb
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Wu3Ev::AuxCompa
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == Wu3Ev::AuxSwev2
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == Wu3Ev::AuxSwev1
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == Wu3Ev::AuxSwev0
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == Wu3Ev::Jtag
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == Wu3Ev::RtcUpd
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == Wu3Ev::RtcCombDly
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == Wu3Ev::RtcCh2Dly
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == Wu3Ev::RtcCh1Dly
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == Wu3Ev::RtcCh0Dly
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == Wu3Ev::RtcCh2
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == Wu3Ev::RtcCh1
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == Wu3Ev::RtcCh0
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == Wu3Ev::Pad
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == Wu3Ev::BatmonCombined
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == Wu3Ev::BatmonTempLl
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == Wu3Ev::BatmonTempUl
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == Wu3Ev::BatmonBattLl
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == Wu3Ev::BatmonBattUl
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == Wu3Ev::AuxTimer2Ev3
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == Wu3Ev::AuxTimer2Ev2
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == Wu3Ev::AuxTimer2Ev1
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == Wu3Ev::AuxTimer2Ev0
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == Wu3Ev::IoevMcuWu
    }
}
#[doc = "Field `WU3_EV` writer - 29:24\\]
MCU Wakeup Source #3 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
pub type Wu3EvW<'a, REG> = crate::FieldWriter<'a, REG, 6, Wu3Ev>;
impl<'a, REG> Wu3EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::None)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxCompbAsyncN)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxCompbAsync)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::BatmonVolt)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::BatmonTemp)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxTimer1Ev)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxTimer0Ev)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxTdcDone)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxAdcDone)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxCompb)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxCompa)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxSwev2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxSwev1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxSwev0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::Jtag)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::RtcUpd)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::RtcCombDly)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::RtcCh2Dly)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::RtcCh1Dly)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::RtcCh0Dly)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::RtcCh2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::RtcCh1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::RtcCh0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::Pad)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::BatmonCombined)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::BatmonTempLl)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::BatmonTempUl)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::BatmonBattLl)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::BatmonBattUl)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxTimer2Ev3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxTimer2Ev2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxTimer2Ev1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::AuxTimer2Ev0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut crate::W<REG> {
        self.variant(Wu3Ev::IoevMcuWu)
    }
}
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30R = crate::FieldReader;
#[doc = "Field `RESERVED30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
MCU Wakeup Source #0 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
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
MCU Wakeup Source #1 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
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
MCU Wakeup Source #2 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu2_ev(&self) -> Wu2EvR {
        Wu2EvR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> Reserved22R {
        Reserved22R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
MCU Wakeup Source #3 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu3_ev(&self) -> Wu3EvR {
        Wu3EvR::new(((self.bits >> 24) & 0x3f) as u8)
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
MCU Wakeup Source #0 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu0_ev(&mut self) -> Wu0EvW<McuwuselSpec> {
        Wu0EvW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<McuwuselSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bits 8:13 - 13:8\\]
MCU Wakeup Source #1 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu1_ev(&mut self) -> Wu1EvW<McuwuselSpec> {
        Wu1EvW::new(self, 8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<McuwuselSpec> {
        Reserved14W::new(self, 14)
    }
    #[doc = "Bits 16:21 - 21:16\\]
MCU Wakeup Source #2 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu2_ev(&mut self) -> Wu2EvW<McuwuselSpec> {
        Wu2EvW::new(self, 16)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> Reserved22W<McuwuselSpec> {
        Reserved22W::new(self, 22)
    }
    #[doc = "Bits 24:29 - 29:24\\]
MCU Wakeup Source #3 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    #[must_use]
    pub fn wu3_ev(&mut self) -> Wu3EvW<McuwuselSpec> {
        Wu3EvW::new(self, 24)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved30(&mut self) -> Reserved30W<McuwuselSpec> {
        Reserved30W::new(self, 30)
    }
}
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 0 to 3) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcuwusel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcuwusel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McuwuselSpec;
impl crate::RegisterSpec for McuwuselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcuwusel::R`](R) reader structure"]
impl crate::Readable for McuwuselSpec {}
#[doc = "`write(|w| ..)` method takes [`mcuwusel::W`](W) writer structure"]
impl crate::Writable for McuwuselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCUWUSEL to value 0x3f3f_3f3f"]
impl crate::Resettable for McuwuselSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}

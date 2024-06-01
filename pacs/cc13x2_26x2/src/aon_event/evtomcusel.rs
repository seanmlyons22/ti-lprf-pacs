#[doc = "Register `EVTOMCUSEL` reader"]
pub type R = crate::R<EvtomcuselSpec>;
#[doc = "Register `EVTOMCUSEL` writer"]
pub type W = crate::W<EvtomcuselSpec>;
#[doc = "5:0\\]
Event selector for AON_PROG0 event. AON Event Source id# selecting event routed to EVENT as AON_PROG0 event.\n\nValue on reset: 43"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AonProg0Ev {
    #[doc = "63: 0"]
    None = 63,
    #[doc = "56: 0"]
    AuxCompbAsyncN = 56,
    #[doc = "55: 0"]
    AuxCompbAsync = 55,
    #[doc = "54: 0"]
    BatmonVolt = 54,
    #[doc = "53: 0"]
    BatmonTemp = 53,
    #[doc = "52: 0"]
    AuxTimer1Ev = 52,
    #[doc = "51: 0"]
    AuxTimer0Ev = 51,
    #[doc = "50: 0"]
    AuxTdcDone = 50,
    #[doc = "49: 0"]
    AuxAdcDone = 49,
    #[doc = "48: 0"]
    AuxCompb = 48,
    #[doc = "47: 0"]
    AuxCompa = 47,
    #[doc = "46: 0"]
    AuxSwev2 = 46,
    #[doc = "45: 0"]
    AuxSwev1 = 45,
    #[doc = "44: 0"]
    AuxSwev0 = 44,
    #[doc = "43: 0"]
    Jtag = 43,
    #[doc = "42: 0"]
    RtcUpd = 42,
    #[doc = "41: 0"]
    RtcCombDly = 41,
    #[doc = "40: 0"]
    RtcCh2Dly = 40,
    #[doc = "39: 0"]
    RtcCh1Dly = 39,
    #[doc = "38: 0"]
    RtcCh0Dly = 38,
    #[doc = "37: 0"]
    RtcCh2 = 37,
    #[doc = "36: 0"]
    RtcCh1 = 36,
    #[doc = "35: 0"]
    RtcCh0 = 35,
    #[doc = "32: 0"]
    Pad = 32,
    #[doc = "9: 0"]
    BatmonCombined = 9,
    #[doc = "8: 0"]
    BatmonTempLl = 8,
    #[doc = "7: 0"]
    BatmonTempUl = 7,
    #[doc = "6: 0"]
    BatmonBattLl = 6,
    #[doc = "5: 0"]
    BatmonBattUl = 5,
    #[doc = "4: 0"]
    AuxTimer2Ev3 = 4,
    #[doc = "3: 0"]
    AuxTimer2Ev2 = 3,
    #[doc = "2: 0"]
    AuxTimer2Ev1 = 2,
    #[doc = "1: 0"]
    AuxTimer2Ev0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG0 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG0_EN\\]"]
    IoevAonProg0 = 0,
}
impl From<AonProg0Ev> for u8 {
    #[inline(always)]
    fn from(variant: AonProg0Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AonProg0Ev {
    type Ux = u8;
}
impl crate::IsEnum for AonProg0Ev {}
#[doc = "Field `AON_PROG0_EV` reader - 5:0\\]
Event selector for AON_PROG0 event. AON Event Source id# selecting event routed to EVENT as AON_PROG0 event."]
pub type AonProg0EvR = crate::FieldReader<AonProg0Ev>;
impl AonProg0EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AonProg0Ev> {
        match self.bits {
            63 => Some(AonProg0Ev::None),
            56 => Some(AonProg0Ev::AuxCompbAsyncN),
            55 => Some(AonProg0Ev::AuxCompbAsync),
            54 => Some(AonProg0Ev::BatmonVolt),
            53 => Some(AonProg0Ev::BatmonTemp),
            52 => Some(AonProg0Ev::AuxTimer1Ev),
            51 => Some(AonProg0Ev::AuxTimer0Ev),
            50 => Some(AonProg0Ev::AuxTdcDone),
            49 => Some(AonProg0Ev::AuxAdcDone),
            48 => Some(AonProg0Ev::AuxCompb),
            47 => Some(AonProg0Ev::AuxCompa),
            46 => Some(AonProg0Ev::AuxSwev2),
            45 => Some(AonProg0Ev::AuxSwev1),
            44 => Some(AonProg0Ev::AuxSwev0),
            43 => Some(AonProg0Ev::Jtag),
            42 => Some(AonProg0Ev::RtcUpd),
            41 => Some(AonProg0Ev::RtcCombDly),
            40 => Some(AonProg0Ev::RtcCh2Dly),
            39 => Some(AonProg0Ev::RtcCh1Dly),
            38 => Some(AonProg0Ev::RtcCh0Dly),
            37 => Some(AonProg0Ev::RtcCh2),
            36 => Some(AonProg0Ev::RtcCh1),
            35 => Some(AonProg0Ev::RtcCh0),
            32 => Some(AonProg0Ev::Pad),
            9 => Some(AonProg0Ev::BatmonCombined),
            8 => Some(AonProg0Ev::BatmonTempLl),
            7 => Some(AonProg0Ev::BatmonTempUl),
            6 => Some(AonProg0Ev::BatmonBattLl),
            5 => Some(AonProg0Ev::BatmonBattUl),
            4 => Some(AonProg0Ev::AuxTimer2Ev3),
            3 => Some(AonProg0Ev::AuxTimer2Ev2),
            2 => Some(AonProg0Ev::AuxTimer2Ev1),
            1 => Some(AonProg0Ev::AuxTimer2Ev0),
            0 => Some(AonProg0Ev::IoevAonProg0),
            _ => None,
        }
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AonProg0Ev::None
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == AonProg0Ev::AuxCompbAsyncN
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == AonProg0Ev::AuxCompbAsync
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == AonProg0Ev::BatmonVolt
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == AonProg0Ev::BatmonTemp
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == AonProg0Ev::AuxTimer1Ev
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == AonProg0Ev::AuxTimer0Ev
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == AonProg0Ev::AuxTdcDone
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == AonProg0Ev::AuxAdcDone
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == AonProg0Ev::AuxCompb
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == AonProg0Ev::AuxCompa
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == AonProg0Ev::AuxSwev2
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == AonProg0Ev::AuxSwev1
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == AonProg0Ev::AuxSwev0
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == AonProg0Ev::Jtag
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == AonProg0Ev::RtcUpd
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == AonProg0Ev::RtcCombDly
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == AonProg0Ev::RtcCh2Dly
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == AonProg0Ev::RtcCh1Dly
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == AonProg0Ev::RtcCh0Dly
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == AonProg0Ev::RtcCh2
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == AonProg0Ev::RtcCh1
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == AonProg0Ev::RtcCh0
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == AonProg0Ev::Pad
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == AonProg0Ev::BatmonCombined
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == AonProg0Ev::BatmonTempLl
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == AonProg0Ev::BatmonTempUl
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == AonProg0Ev::BatmonBattLl
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == AonProg0Ev::BatmonBattUl
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == AonProg0Ev::AuxTimer2Ev3
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == AonProg0Ev::AuxTimer2Ev2
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == AonProg0Ev::AuxTimer2Ev1
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == AonProg0Ev::AuxTimer2Ev0
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG0 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG0_EN\\]"]
    #[inline(always)]
    pub fn is_ioev_aon_prog0(&self) -> bool {
        *self == AonProg0Ev::IoevAonProg0
    }
}
#[doc = "Field `AON_PROG0_EV` writer - 5:0\\]
Event selector for AON_PROG0 event. AON Event Source id# selecting event routed to EVENT as AON_PROG0 event."]
pub type AonProg0EvW<'a, REG> = crate::FieldWriter<'a, REG, 6, AonProg0Ev>;
impl<'a, REG> AonProg0EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::None)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxCompbAsyncN)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxCompbAsync)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::BatmonVolt)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::BatmonTemp)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxTimer1Ev)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxTimer0Ev)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxTdcDone)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxAdcDone)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxCompb)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxCompa)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxSwev2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxSwev1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxSwev0)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::Jtag)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::RtcUpd)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::RtcCombDly)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::RtcCh2Dly)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::RtcCh1Dly)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::RtcCh0Dly)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::RtcCh2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::RtcCh1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::RtcCh0)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::Pad)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::BatmonCombined)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::BatmonTempLl)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::BatmonTempUl)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::BatmonBattLl)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::BatmonBattUl)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxTimer2Ev3)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxTimer2Ev2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxTimer2Ev1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::AuxTimer2Ev0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG0 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG0_EN\\]"]
    #[inline(always)]
    pub fn ioev_aon_prog0(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg0Ev::IoevAonProg0)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "13:8\\]
Event selector for AON_PROG1 event. AON Event Source id# selecting event routed to EVENT as AON_PROG1 event.\n\nValue on reset: 43"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AonProg1Ev {
    #[doc = "63: 0"]
    None = 63,
    #[doc = "56: 0"]
    AuxCompbAsyncN = 56,
    #[doc = "55: 0"]
    AuxCompbAsync = 55,
    #[doc = "54: 0"]
    BatmonVolt = 54,
    #[doc = "53: 0"]
    BatmonTemp = 53,
    #[doc = "52: 0"]
    AuxTimer1Ev = 52,
    #[doc = "51: 0"]
    AuxTimer0Ev = 51,
    #[doc = "50: 0"]
    AuxTdcDone = 50,
    #[doc = "49: 0"]
    AuxAdcDone = 49,
    #[doc = "48: 0"]
    AuxCompb = 48,
    #[doc = "47: 0"]
    AuxCompa = 47,
    #[doc = "46: 0"]
    AuxSwev2 = 46,
    #[doc = "45: 0"]
    AuxSwev1 = 45,
    #[doc = "44: 0"]
    AuxSwev0 = 44,
    #[doc = "43: 0"]
    Jtag = 43,
    #[doc = "42: 0"]
    RtcUpd = 42,
    #[doc = "41: 0"]
    RtcCombDly = 41,
    #[doc = "40: 0"]
    RtcCh2Dly = 40,
    #[doc = "39: 0"]
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
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG1 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG1_EN\\]"]
    IoevAonProg1 = 0,
}
impl From<AonProg1Ev> for u8 {
    #[inline(always)]
    fn from(variant: AonProg1Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AonProg1Ev {
    type Ux = u8;
}
impl crate::IsEnum for AonProg1Ev {}
#[doc = "Field `AON_PROG1_EV` reader - 13:8\\]
Event selector for AON_PROG1 event. AON Event Source id# selecting event routed to EVENT as AON_PROG1 event."]
pub type AonProg1EvR = crate::FieldReader<AonProg1Ev>;
impl AonProg1EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AonProg1Ev> {
        match self.bits {
            63 => Some(AonProg1Ev::None),
            56 => Some(AonProg1Ev::AuxCompbAsyncN),
            55 => Some(AonProg1Ev::AuxCompbAsync),
            54 => Some(AonProg1Ev::BatmonVolt),
            53 => Some(AonProg1Ev::BatmonTemp),
            52 => Some(AonProg1Ev::AuxTimer1Ev),
            51 => Some(AonProg1Ev::AuxTimer0Ev),
            50 => Some(AonProg1Ev::AuxTdcDone),
            49 => Some(AonProg1Ev::AuxAdcDone),
            48 => Some(AonProg1Ev::AuxCompb),
            47 => Some(AonProg1Ev::AuxCompa),
            46 => Some(AonProg1Ev::AuxSwev2),
            45 => Some(AonProg1Ev::AuxSwev1),
            44 => Some(AonProg1Ev::AuxSwev0),
            43 => Some(AonProg1Ev::Jtag),
            42 => Some(AonProg1Ev::RtcUpd),
            41 => Some(AonProg1Ev::RtcCombDly),
            40 => Some(AonProg1Ev::RtcCh2Dly),
            39 => Some(AonProg1Ev::RtcCh1Dly),
            38 => Some(AonProg1Ev::RtcCh0Dly),
            37 => Some(AonProg1Ev::RtcCh2),
            36 => Some(AonProg1Ev::RtcCh1),
            35 => Some(AonProg1Ev::RtcCh0),
            32 => Some(AonProg1Ev::Pad),
            9 => Some(AonProg1Ev::BatmonCombined),
            8 => Some(AonProg1Ev::BatmonTempLl),
            7 => Some(AonProg1Ev::BatmonTempUl),
            6 => Some(AonProg1Ev::BatmonBattLl),
            5 => Some(AonProg1Ev::BatmonBattUl),
            4 => Some(AonProg1Ev::AuxTimer2Ev3),
            3 => Some(AonProg1Ev::AuxTimer2Ev2),
            2 => Some(AonProg1Ev::AuxTimer2Ev1),
            1 => Some(AonProg1Ev::AuxTimer2Ev0),
            0 => Some(AonProg1Ev::IoevAonProg1),
            _ => None,
        }
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AonProg1Ev::None
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == AonProg1Ev::AuxCompbAsyncN
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == AonProg1Ev::AuxCompbAsync
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == AonProg1Ev::BatmonVolt
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == AonProg1Ev::BatmonTemp
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == AonProg1Ev::AuxTimer1Ev
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == AonProg1Ev::AuxTimer0Ev
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == AonProg1Ev::AuxTdcDone
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == AonProg1Ev::AuxAdcDone
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == AonProg1Ev::AuxCompb
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == AonProg1Ev::AuxCompa
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == AonProg1Ev::AuxSwev2
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == AonProg1Ev::AuxSwev1
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == AonProg1Ev::AuxSwev0
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == AonProg1Ev::Jtag
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == AonProg1Ev::RtcUpd
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == AonProg1Ev::RtcCombDly
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == AonProg1Ev::RtcCh2Dly
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == AonProg1Ev::RtcCh1Dly
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == AonProg1Ev::RtcCh0Dly
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == AonProg1Ev::RtcCh2
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == AonProg1Ev::RtcCh1
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == AonProg1Ev::RtcCh0
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == AonProg1Ev::Pad
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == AonProg1Ev::BatmonCombined
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == AonProg1Ev::BatmonTempLl
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == AonProg1Ev::BatmonTempUl
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == AonProg1Ev::BatmonBattLl
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == AonProg1Ev::BatmonBattUl
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == AonProg1Ev::AuxTimer2Ev3
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == AonProg1Ev::AuxTimer2Ev2
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == AonProg1Ev::AuxTimer2Ev1
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == AonProg1Ev::AuxTimer2Ev0
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG1 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG1_EN\\]"]
    #[inline(always)]
    pub fn is_ioev_aon_prog1(&self) -> bool {
        *self == AonProg1Ev::IoevAonProg1
    }
}
#[doc = "Field `AON_PROG1_EV` writer - 13:8\\]
Event selector for AON_PROG1 event. AON Event Source id# selecting event routed to EVENT as AON_PROG1 event."]
pub type AonProg1EvW<'a, REG> = crate::FieldWriter<'a, REG, 6, AonProg1Ev>;
impl<'a, REG> AonProg1EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::None)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxCompbAsyncN)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxCompbAsync)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::BatmonVolt)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::BatmonTemp)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxTimer1Ev)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxTimer0Ev)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxTdcDone)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxAdcDone)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxCompb)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxCompa)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxSwev2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxSwev1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxSwev0)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::Jtag)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::RtcUpd)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::RtcCombDly)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::RtcCh2Dly)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::RtcCh1Dly)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::RtcCh0Dly)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::RtcCh2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::RtcCh1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::RtcCh0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::Pad)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::BatmonCombined)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::BatmonTempLl)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::BatmonTempUl)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::BatmonBattLl)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::BatmonBattUl)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxTimer2Ev3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxTimer2Ev2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxTimer2Ev1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::AuxTimer2Ev0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG1 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG1_EN\\]"]
    #[inline(always)]
    pub fn ioev_aon_prog1(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg1Ev::IoevAonProg1)
    }
}
#[doc = "Field `RESERVED14` reader - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader;
#[doc = "Field `RESERVED14` writer - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "21:16\\]
Event selector for AON_PROG2 event. AON Event Source id# selecting event routed to EVENT as AON_PROG2 event.\n\nValue on reset: 43"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AonProg2Ev {
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
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG2 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG2_EN\\]"]
    IoevAonProg2 = 0,
}
impl From<AonProg2Ev> for u8 {
    #[inline(always)]
    fn from(variant: AonProg2Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AonProg2Ev {
    type Ux = u8;
}
impl crate::IsEnum for AonProg2Ev {}
#[doc = "Field `AON_PROG2_EV` reader - 21:16\\]
Event selector for AON_PROG2 event. AON Event Source id# selecting event routed to EVENT as AON_PROG2 event."]
pub type AonProg2EvR = crate::FieldReader<AonProg2Ev>;
impl AonProg2EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AonProg2Ev> {
        match self.bits {
            63 => Some(AonProg2Ev::None),
            56 => Some(AonProg2Ev::AuxCompbAsyncN),
            55 => Some(AonProg2Ev::AuxCompbAsync),
            54 => Some(AonProg2Ev::BatmonVolt),
            53 => Some(AonProg2Ev::BatmonTemp),
            52 => Some(AonProg2Ev::AuxTimer1Ev),
            51 => Some(AonProg2Ev::AuxTimer0Ev),
            50 => Some(AonProg2Ev::AuxTdcDone),
            49 => Some(AonProg2Ev::AuxAdcDone),
            48 => Some(AonProg2Ev::AuxCompb),
            47 => Some(AonProg2Ev::AuxCompa),
            46 => Some(AonProg2Ev::AuxSwev2),
            45 => Some(AonProg2Ev::AuxSwev1),
            44 => Some(AonProg2Ev::AuxSwev0),
            43 => Some(AonProg2Ev::Jtag),
            42 => Some(AonProg2Ev::RtcUpd),
            41 => Some(AonProg2Ev::RtcCombDly),
            40 => Some(AonProg2Ev::RtcCh2Dly),
            39 => Some(AonProg2Ev::RtcCh1Dly),
            38 => Some(AonProg2Ev::RtcCh0Dly),
            37 => Some(AonProg2Ev::RtcCh2),
            36 => Some(AonProg2Ev::RtcCh1),
            35 => Some(AonProg2Ev::RtcCh0),
            32 => Some(AonProg2Ev::Pad),
            9 => Some(AonProg2Ev::BatmonCombined),
            8 => Some(AonProg2Ev::BatmonTempLl),
            7 => Some(AonProg2Ev::BatmonTempUl),
            6 => Some(AonProg2Ev::BatmonBattLl),
            5 => Some(AonProg2Ev::BatmonBattUl),
            4 => Some(AonProg2Ev::AuxTimer2Ev3),
            3 => Some(AonProg2Ev::AuxTimer2Ev2),
            2 => Some(AonProg2Ev::AuxTimer2Ev1),
            1 => Some(AonProg2Ev::AuxTimer2Ev0),
            0 => Some(AonProg2Ev::IoevAonProg2),
            _ => None,
        }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AonProg2Ev::None
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == AonProg2Ev::AuxCompbAsyncN
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == AonProg2Ev::AuxCompbAsync
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == AonProg2Ev::BatmonVolt
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == AonProg2Ev::BatmonTemp
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == AonProg2Ev::AuxTimer1Ev
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == AonProg2Ev::AuxTimer0Ev
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == AonProg2Ev::AuxTdcDone
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == AonProg2Ev::AuxAdcDone
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == AonProg2Ev::AuxCompb
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == AonProg2Ev::AuxCompa
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == AonProg2Ev::AuxSwev2
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == AonProg2Ev::AuxSwev1
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == AonProg2Ev::AuxSwev0
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == AonProg2Ev::Jtag
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == AonProg2Ev::RtcUpd
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == AonProg2Ev::RtcCombDly
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == AonProg2Ev::RtcCh2Dly
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == AonProg2Ev::RtcCh1Dly
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == AonProg2Ev::RtcCh0Dly
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == AonProg2Ev::RtcCh2
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == AonProg2Ev::RtcCh1
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == AonProg2Ev::RtcCh0
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == AonProg2Ev::Pad
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == AonProg2Ev::BatmonCombined
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == AonProg2Ev::BatmonTempLl
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == AonProg2Ev::BatmonTempUl
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == AonProg2Ev::BatmonBattLl
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == AonProg2Ev::BatmonBattUl
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == AonProg2Ev::AuxTimer2Ev3
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == AonProg2Ev::AuxTimer2Ev2
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == AonProg2Ev::AuxTimer2Ev1
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == AonProg2Ev::AuxTimer2Ev0
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG2 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG2_EN\\]"]
    #[inline(always)]
    pub fn is_ioev_aon_prog2(&self) -> bool {
        *self == AonProg2Ev::IoevAonProg2
    }
}
#[doc = "Field `AON_PROG2_EV` writer - 21:16\\]
Event selector for AON_PROG2 event. AON Event Source id# selecting event routed to EVENT as AON_PROG2 event."]
pub type AonProg2EvW<'a, REG> = crate::FieldWriter<'a, REG, 6, AonProg2Ev>;
impl<'a, REG> AonProg2EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::None)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxCompbAsyncN)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxCompbAsync)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::BatmonVolt)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::BatmonTemp)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxTimer1Ev)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxTimer0Ev)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxTdcDone)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxAdcDone)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxCompb)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxCompa)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxSwev2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxSwev1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxSwev0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::Jtag)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::RtcUpd)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::RtcCombDly)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::RtcCh2Dly)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::RtcCh1Dly)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::RtcCh0Dly)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::RtcCh2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::RtcCh1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::RtcCh0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::Pad)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::BatmonCombined)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::BatmonTempLl)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::BatmonTempUl)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::BatmonBattLl)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::BatmonBattUl)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxTimer2Ev3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxTimer2Ev2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxTimer2Ev1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::AuxTimer2Ev0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_AON_PROG2 in \\[MCU_IOC:IOCFGx.IOEV_AON_PROG2_EN\\]"]
    #[inline(always)]
    pub fn ioev_aon_prog2(self) -> &'a mut crate::W<REG> {
        self.variant(AonProg2Ev::IoevAonProg2)
    }
}
#[doc = "Field `RESERVED22` reader - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved22R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED22` writer - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved22W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Event selector for AON_PROG0 event. AON Event Source id# selecting event routed to EVENT as AON_PROG0 event."]
    #[inline(always)]
    pub fn aon_prog0_ev(&self) -> AonProg0EvR {
        AonProg0EvR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Event selector for AON_PROG1 event. AON Event Source id# selecting event routed to EVENT as AON_PROG1 event."]
    #[inline(always)]
    pub fn aon_prog1_ev(&self) -> AonProg1EvR {
        AonProg1EvR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Event selector for AON_PROG2 event. AON Event Source id# selecting event routed to EVENT as AON_PROG2 event."]
    #[inline(always)]
    pub fn aon_prog2_ev(&self) -> AonProg2EvR {
        AonProg2EvR::new(((self.bits >> 16) & 0x3f) as u8)
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
Event selector for AON_PROG0 event. AON Event Source id# selecting event routed to EVENT as AON_PROG0 event."]
    #[inline(always)]
    #[must_use]
    pub fn aon_prog0_ev(&mut self) -> AonProg0EvW<EvtomcuselSpec> {
        AonProg0EvW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<EvtomcuselSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Event selector for AON_PROG1 event. AON Event Source id# selecting event routed to EVENT as AON_PROG1 event."]
    #[inline(always)]
    #[must_use]
    pub fn aon_prog1_ev(&mut self) -> AonProg1EvW<EvtomcuselSpec> {
        AonProg1EvW::new(self, 8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<EvtomcuselSpec> {
        Reserved14W::new(self, 14)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Event selector for AON_PROG2 event. AON Event Source id# selecting event routed to EVENT as AON_PROG2 event."]
    #[inline(always)]
    #[must_use]
    pub fn aon_prog2_ev(&mut self) -> AonProg2EvW<EvtomcuselSpec> {
        AonProg2EvW::new(self, 16)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> Reserved22W<EvtomcuselSpec> {
        Reserved22W::new(self, 22)
    }
}
#[doc = "Event Selector For MCU Event Fabric This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtomcusel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtomcusel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtomcuselSpec;
impl crate::RegisterSpec for EvtomcuselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtomcusel::R`](R) reader structure"]
impl crate::Readable for EvtomcuselSpec {}
#[doc = "`write(|w| ..)` method takes [`evtomcusel::W`](W) writer structure"]
impl crate::Writable for EvtomcuselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVTOMCUSEL to value 0x002b_2b2b"]
impl crate::Resettable for EvtomcuselSpec {
    const RESET_VALUE: u32 = 0x002b_2b2b;
}

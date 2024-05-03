#[doc = "Register `ADCCTL` reader"]
pub type R = crate::R<AdcctlSpec>;
#[doc = "Register `ADCCTL` writer"]
pub type W = crate::W<AdcctlSpec>;
#[doc = "1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd {
    #[doc = "3: Flush ADC FIFO. You must set CMD to EN or DIS after flush. System CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    Flush = 3,
    #[doc = "1: Enable ADC interface."]
    En = 1,
    #[doc = "0: Disable ADC interface."]
    Dis = 0,
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(variant: Cmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd {
    type Ux = u8;
}
impl crate::IsEnum for Cmd {}
#[doc = "Field `CMD` reader - 1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
pub type CmdR = crate::FieldReader<Cmd>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmd> {
        match self.bits {
            3 => Some(Cmd::Flush),
            1 => Some(Cmd::En),
            0 => Some(Cmd::Dis),
            _ => None,
        }
    }
    #[doc = "Flush ADC FIFO. You must set CMD to EN or DIS after flush. System CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == Cmd::Flush
    }
    #[doc = "Enable ADC interface."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Cmd::En
    }
    #[doc = "Disable ADC interface."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Cmd::Dis
    }
}
#[doc = "Field `CMD` writer - 1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flush ADC FIFO. You must set CMD to EN or DIS after flush. System CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Flush)
    }
    #[doc = "Enable ADC interface."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::En)
    }
    #[doc = "Disable ADC interface."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Dis)
    }
}
#[doc = "Field `RESERVED2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "13:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT if you want to trigger the ADC manually through ADCTRIG.START. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read.\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StartSrc {
    #[doc = "63: No event."]
    NoEvent = 63,
    #[doc = "61: AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    AuxSmphAutotakeDone = 61,
    #[doc = "56: AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    AuxIsrcResetN = 56,
    #[doc = "55: AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    AuxTdcDone = 55,
    #[doc = "54: AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    AuxTimer0Ev = 54,
    #[doc = "53: AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    AuxTimer1Ev = 53,
    #[doc = "52: AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    AuxTimer2Pulse = 52,
    #[doc = "51: AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    AuxTimer2Ev3 = 51,
    #[doc = "50: AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    AuxTimer2Ev2 = 50,
    #[doc = "49: AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    AuxTimer2Ev1 = 49,
    #[doc = "48: AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    AuxTimer2Ev0 = 48,
    #[doc = "47: AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    AuxCompb = 47,
    #[doc = "46: AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    AuxCompa = 46,
    #[doc = "43: AUX_EVCTL:EVSTAT2.MCU_EV"]
    McuEv = 43,
    #[doc = "42: AUX_EVCTL:EVSTAT2.ACLK_REF"]
    AclkRef = 42,
    #[doc = "41: AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    VddrRecharge = 41,
    #[doc = "40: AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    McuActive = 40,
    #[doc = "39: AUX_EVCTL:EVSTAT2.PWR_DWN"]
    PwrDwn = 39,
    #[doc = "38: AUX_EVCTL:EVSTAT2.SCLK_LF"]
    SclkLf = 38,
    #[doc = "37: AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    AonBatmonTempUpd = 37,
    #[doc = "36: AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    AonBatmonBatUpd = 36,
    #[doc = "35: AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    AonRtc4khz = 35,
    #[doc = "34: AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    AonRtcCh2Dly = 34,
    #[doc = "33: AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    AonRtcCh2 = 33,
    #[doc = "32: AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    ManualEv = 32,
    #[doc = "31: AUX_EVCTL:EVSTAT1.AUXIO31"]
    Auxio31 = 31,
    #[doc = "30: AUX_EVCTL:EVSTAT1.AUXIO30"]
    Auxio30 = 30,
    #[doc = "29: AUX_EVCTL:EVSTAT1.AUXIO29"]
    Auxio29 = 29,
    #[doc = "28: AUX_EVCTL:EVSTAT1.AUXIO28"]
    Auxio28 = 28,
    #[doc = "27: AUX_EVCTL:EVSTAT1.AUXIO27"]
    Auxio27 = 27,
    #[doc = "26: AUX_EVCTL:EVSTAT1.AUXIO26"]
    Auxio26 = 26,
    #[doc = "25: AUX_EVCTL:EVSTAT1.AUXIO25"]
    Auxio25 = 25,
    #[doc = "24: AUX_EVCTL:EVSTAT1.AUXIO24"]
    Auxio24 = 24,
    #[doc = "23: AUX_EVCTL:EVSTAT1.AUXIO23"]
    Auxio23 = 23,
    #[doc = "22: AUX_EVCTL:EVSTAT1.AUXIO22"]
    Auxio22 = 22,
    #[doc = "21: AUX_EVCTL:EVSTAT1.AUXIO21"]
    Auxio21 = 21,
    #[doc = "20: AUX_EVCTL:EVSTAT1.AUXIO20"]
    Auxio20 = 20,
    #[doc = "19: AUX_EVCTL:EVSTAT1.AUXIO19"]
    Auxio19 = 19,
    #[doc = "18: AUX_EVCTL:EVSTAT1.AUXIO18"]
    Auxio18 = 18,
    #[doc = "17: AUX_EVCTL:EVSTAT1.AUXIO17"]
    Auxio17 = 17,
    #[doc = "16: AUX_EVCTL:EVSTAT1.AUXIO16"]
    Auxio16 = 16,
    #[doc = "15: AUX_EVCTL:EVSTAT0.AUXIO15"]
    Auxio15 = 15,
    #[doc = "14: AUX_EVCTL:EVSTAT0.AUXIO14"]
    Auxio14 = 14,
    #[doc = "13: AUX_EVCTL:EVSTAT0.AUXIO13"]
    Auxio13 = 13,
    #[doc = "12: AUX_EVCTL:EVSTAT0.AUXIO12"]
    Auxio12 = 12,
    #[doc = "11: AUX_EVCTL:EVSTAT0.AUXIO11"]
    Auxio11 = 11,
    #[doc = "10: AUX_EVCTL:EVSTAT0.AUXIO10"]
    Auxio10 = 10,
    #[doc = "9: AUX_EVCTL:EVSTAT0.AUXIO9"]
    Auxio9 = 9,
    #[doc = "8: AUX_EVCTL:EVSTAT0.AUXIO8"]
    Auxio8 = 8,
    #[doc = "7: AUX_EVCTL:EVSTAT0.AUXIO7"]
    Auxio7 = 7,
    #[doc = "6: AUX_EVCTL:EVSTAT0.AUXIO6"]
    Auxio6 = 6,
    #[doc = "5: AUX_EVCTL:EVSTAT0.AUXIO5"]
    Auxio5 = 5,
    #[doc = "4: AUX_EVCTL:EVSTAT0.AUXIO4"]
    Auxio4 = 4,
    #[doc = "3: AUX_EVCTL:EVSTAT0.AUXIO3"]
    Auxio3 = 3,
    #[doc = "2: AUX_EVCTL:EVSTAT0.AUXIO2"]
    Auxio2 = 2,
    #[doc = "1: AUX_EVCTL:EVSTAT0.AUXIO1"]
    Auxio1 = 1,
    #[doc = "0: AUX_EVCTL:EVSTAT0.AUXIO0"]
    Auxio0 = 0,
}
impl From<StartSrc> for u8 {
    #[inline(always)]
    fn from(variant: StartSrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for StartSrc {
    type Ux = u8;
}
impl crate::IsEnum for StartSrc {}
#[doc = "Field `START_SRC` reader - 13:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT if you want to trigger the ADC manually through ADCTRIG.START. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
pub type StartSrcR = crate::FieldReader<StartSrc>;
impl StartSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<StartSrc> {
        match self.bits {
            63 => Some(StartSrc::NoEvent),
            61 => Some(StartSrc::AuxSmphAutotakeDone),
            56 => Some(StartSrc::AuxIsrcResetN),
            55 => Some(StartSrc::AuxTdcDone),
            54 => Some(StartSrc::AuxTimer0Ev),
            53 => Some(StartSrc::AuxTimer1Ev),
            52 => Some(StartSrc::AuxTimer2Pulse),
            51 => Some(StartSrc::AuxTimer2Ev3),
            50 => Some(StartSrc::AuxTimer2Ev2),
            49 => Some(StartSrc::AuxTimer2Ev1),
            48 => Some(StartSrc::AuxTimer2Ev0),
            47 => Some(StartSrc::AuxCompb),
            46 => Some(StartSrc::AuxCompa),
            43 => Some(StartSrc::McuEv),
            42 => Some(StartSrc::AclkRef),
            41 => Some(StartSrc::VddrRecharge),
            40 => Some(StartSrc::McuActive),
            39 => Some(StartSrc::PwrDwn),
            38 => Some(StartSrc::SclkLf),
            37 => Some(StartSrc::AonBatmonTempUpd),
            36 => Some(StartSrc::AonBatmonBatUpd),
            35 => Some(StartSrc::AonRtc4khz),
            34 => Some(StartSrc::AonRtcCh2Dly),
            33 => Some(StartSrc::AonRtcCh2),
            32 => Some(StartSrc::ManualEv),
            31 => Some(StartSrc::Auxio31),
            30 => Some(StartSrc::Auxio30),
            29 => Some(StartSrc::Auxio29),
            28 => Some(StartSrc::Auxio28),
            27 => Some(StartSrc::Auxio27),
            26 => Some(StartSrc::Auxio26),
            25 => Some(StartSrc::Auxio25),
            24 => Some(StartSrc::Auxio24),
            23 => Some(StartSrc::Auxio23),
            22 => Some(StartSrc::Auxio22),
            21 => Some(StartSrc::Auxio21),
            20 => Some(StartSrc::Auxio20),
            19 => Some(StartSrc::Auxio19),
            18 => Some(StartSrc::Auxio18),
            17 => Some(StartSrc::Auxio17),
            16 => Some(StartSrc::Auxio16),
            15 => Some(StartSrc::Auxio15),
            14 => Some(StartSrc::Auxio14),
            13 => Some(StartSrc::Auxio13),
            12 => Some(StartSrc::Auxio12),
            11 => Some(StartSrc::Auxio11),
            10 => Some(StartSrc::Auxio10),
            9 => Some(StartSrc::Auxio9),
            8 => Some(StartSrc::Auxio8),
            7 => Some(StartSrc::Auxio7),
            6 => Some(StartSrc::Auxio6),
            5 => Some(StartSrc::Auxio5),
            4 => Some(StartSrc::Auxio4),
            3 => Some(StartSrc::Auxio3),
            2 => Some(StartSrc::Auxio2),
            1 => Some(StartSrc::Auxio1),
            0 => Some(StartSrc::Auxio0),
            _ => None,
        }
    }
    #[doc = "No event."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == StartSrc::NoEvent
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == StartSrc::AuxSmphAutotakeDone
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == StartSrc::AuxIsrcResetN
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == StartSrc::AuxTdcDone
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == StartSrc::AuxTimer0Ev
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == StartSrc::AuxTimer1Ev
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == StartSrc::AuxTimer2Pulse
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == StartSrc::AuxTimer2Ev3
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == StartSrc::AuxTimer2Ev2
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == StartSrc::AuxTimer2Ev1
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == StartSrc::AuxTimer2Ev0
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == StartSrc::AuxCompb
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == StartSrc::AuxCompa
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == StartSrc::McuEv
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == StartSrc::AclkRef
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == StartSrc::VddrRecharge
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn is_mcu_active(&self) -> bool {
        *self == StartSrc::McuActive
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == StartSrc::PwrDwn
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == StartSrc::SclkLf
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline(always)]
    pub fn is_aon_batmon_temp_upd(&self) -> bool {
        *self == StartSrc::AonBatmonTempUpd
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline(always)]
    pub fn is_aon_batmon_bat_upd(&self) -> bool {
        *self == StartSrc::AonBatmonBatUpd
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == StartSrc::AonRtc4khz
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == StartSrc::AonRtcCh2Dly
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == StartSrc::AonRtcCh2
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn is_manual_ev(&self) -> bool {
        *self == StartSrc::ManualEv
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn is_auxio31(&self) -> bool {
        *self == StartSrc::Auxio31
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn is_auxio30(&self) -> bool {
        *self == StartSrc::Auxio30
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn is_auxio29(&self) -> bool {
        *self == StartSrc::Auxio29
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn is_auxio28(&self) -> bool {
        *self == StartSrc::Auxio28
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn is_auxio27(&self) -> bool {
        *self == StartSrc::Auxio27
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == StartSrc::Auxio26
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == StartSrc::Auxio25
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == StartSrc::Auxio24
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == StartSrc::Auxio23
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == StartSrc::Auxio22
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == StartSrc::Auxio21
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == StartSrc::Auxio20
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == StartSrc::Auxio19
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn is_auxio18(&self) -> bool {
        *self == StartSrc::Auxio18
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn is_auxio17(&self) -> bool {
        *self == StartSrc::Auxio17
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn is_auxio16(&self) -> bool {
        *self == StartSrc::Auxio16
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == StartSrc::Auxio15
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == StartSrc::Auxio14
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == StartSrc::Auxio13
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == StartSrc::Auxio12
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == StartSrc::Auxio11
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == StartSrc::Auxio10
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == StartSrc::Auxio9
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == StartSrc::Auxio8
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == StartSrc::Auxio7
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == StartSrc::Auxio6
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == StartSrc::Auxio5
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == StartSrc::Auxio4
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == StartSrc::Auxio3
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == StartSrc::Auxio2
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == StartSrc::Auxio1
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == StartSrc::Auxio0
    }
}
#[doc = "Field `START_SRC` writer - 13:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT if you want to trigger the ADC manually through ADCTRIG.START. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
pub type StartSrcW<'a, REG> = crate::FieldWriter<'a, REG, 6, StartSrc>;
impl<'a, REG> StartSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::NoEvent)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxSmphAutotakeDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxIsrcResetN)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxTdcDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxTimer0Ev)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxTimer1Ev)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxTimer2Pulse)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxTimer2Ev3)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxTimer2Ev2)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxTimer2Ev1)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxTimer2Ev0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxCompb)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxCompa)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::McuEv)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AclkRef)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn vddr_recharge(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::VddrRecharge)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn mcu_active(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::McuActive)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn pwr_dwn(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::PwrDwn)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::SclkLf)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline(always)]
    pub fn aon_batmon_temp_upd(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AonBatmonTempUpd)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline(always)]
    pub fn aon_batmon_bat_upd(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AonBatmonBatUpd)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn aon_rtc_4khz(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AonRtc4khz)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AonRtcCh2Dly)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AonRtcCh2)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn manual_ev(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::ManualEv)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn auxio31(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio31)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn auxio30(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio30)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn auxio29(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio29)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn auxio28(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio28)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn auxio27(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio27)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio26)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio25)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio24)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio23)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio22)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio21)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio20)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio19)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn auxio18(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio18)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn auxio17(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio17)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn auxio16(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio16)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio15)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio14)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio13)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio12)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio11)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio10)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio9)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio8)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio7)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio6)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio5)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio4)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio0)
    }
}
#[doc = "14:14\\]
Select active polarity for START_SRC event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartPol {
    #[doc = "1: Set ADC trigger on falling edge of event source."]
    Fall = 1,
    #[doc = "0: Set ADC trigger on rising edge of event source."]
    Rise = 0,
}
impl From<StartPol> for bool {
    #[inline(always)]
    fn from(variant: StartPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_POL` reader - 14:14\\]
Select active polarity for START_SRC event."]
pub type StartPolR = crate::BitReader<StartPol>;
impl StartPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StartPol {
        match self.bits {
            true => StartPol::Fall,
            false => StartPol::Rise,
        }
    }
    #[doc = "Set ADC trigger on falling edge of event source."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == StartPol::Fall
    }
    #[doc = "Set ADC trigger on rising edge of event source."]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == StartPol::Rise
    }
}
#[doc = "Field `START_POL` writer - 14:14\\]
Select active polarity for START_SRC event."]
pub type StartPolW<'a, REG> = crate::BitWriter<'a, REG, StartPol>;
impl<'a, REG> StartPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set ADC trigger on falling edge of event source."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(StartPol::Fall)
    }
    #[doc = "Set ADC trigger on rising edge of event source."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(StartPol::Rise)
    }
}
#[doc = "Field `RESERVED15` reader - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED15` writer - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT if you want to trigger the ADC manually through ADCTRIG.START. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
    #[inline(always)]
    pub fn start_src(&self) -> StartSrcR {
        StartSrcR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Select active polarity for START_SRC event."]
    #[inline(always)]
    pub fn start_pol(&self) -> StartPolR {
        StartPolR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<AdcctlSpec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<AdcctlSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT if you want to trigger the ADC manually through ADCTRIG.START. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
    #[inline(always)]
    #[must_use]
    pub fn start_src(&mut self) -> StartSrcW<AdcctlSpec> {
        StartSrcW::new(self, 8)
    }
    #[doc = "Bit 14 - 14:14\\]
Select active polarity for START_SRC event."]
    #[inline(always)]
    #[must_use]
    pub fn start_pol(&mut self) -> StartPolW<AdcctlSpec> {
        StartPolW::new(self, 14)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<AdcctlSpec> {
        Reserved15W::new(self, 15)
    }
}
#[doc = "ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcctlSpec;
impl crate::RegisterSpec for AdcctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcctl::R`](R) reader structure"]
impl crate::Readable for AdcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`adcctl::W`](W) writer structure"]
impl crate::Writable for AdcctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCCTL to value 0x3f00"]
impl crate::Resettable for AdcctlSpec {
    const RESET_VALUE: u32 = 0x3f00;
}

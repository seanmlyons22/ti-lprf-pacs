#[doc = "Register `PROGWU1CFG` reader"]
pub type R = crate::R<Progwu1cfgSpec>;
#[doc = "Register `PROGWU1CFG` writer"]
pub type W = crate::W<Progwu1cfgSpec>;
#[doc = "5:0\\]
Wakeup source from the asynchronous AUX event bus. Only change WU_SRC when EN is 0 or WUFLAGSCLR.PROG_WU1 is 1. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WuSrc {
    #[doc = "63: No event."]
    NoEvent = 63,
    #[doc = "61: AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    AuxSmphAutotakeDone = 61,
    #[doc = "60: AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    AuxAdcFifoNotEmpty = 60,
    #[doc = "59: AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    AuxAdcFifoAlmostFull = 59,
    #[doc = "58: AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    AuxAdcIrq = 58,
    #[doc = "57: AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    AuxAdcDone = 57,
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
    #[doc = "45: AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    McuObsmux1 = 45,
    #[doc = "44: AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    McuObsmux0 = 44,
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
impl From<WuSrc> for u8 {
    #[inline(always)]
    fn from(variant: WuSrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WuSrc {
    type Ux = u8;
}
impl crate::IsEnum for WuSrc {}
#[doc = "Field `WU_SRC` reader - 5:0\\]
Wakeup source from the asynchronous AUX event bus. Only change WU_SRC when EN is 0 or WUFLAGSCLR.PROG_WU1 is 1. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
pub type WuSrcR = crate::FieldReader<WuSrc>;
impl WuSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WuSrc> {
        match self.bits {
            63 => Some(WuSrc::NoEvent),
            61 => Some(WuSrc::AuxSmphAutotakeDone),
            60 => Some(WuSrc::AuxAdcFifoNotEmpty),
            59 => Some(WuSrc::AuxAdcFifoAlmostFull),
            58 => Some(WuSrc::AuxAdcIrq),
            57 => Some(WuSrc::AuxAdcDone),
            56 => Some(WuSrc::AuxIsrcResetN),
            55 => Some(WuSrc::AuxTdcDone),
            54 => Some(WuSrc::AuxTimer0Ev),
            53 => Some(WuSrc::AuxTimer1Ev),
            52 => Some(WuSrc::AuxTimer2Pulse),
            51 => Some(WuSrc::AuxTimer2Ev3),
            50 => Some(WuSrc::AuxTimer2Ev2),
            49 => Some(WuSrc::AuxTimer2Ev1),
            48 => Some(WuSrc::AuxTimer2Ev0),
            47 => Some(WuSrc::AuxCompb),
            46 => Some(WuSrc::AuxCompa),
            45 => Some(WuSrc::McuObsmux1),
            44 => Some(WuSrc::McuObsmux0),
            43 => Some(WuSrc::McuEv),
            42 => Some(WuSrc::AclkRef),
            41 => Some(WuSrc::VddrRecharge),
            40 => Some(WuSrc::McuActive),
            39 => Some(WuSrc::PwrDwn),
            38 => Some(WuSrc::SclkLf),
            37 => Some(WuSrc::AonBatmonTempUpd),
            36 => Some(WuSrc::AonBatmonBatUpd),
            35 => Some(WuSrc::AonRtc4khz),
            34 => Some(WuSrc::AonRtcCh2Dly),
            33 => Some(WuSrc::AonRtcCh2),
            32 => Some(WuSrc::ManualEv),
            31 => Some(WuSrc::Auxio31),
            30 => Some(WuSrc::Auxio30),
            29 => Some(WuSrc::Auxio29),
            28 => Some(WuSrc::Auxio28),
            27 => Some(WuSrc::Auxio27),
            26 => Some(WuSrc::Auxio26),
            25 => Some(WuSrc::Auxio25),
            24 => Some(WuSrc::Auxio24),
            23 => Some(WuSrc::Auxio23),
            22 => Some(WuSrc::Auxio22),
            21 => Some(WuSrc::Auxio21),
            20 => Some(WuSrc::Auxio20),
            19 => Some(WuSrc::Auxio19),
            18 => Some(WuSrc::Auxio18),
            17 => Some(WuSrc::Auxio17),
            16 => Some(WuSrc::Auxio16),
            15 => Some(WuSrc::Auxio15),
            14 => Some(WuSrc::Auxio14),
            13 => Some(WuSrc::Auxio13),
            12 => Some(WuSrc::Auxio12),
            11 => Some(WuSrc::Auxio11),
            10 => Some(WuSrc::Auxio10),
            9 => Some(WuSrc::Auxio9),
            8 => Some(WuSrc::Auxio8),
            7 => Some(WuSrc::Auxio7),
            6 => Some(WuSrc::Auxio6),
            5 => Some(WuSrc::Auxio5),
            4 => Some(WuSrc::Auxio4),
            3 => Some(WuSrc::Auxio3),
            2 => Some(WuSrc::Auxio2),
            1 => Some(WuSrc::Auxio1),
            0 => Some(WuSrc::Auxio0),
            _ => None,
        }
    }
    #[doc = "No event."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == WuSrc::NoEvent
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == WuSrc::AuxSmphAutotakeDone
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == WuSrc::AuxAdcFifoNotEmpty
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == WuSrc::AuxAdcFifoAlmostFull
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == WuSrc::AuxAdcIrq
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WuSrc::AuxAdcDone
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == WuSrc::AuxIsrcResetN
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WuSrc::AuxTdcDone
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WuSrc::AuxTimer0Ev
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WuSrc::AuxTimer1Ev
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == WuSrc::AuxTimer2Pulse
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WuSrc::AuxTimer2Ev3
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WuSrc::AuxTimer2Ev2
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WuSrc::AuxTimer2Ev1
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WuSrc::AuxTimer2Ev0
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == WuSrc::AuxCompb
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == WuSrc::AuxCompa
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn is_mcu_obsmux1(&self) -> bool {
        *self == WuSrc::McuObsmux1
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn is_mcu_obsmux0(&self) -> bool {
        *self == WuSrc::McuObsmux0
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == WuSrc::McuEv
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == WuSrc::AclkRef
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == WuSrc::VddrRecharge
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn is_mcu_active(&self) -> bool {
        *self == WuSrc::McuActive
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == WuSrc::PwrDwn
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == WuSrc::SclkLf
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline(always)]
    pub fn is_aon_batmon_temp_upd(&self) -> bool {
        *self == WuSrc::AonBatmonTempUpd
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline(always)]
    pub fn is_aon_batmon_bat_upd(&self) -> bool {
        *self == WuSrc::AonBatmonBatUpd
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == WuSrc::AonRtc4khz
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == WuSrc::AonRtcCh2Dly
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == WuSrc::AonRtcCh2
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn is_manual_ev(&self) -> bool {
        *self == WuSrc::ManualEv
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn is_auxio31(&self) -> bool {
        *self == WuSrc::Auxio31
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn is_auxio30(&self) -> bool {
        *self == WuSrc::Auxio30
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn is_auxio29(&self) -> bool {
        *self == WuSrc::Auxio29
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn is_auxio28(&self) -> bool {
        *self == WuSrc::Auxio28
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn is_auxio27(&self) -> bool {
        *self == WuSrc::Auxio27
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == WuSrc::Auxio26
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == WuSrc::Auxio25
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == WuSrc::Auxio24
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == WuSrc::Auxio23
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == WuSrc::Auxio22
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == WuSrc::Auxio21
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == WuSrc::Auxio20
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == WuSrc::Auxio19
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn is_auxio18(&self) -> bool {
        *self == WuSrc::Auxio18
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn is_auxio17(&self) -> bool {
        *self == WuSrc::Auxio17
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn is_auxio16(&self) -> bool {
        *self == WuSrc::Auxio16
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == WuSrc::Auxio15
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == WuSrc::Auxio14
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == WuSrc::Auxio13
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == WuSrc::Auxio12
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == WuSrc::Auxio11
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == WuSrc::Auxio10
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == WuSrc::Auxio9
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == WuSrc::Auxio8
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == WuSrc::Auxio7
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == WuSrc::Auxio6
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == WuSrc::Auxio5
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == WuSrc::Auxio4
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == WuSrc::Auxio3
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == WuSrc::Auxio2
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == WuSrc::Auxio1
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == WuSrc::Auxio0
    }
}
#[doc = "Field `WU_SRC` writer - 5:0\\]
Wakeup source from the asynchronous AUX event bus. Only change WU_SRC when EN is 0 or WUFLAGSCLR.PROG_WU1 is 1. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
pub type WuSrcW<'a, REG> = crate::FieldWriter<'a, REG, 6, WuSrc>;
impl<'a, REG> WuSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::NoEvent)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxSmphAutotakeDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxAdcFifoNotEmpty)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxAdcFifoAlmostFull)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn aux_adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxAdcIrq)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxAdcDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxIsrcResetN)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxTdcDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxTimer0Ev)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxTimer1Ev)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxTimer2Pulse)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxTimer2Ev3)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxTimer2Ev2)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxTimer2Ev1)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxTimer2Ev0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxCompb)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AuxCompa)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn mcu_obsmux1(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::McuObsmux1)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn mcu_obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::McuObsmux0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::McuEv)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AclkRef)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn vddr_recharge(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::VddrRecharge)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn mcu_active(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::McuActive)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn pwr_dwn(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::PwrDwn)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::SclkLf)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline(always)]
    pub fn aon_batmon_temp_upd(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AonBatmonTempUpd)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline(always)]
    pub fn aon_batmon_bat_upd(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AonBatmonBatUpd)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn aon_rtc_4khz(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AonRtc4khz)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AonRtcCh2Dly)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::AonRtcCh2)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn manual_ev(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::ManualEv)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn auxio31(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio31)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn auxio30(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio30)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn auxio29(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio29)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn auxio28(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio28)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn auxio27(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio27)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio26)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio25)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio24)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio23)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio22)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio21)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio20)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio19)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn auxio18(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio18)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn auxio17(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio17)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn auxio16(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio16)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio15)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio14)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio13)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio12)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio11)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio10)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio9)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio8)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio7)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio6)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio5)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio4)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(WuSrc::Auxio0)
    }
}
#[doc = "Field `EN` reader - 6:6\\]
Programmable wakeup flag enable. 0: Disable wakeup flag. 1: Enable wakeup flag."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 6:6\\]
Programmable wakeup flag enable. 0: Disable wakeup flag. 1: Enable wakeup flag."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "7:7\\]
Polarity of WU_SRC. The procedure used to clear the wakeup flag decides level or edge sensitivity, see WUFLAGSCLR.PROG_WU1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pol {
    #[doc = "1: The wakeup flag is set when WU_SRC is low or goes low."]
    Low = 1,
    #[doc = "0: The wakeup flag is set when WU_SRC is high or goes high."]
    High = 0,
}
impl From<Pol> for bool {
    #[inline(always)]
    fn from(variant: Pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL` reader - 7:7\\]
Polarity of WU_SRC. The procedure used to clear the wakeup flag decides level or edge sensitivity, see WUFLAGSCLR.PROG_WU1."]
pub type PolR = crate::BitReader<Pol>;
impl PolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pol {
        match self.bits {
            true => Pol::Low,
            false => Pol::High,
        }
    }
    #[doc = "The wakeup flag is set when WU_SRC is low or goes low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pol::Low
    }
    #[doc = "The wakeup flag is set when WU_SRC is high or goes high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pol::High
    }
}
#[doc = "Field `POL` writer - 7:7\\]
Polarity of WU_SRC. The procedure used to clear the wakeup flag decides level or edge sensitivity, see WUFLAGSCLR.PROG_WU1."]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG, Pol>;
impl<'a, REG> PolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The wakeup flag is set when WU_SRC is low or goes low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Pol::Low)
    }
    #[doc = "The wakeup flag is set when WU_SRC is high or goes high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Pol::High)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Wakeup source from the asynchronous AUX event bus. Only change WU_SRC when EN is 0 or WUFLAGSCLR.PROG_WU1 is 1. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
    #[inline(always)]
    pub fn wu_src(&self) -> WuSrcR {
        WuSrcR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Programmable wakeup flag enable. 0: Disable wakeup flag. 1: Enable wakeup flag."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Polarity of WU_SRC. The procedure used to clear the wakeup flag decides level or edge sensitivity, see WUFLAGSCLR.PROG_WU1."]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Wakeup source from the asynchronous AUX event bus. Only change WU_SRC when EN is 0 or WUFLAGSCLR.PROG_WU1 is 1. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
    #[inline(always)]
    #[must_use]
    pub fn wu_src(&mut self) -> WuSrcW<Progwu1cfgSpec> {
        WuSrcW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Programmable wakeup flag enable. 0: Disable wakeup flag. 1: Enable wakeup flag."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Progwu1cfgSpec> {
        EnW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Polarity of WU_SRC. The procedure used to clear the wakeup flag decides level or edge sensitivity, see WUFLAGSCLR.PROG_WU1."]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> PolW<Progwu1cfgSpec> {
        PolW::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Progwu1cfgSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Programmable Wakeup 1 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU1 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`progwu1cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`progwu1cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Progwu1cfgSpec;
impl crate::RegisterSpec for Progwu1cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`progwu1cfg::R`](R) reader structure"]
impl crate::Readable for Progwu1cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`progwu1cfg::W`](W) writer structure"]
impl crate::Writable for Progwu1cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROGWU1CFG to value 0"]
impl crate::Resettable for Progwu1cfgSpec {
    const RESET_VALUE: u32 = 0;
}

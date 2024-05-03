#[doc = "Register `CH3CCFG` reader"]
pub type R = crate::R<Ch3ccfgSpec>;
#[doc = "Register `CH3CCFG` writer"]
pub type W = crate::W<Ch3ccfgSpec>;
#[doc = "0:0\\]
Edge configuration. Channel captures counter value at selected edge on signal source selected by CAPT_SRC. See CH3EVCFG.CCACT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    #[doc = "1: Capture CNTR.VALUE at rising edge of CAPT_SRC."]
    Rising = 1,
    #[doc = "0: Capture CNTR.VALUE at falling edge of CAPT_SRC."]
    Falling = 0,
}
impl From<Edge> for bool {
    #[inline(always)]
    fn from(variant: Edge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDGE` reader - 0:0\\]
Edge configuration. Channel captures counter value at selected edge on signal source selected by CAPT_SRC. See CH3EVCFG.CCACT."]
pub type EdgeR = crate::BitReader<Edge>;
impl EdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edge {
        match self.bits {
            true => Edge::Rising,
            false => Edge::Falling,
        }
    }
    #[doc = "Capture CNTR.VALUE at rising edge of CAPT_SRC."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Edge::Rising
    }
    #[doc = "Capture CNTR.VALUE at falling edge of CAPT_SRC."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Edge::Falling
    }
}
#[doc = "Field `EDGE` writer - 0:0\\]
Edge configuration. Channel captures counter value at selected edge on signal source selected by CAPT_SRC. See CH3EVCFG.CCACT."]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG, Edge>;
impl<'a, REG> EdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture CNTR.VALUE at rising edge of CAPT_SRC."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Rising)
    }
    #[doc = "Capture CNTR.VALUE at falling edge of CAPT_SRC."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Falling)
    }
}
#[doc = "6:1\\]
Select capture signal source from the asynchronous AUX event bus. The selected signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described in CH3EVCFG - this register is reconfigured while CTL.MODE is different from DIS. You can avoid false capture events. When wanted channel function: - SET_ON_CAPT_DIS, see description for SET_ON_CAPT_DIS in CH3EVCFG.CCACT. - SET_ON_CAPT, see description for SET_ON_CAPT in CH3EVCFG.CCACT. - PER_PULSE_WIDTH_MEAS, see description for PER_PULSE_WIDTH_MEAS in CH3EVCFG.CCACT. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CaptSrc {
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
impl From<CaptSrc> for u8 {
    #[inline(always)]
    fn from(variant: CaptSrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CaptSrc {
    type Ux = u8;
}
impl crate::IsEnum for CaptSrc {}
#[doc = "Field `CAPT_SRC` reader - 6:1\\]
Select capture signal source from the asynchronous AUX event bus. The selected signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described in CH3EVCFG - this register is reconfigured while CTL.MODE is different from DIS. You can avoid false capture events. When wanted channel function: - SET_ON_CAPT_DIS, see description for SET_ON_CAPT_DIS in CH3EVCFG.CCACT. - SET_ON_CAPT, see description for SET_ON_CAPT in CH3EVCFG.CCACT. - PER_PULSE_WIDTH_MEAS, see description for PER_PULSE_WIDTH_MEAS in CH3EVCFG.CCACT. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
pub type CaptSrcR = crate::FieldReader<CaptSrc>;
impl CaptSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CaptSrc> {
        match self.bits {
            63 => Some(CaptSrc::NoEvent),
            61 => Some(CaptSrc::AuxSmphAutotakeDone),
            60 => Some(CaptSrc::AuxAdcFifoNotEmpty),
            59 => Some(CaptSrc::AuxAdcFifoAlmostFull),
            58 => Some(CaptSrc::AuxAdcIrq),
            57 => Some(CaptSrc::AuxAdcDone),
            56 => Some(CaptSrc::AuxIsrcResetN),
            55 => Some(CaptSrc::AuxTdcDone),
            54 => Some(CaptSrc::AuxTimer0Ev),
            53 => Some(CaptSrc::AuxTimer1Ev),
            51 => Some(CaptSrc::AuxTimer2Ev3),
            50 => Some(CaptSrc::AuxTimer2Ev2),
            49 => Some(CaptSrc::AuxTimer2Ev1),
            48 => Some(CaptSrc::AuxTimer2Ev0),
            47 => Some(CaptSrc::AuxCompb),
            46 => Some(CaptSrc::AuxCompa),
            45 => Some(CaptSrc::McuObsmux1),
            44 => Some(CaptSrc::McuObsmux0),
            43 => Some(CaptSrc::McuEv),
            42 => Some(CaptSrc::AclkRef),
            41 => Some(CaptSrc::VddrRecharge),
            40 => Some(CaptSrc::McuActive),
            39 => Some(CaptSrc::PwrDwn),
            38 => Some(CaptSrc::SclkLf),
            37 => Some(CaptSrc::AonBatmonTempUpd),
            36 => Some(CaptSrc::AonBatmonBatUpd),
            35 => Some(CaptSrc::AonRtc4khz),
            34 => Some(CaptSrc::AonRtcCh2Dly),
            33 => Some(CaptSrc::AonRtcCh2),
            32 => Some(CaptSrc::ManualEv),
            31 => Some(CaptSrc::Auxio31),
            30 => Some(CaptSrc::Auxio30),
            29 => Some(CaptSrc::Auxio29),
            28 => Some(CaptSrc::Auxio28),
            27 => Some(CaptSrc::Auxio27),
            26 => Some(CaptSrc::Auxio26),
            25 => Some(CaptSrc::Auxio25),
            24 => Some(CaptSrc::Auxio24),
            23 => Some(CaptSrc::Auxio23),
            22 => Some(CaptSrc::Auxio22),
            21 => Some(CaptSrc::Auxio21),
            20 => Some(CaptSrc::Auxio20),
            19 => Some(CaptSrc::Auxio19),
            18 => Some(CaptSrc::Auxio18),
            17 => Some(CaptSrc::Auxio17),
            16 => Some(CaptSrc::Auxio16),
            15 => Some(CaptSrc::Auxio15),
            14 => Some(CaptSrc::Auxio14),
            13 => Some(CaptSrc::Auxio13),
            12 => Some(CaptSrc::Auxio12),
            11 => Some(CaptSrc::Auxio11),
            10 => Some(CaptSrc::Auxio10),
            9 => Some(CaptSrc::Auxio9),
            8 => Some(CaptSrc::Auxio8),
            7 => Some(CaptSrc::Auxio7),
            6 => Some(CaptSrc::Auxio6),
            5 => Some(CaptSrc::Auxio5),
            4 => Some(CaptSrc::Auxio4),
            3 => Some(CaptSrc::Auxio3),
            2 => Some(CaptSrc::Auxio2),
            1 => Some(CaptSrc::Auxio1),
            0 => Some(CaptSrc::Auxio0),
            _ => None,
        }
    }
    #[doc = "No event."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == CaptSrc::NoEvent
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == CaptSrc::AuxSmphAutotakeDone
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == CaptSrc::AuxAdcFifoNotEmpty
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == CaptSrc::AuxAdcFifoAlmostFull
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == CaptSrc::AuxAdcIrq
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == CaptSrc::AuxAdcDone
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == CaptSrc::AuxIsrcResetN
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == CaptSrc::AuxTdcDone
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == CaptSrc::AuxTimer0Ev
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == CaptSrc::AuxTimer1Ev
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == CaptSrc::AuxTimer2Ev3
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == CaptSrc::AuxTimer2Ev2
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == CaptSrc::AuxTimer2Ev1
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == CaptSrc::AuxTimer2Ev0
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == CaptSrc::AuxCompb
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == CaptSrc::AuxCompa
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn is_mcu_obsmux1(&self) -> bool {
        *self == CaptSrc::McuObsmux1
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn is_mcu_obsmux0(&self) -> bool {
        *self == CaptSrc::McuObsmux0
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == CaptSrc::McuEv
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == CaptSrc::AclkRef
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == CaptSrc::VddrRecharge
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn is_mcu_active(&self) -> bool {
        *self == CaptSrc::McuActive
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == CaptSrc::PwrDwn
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == CaptSrc::SclkLf
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline(always)]
    pub fn is_aon_batmon_temp_upd(&self) -> bool {
        *self == CaptSrc::AonBatmonTempUpd
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline(always)]
    pub fn is_aon_batmon_bat_upd(&self) -> bool {
        *self == CaptSrc::AonBatmonBatUpd
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == CaptSrc::AonRtc4khz
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == CaptSrc::AonRtcCh2Dly
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == CaptSrc::AonRtcCh2
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn is_manual_ev(&self) -> bool {
        *self == CaptSrc::ManualEv
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn is_auxio31(&self) -> bool {
        *self == CaptSrc::Auxio31
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn is_auxio30(&self) -> bool {
        *self == CaptSrc::Auxio30
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn is_auxio29(&self) -> bool {
        *self == CaptSrc::Auxio29
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn is_auxio28(&self) -> bool {
        *self == CaptSrc::Auxio28
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn is_auxio27(&self) -> bool {
        *self == CaptSrc::Auxio27
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == CaptSrc::Auxio26
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == CaptSrc::Auxio25
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == CaptSrc::Auxio24
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == CaptSrc::Auxio23
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == CaptSrc::Auxio22
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == CaptSrc::Auxio21
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == CaptSrc::Auxio20
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == CaptSrc::Auxio19
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn is_auxio18(&self) -> bool {
        *self == CaptSrc::Auxio18
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn is_auxio17(&self) -> bool {
        *self == CaptSrc::Auxio17
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn is_auxio16(&self) -> bool {
        *self == CaptSrc::Auxio16
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == CaptSrc::Auxio15
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == CaptSrc::Auxio14
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == CaptSrc::Auxio13
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == CaptSrc::Auxio12
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == CaptSrc::Auxio11
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == CaptSrc::Auxio10
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == CaptSrc::Auxio9
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == CaptSrc::Auxio8
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == CaptSrc::Auxio7
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == CaptSrc::Auxio6
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == CaptSrc::Auxio5
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == CaptSrc::Auxio4
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == CaptSrc::Auxio3
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == CaptSrc::Auxio2
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == CaptSrc::Auxio1
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == CaptSrc::Auxio0
    }
}
#[doc = "Field `CAPT_SRC` writer - 6:1\\]
Select capture signal source from the asynchronous AUX event bus. The selected signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described in CH3EVCFG - this register is reconfigured while CTL.MODE is different from DIS. You can avoid false capture events. When wanted channel function: - SET_ON_CAPT_DIS, see description for SET_ON_CAPT_DIS in CH3EVCFG.CCACT. - SET_ON_CAPT, see description for SET_ON_CAPT in CH3EVCFG.CCACT. - PER_PULSE_WIDTH_MEAS, see description for PER_PULSE_WIDTH_MEAS in CH3EVCFG.CCACT. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
pub type CaptSrcW<'a, REG> = crate::FieldWriter<'a, REG, 6, CaptSrc>;
impl<'a, REG> CaptSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::NoEvent)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxSmphAutotakeDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxAdcFifoNotEmpty)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxAdcFifoAlmostFull)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn aux_adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxAdcIrq)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxAdcDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxIsrcResetN)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxTdcDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxTimer0Ev)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxTimer1Ev)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxTimer2Ev3)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxTimer2Ev2)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxTimer2Ev1)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxTimer2Ev0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxCompb)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AuxCompa)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn mcu_obsmux1(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::McuObsmux1)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn mcu_obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::McuObsmux0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::McuEv)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AclkRef)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn vddr_recharge(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::VddrRecharge)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn mcu_active(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::McuActive)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn pwr_dwn(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::PwrDwn)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::SclkLf)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline(always)]
    pub fn aon_batmon_temp_upd(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AonBatmonTempUpd)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline(always)]
    pub fn aon_batmon_bat_upd(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AonBatmonBatUpd)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn aon_rtc_4khz(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AonRtc4khz)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AonRtcCh2Dly)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::AonRtcCh2)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn manual_ev(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::ManualEv)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn auxio31(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio31)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn auxio30(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio30)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn auxio29(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio29)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn auxio28(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio28)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn auxio27(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio27)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio26)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio25)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio24)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio23)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio22)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio21)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio20)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio19)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn auxio18(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio18)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn auxio17(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio17)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn auxio16(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio16)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio15)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio14)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio13)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio12)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio11)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio10)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio9)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio8)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio7)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio6)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio5)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio4)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(CaptSrc::Auxio0)
    }
}
#[doc = "Field `RESERVED7` reader - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED7` writer - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Edge configuration. Channel captures counter value at selected edge on signal source selected by CAPT_SRC. See CH3EVCFG.CCACT."]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - 6:1\\]
Select capture signal source from the asynchronous AUX event bus. The selected signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described in CH3EVCFG - this register is reconfigured while CTL.MODE is different from DIS. You can avoid false capture events. When wanted channel function: - SET_ON_CAPT_DIS, see description for SET_ON_CAPT_DIS in CH3EVCFG.CCACT. - SET_ON_CAPT, see description for SET_ON_CAPT in CH3EVCFG.CCACT. - PER_PULSE_WIDTH_MEAS, see description for PER_PULSE_WIDTH_MEAS in CH3EVCFG.CCACT. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
    #[inline(always)]
    pub fn capt_src(&self) -> CaptSrcR {
        CaptSrcR::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Edge configuration. Channel captures counter value at selected edge on signal source selected by CAPT_SRC. See CH3EVCFG.CCACT."]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EdgeW<Ch3ccfgSpec> {
        EdgeW::new(self, 0)
    }
    #[doc = "Bits 1:6 - 6:1\\]
Select capture signal source from the asynchronous AUX event bus. The selected signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described in CH3EVCFG - this register is reconfigured while CTL.MODE is different from DIS. You can avoid false capture events. When wanted channel function: - SET_ON_CAPT_DIS, see description for SET_ON_CAPT_DIS in CH3EVCFG.CCACT. - SET_ON_CAPT, see description for SET_ON_CAPT in CH3EVCFG.CCACT. - PER_PULSE_WIDTH_MEAS, see description for PER_PULSE_WIDTH_MEAS in CH3EVCFG.CCACT. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
    #[inline(always)]
    #[must_use]
    pub fn capt_src(&mut self) -> CaptSrcW<Ch3ccfgSpec> {
        CaptSrcW::new(self, 1)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<Ch3ccfgSpec> {
        Reserved7W::new(self, 7)
    }
}
#[doc = "Channel 3 Capture Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3ccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3ccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3ccfgSpec;
impl crate::RegisterSpec for Ch3ccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3ccfg::R`](R) reader structure"]
impl crate::Readable for Ch3ccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3ccfg::W`](W) writer structure"]
impl crate::Writable for Ch3ccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH3CCFG to value 0"]
impl crate::Resettable for Ch3ccfgSpec {
    const RESET_VALUE: u32 = 0;
}

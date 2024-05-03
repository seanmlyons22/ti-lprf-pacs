#[doc = "Register `SCEWEVCFG1` reader"]
pub type R = crate::R<Scewevcfg1Spec>;
#[doc = "Register `SCEWEVCFG1` writer"]
pub type W = crate::W<Scewevcfg1Spec>;
#[doc = "5:0\\]
Select the event source from the synchronous event bus to be used in event equation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev1Sel {
    #[doc = "63: EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY"]
    AuxTimer2ClkswitchRdy = 63,
    #[doc = "62: EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    AuxDacHoldActive = 62,
    #[doc = "61: EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    AuxSmphAutotakeDone = 61,
    #[doc = "60: EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    AuxAdcFifoNotEmpty = 60,
    #[doc = "59: EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    AuxAdcFifoAlmostFull = 59,
    #[doc = "58: EVSTAT3.AUX_ADC_IRQ"]
    AuxAdcIrq = 58,
    #[doc = "57: EVSTAT3.AUX_ADC_DONE"]
    AuxAdcDone = 57,
    #[doc = "56: EVSTAT3.AUX_ISRC_RESET_N"]
    AuxIsrcResetN = 56,
    #[doc = "55: EVSTAT3.AUX_TDC_DONE"]
    AuxTdcDone = 55,
    #[doc = "54: EVSTAT3.AUX_TIMER0_EV"]
    AuxTimer0Ev = 54,
    #[doc = "53: EVSTAT3.AUX_TIMER1_EV"]
    AuxTimer1Ev = 53,
    #[doc = "52: EVSTAT3.AUX_TIMER2_PULSE"]
    AuxTimer2Pulse = 52,
    #[doc = "51: EVSTAT3.AUX_TIMER2_EV3"]
    AuxTimer2Ev3 = 51,
    #[doc = "50: EVSTAT3.AUX_TIMER2_EV2"]
    AuxTimer2Ev2 = 50,
    #[doc = "49: EVSTAT3.AUX_TIMER2_EV1"]
    AuxTimer2Ev1 = 49,
    #[doc = "48: EVSTAT3.AUX_TIMER2_EV0"]
    AuxTimer2Ev0 = 48,
    #[doc = "47: EVSTAT2.AUX_COMPB"]
    AuxCompb = 47,
    #[doc = "46: EVSTAT2.AUX_COMPA"]
    AuxCompa = 46,
    #[doc = "45: EVSTAT2.MCU_OBSMUX1"]
    McuObsmux1 = 45,
    #[doc = "44: EVSTAT2.MCU_OBSMUX0"]
    McuObsmux0 = 44,
    #[doc = "43: EVSTAT2.MCU_EV"]
    McuEv = 43,
    #[doc = "42: EVSTAT2.ACLK_REF"]
    AclkRef = 42,
    #[doc = "41: EVSTAT2.VDDR_RECHARGE"]
    VddrRecharge = 41,
    #[doc = "40: EVSTAT2.MCU_ACTIVE"]
    McuActive = 40,
    #[doc = "39: EVSTAT2.PWR_DWN"]
    PwrDwn = 39,
    #[doc = "38: EVSTAT2.SCLK_LF"]
    SclkLf = 38,
    #[doc = "37: EVSTAT2.AON_BATMON_TEMP_UPD"]
    AonBatmonTempUpd = 37,
    #[doc = "36: EVSTAT2.AON_BATMON_BAT_UPD"]
    AonBatmonBatUpd = 36,
    #[doc = "35: EVSTAT2.AON_RTC_4KHZ"]
    AonRtc4khz = 35,
    #[doc = "34: EVSTAT2.AON_RTC_CH2_DLY"]
    AonRtcCh2Dly = 34,
    #[doc = "33: EVSTAT2.AON_RTC_CH2"]
    AonRtcCh2 = 33,
    #[doc = "32: Programmable delay event as described in PROGDLY"]
    AuxProgDlyIdle = 32,
    #[doc = "31: EVSTAT1.AUXIO31"]
    Auxio31 = 31,
    #[doc = "30: EVSTAT1.AUXIO30"]
    Auxio30 = 30,
    #[doc = "29: EVSTAT1.AUXIO29"]
    Auxio29 = 29,
    #[doc = "28: EVSTAT1.AUXIO28"]
    Auxio28 = 28,
    #[doc = "27: EVSTAT1.AUXIO27"]
    Auxio27 = 27,
    #[doc = "26: EVSTAT1.AUXIO26"]
    Auxio26 = 26,
    #[doc = "25: EVSTAT1.AUXIO25"]
    Auxio25 = 25,
    #[doc = "24: EVSTAT1.AUXIO24"]
    Auxio24 = 24,
    #[doc = "23: EVSTAT1.AUXIO23"]
    Auxio23 = 23,
    #[doc = "22: EVSTAT1.AUXIO22"]
    Auxio22 = 22,
    #[doc = "21: EVSTAT1.AUXIO21"]
    Auxio21 = 21,
    #[doc = "20: EVSTAT1.AUXIO20"]
    Auxio20 = 20,
    #[doc = "19: EVSTAT1.AUXIO19"]
    Auxio19 = 19,
    #[doc = "18: EVSTAT1.AUXIO18"]
    Auxio18 = 18,
    #[doc = "17: EVSTAT1.AUXIO17"]
    Auxio17 = 17,
    #[doc = "16: EVSTAT1.AUXIO16"]
    Auxio16 = 16,
    #[doc = "15: EVSTAT0.AUXIO15"]
    Auxio15 = 15,
    #[doc = "14: EVSTAT0.AUXIO14"]
    Auxio14 = 14,
    #[doc = "13: EVSTAT0.AUXIO13"]
    Auxio13 = 13,
    #[doc = "12: EVSTAT0.AUXIO12"]
    Auxio12 = 12,
    #[doc = "11: EVSTAT0.AUXIO11"]
    Auxio11 = 11,
    #[doc = "10: EVSTAT0.AUXIO10"]
    Auxio10 = 10,
    #[doc = "9: EVSTAT0.AUXIO9"]
    Auxio9 = 9,
    #[doc = "8: EVSTAT0.AUXIO8"]
    Auxio8 = 8,
    #[doc = "7: EVSTAT0.AUXIO7"]
    Auxio7 = 7,
    #[doc = "6: EVSTAT0.AUXIO6"]
    Auxio6 = 6,
    #[doc = "5: EVSTAT0.AUXIO5"]
    Auxio5 = 5,
    #[doc = "4: EVSTAT0.AUXIO4"]
    Auxio4 = 4,
    #[doc = "3: EVSTAT0.AUXIO3"]
    Auxio3 = 3,
    #[doc = "2: EVSTAT0.AUXIO2"]
    Auxio2 = 2,
    #[doc = "1: EVSTAT0.AUXIO1"]
    Auxio1 = 1,
    #[doc = "0: EVSTAT0.AUXIO0"]
    Auxio0 = 0,
}
impl From<Ev1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Ev1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev1Sel {
    type Ux = u8;
}
impl crate::IsEnum for Ev1Sel {}
#[doc = "Field `EV1_SEL` reader - 5:0\\]
Select the event source from the synchronous event bus to be used in event equation."]
pub type Ev1SelR = crate::FieldReader<Ev1Sel>;
impl Ev1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev1Sel {
        match self.bits {
            63 => Ev1Sel::AuxTimer2ClkswitchRdy,
            62 => Ev1Sel::AuxDacHoldActive,
            61 => Ev1Sel::AuxSmphAutotakeDone,
            60 => Ev1Sel::AuxAdcFifoNotEmpty,
            59 => Ev1Sel::AuxAdcFifoAlmostFull,
            58 => Ev1Sel::AuxAdcIrq,
            57 => Ev1Sel::AuxAdcDone,
            56 => Ev1Sel::AuxIsrcResetN,
            55 => Ev1Sel::AuxTdcDone,
            54 => Ev1Sel::AuxTimer0Ev,
            53 => Ev1Sel::AuxTimer1Ev,
            52 => Ev1Sel::AuxTimer2Pulse,
            51 => Ev1Sel::AuxTimer2Ev3,
            50 => Ev1Sel::AuxTimer2Ev2,
            49 => Ev1Sel::AuxTimer2Ev1,
            48 => Ev1Sel::AuxTimer2Ev0,
            47 => Ev1Sel::AuxCompb,
            46 => Ev1Sel::AuxCompa,
            45 => Ev1Sel::McuObsmux1,
            44 => Ev1Sel::McuObsmux0,
            43 => Ev1Sel::McuEv,
            42 => Ev1Sel::AclkRef,
            41 => Ev1Sel::VddrRecharge,
            40 => Ev1Sel::McuActive,
            39 => Ev1Sel::PwrDwn,
            38 => Ev1Sel::SclkLf,
            37 => Ev1Sel::AonBatmonTempUpd,
            36 => Ev1Sel::AonBatmonBatUpd,
            35 => Ev1Sel::AonRtc4khz,
            34 => Ev1Sel::AonRtcCh2Dly,
            33 => Ev1Sel::AonRtcCh2,
            32 => Ev1Sel::AuxProgDlyIdle,
            31 => Ev1Sel::Auxio31,
            30 => Ev1Sel::Auxio30,
            29 => Ev1Sel::Auxio29,
            28 => Ev1Sel::Auxio28,
            27 => Ev1Sel::Auxio27,
            26 => Ev1Sel::Auxio26,
            25 => Ev1Sel::Auxio25,
            24 => Ev1Sel::Auxio24,
            23 => Ev1Sel::Auxio23,
            22 => Ev1Sel::Auxio22,
            21 => Ev1Sel::Auxio21,
            20 => Ev1Sel::Auxio20,
            19 => Ev1Sel::Auxio19,
            18 => Ev1Sel::Auxio18,
            17 => Ev1Sel::Auxio17,
            16 => Ev1Sel::Auxio16,
            15 => Ev1Sel::Auxio15,
            14 => Ev1Sel::Auxio14,
            13 => Ev1Sel::Auxio13,
            12 => Ev1Sel::Auxio12,
            11 => Ev1Sel::Auxio11,
            10 => Ev1Sel::Auxio10,
            9 => Ev1Sel::Auxio9,
            8 => Ev1Sel::Auxio8,
            7 => Ev1Sel::Auxio7,
            6 => Ev1Sel::Auxio6,
            5 => Ev1Sel::Auxio5,
            4 => Ev1Sel::Auxio4,
            3 => Ev1Sel::Auxio3,
            2 => Ev1Sel::Auxio2,
            1 => Ev1Sel::Auxio1,
            0 => Ev1Sel::Auxio0,
            _ => unreachable!(),
        }
    }
    #[doc = "EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY"]
    #[inline(always)]
    pub fn is_aux_timer2_clkswitch_rdy(&self) -> bool {
        *self == Ev1Sel::AuxTimer2ClkswitchRdy
    }
    #[doc = "EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    #[inline(always)]
    pub fn is_aux_dac_hold_active(&self) -> bool {
        *self == Ev1Sel::AuxDacHoldActive
    }
    #[doc = "EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == Ev1Sel::AuxSmphAutotakeDone
    }
    #[doc = "EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == Ev1Sel::AuxAdcFifoNotEmpty
    }
    #[doc = "EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == Ev1Sel::AuxAdcFifoAlmostFull
    }
    #[doc = "EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == Ev1Sel::AuxAdcIrq
    }
    #[doc = "EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == Ev1Sel::AuxAdcDone
    }
    #[doc = "EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == Ev1Sel::AuxIsrcResetN
    }
    #[doc = "EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == Ev1Sel::AuxTdcDone
    }
    #[doc = "EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == Ev1Sel::AuxTimer0Ev
    }
    #[doc = "EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == Ev1Sel::AuxTimer1Ev
    }
    #[doc = "EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == Ev1Sel::AuxTimer2Pulse
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == Ev1Sel::AuxTimer2Ev3
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == Ev1Sel::AuxTimer2Ev2
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == Ev1Sel::AuxTimer2Ev1
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == Ev1Sel::AuxTimer2Ev0
    }
    #[doc = "EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Ev1Sel::AuxCompb
    }
    #[doc = "EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Ev1Sel::AuxCompa
    }
    #[doc = "EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn is_mcu_obsmux1(&self) -> bool {
        *self == Ev1Sel::McuObsmux1
    }
    #[doc = "EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn is_mcu_obsmux0(&self) -> bool {
        *self == Ev1Sel::McuObsmux0
    }
    #[doc = "EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == Ev1Sel::McuEv
    }
    #[doc = "EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == Ev1Sel::AclkRef
    }
    #[doc = "EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == Ev1Sel::VddrRecharge
    }
    #[doc = "EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn is_mcu_active(&self) -> bool {
        *self == Ev1Sel::McuActive
    }
    #[doc = "EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == Ev1Sel::PwrDwn
    }
    #[doc = "EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == Ev1Sel::SclkLf
    }
    #[doc = "EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline(always)]
    pub fn is_aon_batmon_temp_upd(&self) -> bool {
        *self == Ev1Sel::AonBatmonTempUpd
    }
    #[doc = "EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline(always)]
    pub fn is_aon_batmon_bat_upd(&self) -> bool {
        *self == Ev1Sel::AonBatmonBatUpd
    }
    #[doc = "EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == Ev1Sel::AonRtc4khz
    }
    #[doc = "EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == Ev1Sel::AonRtcCh2Dly
    }
    #[doc = "EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == Ev1Sel::AonRtcCh2
    }
    #[doc = "Programmable delay event as described in PROGDLY"]
    #[inline(always)]
    pub fn is_aux_prog_dly_idle(&self) -> bool {
        *self == Ev1Sel::AuxProgDlyIdle
    }
    #[doc = "EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn is_auxio31(&self) -> bool {
        *self == Ev1Sel::Auxio31
    }
    #[doc = "EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn is_auxio30(&self) -> bool {
        *self == Ev1Sel::Auxio30
    }
    #[doc = "EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn is_auxio29(&self) -> bool {
        *self == Ev1Sel::Auxio29
    }
    #[doc = "EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn is_auxio28(&self) -> bool {
        *self == Ev1Sel::Auxio28
    }
    #[doc = "EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn is_auxio27(&self) -> bool {
        *self == Ev1Sel::Auxio27
    }
    #[doc = "EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == Ev1Sel::Auxio26
    }
    #[doc = "EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == Ev1Sel::Auxio25
    }
    #[doc = "EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == Ev1Sel::Auxio24
    }
    #[doc = "EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == Ev1Sel::Auxio23
    }
    #[doc = "EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == Ev1Sel::Auxio22
    }
    #[doc = "EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == Ev1Sel::Auxio21
    }
    #[doc = "EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == Ev1Sel::Auxio20
    }
    #[doc = "EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == Ev1Sel::Auxio19
    }
    #[doc = "EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn is_auxio18(&self) -> bool {
        *self == Ev1Sel::Auxio18
    }
    #[doc = "EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn is_auxio17(&self) -> bool {
        *self == Ev1Sel::Auxio17
    }
    #[doc = "EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn is_auxio16(&self) -> bool {
        *self == Ev1Sel::Auxio16
    }
    #[doc = "EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == Ev1Sel::Auxio15
    }
    #[doc = "EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == Ev1Sel::Auxio14
    }
    #[doc = "EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == Ev1Sel::Auxio13
    }
    #[doc = "EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == Ev1Sel::Auxio12
    }
    #[doc = "EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == Ev1Sel::Auxio11
    }
    #[doc = "EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == Ev1Sel::Auxio10
    }
    #[doc = "EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == Ev1Sel::Auxio9
    }
    #[doc = "EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == Ev1Sel::Auxio8
    }
    #[doc = "EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == Ev1Sel::Auxio7
    }
    #[doc = "EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == Ev1Sel::Auxio6
    }
    #[doc = "EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == Ev1Sel::Auxio5
    }
    #[doc = "EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == Ev1Sel::Auxio4
    }
    #[doc = "EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == Ev1Sel::Auxio3
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == Ev1Sel::Auxio2
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == Ev1Sel::Auxio1
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == Ev1Sel::Auxio0
    }
}
#[doc = "Field `EV1_SEL` writer - 5:0\\]
Select the event source from the synchronous event bus to be used in event equation."]
pub type Ev1SelW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ev1Sel, crate::Safe>;
impl<'a, REG> Ev1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY"]
    #[inline(always)]
    pub fn aux_timer2_clkswitch_rdy(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxTimer2ClkswitchRdy)
    }
    #[doc = "EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    #[inline(always)]
    pub fn aux_dac_hold_active(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxDacHoldActive)
    }
    #[doc = "EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxSmphAutotakeDone)
    }
    #[doc = "EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxAdcFifoNotEmpty)
    }
    #[doc = "EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxAdcFifoAlmostFull)
    }
    #[doc = "EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn aux_adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxAdcIrq)
    }
    #[doc = "EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxAdcDone)
    }
    #[doc = "EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxIsrcResetN)
    }
    #[doc = "EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxTdcDone)
    }
    #[doc = "EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxTimer0Ev)
    }
    #[doc = "EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxTimer1Ev)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxTimer2Pulse)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxTimer2Ev3)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxTimer2Ev2)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxTimer2Ev1)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxTimer2Ev0)
    }
    #[doc = "EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxCompb)
    }
    #[doc = "EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxCompa)
    }
    #[doc = "EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn mcu_obsmux1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::McuObsmux1)
    }
    #[doc = "EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn mcu_obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::McuObsmux0)
    }
    #[doc = "EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::McuEv)
    }
    #[doc = "EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AclkRef)
    }
    #[doc = "EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn vddr_recharge(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::VddrRecharge)
    }
    #[doc = "EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn mcu_active(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::McuActive)
    }
    #[doc = "EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn pwr_dwn(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::PwrDwn)
    }
    #[doc = "EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::SclkLf)
    }
    #[doc = "EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline(always)]
    pub fn aon_batmon_temp_upd(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AonBatmonTempUpd)
    }
    #[doc = "EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline(always)]
    pub fn aon_batmon_bat_upd(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AonBatmonBatUpd)
    }
    #[doc = "EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn aon_rtc_4khz(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AonRtc4khz)
    }
    #[doc = "EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AonRtcCh2Dly)
    }
    #[doc = "EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AonRtcCh2)
    }
    #[doc = "Programmable delay event as described in PROGDLY"]
    #[inline(always)]
    pub fn aux_prog_dly_idle(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::AuxProgDlyIdle)
    }
    #[doc = "EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn auxio31(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio31)
    }
    #[doc = "EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn auxio30(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio30)
    }
    #[doc = "EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn auxio29(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio29)
    }
    #[doc = "EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn auxio28(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio28)
    }
    #[doc = "EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn auxio27(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio27)
    }
    #[doc = "EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio26)
    }
    #[doc = "EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio25)
    }
    #[doc = "EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio24)
    }
    #[doc = "EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio23)
    }
    #[doc = "EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio22)
    }
    #[doc = "EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio21)
    }
    #[doc = "EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio20)
    }
    #[doc = "EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio19)
    }
    #[doc = "EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn auxio18(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio18)
    }
    #[doc = "EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn auxio17(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio17)
    }
    #[doc = "EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn auxio16(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio16)
    }
    #[doc = "EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio15)
    }
    #[doc = "EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio14)
    }
    #[doc = "EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio13)
    }
    #[doc = "EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio12)
    }
    #[doc = "EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio11)
    }
    #[doc = "EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio10)
    }
    #[doc = "EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio9)
    }
    #[doc = "EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio8)
    }
    #[doc = "EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio7)
    }
    #[doc = "EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio6)
    }
    #[doc = "EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio5)
    }
    #[doc = "EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio4)
    }
    #[doc = "EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1Sel::Auxio0)
    }
}
#[doc = "Field `EV1_POL` reader - 6:6\\]
Polarity of EV1_SEL event. When SCEWEVCFG0.COMB_EV_EN is 0: 0: Non-inverted. 1: Non-inverted. When SCEWEVCFG0.COMB_EV_EN is 1. 0: Non-inverted. 1: Inverted."]
pub type Ev1PolR = crate::BitReader;
#[doc = "Field `EV1_POL` writer - 6:6\\]
Polarity of EV1_SEL event. When SCEWEVCFG0.COMB_EV_EN is 0: 0: Non-inverted. 1: Non-inverted. When SCEWEVCFG0.COMB_EV_EN is 1. 0: Non-inverted. 1: Inverted."]
pub type Ev1PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EV0_POL` reader - 7:7\\]
Polarity of SCEWEVCFG0.EV0_SEL event. When SCEWEVCFG0.COMB_EV_EN is 0: 0: Non-inverted. 1: Non-inverted. When SCEWEVCFG0.COMB_EV_EN is 1. 0: Non-inverted. 1: Inverted."]
pub type Ev0PolR = crate::BitReader;
#[doc = "Field `EV0_POL` writer - 7:7\\]
Polarity of SCEWEVCFG0.EV0_SEL event. When SCEWEVCFG0.COMB_EV_EN is 0: 0: Non-inverted. 1: Non-inverted. When SCEWEVCFG0.COMB_EV_EN is 1. 0: Non-inverted. 1: Inverted."]
pub type Ev0PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Select the event source from the synchronous event bus to be used in event equation."]
    #[inline(always)]
    pub fn ev1_sel(&self) -> Ev1SelR {
        Ev1SelR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Polarity of EV1_SEL event. When SCEWEVCFG0.COMB_EV_EN is 0: 0: Non-inverted. 1: Non-inverted. When SCEWEVCFG0.COMB_EV_EN is 1. 0: Non-inverted. 1: Inverted."]
    #[inline(always)]
    pub fn ev1_pol(&self) -> Ev1PolR {
        Ev1PolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Polarity of SCEWEVCFG0.EV0_SEL event. When SCEWEVCFG0.COMB_EV_EN is 0: 0: Non-inverted. 1: Non-inverted. When SCEWEVCFG0.COMB_EV_EN is 1. 0: Non-inverted. 1: Inverted."]
    #[inline(always)]
    pub fn ev0_pol(&self) -> Ev0PolR {
        Ev0PolR::new(((self.bits >> 7) & 1) != 0)
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
Select the event source from the synchronous event bus to be used in event equation."]
    #[inline(always)]
    #[must_use]
    pub fn ev1_sel(&mut self) -> Ev1SelW<Scewevcfg1Spec> {
        Ev1SelW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Polarity of EV1_SEL event. When SCEWEVCFG0.COMB_EV_EN is 0: 0: Non-inverted. 1: Non-inverted. When SCEWEVCFG0.COMB_EV_EN is 1. 0: Non-inverted. 1: Inverted."]
    #[inline(always)]
    #[must_use]
    pub fn ev1_pol(&mut self) -> Ev1PolW<Scewevcfg1Spec> {
        Ev1PolW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Polarity of SCEWEVCFG0.EV0_SEL event. When SCEWEVCFG0.COMB_EV_EN is 0: 0: Non-inverted. 1: Non-inverted. When SCEWEVCFG0.COMB_EV_EN is 1. 0: Non-inverted. 1: Inverted."]
    #[inline(always)]
    #[must_use]
    pub fn ev0_pol(&mut self) -> Ev0PolW<Scewevcfg1Spec> {
        Ev0PolW::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Scewevcfg1Spec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Sensor Controller Engine Wait Event Configuration 1 See SCEWEVCFG0 for description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scewevcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scewevcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scewevcfg1Spec;
impl crate::RegisterSpec for Scewevcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scewevcfg1::R`](R) reader structure"]
impl crate::Readable for Scewevcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`scewevcfg1::W`](W) writer structure"]
impl crate::Writable for Scewevcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCEWEVCFG1 to value 0"]
impl crate::Resettable for Scewevcfg1Spec {
    const RESET_VALUE: u32 = 0;
}

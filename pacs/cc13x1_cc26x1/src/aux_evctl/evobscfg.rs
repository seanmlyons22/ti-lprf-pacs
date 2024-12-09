#[doc = "Register `EVOBSCFG` reader"]
pub type R = crate::R<EvobscfgSpec>;
#[doc = "Register `EVOBSCFG` writer"]
pub type W = crate::W<EvobscfgSpec>;
#[doc = "5:0\\]
Select which event from the asynchronous event bus that represents AUX_EV_OBS in AUX_AIODIOn.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EvobsSel {
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
    #[doc = "35: EVSTAT2.AON_RTC_4KHZ"]
    AonRtc4khz = 35,
    #[doc = "34: EVSTAT2.AON_RTC_CH2_DLY"]
    AonRtcCh2Dly = 34,
    #[doc = "33: EVSTAT2.AON_RTC_CH2"]
    AonRtcCh2 = 33,
    #[doc = "32: EVSTAT2.MANUAL_EV"]
    ManualEv = 32,
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
impl From<EvobsSel> for u8 {
    #[inline(always)]
    fn from(variant: EvobsSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EvobsSel {
    type Ux = u8;
}
impl crate::IsEnum for EvobsSel {}
#[doc = "Field `EVOBS_SEL` reader - 5:0\\]
Select which event from the asynchronous event bus that represents AUX_EV_OBS in AUX_AIODIOn."]
pub type EvobsSelR = crate::FieldReader<EvobsSel>;
impl EvobsSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EvobsSel> {
        match self.bits {
            62 => Some(EvobsSel::AuxDacHoldActive),
            61 => Some(EvobsSel::AuxSmphAutotakeDone),
            60 => Some(EvobsSel::AuxAdcFifoNotEmpty),
            59 => Some(EvobsSel::AuxAdcFifoAlmostFull),
            58 => Some(EvobsSel::AuxAdcIrq),
            57 => Some(EvobsSel::AuxAdcDone),
            56 => Some(EvobsSel::AuxIsrcResetN),
            55 => Some(EvobsSel::AuxTdcDone),
            54 => Some(EvobsSel::AuxTimer0Ev),
            53 => Some(EvobsSel::AuxTimer1Ev),
            47 => Some(EvobsSel::AuxCompb),
            46 => Some(EvobsSel::AuxCompa),
            45 => Some(EvobsSel::McuObsmux1),
            44 => Some(EvobsSel::McuObsmux0),
            43 => Some(EvobsSel::McuEv),
            42 => Some(EvobsSel::AclkRef),
            41 => Some(EvobsSel::VddrRecharge),
            40 => Some(EvobsSel::McuActive),
            39 => Some(EvobsSel::PwrDwn),
            38 => Some(EvobsSel::SclkLf),
            35 => Some(EvobsSel::AonRtc4khz),
            34 => Some(EvobsSel::AonRtcCh2Dly),
            33 => Some(EvobsSel::AonRtcCh2),
            32 => Some(EvobsSel::ManualEv),
            31 => Some(EvobsSel::Auxio31),
            30 => Some(EvobsSel::Auxio30),
            29 => Some(EvobsSel::Auxio29),
            28 => Some(EvobsSel::Auxio28),
            27 => Some(EvobsSel::Auxio27),
            26 => Some(EvobsSel::Auxio26),
            25 => Some(EvobsSel::Auxio25),
            24 => Some(EvobsSel::Auxio24),
            23 => Some(EvobsSel::Auxio23),
            22 => Some(EvobsSel::Auxio22),
            21 => Some(EvobsSel::Auxio21),
            20 => Some(EvobsSel::Auxio20),
            19 => Some(EvobsSel::Auxio19),
            18 => Some(EvobsSel::Auxio18),
            17 => Some(EvobsSel::Auxio17),
            16 => Some(EvobsSel::Auxio16),
            15 => Some(EvobsSel::Auxio15),
            14 => Some(EvobsSel::Auxio14),
            13 => Some(EvobsSel::Auxio13),
            12 => Some(EvobsSel::Auxio12),
            11 => Some(EvobsSel::Auxio11),
            10 => Some(EvobsSel::Auxio10),
            9 => Some(EvobsSel::Auxio9),
            8 => Some(EvobsSel::Auxio8),
            7 => Some(EvobsSel::Auxio7),
            6 => Some(EvobsSel::Auxio6),
            5 => Some(EvobsSel::Auxio5),
            4 => Some(EvobsSel::Auxio4),
            3 => Some(EvobsSel::Auxio3),
            2 => Some(EvobsSel::Auxio2),
            1 => Some(EvobsSel::Auxio1),
            0 => Some(EvobsSel::Auxio0),
            _ => None,
        }
    }
    #[doc = "EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    #[inline(always)]
    pub fn is_aux_dac_hold_active(&self) -> bool {
        *self == EvobsSel::AuxDacHoldActive
    }
    #[doc = "EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == EvobsSel::AuxSmphAutotakeDone
    }
    #[doc = "EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == EvobsSel::AuxAdcFifoNotEmpty
    }
    #[doc = "EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == EvobsSel::AuxAdcFifoAlmostFull
    }
    #[doc = "EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == EvobsSel::AuxAdcIrq
    }
    #[doc = "EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == EvobsSel::AuxAdcDone
    }
    #[doc = "EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == EvobsSel::AuxIsrcResetN
    }
    #[doc = "EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == EvobsSel::AuxTdcDone
    }
    #[doc = "EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == EvobsSel::AuxTimer0Ev
    }
    #[doc = "EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == EvobsSel::AuxTimer1Ev
    }
    #[doc = "EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == EvobsSel::AuxCompb
    }
    #[doc = "EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == EvobsSel::AuxCompa
    }
    #[doc = "EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn is_mcu_obsmux1(&self) -> bool {
        *self == EvobsSel::McuObsmux1
    }
    #[doc = "EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn is_mcu_obsmux0(&self) -> bool {
        *self == EvobsSel::McuObsmux0
    }
    #[doc = "EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == EvobsSel::McuEv
    }
    #[doc = "EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == EvobsSel::AclkRef
    }
    #[doc = "EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == EvobsSel::VddrRecharge
    }
    #[doc = "EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn is_mcu_active(&self) -> bool {
        *self == EvobsSel::McuActive
    }
    #[doc = "EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == EvobsSel::PwrDwn
    }
    #[doc = "EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == EvobsSel::SclkLf
    }
    #[doc = "EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == EvobsSel::AonRtc4khz
    }
    #[doc = "EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == EvobsSel::AonRtcCh2Dly
    }
    #[doc = "EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == EvobsSel::AonRtcCh2
    }
    #[doc = "EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn is_manual_ev(&self) -> bool {
        *self == EvobsSel::ManualEv
    }
    #[doc = "EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn is_auxio31(&self) -> bool {
        *self == EvobsSel::Auxio31
    }
    #[doc = "EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn is_auxio30(&self) -> bool {
        *self == EvobsSel::Auxio30
    }
    #[doc = "EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn is_auxio29(&self) -> bool {
        *self == EvobsSel::Auxio29
    }
    #[doc = "EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn is_auxio28(&self) -> bool {
        *self == EvobsSel::Auxio28
    }
    #[doc = "EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn is_auxio27(&self) -> bool {
        *self == EvobsSel::Auxio27
    }
    #[doc = "EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == EvobsSel::Auxio26
    }
    #[doc = "EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == EvobsSel::Auxio25
    }
    #[doc = "EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == EvobsSel::Auxio24
    }
    #[doc = "EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == EvobsSel::Auxio23
    }
    #[doc = "EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == EvobsSel::Auxio22
    }
    #[doc = "EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == EvobsSel::Auxio21
    }
    #[doc = "EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == EvobsSel::Auxio20
    }
    #[doc = "EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == EvobsSel::Auxio19
    }
    #[doc = "EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn is_auxio18(&self) -> bool {
        *self == EvobsSel::Auxio18
    }
    #[doc = "EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn is_auxio17(&self) -> bool {
        *self == EvobsSel::Auxio17
    }
    #[doc = "EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn is_auxio16(&self) -> bool {
        *self == EvobsSel::Auxio16
    }
    #[doc = "EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == EvobsSel::Auxio15
    }
    #[doc = "EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == EvobsSel::Auxio14
    }
    #[doc = "EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == EvobsSel::Auxio13
    }
    #[doc = "EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == EvobsSel::Auxio12
    }
    #[doc = "EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == EvobsSel::Auxio11
    }
    #[doc = "EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == EvobsSel::Auxio10
    }
    #[doc = "EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == EvobsSel::Auxio9
    }
    #[doc = "EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == EvobsSel::Auxio8
    }
    #[doc = "EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == EvobsSel::Auxio7
    }
    #[doc = "EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == EvobsSel::Auxio6
    }
    #[doc = "EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == EvobsSel::Auxio5
    }
    #[doc = "EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == EvobsSel::Auxio4
    }
    #[doc = "EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == EvobsSel::Auxio3
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == EvobsSel::Auxio2
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == EvobsSel::Auxio1
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == EvobsSel::Auxio0
    }
}
#[doc = "Field `EVOBS_SEL` writer - 5:0\\]
Select which event from the asynchronous event bus that represents AUX_EV_OBS in AUX_AIODIOn."]
pub type EvobsSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, EvobsSel>;
impl<'a, REG> EvobsSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    #[inline(always)]
    pub fn aux_dac_hold_active(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AuxDacHoldActive)
    }
    #[doc = "EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AuxSmphAutotakeDone)
    }
    #[doc = "EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AuxAdcFifoNotEmpty)
    }
    #[doc = "EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AuxAdcFifoAlmostFull)
    }
    #[doc = "EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn aux_adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AuxAdcIrq)
    }
    #[doc = "EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AuxAdcDone)
    }
    #[doc = "EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AuxIsrcResetN)
    }
    #[doc = "EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AuxTdcDone)
    }
    #[doc = "EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AuxTimer0Ev)
    }
    #[doc = "EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AuxTimer1Ev)
    }
    #[doc = "EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AuxCompb)
    }
    #[doc = "EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AuxCompa)
    }
    #[doc = "EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn mcu_obsmux1(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::McuObsmux1)
    }
    #[doc = "EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn mcu_obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::McuObsmux0)
    }
    #[doc = "EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::McuEv)
    }
    #[doc = "EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AclkRef)
    }
    #[doc = "EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn vddr_recharge(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::VddrRecharge)
    }
    #[doc = "EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn mcu_active(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::McuActive)
    }
    #[doc = "EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn pwr_dwn(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::PwrDwn)
    }
    #[doc = "EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::SclkLf)
    }
    #[doc = "EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn aon_rtc_4khz(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AonRtc4khz)
    }
    #[doc = "EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AonRtcCh2Dly)
    }
    #[doc = "EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::AonRtcCh2)
    }
    #[doc = "EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn manual_ev(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::ManualEv)
    }
    #[doc = "EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn auxio31(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio31)
    }
    #[doc = "EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn auxio30(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio30)
    }
    #[doc = "EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn auxio29(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio29)
    }
    #[doc = "EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn auxio28(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio28)
    }
    #[doc = "EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn auxio27(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio27)
    }
    #[doc = "EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio26)
    }
    #[doc = "EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio25)
    }
    #[doc = "EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio24)
    }
    #[doc = "EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio23)
    }
    #[doc = "EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio22)
    }
    #[doc = "EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio21)
    }
    #[doc = "EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio20)
    }
    #[doc = "EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio19)
    }
    #[doc = "EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn auxio18(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio18)
    }
    #[doc = "EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn auxio17(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio17)
    }
    #[doc = "EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn auxio16(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio16)
    }
    #[doc = "EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio15)
    }
    #[doc = "EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio14)
    }
    #[doc = "EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio13)
    }
    #[doc = "EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio12)
    }
    #[doc = "EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio11)
    }
    #[doc = "EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio10)
    }
    #[doc = "EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio9)
    }
    #[doc = "EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio8)
    }
    #[doc = "EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio7)
    }
    #[doc = "EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio6)
    }
    #[doc = "EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio5)
    }
    #[doc = "EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio4)
    }
    #[doc = "EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(EvobsSel::Auxio0)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Select which event from the asynchronous event bus that represents AUX_EV_OBS in AUX_AIODIOn."]
    #[inline(always)]
    pub fn evobs_sel(&self) -> EvobsSelR {
        EvobsSelR::new((self.bits & 0x3f) as u8)
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
Select which event from the asynchronous event bus that represents AUX_EV_OBS in AUX_AIODIOn."]
    #[inline(always)]
    #[must_use]
    pub fn evobs_sel(&mut self) -> EvobsSelW<EvobscfgSpec> {
        EvobsSelW::new(self, 0)
    }
}
#[doc = "Event Observation Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evobscfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evobscfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvobscfgSpec;
impl crate::RegisterSpec for EvobscfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evobscfg::R`](R) reader structure"]
impl crate::Readable for EvobscfgSpec {}
#[doc = "`write(|w| ..)` method takes [`evobscfg::W`](W) writer structure"]
impl crate::Writable for EvobscfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVOBSCFG to value 0"]
impl crate::Resettable for EvobscfgSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `PRECTL` reader"]
pub type R = crate::R<PrectlSpec>;
#[doc = "Register `PRECTL` writer"]
pub type W = crate::W<PrectlSpec>;
#[doc = "5:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0.\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
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
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - 5:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            63 => Some(Src::NoEvent),
            61 => Some(Src::AuxSmphAutotakeDone),
            60 => Some(Src::AuxAdcFifoNotEmpty),
            59 => Some(Src::AuxAdcFifoAlmostFull),
            58 => Some(Src::AuxAdcIrq),
            57 => Some(Src::AuxAdcDone),
            56 => Some(Src::AuxIsrcResetN),
            55 => Some(Src::AuxTdcDone),
            54 => Some(Src::AuxTimer0Ev),
            53 => Some(Src::AuxTimer1Ev),
            47 => Some(Src::AuxCompb),
            46 => Some(Src::AuxCompa),
            45 => Some(Src::McuObsmux1),
            44 => Some(Src::McuObsmux0),
            43 => Some(Src::McuEv),
            42 => Some(Src::AclkRef),
            41 => Some(Src::VddrRecharge),
            40 => Some(Src::McuActive),
            39 => Some(Src::PwrDwn),
            38 => Some(Src::SclkLf),
            35 => Some(Src::AonRtc4khz),
            34 => Some(Src::AonRtcCh2Dly),
            33 => Some(Src::AonRtcCh2),
            32 => Some(Src::ManualEv),
            31 => Some(Src::Auxio31),
            30 => Some(Src::Auxio30),
            29 => Some(Src::Auxio29),
            28 => Some(Src::Auxio28),
            27 => Some(Src::Auxio27),
            26 => Some(Src::Auxio26),
            25 => Some(Src::Auxio25),
            24 => Some(Src::Auxio24),
            23 => Some(Src::Auxio23),
            22 => Some(Src::Auxio22),
            21 => Some(Src::Auxio21),
            20 => Some(Src::Auxio20),
            19 => Some(Src::Auxio19),
            18 => Some(Src::Auxio18),
            17 => Some(Src::Auxio17),
            16 => Some(Src::Auxio16),
            15 => Some(Src::Auxio15),
            14 => Some(Src::Auxio14),
            13 => Some(Src::Auxio13),
            12 => Some(Src::Auxio12),
            11 => Some(Src::Auxio11),
            10 => Some(Src::Auxio10),
            9 => Some(Src::Auxio9),
            8 => Some(Src::Auxio8),
            7 => Some(Src::Auxio7),
            6 => Some(Src::Auxio6),
            5 => Some(Src::Auxio5),
            4 => Some(Src::Auxio4),
            3 => Some(Src::Auxio3),
            2 => Some(Src::Auxio2),
            1 => Some(Src::Auxio1),
            0 => Some(Src::Auxio0),
            _ => None,
        }
    }
    #[doc = "No event."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == Src::NoEvent
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == Src::AuxSmphAutotakeDone
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == Src::AuxAdcFifoNotEmpty
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == Src::AuxAdcFifoAlmostFull
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == Src::AuxAdcIrq
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == Src::AuxAdcDone
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == Src::AuxIsrcResetN
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == Src::AuxTdcDone
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == Src::AuxTimer0Ev
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == Src::AuxTimer1Ev
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Src::AuxCompb
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Src::AuxCompa
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn is_mcu_obsmux1(&self) -> bool {
        *self == Src::McuObsmux1
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn is_mcu_obsmux0(&self) -> bool {
        *self == Src::McuObsmux0
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == Src::McuEv
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == Src::AclkRef
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == Src::VddrRecharge
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn is_mcu_active(&self) -> bool {
        *self == Src::McuActive
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == Src::PwrDwn
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == Src::SclkLf
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == Src::AonRtc4khz
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == Src::AonRtcCh2Dly
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == Src::AonRtcCh2
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn is_manual_ev(&self) -> bool {
        *self == Src::ManualEv
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn is_auxio31(&self) -> bool {
        *self == Src::Auxio31
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn is_auxio30(&self) -> bool {
        *self == Src::Auxio30
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn is_auxio29(&self) -> bool {
        *self == Src::Auxio29
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn is_auxio28(&self) -> bool {
        *self == Src::Auxio28
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn is_auxio27(&self) -> bool {
        *self == Src::Auxio27
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == Src::Auxio26
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == Src::Auxio25
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == Src::Auxio24
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == Src::Auxio23
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == Src::Auxio22
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == Src::Auxio21
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == Src::Auxio20
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == Src::Auxio19
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn is_auxio18(&self) -> bool {
        *self == Src::Auxio18
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn is_auxio17(&self) -> bool {
        *self == Src::Auxio17
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn is_auxio16(&self) -> bool {
        *self == Src::Auxio16
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == Src::Auxio15
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == Src::Auxio14
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == Src::Auxio13
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == Src::Auxio12
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == Src::Auxio11
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == Src::Auxio10
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == Src::Auxio9
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == Src::Auxio8
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == Src::Auxio7
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == Src::Auxio6
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == Src::Auxio5
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == Src::Auxio4
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == Src::Auxio3
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == Src::Auxio2
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == Src::Auxio1
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == Src::Auxio0
    }
}
#[doc = "Field `SRC` writer - 5:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 6, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(Src::NoEvent)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxSmphAutotakeDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxAdcFifoNotEmpty)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxAdcFifoAlmostFull)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn aux_adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxAdcIrq)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxAdcDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxIsrcResetN)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxTdcDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxTimer0Ev)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxTimer1Ev)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxCompb)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxCompa)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn mcu_obsmux1(self) -> &'a mut crate::W<REG> {
        self.variant(Src::McuObsmux1)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn mcu_obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(Src::McuObsmux0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Src::McuEv)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AclkRef)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn vddr_recharge(self) -> &'a mut crate::W<REG> {
        self.variant(Src::VddrRecharge)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn mcu_active(self) -> &'a mut crate::W<REG> {
        self.variant(Src::McuActive)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn pwr_dwn(self) -> &'a mut crate::W<REG> {
        self.variant(Src::PwrDwn)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(Src::SclkLf)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn aon_rtc_4khz(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AonRtc4khz)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AonRtcCh2Dly)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AonRtcCh2)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn manual_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Src::ManualEv)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn auxio31(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio31)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn auxio30(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio30)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn auxio29(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio29)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn auxio28(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio28)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn auxio27(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio27)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio26)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio25)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio24)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio23)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio22)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio21)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio20)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio19)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn auxio18(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio18)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn auxio17(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio17)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn auxio16(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio16)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio15)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio14)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio13)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio12)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio11)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio10)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio9)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio8)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio7)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio6)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio5)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio4)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Auxio0)
    }
}
#[doc = "6:6\\]
Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ratio {
    #[doc = "1: Prescaler divides input by 64. AUX_TDC_PRE event has a rising edge for every 64 rising edges of the input. AUX_TDC_PRE event toggles on every 32nd rising edge of the input."]
    Div64 = 1,
    #[doc = "0: Prescaler divides input by 16. AUX_TDC_PRE event has a rising edge for every 16 rising edges of the input. AUX_TDC_PRE event toggles on every 8th rising edge of the input."]
    Div16 = 0,
}
impl From<Ratio> for bool {
    #[inline(always)]
    fn from(variant: Ratio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RATIO` reader - 6:6\\]
Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
pub type RatioR = crate::BitReader<Ratio>;
impl RatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ratio {
        match self.bits {
            true => Ratio::Div64,
            false => Ratio::Div16,
        }
    }
    #[doc = "Prescaler divides input by 64. AUX_TDC_PRE event has a rising edge for every 64 rising edges of the input. AUX_TDC_PRE event toggles on every 32nd rising edge of the input."]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Ratio::Div64
    }
    #[doc = "Prescaler divides input by 16. AUX_TDC_PRE event has a rising edge for every 16 rising edges of the input. AUX_TDC_PRE event toggles on every 8th rising edge of the input."]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Ratio::Div16
    }
}
#[doc = "Field `RATIO` writer - 6:6\\]
Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
pub type RatioW<'a, REG> = crate::BitWriter<'a, REG, Ratio>;
impl<'a, REG> RatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prescaler divides input by 64. AUX_TDC_PRE event has a rising edge for every 64 rising edges of the input. AUX_TDC_PRE event toggles on every 32nd rising edge of the input."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div64)
    }
    #[doc = "Prescaler divides input by 16. AUX_TDC_PRE event has a rising edge for every 16 rising edges of the input. AUX_TDC_PRE event toggles on every 8th rising edge of the input."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div16)
    }
}
#[doc = "Field `RESET_N` reader - 7:7\\]
Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
pub type ResetNR = crate::BitReader;
#[doc = "Field `RESET_N` writer - 7:7\\]
Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
pub type ResetNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
    #[inline(always)]
    pub fn ratio(&self) -> RatioR {
        RatioR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
    #[inline(always)]
    pub fn reset_n(&self) -> ResetNR {
        ResetNR::new(((self.bits >> 7) & 1) != 0)
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
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<PrectlSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RatioW<PrectlSpec> {
        RatioW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
    #[inline(always)]
    #[must_use]
    pub fn reset_n(&mut self) -> ResetNW<PrectlSpec> {
        ResetNW::new(self, 7)
    }
}
#[doc = "Prescaler Control The prescaler can be used to count events that are faster than the AUX bus rate. It can be used to: - count pulses on a specified event from the asynchronous event bus. - prescale a specified event from the asynchronous event bus. To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE. It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX bus rate.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prectl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prectl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrectlSpec;
impl crate::RegisterSpec for PrectlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prectl::R`](R) reader structure"]
impl crate::Readable for PrectlSpec {}
#[doc = "`write(|w| ..)` method takes [`prectl::W`](W) writer structure"]
impl crate::Writable for PrectlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRECTL to value 0x3f"]
impl crate::Resettable for PrectlSpec {
    const RESET_VALUE: u32 = 0x3f;
}

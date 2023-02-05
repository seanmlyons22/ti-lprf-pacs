#[doc = "Register `PRECTL` reader"]
pub struct R(crate::R<PRECTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRECTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRECTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRECTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRECTL` writer"]
pub struct W(crate::W<PRECTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRECTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PRECTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRECTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC` reader - 5:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
pub type SRC_R = crate::FieldReader<u8, SRC_A>;
#[doc = "5:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0.\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "63: No event."]
    NO_EVENT = 63,
    #[doc = "61: AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    AUX_SMPH_AUTOTAKE_DONE = 61,
    #[doc = "60: AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    AUX_ADC_FIFO_NOT_EMPTY = 60,
    #[doc = "59: AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    AUX_ADC_FIFO_ALMOST_FULL = 59,
    #[doc = "58: AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    AUX_ADC_IRQ = 58,
    #[doc = "57: AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    AUX_ADC_DONE = 57,
    #[doc = "56: AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    AUX_ISRC_RESET_N = 56,
    #[doc = "55: AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    AUX_TDC_DONE = 55,
    #[doc = "54: AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    AUX_TIMER0_EV = 54,
    #[doc = "53: AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    AUX_TIMER1_EV = 53,
    #[doc = "52: AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    AUX_TIMER2_PULSE = 52,
    #[doc = "51: AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    AUX_TIMER2_EV3 = 51,
    #[doc = "50: AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    AUX_TIMER2_EV2 = 50,
    #[doc = "49: AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    AUX_TIMER2_EV1 = 49,
    #[doc = "48: AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    AUX_TIMER2_EV0 = 48,
    #[doc = "47: AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    AUX_COMPB = 47,
    #[doc = "46: AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    AUX_COMPA = 46,
    #[doc = "45: AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    MCU_OBSMUX1 = 45,
    #[doc = "44: AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    MCU_OBSMUX0 = 44,
    #[doc = "43: AUX_EVCTL:EVSTAT2.MCU_EV"]
    MCU_EV = 43,
    #[doc = "42: AUX_EVCTL:EVSTAT2.ACLK_REF"]
    ACLK_REF = 42,
    #[doc = "41: AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    VDDR_RECHARGE = 41,
    #[doc = "40: AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    MCU_ACTIVE = 40,
    #[doc = "39: AUX_EVCTL:EVSTAT2.PWR_DWN"]
    PWR_DWN = 39,
    #[doc = "38: AUX_EVCTL:EVSTAT2.SCLK_LF"]
    SCLK_LF = 38,
    #[doc = "37: AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    AON_BATMON_TEMP_UPD = 37,
    #[doc = "36: AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    AON_BATMON_BAT_UPD = 36,
    #[doc = "35: AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    AON_RTC_4KHZ = 35,
    #[doc = "34: AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    AON_RTC_CH2_DLY = 34,
    #[doc = "33: AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    AON_RTC_CH2 = 33,
    #[doc = "32: AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    MANUAL_EV = 32,
    #[doc = "31: AUX_EVCTL:EVSTAT1.AUXIO31"]
    AUXIO31 = 31,
    #[doc = "30: AUX_EVCTL:EVSTAT1.AUXIO30"]
    AUXIO30 = 30,
    #[doc = "29: AUX_EVCTL:EVSTAT1.AUXIO29"]
    AUXIO29 = 29,
    #[doc = "28: AUX_EVCTL:EVSTAT1.AUXIO28"]
    AUXIO28 = 28,
    #[doc = "27: AUX_EVCTL:EVSTAT1.AUXIO27"]
    AUXIO27 = 27,
    #[doc = "26: AUX_EVCTL:EVSTAT1.AUXIO26"]
    AUXIO26 = 26,
    #[doc = "25: AUX_EVCTL:EVSTAT1.AUXIO25"]
    AUXIO25 = 25,
    #[doc = "24: AUX_EVCTL:EVSTAT1.AUXIO24"]
    AUXIO24 = 24,
    #[doc = "23: AUX_EVCTL:EVSTAT1.AUXIO23"]
    AUXIO23 = 23,
    #[doc = "22: AUX_EVCTL:EVSTAT1.AUXIO22"]
    AUXIO22 = 22,
    #[doc = "21: AUX_EVCTL:EVSTAT1.AUXIO21"]
    AUXIO21 = 21,
    #[doc = "20: AUX_EVCTL:EVSTAT1.AUXIO20"]
    AUXIO20 = 20,
    #[doc = "19: AUX_EVCTL:EVSTAT1.AUXIO19"]
    AUXIO19 = 19,
    #[doc = "18: AUX_EVCTL:EVSTAT1.AUXIO18"]
    AUXIO18 = 18,
    #[doc = "17: AUX_EVCTL:EVSTAT1.AUXIO17"]
    AUXIO17 = 17,
    #[doc = "16: AUX_EVCTL:EVSTAT1.AUXIO16"]
    AUXIO16 = 16,
    #[doc = "15: AUX_EVCTL:EVSTAT0.AUXIO15"]
    AUXIO15 = 15,
    #[doc = "14: AUX_EVCTL:EVSTAT0.AUXIO14"]
    AUXIO14 = 14,
    #[doc = "13: AUX_EVCTL:EVSTAT0.AUXIO13"]
    AUXIO13 = 13,
    #[doc = "12: AUX_EVCTL:EVSTAT0.AUXIO12"]
    AUXIO12 = 12,
    #[doc = "11: AUX_EVCTL:EVSTAT0.AUXIO11"]
    AUXIO11 = 11,
    #[doc = "10: AUX_EVCTL:EVSTAT0.AUXIO10"]
    AUXIO10 = 10,
    #[doc = "9: AUX_EVCTL:EVSTAT0.AUXIO9"]
    AUXIO9 = 9,
    #[doc = "8: AUX_EVCTL:EVSTAT0.AUXIO8"]
    AUXIO8 = 8,
    #[doc = "7: AUX_EVCTL:EVSTAT0.AUXIO7"]
    AUXIO7 = 7,
    #[doc = "6: AUX_EVCTL:EVSTAT0.AUXIO6"]
    AUXIO6 = 6,
    #[doc = "5: AUX_EVCTL:EVSTAT0.AUXIO5"]
    AUXIO5 = 5,
    #[doc = "4: AUX_EVCTL:EVSTAT0.AUXIO4"]
    AUXIO4 = 4,
    #[doc = "3: AUX_EVCTL:EVSTAT0.AUXIO3"]
    AUXIO3 = 3,
    #[doc = "2: AUX_EVCTL:EVSTAT0.AUXIO2"]
    AUXIO2 = 2,
    #[doc = "1: AUX_EVCTL:EVSTAT0.AUXIO1"]
    AUXIO1 = 1,
    #[doc = "0: AUX_EVCTL:EVSTAT0.AUXIO0"]
    AUXIO0 = 0,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            63 => Some(SRC_A::NO_EVENT),
            61 => Some(SRC_A::AUX_SMPH_AUTOTAKE_DONE),
            60 => Some(SRC_A::AUX_ADC_FIFO_NOT_EMPTY),
            59 => Some(SRC_A::AUX_ADC_FIFO_ALMOST_FULL),
            58 => Some(SRC_A::AUX_ADC_IRQ),
            57 => Some(SRC_A::AUX_ADC_DONE),
            56 => Some(SRC_A::AUX_ISRC_RESET_N),
            55 => Some(SRC_A::AUX_TDC_DONE),
            54 => Some(SRC_A::AUX_TIMER0_EV),
            53 => Some(SRC_A::AUX_TIMER1_EV),
            52 => Some(SRC_A::AUX_TIMER2_PULSE),
            51 => Some(SRC_A::AUX_TIMER2_EV3),
            50 => Some(SRC_A::AUX_TIMER2_EV2),
            49 => Some(SRC_A::AUX_TIMER2_EV1),
            48 => Some(SRC_A::AUX_TIMER2_EV0),
            47 => Some(SRC_A::AUX_COMPB),
            46 => Some(SRC_A::AUX_COMPA),
            45 => Some(SRC_A::MCU_OBSMUX1),
            44 => Some(SRC_A::MCU_OBSMUX0),
            43 => Some(SRC_A::MCU_EV),
            42 => Some(SRC_A::ACLK_REF),
            41 => Some(SRC_A::VDDR_RECHARGE),
            40 => Some(SRC_A::MCU_ACTIVE),
            39 => Some(SRC_A::PWR_DWN),
            38 => Some(SRC_A::SCLK_LF),
            37 => Some(SRC_A::AON_BATMON_TEMP_UPD),
            36 => Some(SRC_A::AON_BATMON_BAT_UPD),
            35 => Some(SRC_A::AON_RTC_4KHZ),
            34 => Some(SRC_A::AON_RTC_CH2_DLY),
            33 => Some(SRC_A::AON_RTC_CH2),
            32 => Some(SRC_A::MANUAL_EV),
            31 => Some(SRC_A::AUXIO31),
            30 => Some(SRC_A::AUXIO30),
            29 => Some(SRC_A::AUXIO29),
            28 => Some(SRC_A::AUXIO28),
            27 => Some(SRC_A::AUXIO27),
            26 => Some(SRC_A::AUXIO26),
            25 => Some(SRC_A::AUXIO25),
            24 => Some(SRC_A::AUXIO24),
            23 => Some(SRC_A::AUXIO23),
            22 => Some(SRC_A::AUXIO22),
            21 => Some(SRC_A::AUXIO21),
            20 => Some(SRC_A::AUXIO20),
            19 => Some(SRC_A::AUXIO19),
            18 => Some(SRC_A::AUXIO18),
            17 => Some(SRC_A::AUXIO17),
            16 => Some(SRC_A::AUXIO16),
            15 => Some(SRC_A::AUXIO15),
            14 => Some(SRC_A::AUXIO14),
            13 => Some(SRC_A::AUXIO13),
            12 => Some(SRC_A::AUXIO12),
            11 => Some(SRC_A::AUXIO11),
            10 => Some(SRC_A::AUXIO10),
            9 => Some(SRC_A::AUXIO9),
            8 => Some(SRC_A::AUXIO8),
            7 => Some(SRC_A::AUXIO7),
            6 => Some(SRC_A::AUXIO6),
            5 => Some(SRC_A::AUXIO5),
            4 => Some(SRC_A::AUXIO4),
            3 => Some(SRC_A::AUXIO3),
            2 => Some(SRC_A::AUXIO2),
            1 => Some(SRC_A::AUXIO1),
            0 => Some(SRC_A::AUXIO0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == SRC_A::NO_EVENT
    }
    #[doc = "Checks if the value of the field is `AUX_SMPH_AUTOTAKE_DONE`"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == SRC_A::AUX_SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == SRC_A::AUX_ADC_FIFO_NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_ALMOST_FULL`"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == SRC_A::AUX_ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_IRQ`"]
    #[inline(always)]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == SRC_A::AUX_ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == SRC_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ISRC_RESET_N`"]
    #[inline(always)]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == SRC_A::AUX_ISRC_RESET_N
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == SRC_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == SRC_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == SRC_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_PULSE`"]
    #[inline(always)]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_PULSE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == SRC_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == SRC_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX1`"]
    #[inline(always)]
    pub fn is_mcu_obsmux1(&self) -> bool {
        *self == SRC_A::MCU_OBSMUX1
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX0`"]
    #[inline(always)]
    pub fn is_mcu_obsmux0(&self) -> bool {
        *self == SRC_A::MCU_OBSMUX0
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == SRC_A::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == SRC_A::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `VDDR_RECHARGE`"]
    #[inline(always)]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == SRC_A::VDDR_RECHARGE
    }
    #[doc = "Checks if the value of the field is `MCU_ACTIVE`"]
    #[inline(always)]
    pub fn is_mcu_active(&self) -> bool {
        *self == SRC_A::MCU_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PWR_DWN`"]
    #[inline(always)]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == SRC_A::PWR_DWN
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == SRC_A::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_TEMP_UPD`"]
    #[inline(always)]
    pub fn is_aon_batmon_temp_upd(&self) -> bool {
        *self == SRC_A::AON_BATMON_TEMP_UPD
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_BAT_UPD`"]
    #[inline(always)]
    pub fn is_aon_batmon_bat_upd(&self) -> bool {
        *self == SRC_A::AON_BATMON_BAT_UPD
    }
    #[doc = "Checks if the value of the field is `AON_RTC_4KHZ`"]
    #[inline(always)]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == SRC_A::AON_RTC_4KHZ
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == SRC_A::AON_RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == SRC_A::AON_RTC_CH2
    }
    #[doc = "Checks if the value of the field is `MANUAL_EV`"]
    #[inline(always)]
    pub fn is_manual_ev(&self) -> bool {
        *self == SRC_A::MANUAL_EV
    }
    #[doc = "Checks if the value of the field is `AUXIO31`"]
    #[inline(always)]
    pub fn is_auxio31(&self) -> bool {
        *self == SRC_A::AUXIO31
    }
    #[doc = "Checks if the value of the field is `AUXIO30`"]
    #[inline(always)]
    pub fn is_auxio30(&self) -> bool {
        *self == SRC_A::AUXIO30
    }
    #[doc = "Checks if the value of the field is `AUXIO29`"]
    #[inline(always)]
    pub fn is_auxio29(&self) -> bool {
        *self == SRC_A::AUXIO29
    }
    #[doc = "Checks if the value of the field is `AUXIO28`"]
    #[inline(always)]
    pub fn is_auxio28(&self) -> bool {
        *self == SRC_A::AUXIO28
    }
    #[doc = "Checks if the value of the field is `AUXIO27`"]
    #[inline(always)]
    pub fn is_auxio27(&self) -> bool {
        *self == SRC_A::AUXIO27
    }
    #[doc = "Checks if the value of the field is `AUXIO26`"]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == SRC_A::AUXIO26
    }
    #[doc = "Checks if the value of the field is `AUXIO25`"]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == SRC_A::AUXIO25
    }
    #[doc = "Checks if the value of the field is `AUXIO24`"]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == SRC_A::AUXIO24
    }
    #[doc = "Checks if the value of the field is `AUXIO23`"]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == SRC_A::AUXIO23
    }
    #[doc = "Checks if the value of the field is `AUXIO22`"]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == SRC_A::AUXIO22
    }
    #[doc = "Checks if the value of the field is `AUXIO21`"]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == SRC_A::AUXIO21
    }
    #[doc = "Checks if the value of the field is `AUXIO20`"]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == SRC_A::AUXIO20
    }
    #[doc = "Checks if the value of the field is `AUXIO19`"]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == SRC_A::AUXIO19
    }
    #[doc = "Checks if the value of the field is `AUXIO18`"]
    #[inline(always)]
    pub fn is_auxio18(&self) -> bool {
        *self == SRC_A::AUXIO18
    }
    #[doc = "Checks if the value of the field is `AUXIO17`"]
    #[inline(always)]
    pub fn is_auxio17(&self) -> bool {
        *self == SRC_A::AUXIO17
    }
    #[doc = "Checks if the value of the field is `AUXIO16`"]
    #[inline(always)]
    pub fn is_auxio16(&self) -> bool {
        *self == SRC_A::AUXIO16
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == SRC_A::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == SRC_A::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == SRC_A::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == SRC_A::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == SRC_A::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == SRC_A::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == SRC_A::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == SRC_A::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == SRC_A::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == SRC_A::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == SRC_A::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == SRC_A::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == SRC_A::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == SRC_A::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == SRC_A::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == SRC_A::AUXIO0
    }
}
#[doc = "Field `SRC` writer - 5:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
pub type SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRECTL_SPEC, u8, SRC_A, 6, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
    #[doc = "No event."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(SRC_A::NO_EVENT)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut W {
        self.variant(SRC_A::AUX_SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut W {
        self.variant(SRC_A::AUX_ADC_FIFO_NOT_EMPTY)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(SRC_A::AUX_ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn aux_adc_irq(self) -> &'a mut W {
        self.variant(SRC_A::AUX_ADC_IRQ)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(SRC_A::AUX_ADC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(self) -> &'a mut W {
        self.variant(SRC_A::AUX_ISRC_RESET_N)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TDC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER0_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_pulse(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_PULSE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_EV3)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_EV2)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_EV1)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_EV0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(SRC_A::AUX_COMPB)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(SRC_A::AUX_COMPA)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn mcu_obsmux1(self) -> &'a mut W {
        self.variant(SRC_A::MCU_OBSMUX1)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn mcu_obsmux0(self) -> &'a mut W {
        self.variant(SRC_A::MCU_OBSMUX0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(SRC_A::MCU_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(SRC_A::ACLK_REF)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn vddr_recharge(self) -> &'a mut W {
        self.variant(SRC_A::VDDR_RECHARGE)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn mcu_active(self) -> &'a mut W {
        self.variant(SRC_A::MCU_ACTIVE)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn pwr_dwn(self) -> &'a mut W {
        self.variant(SRC_A::PWR_DWN)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(SRC_A::SCLK_LF)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline(always)]
    pub fn aon_batmon_temp_upd(self) -> &'a mut W {
        self.variant(SRC_A::AON_BATMON_TEMP_UPD)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline(always)]
    pub fn aon_batmon_bat_upd(self) -> &'a mut W {
        self.variant(SRC_A::AON_BATMON_BAT_UPD)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn aon_rtc_4khz(self) -> &'a mut W {
        self.variant(SRC_A::AON_RTC_4KHZ)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut W {
        self.variant(SRC_A::AON_RTC_CH2_DLY)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(SRC_A::AON_RTC_CH2)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn manual_ev(self) -> &'a mut W {
        self.variant(SRC_A::MANUAL_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn auxio31(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO31)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn auxio30(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO30)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn auxio29(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO29)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn auxio28(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO28)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn auxio27(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO27)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO26)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO25)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO24)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO23)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO22)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO21)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO20)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO19)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn auxio18(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO18)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn auxio17(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO17)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn auxio16(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO16)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO15)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO14)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO13)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO12)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO11)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO10)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO9)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO8)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO7)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO6)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO5)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO4)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO0)
    }
}
#[doc = "Field `RATIO` reader - 6:6\\]
Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
pub type RATIO_R = crate::BitReader<RATIO_A>;
#[doc = "6:6\\]
Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RATIO_A {
    #[doc = "1: Prescaler divides input by 64. AUX_TDC_PRE event has a rising edge for every 64 rising edges of the input. AUX_TDC_PRE event toggles on every 32nd rising edge of the input."]
    DIV64 = 1,
    #[doc = "0: Prescaler divides input by 16. AUX_TDC_PRE event has a rising edge for every 16 rising edges of the input. AUX_TDC_PRE event toggles on every 8th rising edge of the input."]
    DIV16 = 0,
}
impl From<RATIO_A> for bool {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as u8 != 0
    }
}
impl RATIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATIO_A {
        match self.bits {
            true => RATIO_A::DIV64,
            false => RATIO_A::DIV16,
        }
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == RATIO_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == RATIO_A::DIV16
    }
}
#[doc = "Field `RATIO` writer - 6:6\\]
Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
pub type RATIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRECTL_SPEC, RATIO_A, O>;
impl<'a, const O: u8> RATIO_W<'a, O> {
    #[doc = "Prescaler divides input by 64. AUX_TDC_PRE event has a rising edge for every 64 rising edges of the input. AUX_TDC_PRE event toggles on every 32nd rising edge of the input."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(RATIO_A::DIV64)
    }
    #[doc = "Prescaler divides input by 16. AUX_TDC_PRE event has a rising edge for every 16 rising edges of the input. AUX_TDC_PRE event toggles on every 8th rising edge of the input."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(RATIO_A::DIV16)
    }
}
#[doc = "Field `RESET_N` reader - 7:7\\]
Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
pub type RESET_N_R = crate::BitReader<bool>;
#[doc = "Field `RESET_N` writer - 7:7\\]
Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
pub type RESET_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRECTL_SPEC, bool, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRECTL_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
    #[inline(always)]
    pub fn reset_n(&self) -> RESET_N_R {
        RESET_N_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RATIO_W<6> {
        RATIO_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
    #[inline(always)]
    #[must_use]
    pub fn reset_n(&mut self) -> RESET_N_W<7> {
        RESET_N_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prescaler Control The prescaler can be used to count events that are faster than the AUX bus rate. It can be used to: - count pulses on a specified event from the asynchronous event bus. - prescale a specified event from the asynchronous event bus. To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE. It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX bus rate.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prectl](index.html) module"]
pub struct PRECTL_SPEC;
impl crate::RegisterSpec for PRECTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prectl::R](R) reader structure"]
impl crate::Readable for PRECTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prectl::W](W) writer structure"]
impl crate::Writable for PRECTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRECTL to value 0x3f"]
impl crate::Resettable for PRECTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}

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
#[doc = "Field `SRC` reader - 4:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
pub type SRC_R = crate::FieldReader<u8, SRC_A>;
#[doc = "4:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0.\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "31: AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    ADC_IRQ = 31,
    #[doc = "30: AUX_EVCTL:EVSTAT1.MCU_EV"]
    MCU_EV = 30,
    #[doc = "29: AUX_EVCTL:EVSTAT1.ACLK_REF"]
    ACLK_REF = 29,
    #[doc = "28: AUX_EVCTL:EVSTAT1.AUXIO15"]
    AUXIO15 = 28,
    #[doc = "27: AUX_EVCTL:EVSTAT1.AUXIO14"]
    AUXIO14 = 27,
    #[doc = "26: AUX_EVCTL:EVSTAT1.AUXIO13"]
    AUXIO13 = 26,
    #[doc = "25: AUX_EVCTL:EVSTAT1.AUXIO12"]
    AUXIO12 = 25,
    #[doc = "24: AUX_EVCTL:EVSTAT1.AUXIO11"]
    AUXIO11 = 24,
    #[doc = "23: AUX_EVCTL:EVSTAT1.AUXIO10"]
    AUXIO10 = 23,
    #[doc = "22: AUX_EVCTL:EVSTAT1.AUXIO9"]
    AUXIO9 = 22,
    #[doc = "21: AUX_EVCTL:EVSTAT1.AUXIO8"]
    AUXIO8 = 21,
    #[doc = "20: AUX_EVCTL:EVSTAT1.AUXIO7"]
    AUXIO7 = 20,
    #[doc = "19: AUX_EVCTL:EVSTAT1.AUXIO6"]
    AUXIO6 = 19,
    #[doc = "18: AUX_EVCTL:EVSTAT1.AUXIO5"]
    AUXIO5 = 18,
    #[doc = "17: AUX_EVCTL:EVSTAT1.AUXIO4"]
    AUXIO4 = 17,
    #[doc = "16: AUX_EVCTL:EVSTAT1.AUXIO3"]
    AUXIO3 = 16,
    #[doc = "15: AUX_EVCTL:EVSTAT0.AUXIO2"]
    AUXIO2 = 15,
    #[doc = "14: AUX_EVCTL:EVSTAT0.AUXIO1"]
    AUXIO1 = 14,
    #[doc = "13: AUX_EVCTL:EVSTAT0.AUXIO0"]
    AUXIO0 = 13,
    #[doc = "12: AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    AON_PROG_WU = 12,
    #[doc = "11: AUX_EVCTL:EVSTAT0.AON_SW"]
    AON_SW = 11,
    #[doc = "10: AUX_EVCTL:EVSTAT0.OBSMUX1"]
    OBSMUX1 = 10,
    #[doc = "9: AUX_EVCTL:EVSTAT0.OBSMUX0"]
    OBSMUX0 = 9,
    #[doc = "8: AUX_EVCTL:EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    ADC_FIFO_ALMOST_FULL = 8,
    #[doc = "7: AUX_EVCTL:EVSTAT0.ADC_DONE"]
    ADC_DONE = 7,
    #[doc = "6: AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE = 6,
    #[doc = "5: AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    TIMER1_EV = 5,
    #[doc = "4: AUX_EVCTL:EVSTAT0.TIMER0_EV"]
    TIMER0_EV = 4,
    #[doc = "3: AUX_ANAIF:ISRCCTL.RESET_N"]
    ISRC_RESET = 3,
    #[doc = "2: AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    AUX_COMPB = 2,
    #[doc = "1: AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    AUX_COMPA = 1,
    #[doc = "0: AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    AON_RTC_CH2 = 0,
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
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            31 => SRC_A::ADC_IRQ,
            30 => SRC_A::MCU_EV,
            29 => SRC_A::ACLK_REF,
            28 => SRC_A::AUXIO15,
            27 => SRC_A::AUXIO14,
            26 => SRC_A::AUXIO13,
            25 => SRC_A::AUXIO12,
            24 => SRC_A::AUXIO11,
            23 => SRC_A::AUXIO10,
            22 => SRC_A::AUXIO9,
            21 => SRC_A::AUXIO8,
            20 => SRC_A::AUXIO7,
            19 => SRC_A::AUXIO6,
            18 => SRC_A::AUXIO5,
            17 => SRC_A::AUXIO4,
            16 => SRC_A::AUXIO3,
            15 => SRC_A::AUXIO2,
            14 => SRC_A::AUXIO1,
            13 => SRC_A::AUXIO0,
            12 => SRC_A::AON_PROG_WU,
            11 => SRC_A::AON_SW,
            10 => SRC_A::OBSMUX1,
            9 => SRC_A::OBSMUX0,
            8 => SRC_A::ADC_FIFO_ALMOST_FULL,
            7 => SRC_A::ADC_DONE,
            6 => SRC_A::SMPH_AUTOTAKE_DONE,
            5 => SRC_A::TIMER1_EV,
            4 => SRC_A::TIMER0_EV,
            3 => SRC_A::ISRC_RESET,
            2 => SRC_A::AUX_COMPB,
            1 => SRC_A::AUX_COMPA,
            0 => SRC_A::AON_RTC_CH2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_IRQ`"]
    #[inline(always)]
    pub fn is_adc_irq(&self) -> bool {
        *self == SRC_A::ADC_IRQ
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
    #[doc = "Checks if the value of the field is `AON_PROG_WU`"]
    #[inline(always)]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == SRC_A::AON_PROG_WU
    }
    #[doc = "Checks if the value of the field is `AON_SW`"]
    #[inline(always)]
    pub fn is_aon_sw(&self) -> bool {
        *self == SRC_A::AON_SW
    }
    #[doc = "Checks if the value of the field is `OBSMUX1`"]
    #[inline(always)]
    pub fn is_obsmux1(&self) -> bool {
        *self == SRC_A::OBSMUX1
    }
    #[doc = "Checks if the value of the field is `OBSMUX0`"]
    #[inline(always)]
    pub fn is_obsmux0(&self) -> bool {
        *self == SRC_A::OBSMUX0
    }
    #[doc = "Checks if the value of the field is `ADC_FIFO_ALMOST_FULL`"]
    #[inline(always)]
    pub fn is_adc_fifo_almost_full(&self) -> bool {
        *self == SRC_A::ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `ADC_DONE`"]
    #[inline(always)]
    pub fn is_adc_done(&self) -> bool {
        *self == SRC_A::ADC_DONE
    }
    #[doc = "Checks if the value of the field is `SMPH_AUTOTAKE_DONE`"]
    #[inline(always)]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == SRC_A::SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `TIMER1_EV`"]
    #[inline(always)]
    pub fn is_timer1_ev(&self) -> bool {
        *self == SRC_A::TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `TIMER0_EV`"]
    #[inline(always)]
    pub fn is_timer0_ev(&self) -> bool {
        *self == SRC_A::TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `ISRC_RESET`"]
    #[inline(always)]
    pub fn is_isrc_reset(&self) -> bool {
        *self == SRC_A::ISRC_RESET
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
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == SRC_A::AON_RTC_CH2
    }
}
#[doc = "Field `SRC` writer - 4:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
pub type SRC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PRECTL_SPEC, u8, SRC_A, 5, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn adc_irq(self) -> &'a mut W {
        self.variant(SRC_A::ADC_IRQ)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(SRC_A::MCU_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(SRC_A::ACLK_REF)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO15)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO14)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO13)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO12)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO11)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO10)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO9)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO8)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO7)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO6)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO5)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(SRC_A::AUXIO4)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3"]
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
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(self) -> &'a mut W {
        self.variant(SRC_A::AON_PROG_WU)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn aon_sw(self) -> &'a mut W {
        self.variant(SRC_A::AON_SW)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn obsmux1(self) -> &'a mut W {
        self.variant(SRC_A::OBSMUX1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn obsmux0(self) -> &'a mut W {
        self.variant(SRC_A::OBSMUX0)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(SRC_A::ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn adc_done(self) -> &'a mut W {
        self.variant(SRC_A::ADC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn smph_autotake_done(self) -> &'a mut W {
        self.variant(SRC_A::SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(self) -> &'a mut W {
        self.variant(SRC_A::TIMER1_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn timer0_ev(self) -> &'a mut W {
        self.variant(SRC_A::TIMER0_EV)
    }
    #[doc = "AUX_ANAIF:ISRCCTL.RESET_N"]
    #[inline(always)]
    pub fn isrc_reset(self) -> &'a mut W {
        self.variant(SRC_A::ISRC_RESET)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(SRC_A::AUX_COMPB)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(SRC_A::AUX_COMPA)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(SRC_A::AON_RTC_CH2)
    }
}
#[doc = "Field `RESERVED5` reader - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED5` writer - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRECTL_SPEC, bool, O>;
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
    #[doc = "Bits 0:4 - 4:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bits 0:4 - 4:0\\]
Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
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
#[doc = "Prescaler Control The prescaler can be used to count events that are faster than the AUX clock frequency. It can be used to: - count pulses on a specified event from the asynchronous event bus. - prescale a specified event from the asynchronous event bus. To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE. It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX clock frequency.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prectl](index.html) module"]
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
#[doc = "`reset()` method sets PRECTL to value 0x1f"]
impl crate::Resettable for PRECTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}

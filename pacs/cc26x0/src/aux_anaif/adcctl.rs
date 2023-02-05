#[doc = "Register `ADCCTL` reader"]
pub struct R(crate::R<ADCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCTL` writer"]
pub struct W(crate::W<ADCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCTL_SPEC>;
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
impl From<crate::W<ADCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD` reader - 1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
pub type CMD_R = crate::FieldReader<u8, CMD_A>;
#[doc = "1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "3: Flush ADC FIFO. You must set CMD to EN or DIS after flush. System CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    FLUSH = 3,
    #[doc = "1: Enable ADC interface."]
    EN = 1,
    #[doc = "0: Disable ADC interface."]
    DIS = 0,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            3 => Some(CMD_A::FLUSH),
            1 => Some(CMD_A::EN),
            0 => Some(CMD_A::DIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == CMD_A::FLUSH
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CMD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CMD_A::DIS
    }
}
#[doc = "Field `CMD` writer - 1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCTL_SPEC, u8, CMD_A, 2, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "Flush ADC FIFO. You must set CMD to EN or DIS after flush. System CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(CMD_A::FLUSH)
    }
    #[doc = "Enable ADC interface."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CMD_A::EN)
    }
    #[doc = "Disable ADC interface."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CMD_A::DIS)
    }
}
#[doc = "Field `RESERVED2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `START_SRC` reader - 12:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT<n> if you want to trigger the ADC manually through ADCTRIG.START."]
pub type START_SRC_R = crate::FieldReader<u8, START_SRC_A>;
#[doc = "12:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT<n> if you want to trigger the ADC manually through ADCTRIG.START.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum START_SRC_A {
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
    #[doc = "10: No event."]
    NO_EVENT1 = 10,
    #[doc = "9: No event."]
    NO_EVENT0 = 9,
    #[doc = "8: Reserved - Do not use."]
    RESERVED1 = 8,
    #[doc = "7: Reserved - Do not use."]
    RESERVED0 = 7,
    #[doc = "6: AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE = 6,
    #[doc = "5: AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    TIMER1_EV = 5,
    #[doc = "4: AUX_EVCTL:EVSTAT0.TIMER0_EV"]
    TIMER0_EV = 4,
    #[doc = "3: AUX_EVCTL:EVSTAT0.TDC_DONE"]
    TDC_DONE = 3,
    #[doc = "2: AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    AUX_COMPB = 2,
    #[doc = "1: AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    AUX_COMPA = 1,
    #[doc = "0: AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    RTC_CH2_EV = 0,
}
impl From<START_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: START_SRC_A) -> Self {
        variant as _
    }
}
impl START_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_SRC_A {
        match self.bits {
            31 => START_SRC_A::ADC_IRQ,
            30 => START_SRC_A::MCU_EV,
            29 => START_SRC_A::ACLK_REF,
            28 => START_SRC_A::AUXIO15,
            27 => START_SRC_A::AUXIO14,
            26 => START_SRC_A::AUXIO13,
            25 => START_SRC_A::AUXIO12,
            24 => START_SRC_A::AUXIO11,
            23 => START_SRC_A::AUXIO10,
            22 => START_SRC_A::AUXIO9,
            21 => START_SRC_A::AUXIO8,
            20 => START_SRC_A::AUXIO7,
            19 => START_SRC_A::AUXIO6,
            18 => START_SRC_A::AUXIO5,
            17 => START_SRC_A::AUXIO4,
            16 => START_SRC_A::AUXIO3,
            15 => START_SRC_A::AUXIO2,
            14 => START_SRC_A::AUXIO1,
            13 => START_SRC_A::AUXIO0,
            12 => START_SRC_A::AON_PROG_WU,
            11 => START_SRC_A::AON_SW,
            10 => START_SRC_A::NO_EVENT1,
            9 => START_SRC_A::NO_EVENT0,
            8 => START_SRC_A::RESERVED1,
            7 => START_SRC_A::RESERVED0,
            6 => START_SRC_A::SMPH_AUTOTAKE_DONE,
            5 => START_SRC_A::TIMER1_EV,
            4 => START_SRC_A::TIMER0_EV,
            3 => START_SRC_A::TDC_DONE,
            2 => START_SRC_A::AUX_COMPB,
            1 => START_SRC_A::AUX_COMPA,
            0 => START_SRC_A::RTC_CH2_EV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_IRQ`"]
    #[inline(always)]
    pub fn is_adc_irq(&self) -> bool {
        *self == START_SRC_A::ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == START_SRC_A::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == START_SRC_A::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == START_SRC_A::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == START_SRC_A::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == START_SRC_A::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == START_SRC_A::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == START_SRC_A::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == START_SRC_A::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == START_SRC_A::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == START_SRC_A::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == START_SRC_A::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == START_SRC_A::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == START_SRC_A::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == START_SRC_A::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == START_SRC_A::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == START_SRC_A::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == START_SRC_A::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == START_SRC_A::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AON_PROG_WU`"]
    #[inline(always)]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == START_SRC_A::AON_PROG_WU
    }
    #[doc = "Checks if the value of the field is `AON_SW`"]
    #[inline(always)]
    pub fn is_aon_sw(&self) -> bool {
        *self == START_SRC_A::AON_SW
    }
    #[doc = "Checks if the value of the field is `NO_EVENT1`"]
    #[inline(always)]
    pub fn is_no_event1(&self) -> bool {
        *self == START_SRC_A::NO_EVENT1
    }
    #[doc = "Checks if the value of the field is `NO_EVENT0`"]
    #[inline(always)]
    pub fn is_no_event0(&self) -> bool {
        *self == START_SRC_A::NO_EVENT0
    }
    #[doc = "Checks if the value of the field is `RESERVED1`"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == START_SRC_A::RESERVED1
    }
    #[doc = "Checks if the value of the field is `RESERVED0`"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == START_SRC_A::RESERVED0
    }
    #[doc = "Checks if the value of the field is `SMPH_AUTOTAKE_DONE`"]
    #[inline(always)]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == START_SRC_A::SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `TIMER1_EV`"]
    #[inline(always)]
    pub fn is_timer1_ev(&self) -> bool {
        *self == START_SRC_A::TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `TIMER0_EV`"]
    #[inline(always)]
    pub fn is_timer0_ev(&self) -> bool {
        *self == START_SRC_A::TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `TDC_DONE`"]
    #[inline(always)]
    pub fn is_tdc_done(&self) -> bool {
        *self == START_SRC_A::TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == START_SRC_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == START_SRC_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_EV`"]
    #[inline(always)]
    pub fn is_rtc_ch2_ev(&self) -> bool {
        *self == START_SRC_A::RTC_CH2_EV
    }
}
#[doc = "Field `START_SRC` writer - 12:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT<n> if you want to trigger the ADC manually through ADCTRIG.START."]
pub type START_SRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCCTL_SPEC, u8, START_SRC_A, 5, O>;
impl<'a, const O: u8> START_SRC_W<'a, O> {
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn adc_irq(self) -> &'a mut W {
        self.variant(START_SRC_A::ADC_IRQ)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(START_SRC_A::MCU_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(START_SRC_A::ACLK_REF)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO15)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO14)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO13)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO12)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO11)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO10)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO9)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO8)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO7)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO6)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO5)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO4)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(START_SRC_A::AUXIO0)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(self) -> &'a mut W {
        self.variant(START_SRC_A::AON_PROG_WU)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn aon_sw(self) -> &'a mut W {
        self.variant(START_SRC_A::AON_SW)
    }
    #[doc = "No event."]
    #[inline(always)]
    pub fn no_event1(self) -> &'a mut W {
        self.variant(START_SRC_A::NO_EVENT1)
    }
    #[doc = "No event."]
    #[inline(always)]
    pub fn no_event0(self) -> &'a mut W {
        self.variant(START_SRC_A::NO_EVENT0)
    }
    #[doc = "Reserved - Do not use."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut W {
        self.variant(START_SRC_A::RESERVED1)
    }
    #[doc = "Reserved - Do not use."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut W {
        self.variant(START_SRC_A::RESERVED0)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn smph_autotake_done(self) -> &'a mut W {
        self.variant(START_SRC_A::SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(self) -> &'a mut W {
        self.variant(START_SRC_A::TIMER1_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn timer0_ev(self) -> &'a mut W {
        self.variant(START_SRC_A::TIMER0_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(self) -> &'a mut W {
        self.variant(START_SRC_A::TDC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(START_SRC_A::AUX_COMPB)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(START_SRC_A::AUX_COMPA)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn rtc_ch2_ev(self) -> &'a mut W {
        self.variant(START_SRC_A::RTC_CH2_EV)
    }
}
#[doc = "Field `START_POL` reader - 13:13\\]
Select active polarity for START_SRC event."]
pub type START_POL_R = crate::BitReader<START_POL_A>;
#[doc = "13:13\\]
Select active polarity for START_SRC event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_POL_A {
    #[doc = "1: Set ADC trigger on falling edge of event source."]
    FALL = 1,
    #[doc = "0: Set ADC trigger on rising edge of event source."]
    RISE = 0,
}
impl From<START_POL_A> for bool {
    #[inline(always)]
    fn from(variant: START_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl START_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_POL_A {
        match self.bits {
            true => START_POL_A::FALL,
            false => START_POL_A::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == START_POL_A::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == START_POL_A::RISE
    }
}
#[doc = "Field `START_POL` writer - 13:13\\]
Select active polarity for START_SRC event."]
pub type START_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTL_SPEC, START_POL_A, O>;
impl<'a, const O: u8> START_POL_W<'a, O> {
    #[doc = "Set ADC trigger on falling edge of event source."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(START_POL_A::FALL)
    }
    #[doc = "Set ADC trigger on rising edge of event source."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(START_POL_A::RISE)
    }
}
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCTL_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT<n> if you want to trigger the ADC manually through ADCTRIG.START."]
    #[inline(always)]
    pub fn start_src(&self) -> START_SRC_R {
        START_SRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Select active polarity for START_SRC event."]
    #[inline(always)]
    pub fn start_pol(&self) -> START_POL_R {
        START_POL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT<n> if you want to trigger the ADC manually through ADCTRIG.START."]
    #[inline(always)]
    #[must_use]
    pub fn start_src(&mut self) -> START_SRC_W<8> {
        START_SRC_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Select active polarity for START_SRC event."]
    #[inline(always)]
    #[must_use]
    pub fn start_pol(&mut self) -> START_POL_W<13> {
        START_POL_W::new(self)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> RESERVED14_W<14> {
        RESERVED14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctl](index.html) module"]
pub struct ADCCTL_SPEC;
impl crate::RegisterSpec for ADCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcctl::R](R) reader structure"]
impl crate::Readable for ADCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcctl::W](W) writer structure"]
impl crate::Writable for ADCCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCCTL to value 0"]
impl crate::Resettable for ADCCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

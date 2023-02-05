#[doc = "Register `T0CFG` reader"]
pub struct R(crate::R<T0CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0CFG` writer"]
pub struct W(crate::W<T0CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0CFG_SPEC>;
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
impl From<crate::W<T0CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELOAD` reader - 0:0\\]
Timer 0 reload mode."]
pub type RELOAD_R = crate::BitReader<RELOAD_A>;
#[doc = "0:0\\]
Timer 0 reload mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RELOAD_A {
    #[doc = "1: Continuous mode. Timer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    CONT = 1,
    #[doc = "0: Manual mode. Timer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE."]
    MAN = 0,
}
impl From<RELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: RELOAD_A) -> Self {
        variant as u8 != 0
    }
}
impl RELOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_A {
        match self.bits {
            true => RELOAD_A::CONT,
            false => RELOAD_A::MAN,
        }
    }
    #[doc = "Checks if the value of the field is `CONT`"]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == RELOAD_A::CONT
    }
    #[doc = "Checks if the value of the field is `MAN`"]
    #[inline(always)]
    pub fn is_man(&self) -> bool {
        *self == RELOAD_A::MAN
    }
}
#[doc = "Field `RELOAD` writer - 0:0\\]
Timer 0 reload mode."]
pub type RELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CFG_SPEC, RELOAD_A, O>;
impl<'a, const O: u8> RELOAD_W<'a, O> {
    #[doc = "Continuous mode. Timer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    #[inline(always)]
    pub fn cont(self) -> &'a mut W {
        self.variant(RELOAD_A::CONT)
    }
    #[doc = "Manual mode. Timer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE."]
    #[inline(always)]
    pub fn man(self) -> &'a mut W {
        self.variant(RELOAD_A::MAN)
    }
}
#[doc = "Field `MODE` reader - 1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler."]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "1: Use event set by TICK_SRC as source for prescaler."]
    TICK = 1,
    #[doc = "0: Use AUX clock as source for prescaler."]
    CLK = 0,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            true => MODE_A::TICK,
            false => MODE_A::CLK,
        }
    }
    #[doc = "Checks if the value of the field is `TICK`"]
    #[inline(always)]
    pub fn is_tick(&self) -> bool {
        *self == MODE_A::TICK
    }
    #[doc = "Checks if the value of the field is `CLK`"]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == MODE_A::CLK
    }
}
#[doc = "Field `MODE` writer - 1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CFG_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Use event set by TICK_SRC as source for prescaler."]
    #[inline(always)]
    pub fn tick(self) -> &'a mut W {
        self.variant(MODE_A::TICK)
    }
    #[doc = "Use AUX clock as source for prescaler."]
    #[inline(always)]
    pub fn clk(self) -> &'a mut W {
        self.variant(MODE_A::CLK)
    }
}
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRE` reader - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
pub type PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE` writer - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
pub type PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TICK_SRC` reader - 12:8\\]
Select Timer 0 tick source from the synchronous event bus."]
pub type TICK_SRC_R = crate::FieldReader<u8, TICK_SRC_A>;
#[doc = "12:8\\]
Select Timer 0 tick source from the synchronous event bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TICK_SRC_A {
    #[doc = "31: AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    ADC_IRQ = 31,
    #[doc = "30: AUX_EVCTL:EVSTAT1.MCU_EV"]
    MCU_EVENT = 30,
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
    #[doc = "8: AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    RTC_4KHZ = 8,
    #[doc = "7: AUX_EVCTL:EVSTAT0.ADC_DONE"]
    ADC_DONE = 7,
    #[doc = "6: AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE = 6,
    #[doc = "5: AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    TIMER1_EV = 5,
    #[doc = "3: AUX_EVCTL:EVSTAT0.TDC_DONE"]
    TDC_DONE = 3,
    #[doc = "2: AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    AUX_COMPB = 2,
    #[doc = "1: AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    AUX_COMPA = 1,
    #[doc = "0: AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    RTC_CH2_EV = 0,
}
impl From<TICK_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TICK_SRC_A) -> Self {
        variant as _
    }
}
impl TICK_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TICK_SRC_A> {
        match self.bits {
            31 => Some(TICK_SRC_A::ADC_IRQ),
            30 => Some(TICK_SRC_A::MCU_EVENT),
            29 => Some(TICK_SRC_A::ACLK_REF),
            28 => Some(TICK_SRC_A::AUXIO15),
            27 => Some(TICK_SRC_A::AUXIO14),
            26 => Some(TICK_SRC_A::AUXIO13),
            25 => Some(TICK_SRC_A::AUXIO12),
            24 => Some(TICK_SRC_A::AUXIO11),
            23 => Some(TICK_SRC_A::AUXIO10),
            22 => Some(TICK_SRC_A::AUXIO9),
            21 => Some(TICK_SRC_A::AUXIO8),
            20 => Some(TICK_SRC_A::AUXIO7),
            19 => Some(TICK_SRC_A::AUXIO6),
            18 => Some(TICK_SRC_A::AUXIO5),
            17 => Some(TICK_SRC_A::AUXIO4),
            16 => Some(TICK_SRC_A::AUXIO3),
            15 => Some(TICK_SRC_A::AUXIO2),
            14 => Some(TICK_SRC_A::AUXIO1),
            13 => Some(TICK_SRC_A::AUXIO0),
            12 => Some(TICK_SRC_A::AON_PROG_WU),
            11 => Some(TICK_SRC_A::AON_SW),
            10 => Some(TICK_SRC_A::OBSMUX1),
            9 => Some(TICK_SRC_A::OBSMUX0),
            8 => Some(TICK_SRC_A::RTC_4KHZ),
            7 => Some(TICK_SRC_A::ADC_DONE),
            6 => Some(TICK_SRC_A::SMPH_AUTOTAKE_DONE),
            5 => Some(TICK_SRC_A::TIMER1_EV),
            3 => Some(TICK_SRC_A::TDC_DONE),
            2 => Some(TICK_SRC_A::AUX_COMPB),
            1 => Some(TICK_SRC_A::AUX_COMPA),
            0 => Some(TICK_SRC_A::RTC_CH2_EV),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_IRQ`"]
    #[inline(always)]
    pub fn is_adc_irq(&self) -> bool {
        *self == TICK_SRC_A::ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `MCU_EVENT`"]
    #[inline(always)]
    pub fn is_mcu_event(&self) -> bool {
        *self == TICK_SRC_A::MCU_EVENT
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == TICK_SRC_A::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == TICK_SRC_A::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == TICK_SRC_A::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == TICK_SRC_A::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == TICK_SRC_A::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == TICK_SRC_A::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == TICK_SRC_A::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == TICK_SRC_A::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == TICK_SRC_A::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == TICK_SRC_A::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == TICK_SRC_A::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == TICK_SRC_A::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == TICK_SRC_A::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == TICK_SRC_A::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == TICK_SRC_A::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == TICK_SRC_A::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == TICK_SRC_A::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AON_PROG_WU`"]
    #[inline(always)]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == TICK_SRC_A::AON_PROG_WU
    }
    #[doc = "Checks if the value of the field is `AON_SW`"]
    #[inline(always)]
    pub fn is_aon_sw(&self) -> bool {
        *self == TICK_SRC_A::AON_SW
    }
    #[doc = "Checks if the value of the field is `OBSMUX1`"]
    #[inline(always)]
    pub fn is_obsmux1(&self) -> bool {
        *self == TICK_SRC_A::OBSMUX1
    }
    #[doc = "Checks if the value of the field is `OBSMUX0`"]
    #[inline(always)]
    pub fn is_obsmux0(&self) -> bool {
        *self == TICK_SRC_A::OBSMUX0
    }
    #[doc = "Checks if the value of the field is `RTC_4KHZ`"]
    #[inline(always)]
    pub fn is_rtc_4khz(&self) -> bool {
        *self == TICK_SRC_A::RTC_4KHZ
    }
    #[doc = "Checks if the value of the field is `ADC_DONE`"]
    #[inline(always)]
    pub fn is_adc_done(&self) -> bool {
        *self == TICK_SRC_A::ADC_DONE
    }
    #[doc = "Checks if the value of the field is `SMPH_AUTOTAKE_DONE`"]
    #[inline(always)]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == TICK_SRC_A::SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `TIMER1_EV`"]
    #[inline(always)]
    pub fn is_timer1_ev(&self) -> bool {
        *self == TICK_SRC_A::TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `TDC_DONE`"]
    #[inline(always)]
    pub fn is_tdc_done(&self) -> bool {
        *self == TICK_SRC_A::TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == TICK_SRC_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == TICK_SRC_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_EV`"]
    #[inline(always)]
    pub fn is_rtc_ch2_ev(&self) -> bool {
        *self == TICK_SRC_A::RTC_CH2_EV
    }
}
#[doc = "Field `TICK_SRC` writer - 12:8\\]
Select Timer 0 tick source from the synchronous event bus."]
pub type TICK_SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, T0CFG_SPEC, u8, TICK_SRC_A, 5, O>;
impl<'a, const O: u8> TICK_SRC_W<'a, O> {
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn adc_irq(self) -> &'a mut W {
        self.variant(TICK_SRC_A::ADC_IRQ)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn mcu_event(self) -> &'a mut W {
        self.variant(TICK_SRC_A::MCU_EVENT)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(TICK_SRC_A::ACLK_REF)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO15)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO14)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO13)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO12)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO11)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO10)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO9)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO8)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO7)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO6)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO5)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO4)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO0)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AON_PROG_WU)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn aon_sw(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AON_SW)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn obsmux1(self) -> &'a mut W {
        self.variant(TICK_SRC_A::OBSMUX1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn obsmux0(self) -> &'a mut W {
        self.variant(TICK_SRC_A::OBSMUX0)
    }
    #[doc = "AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    #[inline(always)]
    pub fn rtc_4khz(self) -> &'a mut W {
        self.variant(TICK_SRC_A::RTC_4KHZ)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn adc_done(self) -> &'a mut W {
        self.variant(TICK_SRC_A::ADC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn smph_autotake_done(self) -> &'a mut W {
        self.variant(TICK_SRC_A::SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(self) -> &'a mut W {
        self.variant(TICK_SRC_A::TIMER1_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(self) -> &'a mut W {
        self.variant(TICK_SRC_A::TDC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_COMPB)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_COMPA)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn rtc_ch2_ev(self) -> &'a mut W {
        self.variant(TICK_SRC_A::RTC_CH2_EV)
    }
}
#[doc = "Field `TICK_SRC_POL` reader - 13:13\\]
Tick source polarity for Timer 0."]
pub type TICK_SRC_POL_R = crate::BitReader<TICK_SRC_POL_A>;
#[doc = "13:13\\]
Tick source polarity for Timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TICK_SRC_POL_A {
    #[doc = "1: Count on falling edges of TICK_SRC."]
    FALL = 1,
    #[doc = "0: Count on rising edges of TICK_SRC."]
    RISE = 0,
}
impl From<TICK_SRC_POL_A> for bool {
    #[inline(always)]
    fn from(variant: TICK_SRC_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl TICK_SRC_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICK_SRC_POL_A {
        match self.bits {
            true => TICK_SRC_POL_A::FALL,
            false => TICK_SRC_POL_A::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == TICK_SRC_POL_A::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == TICK_SRC_POL_A::RISE
    }
}
#[doc = "Field `TICK_SRC_POL` writer - 13:13\\]
Tick source polarity for Timer 0."]
pub type TICK_SRC_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CFG_SPEC, TICK_SRC_POL_A, O>;
impl<'a, const O: u8> TICK_SRC_POL_W<'a, O> {
    #[doc = "Count on falling edges of TICK_SRC."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(TICK_SRC_POL_A::FALL)
    }
    #[doc = "Count on rising edges of TICK_SRC."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(TICK_SRC_POL_A::RISE)
    }
}
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0CFG_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Timer 0 reload mode."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select Timer 0 tick source from the synchronous event bus."]
    #[inline(always)]
    pub fn tick_src(&self) -> TICK_SRC_R {
        TICK_SRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Tick source polarity for Timer 0."]
    #[inline(always)]
    pub fn tick_src_pol(&self) -> TICK_SRC_POL_R {
        TICK_SRC_POL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Timer 0 reload mode."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<0> {
        RELOAD_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<1> {
        MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PRE_W<4> {
        PRE_W::new(self)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select Timer 0 tick source from the synchronous event bus."]
    #[inline(always)]
    #[must_use]
    pub fn tick_src(&mut self) -> TICK_SRC_W<8> {
        TICK_SRC_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Tick source polarity for Timer 0."]
    #[inline(always)]
    #[must_use]
    pub fn tick_src_pol(&mut self) -> TICK_SRC_POL_W<13> {
        TICK_SRC_POL_W::new(self)
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
#[doc = "Timer 0 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0cfg](index.html) module"]
pub struct T0CFG_SPEC;
impl crate::RegisterSpec for T0CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0cfg::R](R) reader structure"]
impl crate::Readable for T0CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0cfg::W](W) writer structure"]
impl crate::Writable for T0CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0CFG to value 0"]
impl crate::Resettable for T0CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

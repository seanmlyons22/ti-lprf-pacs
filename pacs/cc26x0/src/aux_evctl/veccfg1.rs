#[doc = "Register `VECCFG1` reader"]
pub struct R(crate::R<VECCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VECCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VECCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VECCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VECCFG1` writer"]
pub struct W(crate::W<VECCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VECCFG1_SPEC>;
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
impl From<crate::W<VECCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VECCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VEC2_EV` reader - 4:0\\]
Select vector 2 trigger source event."]
pub type VEC2_EV_R = crate::FieldReader<u8, VEC2_EV_A>;
#[doc = "4:0\\]
Select vector 2 trigger source event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VEC2_EV_A {
    #[doc = "31: EVSTAT1.ADC_IRQ"]
    ADC_IRQ = 31,
    #[doc = "30: EVSTAT1.MCU_EV"]
    MCU_EV = 30,
    #[doc = "29: EVSTAT1.ACLK_REF"]
    ACLK_REF = 29,
    #[doc = "28: EVSTAT1.AUXIO15"]
    AUXIO15 = 28,
    #[doc = "27: EVSTAT1.AUXIO14"]
    AUXIO14 = 27,
    #[doc = "26: EVSTAT1.AUXIO13"]
    AUXIO13 = 26,
    #[doc = "25: EVSTAT1.AUXIO12"]
    AUXIO12 = 25,
    #[doc = "24: EVSTAT1.AUXIO11"]
    AUXIO11 = 24,
    #[doc = "23: EVSTAT1.AUXIO10"]
    AUXIO10 = 23,
    #[doc = "22: EVSTAT1.AUXIO9"]
    AUXIO9 = 22,
    #[doc = "21: EVSTAT1.AUXIO8"]
    AUXIO8 = 21,
    #[doc = "20: EVSTAT1.AUXIO7"]
    AUXIO7 = 20,
    #[doc = "19: EVSTAT1.AUXIO6"]
    AUXIO6 = 19,
    #[doc = "18: EVSTAT1.AUXIO5"]
    AUXIO5 = 18,
    #[doc = "17: EVSTAT1.AUXIO4"]
    AUXIO4 = 17,
    #[doc = "16: EVSTAT1.AUXIO3"]
    AUXIO3 = 16,
    #[doc = "15: EVSTAT0.AUXIO2"]
    AUXIO2 = 15,
    #[doc = "14: EVSTAT0.AUXIO1"]
    AUXIO1 = 14,
    #[doc = "13: EVSTAT0.AUXIO0"]
    AUXIO0 = 13,
    #[doc = "12: EVSTAT0.AON_PROG_WU"]
    AON_PROG_WU = 12,
    #[doc = "11: EVSTAT0.AON_SW"]
    AON_SW = 11,
    #[doc = "10: EVSTAT0.OBSMUX1"]
    OBSMUX1 = 10,
    #[doc = "9: EVSTAT0.OBSMUX0"]
    OBSMUX0 = 9,
    #[doc = "8: EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    ADC_FIFO_ALMOST_FULL = 8,
    #[doc = "7: EVSTAT0.ADC_DONE"]
    ADC_DONE = 7,
    #[doc = "6: EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE = 6,
    #[doc = "5: EVSTAT0.TIMER1_EV"]
    TIMER1_EV = 5,
    #[doc = "4: EVSTAT0.TIMER0_EV"]
    TIMER0_EV = 4,
    #[doc = "3: EVSTAT0.TDC_DONE"]
    TDC_DONE = 3,
    #[doc = "2: EVSTAT0.AUX_COMPB"]
    AUX_COMPB = 2,
    #[doc = "1: EVSTAT0.AUX_COMPA"]
    AUX_COMPA = 1,
    #[doc = "0: EVSTAT0.AON_RTC_CH2"]
    AON_RTC_CH2 = 0,
}
impl From<VEC2_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: VEC2_EV_A) -> Self {
        variant as _
    }
}
impl VEC2_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VEC2_EV_A {
        match self.bits {
            31 => VEC2_EV_A::ADC_IRQ,
            30 => VEC2_EV_A::MCU_EV,
            29 => VEC2_EV_A::ACLK_REF,
            28 => VEC2_EV_A::AUXIO15,
            27 => VEC2_EV_A::AUXIO14,
            26 => VEC2_EV_A::AUXIO13,
            25 => VEC2_EV_A::AUXIO12,
            24 => VEC2_EV_A::AUXIO11,
            23 => VEC2_EV_A::AUXIO10,
            22 => VEC2_EV_A::AUXIO9,
            21 => VEC2_EV_A::AUXIO8,
            20 => VEC2_EV_A::AUXIO7,
            19 => VEC2_EV_A::AUXIO6,
            18 => VEC2_EV_A::AUXIO5,
            17 => VEC2_EV_A::AUXIO4,
            16 => VEC2_EV_A::AUXIO3,
            15 => VEC2_EV_A::AUXIO2,
            14 => VEC2_EV_A::AUXIO1,
            13 => VEC2_EV_A::AUXIO0,
            12 => VEC2_EV_A::AON_PROG_WU,
            11 => VEC2_EV_A::AON_SW,
            10 => VEC2_EV_A::OBSMUX1,
            9 => VEC2_EV_A::OBSMUX0,
            8 => VEC2_EV_A::ADC_FIFO_ALMOST_FULL,
            7 => VEC2_EV_A::ADC_DONE,
            6 => VEC2_EV_A::SMPH_AUTOTAKE_DONE,
            5 => VEC2_EV_A::TIMER1_EV,
            4 => VEC2_EV_A::TIMER0_EV,
            3 => VEC2_EV_A::TDC_DONE,
            2 => VEC2_EV_A::AUX_COMPB,
            1 => VEC2_EV_A::AUX_COMPA,
            0 => VEC2_EV_A::AON_RTC_CH2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_IRQ`"]
    #[inline(always)]
    pub fn is_adc_irq(&self) -> bool {
        *self == VEC2_EV_A::ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == VEC2_EV_A::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == VEC2_EV_A::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == VEC2_EV_A::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == VEC2_EV_A::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == VEC2_EV_A::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == VEC2_EV_A::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == VEC2_EV_A::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == VEC2_EV_A::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == VEC2_EV_A::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == VEC2_EV_A::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == VEC2_EV_A::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == VEC2_EV_A::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == VEC2_EV_A::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == VEC2_EV_A::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == VEC2_EV_A::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == VEC2_EV_A::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == VEC2_EV_A::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == VEC2_EV_A::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AON_PROG_WU`"]
    #[inline(always)]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == VEC2_EV_A::AON_PROG_WU
    }
    #[doc = "Checks if the value of the field is `AON_SW`"]
    #[inline(always)]
    pub fn is_aon_sw(&self) -> bool {
        *self == VEC2_EV_A::AON_SW
    }
    #[doc = "Checks if the value of the field is `OBSMUX1`"]
    #[inline(always)]
    pub fn is_obsmux1(&self) -> bool {
        *self == VEC2_EV_A::OBSMUX1
    }
    #[doc = "Checks if the value of the field is `OBSMUX0`"]
    #[inline(always)]
    pub fn is_obsmux0(&self) -> bool {
        *self == VEC2_EV_A::OBSMUX0
    }
    #[doc = "Checks if the value of the field is `ADC_FIFO_ALMOST_FULL`"]
    #[inline(always)]
    pub fn is_adc_fifo_almost_full(&self) -> bool {
        *self == VEC2_EV_A::ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `ADC_DONE`"]
    #[inline(always)]
    pub fn is_adc_done(&self) -> bool {
        *self == VEC2_EV_A::ADC_DONE
    }
    #[doc = "Checks if the value of the field is `SMPH_AUTOTAKE_DONE`"]
    #[inline(always)]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == VEC2_EV_A::SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `TIMER1_EV`"]
    #[inline(always)]
    pub fn is_timer1_ev(&self) -> bool {
        *self == VEC2_EV_A::TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `TIMER0_EV`"]
    #[inline(always)]
    pub fn is_timer0_ev(&self) -> bool {
        *self == VEC2_EV_A::TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `TDC_DONE`"]
    #[inline(always)]
    pub fn is_tdc_done(&self) -> bool {
        *self == VEC2_EV_A::TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == VEC2_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == VEC2_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == VEC2_EV_A::AON_RTC_CH2
    }
}
#[doc = "Field `VEC2_EV` writer - 4:0\\]
Select vector 2 trigger source event."]
pub type VEC2_EV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, VECCFG1_SPEC, u8, VEC2_EV_A, 5, O>;
impl<'a, const O: u8> VEC2_EV_W<'a, O> {
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn adc_irq(self) -> &'a mut W {
        self.variant(VEC2_EV_A::ADC_IRQ)
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(VEC2_EV_A::MCU_EV)
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(VEC2_EV_A::ACLK_REF)
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO15)
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO14)
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO13)
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO12)
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO11)
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO10)
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO9)
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO8)
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO7)
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO6)
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO5)
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO4)
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUXIO0)
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AON_PROG_WU)
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn aon_sw(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AON_SW)
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn obsmux1(self) -> &'a mut W {
        self.variant(VEC2_EV_A::OBSMUX1)
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn obsmux0(self) -> &'a mut W {
        self.variant(VEC2_EV_A::OBSMUX0)
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(VEC2_EV_A::ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn adc_done(self) -> &'a mut W {
        self.variant(VEC2_EV_A::ADC_DONE)
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn smph_autotake_done(self) -> &'a mut W {
        self.variant(VEC2_EV_A::SMPH_AUTOTAKE_DONE)
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(self) -> &'a mut W {
        self.variant(VEC2_EV_A::TIMER1_EV)
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn timer0_ev(self) -> &'a mut W {
        self.variant(VEC2_EV_A::TIMER0_EV)
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(self) -> &'a mut W {
        self.variant(VEC2_EV_A::TDC_DONE)
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUX_COMPB)
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AUX_COMPA)
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(VEC2_EV_A::AON_RTC_CH2)
    }
}
#[doc = "Field `VEC2_EN` reader - 5:5\\]
Vector 2 trigger enable. When enabled, VEC2_EV event with VEC2_POL polarity triggers a jump to vector # 2 when AUX_SCE sleeps. Lower vectors (0 and 1) have priority."]
pub type VEC2_EN_R = crate::BitReader<VEC2_EN_A>;
#[doc = "5:5\\]
Vector 2 trigger enable. When enabled, VEC2_EV event with VEC2_POL polarity triggers a jump to vector # 2 when AUX_SCE sleeps. Lower vectors (0 and 1) have priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VEC2_EN_A {
    #[doc = "1: Enable vector 2 trigger."]
    EN = 1,
    #[doc = "0: Disable vector 2 trigger."]
    DIS = 0,
}
impl From<VEC2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VEC2_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl VEC2_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VEC2_EN_A {
        match self.bits {
            true => VEC2_EN_A::EN,
            false => VEC2_EN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VEC2_EN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VEC2_EN_A::DIS
    }
}
#[doc = "Field `VEC2_EN` writer - 5:5\\]
Vector 2 trigger enable. When enabled, VEC2_EV event with VEC2_POL polarity triggers a jump to vector # 2 when AUX_SCE sleeps. Lower vectors (0 and 1) have priority."]
pub type VEC2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VECCFG1_SPEC, VEC2_EN_A, O>;
impl<'a, const O: u8> VEC2_EN_W<'a, O> {
    #[doc = "Enable vector 2 trigger."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VEC2_EN_A::EN)
    }
    #[doc = "Disable vector 2 trigger."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VEC2_EN_A::DIS)
    }
}
#[doc = "Field `VEC2_POL` reader - 6:6\\]
Vector 2 trigger event polarity. To manually trigger vector 2 execution: - AUX_SCE must sleep. - Set VEC2_EV to a known static value. - Toggle VEC2_POL twice."]
pub type VEC2_POL_R = crate::BitReader<VEC2_POL_A>;
#[doc = "6:6\\]
Vector 2 trigger event polarity. To manually trigger vector 2 execution: - AUX_SCE must sleep. - Set VEC2_EV to a known static value. - Toggle VEC2_POL twice.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VEC2_POL_A {
    #[doc = "1: Falling edge triggers vector 2 execution."]
    FALL = 1,
    #[doc = "0: Rising edge triggers vector 2 execution."]
    RISE = 0,
}
impl From<VEC2_POL_A> for bool {
    #[inline(always)]
    fn from(variant: VEC2_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl VEC2_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VEC2_POL_A {
        match self.bits {
            true => VEC2_POL_A::FALL,
            false => VEC2_POL_A::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == VEC2_POL_A::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == VEC2_POL_A::RISE
    }
}
#[doc = "Field `VEC2_POL` writer - 6:6\\]
Vector 2 trigger event polarity. To manually trigger vector 2 execution: - AUX_SCE must sleep. - Set VEC2_EV to a known static value. - Toggle VEC2_POL twice."]
pub type VEC2_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, VECCFG1_SPEC, VEC2_POL_A, O>;
impl<'a, const O: u8> VEC2_POL_W<'a, O> {
    #[doc = "Falling edge triggers vector 2 execution."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(VEC2_POL_A::FALL)
    }
    #[doc = "Rising edge triggers vector 2 execution."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(VEC2_POL_A::RISE)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::BitWriter<'a, u32, VECCFG1_SPEC, bool, O>;
#[doc = "Field `VEC3_EV` reader - 12:8\\]
Select vector 3 trigger source event."]
pub type VEC3_EV_R = crate::FieldReader<u8, VEC3_EV_A>;
#[doc = "12:8\\]
Select vector 3 trigger source event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VEC3_EV_A {
    #[doc = "31: EVSTAT1.ADC_IRQ"]
    ADC_IRQ = 31,
    #[doc = "30: EVSTAT1.MCU_EV"]
    MCU_EV = 30,
    #[doc = "29: EVSTAT1.ACLK_REF"]
    ACLK_REF = 29,
    #[doc = "28: EVSTAT1.AUXIO15"]
    AUXIO15 = 28,
    #[doc = "27: EVSTAT1.AUXIO14"]
    AUXIO14 = 27,
    #[doc = "26: EVSTAT1.AUXIO13"]
    AUXIO13 = 26,
    #[doc = "25: EVSTAT1.AUXIO12"]
    AUXIO12 = 25,
    #[doc = "24: EVSTAT1.AUXIO11"]
    AUXIO11 = 24,
    #[doc = "23: EVSTAT1.AUXIO10"]
    AUXIO10 = 23,
    #[doc = "22: EVSTAT1.AUXIO9"]
    AUXIO9 = 22,
    #[doc = "21: EVSTAT1.AUXIO8"]
    AUXIO8 = 21,
    #[doc = "20: EVSTAT1.AUXIO7"]
    AUXIO7 = 20,
    #[doc = "19: EVSTAT1.AUXIO6"]
    AUXIO6 = 19,
    #[doc = "18: EVSTAT1.AUXIO5"]
    AUXIO5 = 18,
    #[doc = "17: EVSTAT1.AUXIO4"]
    AUXIO4 = 17,
    #[doc = "16: EVSTAT1.AUXIO3"]
    AUXIO3 = 16,
    #[doc = "15: EVSTAT0.AUXIO2"]
    AUXIO2 = 15,
    #[doc = "14: EVSTAT0.AUXIO1"]
    AUXIO1 = 14,
    #[doc = "13: EVSTAT0.AUXIO0"]
    AUXIO0 = 13,
    #[doc = "12: EVSTAT0.AON_PROG_WU"]
    AON_PROG_WU = 12,
    #[doc = "11: EVSTAT0.AON_SW"]
    AON_SW = 11,
    #[doc = "10: EVSTAT0.OBSMUX1"]
    OBSMUX1 = 10,
    #[doc = "9: EVSTAT0.OBSMUX0"]
    OBSMUX0 = 9,
    #[doc = "8: EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    ADC_FIFO_ALMOST_FULL = 8,
    #[doc = "7: EVSTAT0.ADC_DONE"]
    ADC_DONE = 7,
    #[doc = "6: EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE = 6,
    #[doc = "5: EVSTAT0.TIMER1_EV"]
    TIMER1_EV = 5,
    #[doc = "4: EVSTAT0.TIMER0_EV"]
    TIMER0_EV = 4,
    #[doc = "3: EVSTAT0.TDC_DONE"]
    TDC_DONE = 3,
    #[doc = "2: EVSTAT0.AUX_COMPB"]
    AUX_COMPB = 2,
    #[doc = "1: EVSTAT0.AUX_COMPA"]
    AUX_COMPA = 1,
    #[doc = "0: EVSTAT0.AON_RTC_CH2"]
    AON_RTC_CH2 = 0,
}
impl From<VEC3_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: VEC3_EV_A) -> Self {
        variant as _
    }
}
impl VEC3_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VEC3_EV_A {
        match self.bits {
            31 => VEC3_EV_A::ADC_IRQ,
            30 => VEC3_EV_A::MCU_EV,
            29 => VEC3_EV_A::ACLK_REF,
            28 => VEC3_EV_A::AUXIO15,
            27 => VEC3_EV_A::AUXIO14,
            26 => VEC3_EV_A::AUXIO13,
            25 => VEC3_EV_A::AUXIO12,
            24 => VEC3_EV_A::AUXIO11,
            23 => VEC3_EV_A::AUXIO10,
            22 => VEC3_EV_A::AUXIO9,
            21 => VEC3_EV_A::AUXIO8,
            20 => VEC3_EV_A::AUXIO7,
            19 => VEC3_EV_A::AUXIO6,
            18 => VEC3_EV_A::AUXIO5,
            17 => VEC3_EV_A::AUXIO4,
            16 => VEC3_EV_A::AUXIO3,
            15 => VEC3_EV_A::AUXIO2,
            14 => VEC3_EV_A::AUXIO1,
            13 => VEC3_EV_A::AUXIO0,
            12 => VEC3_EV_A::AON_PROG_WU,
            11 => VEC3_EV_A::AON_SW,
            10 => VEC3_EV_A::OBSMUX1,
            9 => VEC3_EV_A::OBSMUX0,
            8 => VEC3_EV_A::ADC_FIFO_ALMOST_FULL,
            7 => VEC3_EV_A::ADC_DONE,
            6 => VEC3_EV_A::SMPH_AUTOTAKE_DONE,
            5 => VEC3_EV_A::TIMER1_EV,
            4 => VEC3_EV_A::TIMER0_EV,
            3 => VEC3_EV_A::TDC_DONE,
            2 => VEC3_EV_A::AUX_COMPB,
            1 => VEC3_EV_A::AUX_COMPA,
            0 => VEC3_EV_A::AON_RTC_CH2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_IRQ`"]
    #[inline(always)]
    pub fn is_adc_irq(&self) -> bool {
        *self == VEC3_EV_A::ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == VEC3_EV_A::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == VEC3_EV_A::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == VEC3_EV_A::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == VEC3_EV_A::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == VEC3_EV_A::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == VEC3_EV_A::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == VEC3_EV_A::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == VEC3_EV_A::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == VEC3_EV_A::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == VEC3_EV_A::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == VEC3_EV_A::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == VEC3_EV_A::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == VEC3_EV_A::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == VEC3_EV_A::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == VEC3_EV_A::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == VEC3_EV_A::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == VEC3_EV_A::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == VEC3_EV_A::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AON_PROG_WU`"]
    #[inline(always)]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == VEC3_EV_A::AON_PROG_WU
    }
    #[doc = "Checks if the value of the field is `AON_SW`"]
    #[inline(always)]
    pub fn is_aon_sw(&self) -> bool {
        *self == VEC3_EV_A::AON_SW
    }
    #[doc = "Checks if the value of the field is `OBSMUX1`"]
    #[inline(always)]
    pub fn is_obsmux1(&self) -> bool {
        *self == VEC3_EV_A::OBSMUX1
    }
    #[doc = "Checks if the value of the field is `OBSMUX0`"]
    #[inline(always)]
    pub fn is_obsmux0(&self) -> bool {
        *self == VEC3_EV_A::OBSMUX0
    }
    #[doc = "Checks if the value of the field is `ADC_FIFO_ALMOST_FULL`"]
    #[inline(always)]
    pub fn is_adc_fifo_almost_full(&self) -> bool {
        *self == VEC3_EV_A::ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `ADC_DONE`"]
    #[inline(always)]
    pub fn is_adc_done(&self) -> bool {
        *self == VEC3_EV_A::ADC_DONE
    }
    #[doc = "Checks if the value of the field is `SMPH_AUTOTAKE_DONE`"]
    #[inline(always)]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == VEC3_EV_A::SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `TIMER1_EV`"]
    #[inline(always)]
    pub fn is_timer1_ev(&self) -> bool {
        *self == VEC3_EV_A::TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `TIMER0_EV`"]
    #[inline(always)]
    pub fn is_timer0_ev(&self) -> bool {
        *self == VEC3_EV_A::TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `TDC_DONE`"]
    #[inline(always)]
    pub fn is_tdc_done(&self) -> bool {
        *self == VEC3_EV_A::TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == VEC3_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == VEC3_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == VEC3_EV_A::AON_RTC_CH2
    }
}
#[doc = "Field `VEC3_EV` writer - 12:8\\]
Select vector 3 trigger source event."]
pub type VEC3_EV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, VECCFG1_SPEC, u8, VEC3_EV_A, 5, O>;
impl<'a, const O: u8> VEC3_EV_W<'a, O> {
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn adc_irq(self) -> &'a mut W {
        self.variant(VEC3_EV_A::ADC_IRQ)
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(VEC3_EV_A::MCU_EV)
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(VEC3_EV_A::ACLK_REF)
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO15)
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO14)
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO13)
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO12)
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO11)
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO10)
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO9)
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO8)
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO7)
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO6)
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO5)
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO4)
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUXIO0)
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AON_PROG_WU)
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn aon_sw(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AON_SW)
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn obsmux1(self) -> &'a mut W {
        self.variant(VEC3_EV_A::OBSMUX1)
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn obsmux0(self) -> &'a mut W {
        self.variant(VEC3_EV_A::OBSMUX0)
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(VEC3_EV_A::ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn adc_done(self) -> &'a mut W {
        self.variant(VEC3_EV_A::ADC_DONE)
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn smph_autotake_done(self) -> &'a mut W {
        self.variant(VEC3_EV_A::SMPH_AUTOTAKE_DONE)
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(self) -> &'a mut W {
        self.variant(VEC3_EV_A::TIMER1_EV)
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn timer0_ev(self) -> &'a mut W {
        self.variant(VEC3_EV_A::TIMER0_EV)
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(self) -> &'a mut W {
        self.variant(VEC3_EV_A::TDC_DONE)
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUX_COMPB)
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AUX_COMPA)
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(VEC3_EV_A::AON_RTC_CH2)
    }
}
#[doc = "Field `VEC3_EN` reader - 13:13\\]
Vector 3 trigger enable. When enabled, VEC3_EV event with VEC3_POL polarity triggers a jump to vector # 3 when AUX_SCE sleeps. Lower vectors (0, 1, and 2) have priority."]
pub type VEC3_EN_R = crate::BitReader<VEC3_EN_A>;
#[doc = "13:13\\]
Vector 3 trigger enable. When enabled, VEC3_EV event with VEC3_POL polarity triggers a jump to vector # 3 when AUX_SCE sleeps. Lower vectors (0, 1, and 2) have priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VEC3_EN_A {
    #[doc = "1: Enable vector 3 trigger."]
    EN = 1,
    #[doc = "0: Disable vector 3 trigger."]
    DIS = 0,
}
impl From<VEC3_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VEC3_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl VEC3_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VEC3_EN_A {
        match self.bits {
            true => VEC3_EN_A::EN,
            false => VEC3_EN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VEC3_EN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VEC3_EN_A::DIS
    }
}
#[doc = "Field `VEC3_EN` writer - 13:13\\]
Vector 3 trigger enable. When enabled, VEC3_EV event with VEC3_POL polarity triggers a jump to vector # 3 when AUX_SCE sleeps. Lower vectors (0, 1, and 2) have priority."]
pub type VEC3_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VECCFG1_SPEC, VEC3_EN_A, O>;
impl<'a, const O: u8> VEC3_EN_W<'a, O> {
    #[doc = "Enable vector 3 trigger."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VEC3_EN_A::EN)
    }
    #[doc = "Disable vector 3 trigger."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VEC3_EN_A::DIS)
    }
}
#[doc = "Field `VEC3_POL` reader - 14:14\\]
Vector 3 trigger event polarity. To manually trigger vector 3 execution: - AUX_SCE must sleep. - Set VEC3_EV to a known static value. - Toggle VEC3_POL twice."]
pub type VEC3_POL_R = crate::BitReader<VEC3_POL_A>;
#[doc = "14:14\\]
Vector 3 trigger event polarity. To manually trigger vector 3 execution: - AUX_SCE must sleep. - Set VEC3_EV to a known static value. - Toggle VEC3_POL twice.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VEC3_POL_A {
    #[doc = "1: Falling edge triggers vector 3 execution."]
    FALL = 1,
    #[doc = "0: Rising edge triggers vector 3 execution."]
    RISE = 0,
}
impl From<VEC3_POL_A> for bool {
    #[inline(always)]
    fn from(variant: VEC3_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl VEC3_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VEC3_POL_A {
        match self.bits {
            true => VEC3_POL_A::FALL,
            false => VEC3_POL_A::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == VEC3_POL_A::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == VEC3_POL_A::RISE
    }
}
#[doc = "Field `VEC3_POL` writer - 14:14\\]
Vector 3 trigger event polarity. To manually trigger vector 3 execution: - AUX_SCE must sleep. - Set VEC3_EV to a known static value. - Toggle VEC3_POL twice."]
pub type VEC3_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, VECCFG1_SPEC, VEC3_POL_A, O>;
impl<'a, const O: u8> VEC3_POL_W<'a, O> {
    #[doc = "Falling edge triggers vector 3 execution."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(VEC3_POL_A::FALL)
    }
    #[doc = "Rising edge triggers vector 3 execution."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(VEC3_POL_A::RISE)
    }
}
#[doc = "Field `RESERVED15` reader - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED15` writer - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VECCFG1_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Select vector 2 trigger source event."]
    #[inline(always)]
    pub fn vec2_ev(&self) -> VEC2_EV_R {
        VEC2_EV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Vector 2 trigger enable. When enabled, VEC2_EV event with VEC2_POL polarity triggers a jump to vector # 2 when AUX_SCE sleeps. Lower vectors (0 and 1) have priority."]
    #[inline(always)]
    pub fn vec2_en(&self) -> VEC2_EN_R {
        VEC2_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Vector 2 trigger event polarity. To manually trigger vector 2 execution: - AUX_SCE must sleep. - Set VEC2_EV to a known static value. - Toggle VEC2_POL twice."]
    #[inline(always)]
    pub fn vec2_pol(&self) -> VEC2_POL_R {
        VEC2_POL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select vector 3 trigger source event."]
    #[inline(always)]
    pub fn vec3_ev(&self) -> VEC3_EV_R {
        VEC3_EV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Vector 3 trigger enable. When enabled, VEC3_EV event with VEC3_POL polarity triggers a jump to vector # 3 when AUX_SCE sleeps. Lower vectors (0, 1, and 2) have priority."]
    #[inline(always)]
    pub fn vec3_en(&self) -> VEC3_EN_R {
        VEC3_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Vector 3 trigger event polarity. To manually trigger vector 3 execution: - AUX_SCE must sleep. - Set VEC3_EV to a known static value. - Toggle VEC3_POL twice."]
    #[inline(always)]
    pub fn vec3_pol(&self) -> VEC3_POL_R {
        VEC3_POL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Select vector 2 trigger source event."]
    #[inline(always)]
    #[must_use]
    pub fn vec2_ev(&mut self) -> VEC2_EV_W<0> {
        VEC2_EV_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Vector 2 trigger enable. When enabled, VEC2_EV event with VEC2_POL polarity triggers a jump to vector # 2 when AUX_SCE sleeps. Lower vectors (0 and 1) have priority."]
    #[inline(always)]
    #[must_use]
    pub fn vec2_en(&mut self) -> VEC2_EN_W<5> {
        VEC2_EN_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Vector 2 trigger event polarity. To manually trigger vector 2 execution: - AUX_SCE must sleep. - Set VEC2_EV to a known static value. - Toggle VEC2_POL twice."]
    #[inline(always)]
    #[must_use]
    pub fn vec2_pol(&mut self) -> VEC2_POL_W<6> {
        VEC2_POL_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select vector 3 trigger source event."]
    #[inline(always)]
    #[must_use]
    pub fn vec3_ev(&mut self) -> VEC3_EV_W<8> {
        VEC3_EV_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Vector 3 trigger enable. When enabled, VEC3_EV event with VEC3_POL polarity triggers a jump to vector # 3 when AUX_SCE sleeps. Lower vectors (0, 1, and 2) have priority."]
    #[inline(always)]
    #[must_use]
    pub fn vec3_en(&mut self) -> VEC3_EN_W<13> {
        VEC3_EN_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Vector 3 trigger event polarity. To manually trigger vector 3 execution: - AUX_SCE must sleep. - Set VEC3_EV to a known static value. - Toggle VEC3_POL twice."]
    #[inline(always)]
    #[must_use]
    pub fn vec3_pol(&mut self) -> VEC3_POL_W<14> {
        VEC3_POL_W::new(self)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> RESERVED15_W<15> {
        RESERVED15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vector Configuration 1 AUX_SCE event vectors 2 and 3 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [veccfg1](index.html) module"]
pub struct VECCFG1_SPEC;
impl crate::RegisterSpec for VECCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [veccfg1::R](R) reader structure"]
impl crate::Readable for VECCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [veccfg1::W](W) writer structure"]
impl crate::Writable for VECCFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VECCFG1 to value 0"]
impl crate::Resettable for VECCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

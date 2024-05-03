#[doc = "Register `VECCFG0` reader"]
pub type R = crate::R<Veccfg0Spec>;
#[doc = "Register `VECCFG0` writer"]
pub type W = crate::W<Veccfg0Spec>;
#[doc = "4:0\\]
Select vector 0 trigger source event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vec0Ev {
    #[doc = "31: EVSTAT1.ADC_IRQ"]
    AdcIrq = 31,
    #[doc = "30: EVSTAT1.MCU_EV"]
    McuEv = 30,
    #[doc = "29: EVSTAT1.ACLK_REF"]
    AclkRef = 29,
    #[doc = "28: EVSTAT1.AUXIO15"]
    Auxio15 = 28,
    #[doc = "27: EVSTAT1.AUXIO14"]
    Auxio14 = 27,
    #[doc = "26: EVSTAT1.AUXIO13"]
    Auxio13 = 26,
    #[doc = "25: EVSTAT1.AUXIO12"]
    Auxio12 = 25,
    #[doc = "24: EVSTAT1.AUXIO11"]
    Auxio11 = 24,
    #[doc = "23: EVSTAT1.AUXIO10"]
    Auxio10 = 23,
    #[doc = "22: EVSTAT1.AUXIO9"]
    Auxio9 = 22,
    #[doc = "21: EVSTAT1.AUXIO8"]
    Auxio8 = 21,
    #[doc = "20: EVSTAT1.AUXIO7"]
    Auxio7 = 20,
    #[doc = "19: EVSTAT1.AUXIO6"]
    Auxio6 = 19,
    #[doc = "18: EVSTAT1.AUXIO5"]
    Auxio5 = 18,
    #[doc = "17: EVSTAT1.AUXIO4"]
    Auxio4 = 17,
    #[doc = "16: EVSTAT1.AUXIO3"]
    Auxio3 = 16,
    #[doc = "15: EVSTAT0.AUXIO2"]
    Auxio2 = 15,
    #[doc = "14: EVSTAT0.AUXIO1"]
    Auxio1 = 14,
    #[doc = "13: EVSTAT0.AUXIO0"]
    Auxio0 = 13,
    #[doc = "12: EVSTAT0.AON_PROG_WU"]
    AonProgWu = 12,
    #[doc = "11: EVSTAT0.AON_SW"]
    AonSw = 11,
    #[doc = "10: EVSTAT0.OBSMUX1"]
    Obsmux1 = 10,
    #[doc = "9: EVSTAT0.OBSMUX0"]
    Obsmux0 = 9,
    #[doc = "8: EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    AdcFifoAlmostFull = 8,
    #[doc = "7: EVSTAT0.ADC_DONE"]
    AdcDone = 7,
    #[doc = "6: EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SmphAutotakeDone = 6,
    #[doc = "5: EVSTAT0.TIMER1_EV"]
    Timer1Ev = 5,
    #[doc = "4: EVSTAT0.TIMER0_EV"]
    Timer0Ev = 4,
    #[doc = "3: EVSTAT0.TDC_DONE"]
    TdcDone = 3,
    #[doc = "2: EVSTAT0.AUX_COMPB"]
    AuxCompb = 2,
    #[doc = "1: EVSTAT0.AUX_COMPA"]
    AuxCompa = 1,
    #[doc = "0: EVSTAT0.AON_RTC_CH2"]
    AonRtcCh2 = 0,
}
impl From<Vec0Ev> for u8 {
    #[inline(always)]
    fn from(variant: Vec0Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vec0Ev {
    type Ux = u8;
}
impl crate::IsEnum for Vec0Ev {}
#[doc = "Field `VEC0_EV` reader - 4:0\\]
Select vector 0 trigger source event."]
pub type Vec0EvR = crate::FieldReader<Vec0Ev>;
impl Vec0EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vec0Ev {
        match self.bits {
            31 => Vec0Ev::AdcIrq,
            30 => Vec0Ev::McuEv,
            29 => Vec0Ev::AclkRef,
            28 => Vec0Ev::Auxio15,
            27 => Vec0Ev::Auxio14,
            26 => Vec0Ev::Auxio13,
            25 => Vec0Ev::Auxio12,
            24 => Vec0Ev::Auxio11,
            23 => Vec0Ev::Auxio10,
            22 => Vec0Ev::Auxio9,
            21 => Vec0Ev::Auxio8,
            20 => Vec0Ev::Auxio7,
            19 => Vec0Ev::Auxio6,
            18 => Vec0Ev::Auxio5,
            17 => Vec0Ev::Auxio4,
            16 => Vec0Ev::Auxio3,
            15 => Vec0Ev::Auxio2,
            14 => Vec0Ev::Auxio1,
            13 => Vec0Ev::Auxio0,
            12 => Vec0Ev::AonProgWu,
            11 => Vec0Ev::AonSw,
            10 => Vec0Ev::Obsmux1,
            9 => Vec0Ev::Obsmux0,
            8 => Vec0Ev::AdcFifoAlmostFull,
            7 => Vec0Ev::AdcDone,
            6 => Vec0Ev::SmphAutotakeDone,
            5 => Vec0Ev::Timer1Ev,
            4 => Vec0Ev::Timer0Ev,
            3 => Vec0Ev::TdcDone,
            2 => Vec0Ev::AuxCompb,
            1 => Vec0Ev::AuxCompa,
            0 => Vec0Ev::AonRtcCh2,
            _ => unreachable!(),
        }
    }
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn is_adc_irq(&self) -> bool {
        *self == Vec0Ev::AdcIrq
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == Vec0Ev::McuEv
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == Vec0Ev::AclkRef
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == Vec0Ev::Auxio15
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == Vec0Ev::Auxio14
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == Vec0Ev::Auxio13
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == Vec0Ev::Auxio12
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == Vec0Ev::Auxio11
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == Vec0Ev::Auxio10
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == Vec0Ev::Auxio9
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == Vec0Ev::Auxio8
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == Vec0Ev::Auxio7
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == Vec0Ev::Auxio6
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == Vec0Ev::Auxio5
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == Vec0Ev::Auxio4
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == Vec0Ev::Auxio3
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == Vec0Ev::Auxio2
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == Vec0Ev::Auxio1
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == Vec0Ev::Auxio0
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == Vec0Ev::AonProgWu
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn is_aon_sw(&self) -> bool {
        *self == Vec0Ev::AonSw
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn is_obsmux1(&self) -> bool {
        *self == Vec0Ev::Obsmux1
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn is_obsmux0(&self) -> bool {
        *self == Vec0Ev::Obsmux0
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn is_adc_fifo_almost_full(&self) -> bool {
        *self == Vec0Ev::AdcFifoAlmostFull
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn is_adc_done(&self) -> bool {
        *self == Vec0Ev::AdcDone
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == Vec0Ev::SmphAutotakeDone
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn is_timer1_ev(&self) -> bool {
        *self == Vec0Ev::Timer1Ev
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn is_timer0_ev(&self) -> bool {
        *self == Vec0Ev::Timer0Ev
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn is_tdc_done(&self) -> bool {
        *self == Vec0Ev::TdcDone
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Vec0Ev::AuxCompb
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Vec0Ev::AuxCompa
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == Vec0Ev::AonRtcCh2
    }
}
#[doc = "Field `VEC0_EV` writer - 4:0\\]
Select vector 0 trigger source event."]
pub type Vec0EvW<'a, REG> = crate::FieldWriter<'a, REG, 5, Vec0Ev, crate::Safe>;
impl<'a, REG> Vec0EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::AdcIrq)
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::McuEv)
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::AclkRef)
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio15)
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio14)
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio13)
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio12)
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio11)
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio10)
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio9)
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio8)
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio7)
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio6)
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio5)
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio4)
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Auxio0)
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::AonProgWu)
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn aon_sw(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::AonSw)
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn obsmux1(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Obsmux1)
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Obsmux0)
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::AdcFifoAlmostFull)
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::AdcDone)
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::SmphAutotakeDone)
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Timer1Ev)
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::Timer0Ev)
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::TdcDone)
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::AuxCompb)
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::AuxCompa)
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Ev::AonRtcCh2)
    }
}
#[doc = "5:5\\]
Vector 0 trigger enable. When enabled, VEC0_EV event with VEC0_POL polarity triggers a jump to vector # 0 when AUX_SCE sleeps.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vec0En {
    #[doc = "1: Enable vector 0 trigger."]
    En = 1,
    #[doc = "0: Disable vector 0 trigger."]
    Dis = 0,
}
impl From<Vec0En> for bool {
    #[inline(always)]
    fn from(variant: Vec0En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VEC0_EN` reader - 5:5\\]
Vector 0 trigger enable. When enabled, VEC0_EV event with VEC0_POL polarity triggers a jump to vector # 0 when AUX_SCE sleeps."]
pub type Vec0EnR = crate::BitReader<Vec0En>;
impl Vec0EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vec0En {
        match self.bits {
            true => Vec0En::En,
            false => Vec0En::Dis,
        }
    }
    #[doc = "Enable vector 0 trigger."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Vec0En::En
    }
    #[doc = "Disable vector 0 trigger."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Vec0En::Dis
    }
}
#[doc = "Field `VEC0_EN` writer - 5:5\\]
Vector 0 trigger enable. When enabled, VEC0_EV event with VEC0_POL polarity triggers a jump to vector # 0 when AUX_SCE sleeps."]
pub type Vec0EnW<'a, REG> = crate::BitWriter<'a, REG, Vec0En>;
impl<'a, REG> Vec0EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable vector 0 trigger."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0En::En)
    }
    #[doc = "Disable vector 0 trigger."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0En::Dis)
    }
}
#[doc = "6:6\\]
Vector 0 trigger event polarity. To manually trigger vector 0 execution: - AUX_SCE must sleep. - Set VEC0_EV to a known static value. - Toggle VEC0_POL twice.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vec0Pol {
    #[doc = "1: Falling edge triggers vector 0 execution."]
    Fall = 1,
    #[doc = "0: Rising edge triggers vector 0 execution."]
    Rise = 0,
}
impl From<Vec0Pol> for bool {
    #[inline(always)]
    fn from(variant: Vec0Pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VEC0_POL` reader - 6:6\\]
Vector 0 trigger event polarity. To manually trigger vector 0 execution: - AUX_SCE must sleep. - Set VEC0_EV to a known static value. - Toggle VEC0_POL twice."]
pub type Vec0PolR = crate::BitReader<Vec0Pol>;
impl Vec0PolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vec0Pol {
        match self.bits {
            true => Vec0Pol::Fall,
            false => Vec0Pol::Rise,
        }
    }
    #[doc = "Falling edge triggers vector 0 execution."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Vec0Pol::Fall
    }
    #[doc = "Rising edge triggers vector 0 execution."]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Vec0Pol::Rise
    }
}
#[doc = "Field `VEC0_POL` writer - 6:6\\]
Vector 0 trigger event polarity. To manually trigger vector 0 execution: - AUX_SCE must sleep. - Set VEC0_EV to a known static value. - Toggle VEC0_POL twice."]
pub type Vec0PolW<'a, REG> = crate::BitWriter<'a, REG, Vec0Pol>;
impl<'a, REG> Vec0PolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge triggers vector 0 execution."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Pol::Fall)
    }
    #[doc = "Rising edge triggers vector 0 execution."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Vec0Pol::Rise)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "12:8\\]
Select vector 1 trigger source event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vec1Ev {
    #[doc = "31: EVSTAT1.ADC_IRQ"]
    AdcIrq = 31,
    #[doc = "30: EVSTAT1.MCU_EV"]
    McuEv = 30,
    #[doc = "29: EVSTAT1.ACLK_REF"]
    AclkRef = 29,
    #[doc = "28: EVSTAT1.AUXIO15"]
    Auxio15 = 28,
    #[doc = "27: EVSTAT1.AUXIO14"]
    Auxio14 = 27,
    #[doc = "26: EVSTAT1.AUXIO13"]
    Auxio13 = 26,
    #[doc = "25: EVSTAT1.AUXIO12"]
    Auxio12 = 25,
    #[doc = "24: EVSTAT1.AUXIO11"]
    Auxio11 = 24,
    #[doc = "23: EVSTAT1.AUXIO10"]
    Auxio10 = 23,
    #[doc = "22: EVSTAT1.AUXIO9"]
    Auxio9 = 22,
    #[doc = "21: EVSTAT1.AUXIO8"]
    Auxio8 = 21,
    #[doc = "20: EVSTAT1.AUXIO7"]
    Auxio7 = 20,
    #[doc = "19: EVSTAT1.AUXIO6"]
    Auxio6 = 19,
    #[doc = "18: EVSTAT1.AUXIO5"]
    Auxio5 = 18,
    #[doc = "17: EVSTAT1.AUXIO4"]
    Auxio4 = 17,
    #[doc = "16: EVSTAT1.AUXIO3"]
    Auxio3 = 16,
    #[doc = "15: EVSTAT0.AUXIO2"]
    Auxio2 = 15,
    #[doc = "14: EVSTAT0.AUXIO1"]
    Auxio1 = 14,
    #[doc = "13: EVSTAT0.AUXIO0"]
    Auxio0 = 13,
    #[doc = "12: EVSTAT0.AON_PROG_WU"]
    AonProgWu = 12,
    #[doc = "11: EVSTAT0.AON_SW"]
    AonSw = 11,
    #[doc = "10: EVSTAT0.OBSMUX1"]
    Obsmux1 = 10,
    #[doc = "9: EVSTAT0.OBSMUX0"]
    Obsmux0 = 9,
    #[doc = "8: EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    AdcFifoAlmostFull = 8,
    #[doc = "7: EVSTAT0.ADC_DONE"]
    AdcDone = 7,
    #[doc = "6: EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SmphAutotakeDone = 6,
    #[doc = "5: EVSTAT0.TIMER1_EV"]
    Timer1Ev = 5,
    #[doc = "4: EVSTAT0.TIMER0_EV"]
    Timer0Ev = 4,
    #[doc = "3: EVSTAT0.TDC_DONE"]
    TdcDone = 3,
    #[doc = "2: EVSTAT0.AUX_COMPB"]
    AuxCompb = 2,
    #[doc = "1: EVSTAT0.AUX_COMPA"]
    AuxCompa = 1,
    #[doc = "0: EVSTAT0.AON_RTC_CH2"]
    AonRtcCh2 = 0,
}
impl From<Vec1Ev> for u8 {
    #[inline(always)]
    fn from(variant: Vec1Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vec1Ev {
    type Ux = u8;
}
impl crate::IsEnum for Vec1Ev {}
#[doc = "Field `VEC1_EV` reader - 12:8\\]
Select vector 1 trigger source event."]
pub type Vec1EvR = crate::FieldReader<Vec1Ev>;
impl Vec1EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vec1Ev {
        match self.bits {
            31 => Vec1Ev::AdcIrq,
            30 => Vec1Ev::McuEv,
            29 => Vec1Ev::AclkRef,
            28 => Vec1Ev::Auxio15,
            27 => Vec1Ev::Auxio14,
            26 => Vec1Ev::Auxio13,
            25 => Vec1Ev::Auxio12,
            24 => Vec1Ev::Auxio11,
            23 => Vec1Ev::Auxio10,
            22 => Vec1Ev::Auxio9,
            21 => Vec1Ev::Auxio8,
            20 => Vec1Ev::Auxio7,
            19 => Vec1Ev::Auxio6,
            18 => Vec1Ev::Auxio5,
            17 => Vec1Ev::Auxio4,
            16 => Vec1Ev::Auxio3,
            15 => Vec1Ev::Auxio2,
            14 => Vec1Ev::Auxio1,
            13 => Vec1Ev::Auxio0,
            12 => Vec1Ev::AonProgWu,
            11 => Vec1Ev::AonSw,
            10 => Vec1Ev::Obsmux1,
            9 => Vec1Ev::Obsmux0,
            8 => Vec1Ev::AdcFifoAlmostFull,
            7 => Vec1Ev::AdcDone,
            6 => Vec1Ev::SmphAutotakeDone,
            5 => Vec1Ev::Timer1Ev,
            4 => Vec1Ev::Timer0Ev,
            3 => Vec1Ev::TdcDone,
            2 => Vec1Ev::AuxCompb,
            1 => Vec1Ev::AuxCompa,
            0 => Vec1Ev::AonRtcCh2,
            _ => unreachable!(),
        }
    }
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn is_adc_irq(&self) -> bool {
        *self == Vec1Ev::AdcIrq
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == Vec1Ev::McuEv
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == Vec1Ev::AclkRef
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == Vec1Ev::Auxio15
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == Vec1Ev::Auxio14
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == Vec1Ev::Auxio13
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == Vec1Ev::Auxio12
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == Vec1Ev::Auxio11
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == Vec1Ev::Auxio10
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == Vec1Ev::Auxio9
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == Vec1Ev::Auxio8
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == Vec1Ev::Auxio7
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == Vec1Ev::Auxio6
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == Vec1Ev::Auxio5
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == Vec1Ev::Auxio4
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == Vec1Ev::Auxio3
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == Vec1Ev::Auxio2
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == Vec1Ev::Auxio1
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == Vec1Ev::Auxio0
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == Vec1Ev::AonProgWu
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn is_aon_sw(&self) -> bool {
        *self == Vec1Ev::AonSw
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn is_obsmux1(&self) -> bool {
        *self == Vec1Ev::Obsmux1
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn is_obsmux0(&self) -> bool {
        *self == Vec1Ev::Obsmux0
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn is_adc_fifo_almost_full(&self) -> bool {
        *self == Vec1Ev::AdcFifoAlmostFull
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn is_adc_done(&self) -> bool {
        *self == Vec1Ev::AdcDone
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == Vec1Ev::SmphAutotakeDone
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn is_timer1_ev(&self) -> bool {
        *self == Vec1Ev::Timer1Ev
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn is_timer0_ev(&self) -> bool {
        *self == Vec1Ev::Timer0Ev
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn is_tdc_done(&self) -> bool {
        *self == Vec1Ev::TdcDone
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Vec1Ev::AuxCompb
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Vec1Ev::AuxCompa
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == Vec1Ev::AonRtcCh2
    }
}
#[doc = "Field `VEC1_EV` writer - 12:8\\]
Select vector 1 trigger source event."]
pub type Vec1EvW<'a, REG> = crate::FieldWriter<'a, REG, 5, Vec1Ev, crate::Safe>;
impl<'a, REG> Vec1EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::AdcIrq)
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::McuEv)
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::AclkRef)
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio15)
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio14)
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio13)
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio12)
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio11)
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio10)
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio9)
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio8)
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio7)
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio6)
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio5)
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio4)
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Auxio0)
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::AonProgWu)
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn aon_sw(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::AonSw)
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn obsmux1(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Obsmux1)
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Obsmux0)
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::AdcFifoAlmostFull)
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::AdcDone)
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::SmphAutotakeDone)
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Timer1Ev)
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::Timer0Ev)
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::TdcDone)
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::AuxCompb)
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::AuxCompa)
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Ev::AonRtcCh2)
    }
}
#[doc = "13:13\\]
Vector 1 trigger enable. When enabled, VEC1_EV event with VEC1_POL polarity triggers a jump to vector # 1 when AUX_SCE sleeps. Lower vectors (0) have priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vec1En {
    #[doc = "1: Enable vector 1 trigger."]
    En = 1,
    #[doc = "0: Disable vector 1 trigger."]
    Dis = 0,
}
impl From<Vec1En> for bool {
    #[inline(always)]
    fn from(variant: Vec1En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VEC1_EN` reader - 13:13\\]
Vector 1 trigger enable. When enabled, VEC1_EV event with VEC1_POL polarity triggers a jump to vector # 1 when AUX_SCE sleeps. Lower vectors (0) have priority."]
pub type Vec1EnR = crate::BitReader<Vec1En>;
impl Vec1EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vec1En {
        match self.bits {
            true => Vec1En::En,
            false => Vec1En::Dis,
        }
    }
    #[doc = "Enable vector 1 trigger."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Vec1En::En
    }
    #[doc = "Disable vector 1 trigger."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Vec1En::Dis
    }
}
#[doc = "Field `VEC1_EN` writer - 13:13\\]
Vector 1 trigger enable. When enabled, VEC1_EV event with VEC1_POL polarity triggers a jump to vector # 1 when AUX_SCE sleeps. Lower vectors (0) have priority."]
pub type Vec1EnW<'a, REG> = crate::BitWriter<'a, REG, Vec1En>;
impl<'a, REG> Vec1EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable vector 1 trigger."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1En::En)
    }
    #[doc = "Disable vector 1 trigger."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1En::Dis)
    }
}
#[doc = "14:14\\]
Vector 1 trigger event polarity. To manually trigger vector 1 execution: - AUX_SCE must sleep. - Set VEC1_EV to a known static value. - Toggle VEC1_POL twice.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vec1Pol {
    #[doc = "1: Falling edge triggers vector 1 execution."]
    Fall = 1,
    #[doc = "0: Rising edge triggers vector 1 execution."]
    Rise = 0,
}
impl From<Vec1Pol> for bool {
    #[inline(always)]
    fn from(variant: Vec1Pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VEC1_POL` reader - 14:14\\]
Vector 1 trigger event polarity. To manually trigger vector 1 execution: - AUX_SCE must sleep. - Set VEC1_EV to a known static value. - Toggle VEC1_POL twice."]
pub type Vec1PolR = crate::BitReader<Vec1Pol>;
impl Vec1PolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vec1Pol {
        match self.bits {
            true => Vec1Pol::Fall,
            false => Vec1Pol::Rise,
        }
    }
    #[doc = "Falling edge triggers vector 1 execution."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Vec1Pol::Fall
    }
    #[doc = "Rising edge triggers vector 1 execution."]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Vec1Pol::Rise
    }
}
#[doc = "Field `VEC1_POL` writer - 14:14\\]
Vector 1 trigger event polarity. To manually trigger vector 1 execution: - AUX_SCE must sleep. - Set VEC1_EV to a known static value. - Toggle VEC1_POL twice."]
pub type Vec1PolW<'a, REG> = crate::BitWriter<'a, REG, Vec1Pol>;
impl<'a, REG> Vec1PolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge triggers vector 1 execution."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Pol::Fall)
    }
    #[doc = "Rising edge triggers vector 1 execution."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Vec1Pol::Rise)
    }
}
#[doc = "Field `RESERVED15` reader - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED15` writer - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Select vector 0 trigger source event."]
    #[inline(always)]
    pub fn vec0_ev(&self) -> Vec0EvR {
        Vec0EvR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Vector 0 trigger enable. When enabled, VEC0_EV event with VEC0_POL polarity triggers a jump to vector # 0 when AUX_SCE sleeps."]
    #[inline(always)]
    pub fn vec0_en(&self) -> Vec0EnR {
        Vec0EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Vector 0 trigger event polarity. To manually trigger vector 0 execution: - AUX_SCE must sleep. - Set VEC0_EV to a known static value. - Toggle VEC0_POL twice."]
    #[inline(always)]
    pub fn vec0_pol(&self) -> Vec0PolR {
        Vec0PolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select vector 1 trigger source event."]
    #[inline(always)]
    pub fn vec1_ev(&self) -> Vec1EvR {
        Vec1EvR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Vector 1 trigger enable. When enabled, VEC1_EV event with VEC1_POL polarity triggers a jump to vector # 1 when AUX_SCE sleeps. Lower vectors (0) have priority."]
    #[inline(always)]
    pub fn vec1_en(&self) -> Vec1EnR {
        Vec1EnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Vector 1 trigger event polarity. To manually trigger vector 1 execution: - AUX_SCE must sleep. - Set VEC1_EV to a known static value. - Toggle VEC1_POL twice."]
    #[inline(always)]
    pub fn vec1_pol(&self) -> Vec1PolR {
        Vec1PolR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Select vector 0 trigger source event."]
    #[inline(always)]
    #[must_use]
    pub fn vec0_ev(&mut self) -> Vec0EvW<Veccfg0Spec> {
        Vec0EvW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Vector 0 trigger enable. When enabled, VEC0_EV event with VEC0_POL polarity triggers a jump to vector # 0 when AUX_SCE sleeps."]
    #[inline(always)]
    #[must_use]
    pub fn vec0_en(&mut self) -> Vec0EnW<Veccfg0Spec> {
        Vec0EnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Vector 0 trigger event polarity. To manually trigger vector 0 execution: - AUX_SCE must sleep. - Set VEC0_EV to a known static value. - Toggle VEC0_POL twice."]
    #[inline(always)]
    #[must_use]
    pub fn vec0_pol(&mut self) -> Vec0PolW<Veccfg0Spec> {
        Vec0PolW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<Veccfg0Spec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select vector 1 trigger source event."]
    #[inline(always)]
    #[must_use]
    pub fn vec1_ev(&mut self) -> Vec1EvW<Veccfg0Spec> {
        Vec1EvW::new(self, 8)
    }
    #[doc = "Bit 13 - 13:13\\]
Vector 1 trigger enable. When enabled, VEC1_EV event with VEC1_POL polarity triggers a jump to vector # 1 when AUX_SCE sleeps. Lower vectors (0) have priority."]
    #[inline(always)]
    #[must_use]
    pub fn vec1_en(&mut self) -> Vec1EnW<Veccfg0Spec> {
        Vec1EnW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Vector 1 trigger event polarity. To manually trigger vector 1 execution: - AUX_SCE must sleep. - Set VEC1_EV to a known static value. - Toggle VEC1_POL twice."]
    #[inline(always)]
    #[must_use]
    pub fn vec1_pol(&mut self) -> Vec1PolW<Veccfg0Spec> {
        Vec1PolW::new(self, 14)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<Veccfg0Spec> {
        Reserved15W::new(self, 15)
    }
}
#[doc = "Vector Configuration 0 AUX_SCE wakeup vector 0 and 1 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`veccfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`veccfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Veccfg0Spec;
impl crate::RegisterSpec for Veccfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`veccfg0::R`](R) reader structure"]
impl crate::Readable for Veccfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`veccfg0::W`](W) writer structure"]
impl crate::Writable for Veccfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VECCFG0 to value 0"]
impl crate::Resettable for Veccfg0Spec {
    const RESET_VALUE: u32 = 0;
}

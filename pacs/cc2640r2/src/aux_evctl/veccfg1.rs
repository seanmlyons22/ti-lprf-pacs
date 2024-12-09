#[doc = "Register `VECCFG1` reader"]
pub type R = crate::R<Veccfg1Spec>;
#[doc = "Register `VECCFG1` writer"]
pub type W = crate::W<Veccfg1Spec>;
#[doc = "4:0\\]
Select vector 2 trigger source event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vec2Ev {
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
impl From<Vec2Ev> for u8 {
    #[inline(always)]
    fn from(variant: Vec2Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vec2Ev {
    type Ux = u8;
}
impl crate::IsEnum for Vec2Ev {}
#[doc = "Field `VEC2_EV` reader - 4:0\\]
Select vector 2 trigger source event."]
pub type Vec2EvR = crate::FieldReader<Vec2Ev>;
impl Vec2EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vec2Ev {
        match self.bits {
            31 => Vec2Ev::AdcIrq,
            30 => Vec2Ev::McuEv,
            29 => Vec2Ev::AclkRef,
            28 => Vec2Ev::Auxio15,
            27 => Vec2Ev::Auxio14,
            26 => Vec2Ev::Auxio13,
            25 => Vec2Ev::Auxio12,
            24 => Vec2Ev::Auxio11,
            23 => Vec2Ev::Auxio10,
            22 => Vec2Ev::Auxio9,
            21 => Vec2Ev::Auxio8,
            20 => Vec2Ev::Auxio7,
            19 => Vec2Ev::Auxio6,
            18 => Vec2Ev::Auxio5,
            17 => Vec2Ev::Auxio4,
            16 => Vec2Ev::Auxio3,
            15 => Vec2Ev::Auxio2,
            14 => Vec2Ev::Auxio1,
            13 => Vec2Ev::Auxio0,
            12 => Vec2Ev::AonProgWu,
            11 => Vec2Ev::AonSw,
            10 => Vec2Ev::Obsmux1,
            9 => Vec2Ev::Obsmux0,
            8 => Vec2Ev::AdcFifoAlmostFull,
            7 => Vec2Ev::AdcDone,
            6 => Vec2Ev::SmphAutotakeDone,
            5 => Vec2Ev::Timer1Ev,
            4 => Vec2Ev::Timer0Ev,
            3 => Vec2Ev::TdcDone,
            2 => Vec2Ev::AuxCompb,
            1 => Vec2Ev::AuxCompa,
            0 => Vec2Ev::AonRtcCh2,
            _ => unreachable!(),
        }
    }
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn is_adc_irq(&self) -> bool {
        *self == Vec2Ev::AdcIrq
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == Vec2Ev::McuEv
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == Vec2Ev::AclkRef
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == Vec2Ev::Auxio15
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == Vec2Ev::Auxio14
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == Vec2Ev::Auxio13
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == Vec2Ev::Auxio12
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == Vec2Ev::Auxio11
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == Vec2Ev::Auxio10
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == Vec2Ev::Auxio9
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == Vec2Ev::Auxio8
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == Vec2Ev::Auxio7
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == Vec2Ev::Auxio6
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == Vec2Ev::Auxio5
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == Vec2Ev::Auxio4
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == Vec2Ev::Auxio3
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == Vec2Ev::Auxio2
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == Vec2Ev::Auxio1
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == Vec2Ev::Auxio0
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == Vec2Ev::AonProgWu
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn is_aon_sw(&self) -> bool {
        *self == Vec2Ev::AonSw
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn is_obsmux1(&self) -> bool {
        *self == Vec2Ev::Obsmux1
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn is_obsmux0(&self) -> bool {
        *self == Vec2Ev::Obsmux0
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn is_adc_fifo_almost_full(&self) -> bool {
        *self == Vec2Ev::AdcFifoAlmostFull
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn is_adc_done(&self) -> bool {
        *self == Vec2Ev::AdcDone
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == Vec2Ev::SmphAutotakeDone
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn is_timer1_ev(&self) -> bool {
        *self == Vec2Ev::Timer1Ev
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn is_timer0_ev(&self) -> bool {
        *self == Vec2Ev::Timer0Ev
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn is_tdc_done(&self) -> bool {
        *self == Vec2Ev::TdcDone
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Vec2Ev::AuxCompb
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Vec2Ev::AuxCompa
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == Vec2Ev::AonRtcCh2
    }
}
#[doc = "Field `VEC2_EV` writer - 4:0\\]
Select vector 2 trigger source event."]
pub type Vec2EvW<'a, REG> = crate::FieldWriter<'a, REG, 5, Vec2Ev, crate::Safe>;
impl<'a, REG> Vec2EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::AdcIrq)
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::McuEv)
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::AclkRef)
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio15)
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio14)
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio13)
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio12)
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio11)
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio10)
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio9)
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio8)
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio7)
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio6)
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio5)
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio4)
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Auxio0)
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::AonProgWu)
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn aon_sw(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::AonSw)
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn obsmux1(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Obsmux1)
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Obsmux0)
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::AdcFifoAlmostFull)
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::AdcDone)
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::SmphAutotakeDone)
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Timer1Ev)
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::Timer0Ev)
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::TdcDone)
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::AuxCompb)
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::AuxCompa)
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Ev::AonRtcCh2)
    }
}
#[doc = "5:5\\]
Vector 2 trigger enable. When enabled, VEC2_EV event with VEC2_POL polarity triggers a jump to vector # 2 when AUX_SCE sleeps. Lower vectors (0 and 1) have priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vec2En {
    #[doc = "1: Enable vector 2 trigger."]
    En = 1,
    #[doc = "0: Disable vector 2 trigger."]
    Dis = 0,
}
impl From<Vec2En> for bool {
    #[inline(always)]
    fn from(variant: Vec2En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VEC2_EN` reader - 5:5\\]
Vector 2 trigger enable. When enabled, VEC2_EV event with VEC2_POL polarity triggers a jump to vector # 2 when AUX_SCE sleeps. Lower vectors (0 and 1) have priority."]
pub type Vec2EnR = crate::BitReader<Vec2En>;
impl Vec2EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vec2En {
        match self.bits {
            true => Vec2En::En,
            false => Vec2En::Dis,
        }
    }
    #[doc = "Enable vector 2 trigger."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Vec2En::En
    }
    #[doc = "Disable vector 2 trigger."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Vec2En::Dis
    }
}
#[doc = "Field `VEC2_EN` writer - 5:5\\]
Vector 2 trigger enable. When enabled, VEC2_EV event with VEC2_POL polarity triggers a jump to vector # 2 when AUX_SCE sleeps. Lower vectors (0 and 1) have priority."]
pub type Vec2EnW<'a, REG> = crate::BitWriter<'a, REG, Vec2En>;
impl<'a, REG> Vec2EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable vector 2 trigger."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2En::En)
    }
    #[doc = "Disable vector 2 trigger."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2En::Dis)
    }
}
#[doc = "6:6\\]
Vector 2 trigger event polarity. To manually trigger vector 2 execution: - AUX_SCE must sleep. - Set VEC2_EV to a known static value. - Toggle VEC2_POL twice.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vec2Pol {
    #[doc = "1: Falling edge triggers vector 2 execution."]
    Fall = 1,
    #[doc = "0: Rising edge triggers vector 2 execution."]
    Rise = 0,
}
impl From<Vec2Pol> for bool {
    #[inline(always)]
    fn from(variant: Vec2Pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VEC2_POL` reader - 6:6\\]
Vector 2 trigger event polarity. To manually trigger vector 2 execution: - AUX_SCE must sleep. - Set VEC2_EV to a known static value. - Toggle VEC2_POL twice."]
pub type Vec2PolR = crate::BitReader<Vec2Pol>;
impl Vec2PolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vec2Pol {
        match self.bits {
            true => Vec2Pol::Fall,
            false => Vec2Pol::Rise,
        }
    }
    #[doc = "Falling edge triggers vector 2 execution."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Vec2Pol::Fall
    }
    #[doc = "Rising edge triggers vector 2 execution."]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Vec2Pol::Rise
    }
}
#[doc = "Field `VEC2_POL` writer - 6:6\\]
Vector 2 trigger event polarity. To manually trigger vector 2 execution: - AUX_SCE must sleep. - Set VEC2_EV to a known static value. - Toggle VEC2_POL twice."]
pub type Vec2PolW<'a, REG> = crate::BitWriter<'a, REG, Vec2Pol>;
impl<'a, REG> Vec2PolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge triggers vector 2 execution."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Pol::Fall)
    }
    #[doc = "Rising edge triggers vector 2 execution."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Vec2Pol::Rise)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "12:8\\]
Select vector 3 trigger source event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vec3Ev {
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
impl From<Vec3Ev> for u8 {
    #[inline(always)]
    fn from(variant: Vec3Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vec3Ev {
    type Ux = u8;
}
impl crate::IsEnum for Vec3Ev {}
#[doc = "Field `VEC3_EV` reader - 12:8\\]
Select vector 3 trigger source event."]
pub type Vec3EvR = crate::FieldReader<Vec3Ev>;
impl Vec3EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vec3Ev {
        match self.bits {
            31 => Vec3Ev::AdcIrq,
            30 => Vec3Ev::McuEv,
            29 => Vec3Ev::AclkRef,
            28 => Vec3Ev::Auxio15,
            27 => Vec3Ev::Auxio14,
            26 => Vec3Ev::Auxio13,
            25 => Vec3Ev::Auxio12,
            24 => Vec3Ev::Auxio11,
            23 => Vec3Ev::Auxio10,
            22 => Vec3Ev::Auxio9,
            21 => Vec3Ev::Auxio8,
            20 => Vec3Ev::Auxio7,
            19 => Vec3Ev::Auxio6,
            18 => Vec3Ev::Auxio5,
            17 => Vec3Ev::Auxio4,
            16 => Vec3Ev::Auxio3,
            15 => Vec3Ev::Auxio2,
            14 => Vec3Ev::Auxio1,
            13 => Vec3Ev::Auxio0,
            12 => Vec3Ev::AonProgWu,
            11 => Vec3Ev::AonSw,
            10 => Vec3Ev::Obsmux1,
            9 => Vec3Ev::Obsmux0,
            8 => Vec3Ev::AdcFifoAlmostFull,
            7 => Vec3Ev::AdcDone,
            6 => Vec3Ev::SmphAutotakeDone,
            5 => Vec3Ev::Timer1Ev,
            4 => Vec3Ev::Timer0Ev,
            3 => Vec3Ev::TdcDone,
            2 => Vec3Ev::AuxCompb,
            1 => Vec3Ev::AuxCompa,
            0 => Vec3Ev::AonRtcCh2,
            _ => unreachable!(),
        }
    }
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn is_adc_irq(&self) -> bool {
        *self == Vec3Ev::AdcIrq
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == Vec3Ev::McuEv
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == Vec3Ev::AclkRef
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == Vec3Ev::Auxio15
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == Vec3Ev::Auxio14
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == Vec3Ev::Auxio13
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == Vec3Ev::Auxio12
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == Vec3Ev::Auxio11
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == Vec3Ev::Auxio10
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == Vec3Ev::Auxio9
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == Vec3Ev::Auxio8
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == Vec3Ev::Auxio7
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == Vec3Ev::Auxio6
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == Vec3Ev::Auxio5
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == Vec3Ev::Auxio4
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == Vec3Ev::Auxio3
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == Vec3Ev::Auxio2
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == Vec3Ev::Auxio1
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == Vec3Ev::Auxio0
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == Vec3Ev::AonProgWu
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn is_aon_sw(&self) -> bool {
        *self == Vec3Ev::AonSw
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn is_obsmux1(&self) -> bool {
        *self == Vec3Ev::Obsmux1
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn is_obsmux0(&self) -> bool {
        *self == Vec3Ev::Obsmux0
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn is_adc_fifo_almost_full(&self) -> bool {
        *self == Vec3Ev::AdcFifoAlmostFull
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn is_adc_done(&self) -> bool {
        *self == Vec3Ev::AdcDone
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == Vec3Ev::SmphAutotakeDone
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn is_timer1_ev(&self) -> bool {
        *self == Vec3Ev::Timer1Ev
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn is_timer0_ev(&self) -> bool {
        *self == Vec3Ev::Timer0Ev
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn is_tdc_done(&self) -> bool {
        *self == Vec3Ev::TdcDone
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Vec3Ev::AuxCompb
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Vec3Ev::AuxCompa
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == Vec3Ev::AonRtcCh2
    }
}
#[doc = "Field `VEC3_EV` writer - 12:8\\]
Select vector 3 trigger source event."]
pub type Vec3EvW<'a, REG> = crate::FieldWriter<'a, REG, 5, Vec3Ev, crate::Safe>;
impl<'a, REG> Vec3EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::AdcIrq)
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::McuEv)
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::AclkRef)
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio15)
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio14)
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio13)
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio12)
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio11)
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio10)
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio9)
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio8)
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio7)
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio6)
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio5)
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio4)
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Auxio0)
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::AonProgWu)
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn aon_sw(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::AonSw)
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn obsmux1(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Obsmux1)
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Obsmux0)
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::AdcFifoAlmostFull)
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::AdcDone)
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::SmphAutotakeDone)
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Timer1Ev)
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::Timer0Ev)
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::TdcDone)
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::AuxCompb)
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::AuxCompa)
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Ev::AonRtcCh2)
    }
}
#[doc = "13:13\\]
Vector 3 trigger enable. When enabled, VEC3_EV event with VEC3_POL polarity triggers a jump to vector # 3 when AUX_SCE sleeps. Lower vectors (0, 1, and 2) have priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vec3En {
    #[doc = "1: Enable vector 3 trigger."]
    En = 1,
    #[doc = "0: Disable vector 3 trigger."]
    Dis = 0,
}
impl From<Vec3En> for bool {
    #[inline(always)]
    fn from(variant: Vec3En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VEC3_EN` reader - 13:13\\]
Vector 3 trigger enable. When enabled, VEC3_EV event with VEC3_POL polarity triggers a jump to vector # 3 when AUX_SCE sleeps. Lower vectors (0, 1, and 2) have priority."]
pub type Vec3EnR = crate::BitReader<Vec3En>;
impl Vec3EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vec3En {
        match self.bits {
            true => Vec3En::En,
            false => Vec3En::Dis,
        }
    }
    #[doc = "Enable vector 3 trigger."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Vec3En::En
    }
    #[doc = "Disable vector 3 trigger."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Vec3En::Dis
    }
}
#[doc = "Field `VEC3_EN` writer - 13:13\\]
Vector 3 trigger enable. When enabled, VEC3_EV event with VEC3_POL polarity triggers a jump to vector # 3 when AUX_SCE sleeps. Lower vectors (0, 1, and 2) have priority."]
pub type Vec3EnW<'a, REG> = crate::BitWriter<'a, REG, Vec3En>;
impl<'a, REG> Vec3EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable vector 3 trigger."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3En::En)
    }
    #[doc = "Disable vector 3 trigger."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3En::Dis)
    }
}
#[doc = "14:14\\]
Vector 3 trigger event polarity. To manually trigger vector 3 execution: - AUX_SCE must sleep. - Set VEC3_EV to a known static value. - Toggle VEC3_POL twice.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vec3Pol {
    #[doc = "1: Falling edge triggers vector 3 execution."]
    Fall = 1,
    #[doc = "0: Rising edge triggers vector 3 execution."]
    Rise = 0,
}
impl From<Vec3Pol> for bool {
    #[inline(always)]
    fn from(variant: Vec3Pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VEC3_POL` reader - 14:14\\]
Vector 3 trigger event polarity. To manually trigger vector 3 execution: - AUX_SCE must sleep. - Set VEC3_EV to a known static value. - Toggle VEC3_POL twice."]
pub type Vec3PolR = crate::BitReader<Vec3Pol>;
impl Vec3PolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vec3Pol {
        match self.bits {
            true => Vec3Pol::Fall,
            false => Vec3Pol::Rise,
        }
    }
    #[doc = "Falling edge triggers vector 3 execution."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Vec3Pol::Fall
    }
    #[doc = "Rising edge triggers vector 3 execution."]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Vec3Pol::Rise
    }
}
#[doc = "Field `VEC3_POL` writer - 14:14\\]
Vector 3 trigger event polarity. To manually trigger vector 3 execution: - AUX_SCE must sleep. - Set VEC3_EV to a known static value. - Toggle VEC3_POL twice."]
pub type Vec3PolW<'a, REG> = crate::BitWriter<'a, REG, Vec3Pol>;
impl<'a, REG> Vec3PolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge triggers vector 3 execution."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Pol::Fall)
    }
    #[doc = "Rising edge triggers vector 3 execution."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Vec3Pol::Rise)
    }
}
#[doc = "Field `RESERVED15` reader - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Select vector 2 trigger source event."]
    #[inline(always)]
    pub fn vec2_ev(&self) -> Vec2EvR {
        Vec2EvR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Vector 2 trigger enable. When enabled, VEC2_EV event with VEC2_POL polarity triggers a jump to vector # 2 when AUX_SCE sleeps. Lower vectors (0 and 1) have priority."]
    #[inline(always)]
    pub fn vec2_en(&self) -> Vec2EnR {
        Vec2EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Vector 2 trigger event polarity. To manually trigger vector 2 execution: - AUX_SCE must sleep. - Set VEC2_EV to a known static value. - Toggle VEC2_POL twice."]
    #[inline(always)]
    pub fn vec2_pol(&self) -> Vec2PolR {
        Vec2PolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select vector 3 trigger source event."]
    #[inline(always)]
    pub fn vec3_ev(&self) -> Vec3EvR {
        Vec3EvR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Vector 3 trigger enable. When enabled, VEC3_EV event with VEC3_POL polarity triggers a jump to vector # 3 when AUX_SCE sleeps. Lower vectors (0, 1, and 2) have priority."]
    #[inline(always)]
    pub fn vec3_en(&self) -> Vec3EnR {
        Vec3EnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Vector 3 trigger event polarity. To manually trigger vector 3 execution: - AUX_SCE must sleep. - Set VEC3_EV to a known static value. - Toggle VEC3_POL twice."]
    #[inline(always)]
    pub fn vec3_pol(&self) -> Vec3PolR {
        Vec3PolR::new(((self.bits >> 14) & 1) != 0)
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
Select vector 2 trigger source event."]
    #[inline(always)]
    #[must_use]
    pub fn vec2_ev(&mut self) -> Vec2EvW<Veccfg1Spec> {
        Vec2EvW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Vector 2 trigger enable. When enabled, VEC2_EV event with VEC2_POL polarity triggers a jump to vector # 2 when AUX_SCE sleeps. Lower vectors (0 and 1) have priority."]
    #[inline(always)]
    #[must_use]
    pub fn vec2_en(&mut self) -> Vec2EnW<Veccfg1Spec> {
        Vec2EnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Vector 2 trigger event polarity. To manually trigger vector 2 execution: - AUX_SCE must sleep. - Set VEC2_EV to a known static value. - Toggle VEC2_POL twice."]
    #[inline(always)]
    #[must_use]
    pub fn vec2_pol(&mut self) -> Vec2PolW<Veccfg1Spec> {
        Vec2PolW::new(self, 6)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select vector 3 trigger source event."]
    #[inline(always)]
    #[must_use]
    pub fn vec3_ev(&mut self) -> Vec3EvW<Veccfg1Spec> {
        Vec3EvW::new(self, 8)
    }
    #[doc = "Bit 13 - 13:13\\]
Vector 3 trigger enable. When enabled, VEC3_EV event with VEC3_POL polarity triggers a jump to vector # 3 when AUX_SCE sleeps. Lower vectors (0, 1, and 2) have priority."]
    #[inline(always)]
    #[must_use]
    pub fn vec3_en(&mut self) -> Vec3EnW<Veccfg1Spec> {
        Vec3EnW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Vector 3 trigger event polarity. To manually trigger vector 3 execution: - AUX_SCE must sleep. - Set VEC3_EV to a known static value. - Toggle VEC3_POL twice."]
    #[inline(always)]
    #[must_use]
    pub fn vec3_pol(&mut self) -> Vec3PolW<Veccfg1Spec> {
        Vec3PolW::new(self, 14)
    }
}
#[doc = "Vector Configuration 1 AUX_SCE event vectors 2 and 3 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`veccfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`veccfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Veccfg1Spec;
impl crate::RegisterSpec for Veccfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`veccfg1::R`](R) reader structure"]
impl crate::Readable for Veccfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`veccfg1::W`](W) writer structure"]
impl crate::Writable for Veccfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VECCFG1 to value 0"]
impl crate::Resettable for Veccfg1Spec {
    const RESET_VALUE: u32 = 0;
}

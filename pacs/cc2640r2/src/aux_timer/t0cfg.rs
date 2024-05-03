#[doc = "Register `T0CFG` reader"]
pub type R = crate::R<T0cfgSpec>;
#[doc = "Register `T0CFG` writer"]
pub type W = crate::W<T0cfgSpec>;
#[doc = "0:0\\]
Timer 0 reload mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reload {
    #[doc = "1: Continuous mode. Timer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    Cont = 1,
    #[doc = "0: Manual mode. Timer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE."]
    Man = 0,
}
impl From<Reload> for bool {
    #[inline(always)]
    fn from(variant: Reload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RELOAD` reader - 0:0\\]
Timer 0 reload mode."]
pub type ReloadR = crate::BitReader<Reload>;
impl ReloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reload {
        match self.bits {
            true => Reload::Cont,
            false => Reload::Man,
        }
    }
    #[doc = "Continuous mode. Timer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == Reload::Cont
    }
    #[doc = "Manual mode. Timer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE."]
    #[inline(always)]
    pub fn is_man(&self) -> bool {
        *self == Reload::Man
    }
}
#[doc = "Field `RELOAD` writer - 0:0\\]
Timer 0 reload mode."]
pub type ReloadW<'a, REG> = crate::BitWriter<'a, REG, Reload>;
impl<'a, REG> ReloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous mode. Timer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    #[inline(always)]
    pub fn cont(self) -> &'a mut crate::W<REG> {
        self.variant(Reload::Cont)
    }
    #[doc = "Manual mode. Timer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE."]
    #[inline(always)]
    pub fn man(self) -> &'a mut crate::W<REG> {
        self.variant(Reload::Man)
    }
}
#[doc = "1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "1: Use event set by TICK_SRC as source for prescaler."]
    Tick = 1,
    #[doc = "0: Use AUX clock as source for prescaler."]
    Clk = 0,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - 1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler."]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            true => Mode::Tick,
            false => Mode::Clk,
        }
    }
    #[doc = "Use event set by TICK_SRC as source for prescaler."]
    #[inline(always)]
    pub fn is_tick(&self) -> bool {
        *self == Mode::Tick
    }
    #[doc = "Use AUX clock as source for prescaler."]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == Mode::Clk
    }
}
#[doc = "Field `MODE` writer - 1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use event set by TICK_SRC as source for prescaler."]
    #[inline(always)]
    pub fn tick(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Tick)
    }
    #[doc = "Use AUX clock as source for prescaler."]
    #[inline(always)]
    pub fn clk(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Clk)
    }
}
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRE` reader - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
pub type PreR = crate::FieldReader;
#[doc = "Field `PRE` writer - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
pub type PreW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "12:8\\]
Select Timer 0 tick source from the synchronous event bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TickSrc {
    #[doc = "31: AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    AdcIrq = 31,
    #[doc = "30: AUX_EVCTL:EVSTAT1.MCU_EV"]
    McuEvent = 30,
    #[doc = "29: AUX_EVCTL:EVSTAT1.ACLK_REF"]
    AclkRef = 29,
    #[doc = "28: AUX_EVCTL:EVSTAT1.AUXIO15"]
    Auxio15 = 28,
    #[doc = "27: AUX_EVCTL:EVSTAT1.AUXIO14"]
    Auxio14 = 27,
    #[doc = "26: AUX_EVCTL:EVSTAT1.AUXIO13"]
    Auxio13 = 26,
    #[doc = "25: AUX_EVCTL:EVSTAT1.AUXIO12"]
    Auxio12 = 25,
    #[doc = "24: AUX_EVCTL:EVSTAT1.AUXIO11"]
    Auxio11 = 24,
    #[doc = "23: AUX_EVCTL:EVSTAT1.AUXIO10"]
    Auxio10 = 23,
    #[doc = "22: AUX_EVCTL:EVSTAT1.AUXIO9"]
    Auxio9 = 22,
    #[doc = "21: AUX_EVCTL:EVSTAT1.AUXIO8"]
    Auxio8 = 21,
    #[doc = "20: AUX_EVCTL:EVSTAT1.AUXIO7"]
    Auxio7 = 20,
    #[doc = "19: AUX_EVCTL:EVSTAT1.AUXIO6"]
    Auxio6 = 19,
    #[doc = "18: AUX_EVCTL:EVSTAT1.AUXIO5"]
    Auxio5 = 18,
    #[doc = "17: AUX_EVCTL:EVSTAT1.AUXIO4"]
    Auxio4 = 17,
    #[doc = "16: AUX_EVCTL:EVSTAT1.AUXIO3"]
    Auxio3 = 16,
    #[doc = "15: AUX_EVCTL:EVSTAT0.AUXIO2"]
    Auxio2 = 15,
    #[doc = "14: AUX_EVCTL:EVSTAT0.AUXIO1"]
    Auxio1 = 14,
    #[doc = "13: AUX_EVCTL:EVSTAT0.AUXIO0"]
    Auxio0 = 13,
    #[doc = "12: AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    AonProgWu = 12,
    #[doc = "11: AUX_EVCTL:EVSTAT0.AON_SW"]
    AonSw = 11,
    #[doc = "10: AUX_EVCTL:EVSTAT0.OBSMUX1"]
    Obsmux1 = 10,
    #[doc = "9: AUX_EVCTL:EVSTAT0.OBSMUX0"]
    Obsmux0 = 9,
    #[doc = "8: AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    Rtc4khz = 8,
    #[doc = "7: AUX_EVCTL:EVSTAT0.ADC_DONE"]
    AdcDone = 7,
    #[doc = "6: AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SmphAutotakeDone = 6,
    #[doc = "5: AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    Timer1Ev = 5,
    #[doc = "3: AUX_EVCTL:EVSTAT0.TDC_DONE"]
    TdcDone = 3,
    #[doc = "2: AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    AuxCompb = 2,
    #[doc = "1: AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    AuxCompa = 1,
    #[doc = "0: AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    RtcCh2Ev = 0,
}
impl From<TickSrc> for u8 {
    #[inline(always)]
    fn from(variant: TickSrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TickSrc {
    type Ux = u8;
}
impl crate::IsEnum for TickSrc {}
#[doc = "Field `TICK_SRC` reader - 12:8\\]
Select Timer 0 tick source from the synchronous event bus."]
pub type TickSrcR = crate::FieldReader<TickSrc>;
impl TickSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TickSrc> {
        match self.bits {
            31 => Some(TickSrc::AdcIrq),
            30 => Some(TickSrc::McuEvent),
            29 => Some(TickSrc::AclkRef),
            28 => Some(TickSrc::Auxio15),
            27 => Some(TickSrc::Auxio14),
            26 => Some(TickSrc::Auxio13),
            25 => Some(TickSrc::Auxio12),
            24 => Some(TickSrc::Auxio11),
            23 => Some(TickSrc::Auxio10),
            22 => Some(TickSrc::Auxio9),
            21 => Some(TickSrc::Auxio8),
            20 => Some(TickSrc::Auxio7),
            19 => Some(TickSrc::Auxio6),
            18 => Some(TickSrc::Auxio5),
            17 => Some(TickSrc::Auxio4),
            16 => Some(TickSrc::Auxio3),
            15 => Some(TickSrc::Auxio2),
            14 => Some(TickSrc::Auxio1),
            13 => Some(TickSrc::Auxio0),
            12 => Some(TickSrc::AonProgWu),
            11 => Some(TickSrc::AonSw),
            10 => Some(TickSrc::Obsmux1),
            9 => Some(TickSrc::Obsmux0),
            8 => Some(TickSrc::Rtc4khz),
            7 => Some(TickSrc::AdcDone),
            6 => Some(TickSrc::SmphAutotakeDone),
            5 => Some(TickSrc::Timer1Ev),
            3 => Some(TickSrc::TdcDone),
            2 => Some(TickSrc::AuxCompb),
            1 => Some(TickSrc::AuxCompa),
            0 => Some(TickSrc::RtcCh2Ev),
            _ => None,
        }
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn is_adc_irq(&self) -> bool {
        *self == TickSrc::AdcIrq
    }
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_event(&self) -> bool {
        *self == TickSrc::McuEvent
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == TickSrc::AclkRef
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == TickSrc::Auxio15
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == TickSrc::Auxio14
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == TickSrc::Auxio13
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == TickSrc::Auxio12
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == TickSrc::Auxio11
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == TickSrc::Auxio10
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == TickSrc::Auxio9
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == TickSrc::Auxio8
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == TickSrc::Auxio7
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == TickSrc::Auxio6
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == TickSrc::Auxio5
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == TickSrc::Auxio4
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == TickSrc::Auxio3
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == TickSrc::Auxio2
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == TickSrc::Auxio1
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == TickSrc::Auxio0
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == TickSrc::AonProgWu
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn is_aon_sw(&self) -> bool {
        *self == TickSrc::AonSw
    }
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn is_obsmux1(&self) -> bool {
        *self == TickSrc::Obsmux1
    }
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn is_obsmux0(&self) -> bool {
        *self == TickSrc::Obsmux0
    }
    #[doc = "AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    #[inline(always)]
    pub fn is_rtc_4khz(&self) -> bool {
        *self == TickSrc::Rtc4khz
    }
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn is_adc_done(&self) -> bool {
        *self == TickSrc::AdcDone
    }
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == TickSrc::SmphAutotakeDone
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn is_timer1_ev(&self) -> bool {
        *self == TickSrc::Timer1Ev
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn is_tdc_done(&self) -> bool {
        *self == TickSrc::TdcDone
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == TickSrc::AuxCompb
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == TickSrc::AuxCompa
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_rtc_ch2_ev(&self) -> bool {
        *self == TickSrc::RtcCh2Ev
    }
}
#[doc = "Field `TICK_SRC` writer - 12:8\\]
Select Timer 0 tick source from the synchronous event bus."]
pub type TickSrcW<'a, REG> = crate::FieldWriter<'a, REG, 5, TickSrc>;
impl<'a, REG> TickSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AdcIrq)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn mcu_event(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::McuEvent)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AclkRef)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio15)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio14)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio13)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio12)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio11)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio10)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio9)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio8)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio7)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio6)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio5)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio4)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio0)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AonProgWu)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn aon_sw(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AonSw)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn obsmux1(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Obsmux1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Obsmux0)
    }
    #[doc = "AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    #[inline(always)]
    pub fn rtc_4khz(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Rtc4khz)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AdcDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::SmphAutotakeDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Timer1Ev)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::TdcDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AuxCompb)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AuxCompa)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn rtc_ch2_ev(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::RtcCh2Ev)
    }
}
#[doc = "13:13\\]
Tick source polarity for Timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TickSrcPol {
    #[doc = "1: Count on falling edges of TICK_SRC."]
    Fall = 1,
    #[doc = "0: Count on rising edges of TICK_SRC."]
    Rise = 0,
}
impl From<TickSrcPol> for bool {
    #[inline(always)]
    fn from(variant: TickSrcPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICK_SRC_POL` reader - 13:13\\]
Tick source polarity for Timer 0."]
pub type TickSrcPolR = crate::BitReader<TickSrcPol>;
impl TickSrcPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TickSrcPol {
        match self.bits {
            true => TickSrcPol::Fall,
            false => TickSrcPol::Rise,
        }
    }
    #[doc = "Count on falling edges of TICK_SRC."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == TickSrcPol::Fall
    }
    #[doc = "Count on rising edges of TICK_SRC."]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == TickSrcPol::Rise
    }
}
#[doc = "Field `TICK_SRC_POL` writer - 13:13\\]
Tick source polarity for Timer 0."]
pub type TickSrcPolW<'a, REG> = crate::BitWriter<'a, REG, TickSrcPol>;
impl<'a, REG> TickSrcPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Count on falling edges of TICK_SRC."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrcPol::Fall)
    }
    #[doc = "Count on rising edges of TICK_SRC."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrcPol::Rise)
    }
}
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Timer 0 reload mode."]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline(always)]
    pub fn pre(&self) -> PreR {
        PreR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select Timer 0 tick source from the synchronous event bus."]
    #[inline(always)]
    pub fn tick_src(&self) -> TickSrcR {
        TickSrcR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Tick source polarity for Timer 0."]
    #[inline(always)]
    pub fn tick_src_pol(&self) -> TickSrcPolR {
        TickSrcPolR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Timer 0 reload mode."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> ReloadW<T0cfgSpec> {
        ReloadW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<T0cfgSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<T0cfgSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PreW<T0cfgSpec> {
        PreW::new(self, 4)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select Timer 0 tick source from the synchronous event bus."]
    #[inline(always)]
    #[must_use]
    pub fn tick_src(&mut self) -> TickSrcW<T0cfgSpec> {
        TickSrcW::new(self, 8)
    }
    #[doc = "Bit 13 - 13:13\\]
Tick source polarity for Timer 0."]
    #[inline(always)]
    #[must_use]
    pub fn tick_src_pol(&mut self) -> TickSrcPolW<T0cfgSpec> {
        TickSrcPolW::new(self, 13)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<T0cfgSpec> {
        Reserved14W::new(self, 14)
    }
}
#[doc = "Timer 0 Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0cfgSpec;
impl crate::RegisterSpec for T0cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0cfg::R`](R) reader structure"]
impl crate::Readable for T0cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`t0cfg::W`](W) writer structure"]
impl crate::Writable for T0cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T0CFG to value 0"]
impl crate::Resettable for T0cfgSpec {
    const RESET_VALUE: u32 = 0;
}

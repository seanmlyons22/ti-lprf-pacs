#[doc = "Register `ADCCTL` reader"]
pub type R = crate::R<AdcctlSpec>;
#[doc = "Register `ADCCTL` writer"]
pub type W = crate::W<AdcctlSpec>;
#[doc = "1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd {
    #[doc = "3: Flush ADC FIFO. You must set CMD to EN or DIS after flush. System CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    Flush = 3,
    #[doc = "1: Enable ADC interface."]
    En = 1,
    #[doc = "0: Disable ADC interface."]
    Dis = 0,
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(variant: Cmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd {
    type Ux = u8;
}
impl crate::IsEnum for Cmd {}
#[doc = "Field `CMD` reader - 1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
pub type CmdR = crate::FieldReader<Cmd>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmd> {
        match self.bits {
            3 => Some(Cmd::Flush),
            1 => Some(Cmd::En),
            0 => Some(Cmd::Dis),
            _ => None,
        }
    }
    #[doc = "Flush ADC FIFO. You must set CMD to EN or DIS after flush. System CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == Cmd::Flush
    }
    #[doc = "Enable ADC interface."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Cmd::En
    }
    #[doc = "Disable ADC interface."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Cmd::Dis
    }
}
#[doc = "Field `CMD` writer - 1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flush ADC FIFO. You must set CMD to EN or DIS after flush. System CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Flush)
    }
    #[doc = "Enable ADC interface."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::En)
    }
    #[doc = "Disable ADC interface."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Dis)
    }
}
#[doc = "Field `RESERVED2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "12:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT&lt;n> if you want to trigger the ADC manually through ADCTRIG.START.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StartSrc {
    #[doc = "31: AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    AdcIrq = 31,
    #[doc = "30: AUX_EVCTL:EVSTAT1.MCU_EV"]
    McuEv = 30,
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
    #[doc = "10: No event."]
    NoEvent1 = 10,
    #[doc = "9: No event."]
    NoEvent0 = 9,
    #[doc = "8: Reserved - Do not use."]
    Reserved1 = 8,
    #[doc = "7: Reserved - Do not use."]
    Reserved0 = 7,
    #[doc = "6: AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SmphAutotakeDone = 6,
    #[doc = "5: AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    Timer1Ev = 5,
    #[doc = "4: AUX_EVCTL:EVSTAT0.TIMER0_EV"]
    Timer0Ev = 4,
    #[doc = "3: AUX_EVCTL:EVSTAT0.TDC_DONE"]
    TdcDone = 3,
    #[doc = "2: AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    AuxCompb = 2,
    #[doc = "1: AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    AuxCompa = 1,
    #[doc = "0: AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    RtcCh2Ev = 0,
}
impl From<StartSrc> for u8 {
    #[inline(always)]
    fn from(variant: StartSrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for StartSrc {
    type Ux = u8;
}
impl crate::IsEnum for StartSrc {}
#[doc = "Field `START_SRC` reader - 12:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT&lt;n> if you want to trigger the ADC manually through ADCTRIG.START."]
pub type StartSrcR = crate::FieldReader<StartSrc>;
impl StartSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StartSrc {
        match self.bits {
            31 => StartSrc::AdcIrq,
            30 => StartSrc::McuEv,
            29 => StartSrc::AclkRef,
            28 => StartSrc::Auxio15,
            27 => StartSrc::Auxio14,
            26 => StartSrc::Auxio13,
            25 => StartSrc::Auxio12,
            24 => StartSrc::Auxio11,
            23 => StartSrc::Auxio10,
            22 => StartSrc::Auxio9,
            21 => StartSrc::Auxio8,
            20 => StartSrc::Auxio7,
            19 => StartSrc::Auxio6,
            18 => StartSrc::Auxio5,
            17 => StartSrc::Auxio4,
            16 => StartSrc::Auxio3,
            15 => StartSrc::Auxio2,
            14 => StartSrc::Auxio1,
            13 => StartSrc::Auxio0,
            12 => StartSrc::AonProgWu,
            11 => StartSrc::AonSw,
            10 => StartSrc::NoEvent1,
            9 => StartSrc::NoEvent0,
            8 => StartSrc::Reserved1,
            7 => StartSrc::Reserved0,
            6 => StartSrc::SmphAutotakeDone,
            5 => StartSrc::Timer1Ev,
            4 => StartSrc::Timer0Ev,
            3 => StartSrc::TdcDone,
            2 => StartSrc::AuxCompb,
            1 => StartSrc::AuxCompa,
            0 => StartSrc::RtcCh2Ev,
            _ => unreachable!(),
        }
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn is_adc_irq(&self) -> bool {
        *self == StartSrc::AdcIrq
    }
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == StartSrc::McuEv
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == StartSrc::AclkRef
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == StartSrc::Auxio15
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == StartSrc::Auxio14
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == StartSrc::Auxio13
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == StartSrc::Auxio12
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == StartSrc::Auxio11
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == StartSrc::Auxio10
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == StartSrc::Auxio9
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == StartSrc::Auxio8
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == StartSrc::Auxio7
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == StartSrc::Auxio6
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == StartSrc::Auxio5
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == StartSrc::Auxio4
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == StartSrc::Auxio3
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == StartSrc::Auxio2
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == StartSrc::Auxio1
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == StartSrc::Auxio0
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == StartSrc::AonProgWu
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn is_aon_sw(&self) -> bool {
        *self == StartSrc::AonSw
    }
    #[doc = "No event."]
    #[inline(always)]
    pub fn is_no_event1(&self) -> bool {
        *self == StartSrc::NoEvent1
    }
    #[doc = "No event."]
    #[inline(always)]
    pub fn is_no_event0(&self) -> bool {
        *self == StartSrc::NoEvent0
    }
    #[doc = "Reserved - Do not use."]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == StartSrc::Reserved1
    }
    #[doc = "Reserved - Do not use."]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == StartSrc::Reserved0
    }
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == StartSrc::SmphAutotakeDone
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn is_timer1_ev(&self) -> bool {
        *self == StartSrc::Timer1Ev
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn is_timer0_ev(&self) -> bool {
        *self == StartSrc::Timer0Ev
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn is_tdc_done(&self) -> bool {
        *self == StartSrc::TdcDone
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == StartSrc::AuxCompb
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == StartSrc::AuxCompa
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_rtc_ch2_ev(&self) -> bool {
        *self == StartSrc::RtcCh2Ev
    }
}
#[doc = "Field `START_SRC` writer - 12:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT&lt;n> if you want to trigger the ADC manually through ADCTRIG.START."]
pub type StartSrcW<'a, REG> = crate::FieldWriter<'a, REG, 5, StartSrc, crate::Safe>;
impl<'a, REG> StartSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AdcIrq)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::McuEv)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AclkRef)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio15)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio14)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio13)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio12)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio11)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio10)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio9)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio8)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio7)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio6)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio5)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio4)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Auxio0)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AonProgWu)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn aon_sw(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AonSw)
    }
    #[doc = "No event."]
    #[inline(always)]
    pub fn no_event1(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::NoEvent1)
    }
    #[doc = "No event."]
    #[inline(always)]
    pub fn no_event0(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::NoEvent0)
    }
    #[doc = "Reserved - Do not use."]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Reserved1)
    }
    #[doc = "Reserved - Do not use."]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Reserved0)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::SmphAutotakeDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Timer1Ev)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::Timer0Ev)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::TdcDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxCompb)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::AuxCompa)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn rtc_ch2_ev(self) -> &'a mut crate::W<REG> {
        self.variant(StartSrc::RtcCh2Ev)
    }
}
#[doc = "13:13\\]
Select active polarity for START_SRC event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartPol {
    #[doc = "1: Set ADC trigger on falling edge of event source."]
    Fall = 1,
    #[doc = "0: Set ADC trigger on rising edge of event source."]
    Rise = 0,
}
impl From<StartPol> for bool {
    #[inline(always)]
    fn from(variant: StartPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_POL` reader - 13:13\\]
Select active polarity for START_SRC event."]
pub type StartPolR = crate::BitReader<StartPol>;
impl StartPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StartPol {
        match self.bits {
            true => StartPol::Fall,
            false => StartPol::Rise,
        }
    }
    #[doc = "Set ADC trigger on falling edge of event source."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == StartPol::Fall
    }
    #[doc = "Set ADC trigger on rising edge of event source."]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == StartPol::Rise
    }
}
#[doc = "Field `START_POL` writer - 13:13\\]
Select active polarity for START_SRC event."]
pub type StartPolW<'a, REG> = crate::BitWriter<'a, REG, StartPol>;
impl<'a, REG> StartPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set ADC trigger on falling edge of event source."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(StartPol::Fall)
    }
    #[doc = "Set ADC trigger on rising edge of event source."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(StartPol::Rise)
    }
}
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT&lt;n> if you want to trigger the ADC manually through ADCTRIG.START."]
    #[inline(always)]
    pub fn start_src(&self) -> StartSrcR {
        StartSrcR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Select active polarity for START_SRC event."]
    #[inline(always)]
    pub fn start_pol(&self) -> StartPolR {
        StartPolR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<AdcctlSpec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT&lt;n> if you want to trigger the ADC manually through ADCTRIG.START."]
    #[inline(always)]
    #[must_use]
    pub fn start_src(&mut self) -> StartSrcW<AdcctlSpec> {
        StartSrcW::new(self, 8)
    }
    #[doc = "Bit 13 - 13:13\\]
Select active polarity for START_SRC event."]
    #[inline(always)]
    #[must_use]
    pub fn start_pol(&mut self) -> StartPolW<AdcctlSpec> {
        StartPolW::new(self, 13)
    }
}
#[doc = "ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcctlSpec;
impl crate::RegisterSpec for AdcctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcctl::R`](R) reader structure"]
impl crate::Readable for AdcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`adcctl::W`](W) writer structure"]
impl crate::Writable for AdcctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCCTL to value 0"]
impl crate::Resettable for AdcctlSpec {
    const RESET_VALUE: u32 = 0;
}

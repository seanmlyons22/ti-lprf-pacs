#[doc = "Register `SCEWEVSEL` reader"]
pub type R = crate::R<ScewevselSpec>;
#[doc = "Register `SCEWEVSEL` writer"]
pub type W = crate::W<ScewevselSpec>;
#[doc = "4:0\\]
Select event source to connect to AUX_SCE:WUSTAT.EV_SIGNALS bit 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wev7Ev {
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
impl From<Wev7Ev> for u8 {
    #[inline(always)]
    fn from(variant: Wev7Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wev7Ev {
    type Ux = u8;
}
impl crate::IsEnum for Wev7Ev {}
#[doc = "Field `WEV7_EV` reader - 4:0\\]
Select event source to connect to AUX_SCE:WUSTAT.EV_SIGNALS bit 7."]
pub type Wev7EvR = crate::FieldReader<Wev7Ev>;
impl Wev7EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wev7Ev {
        match self.bits {
            31 => Wev7Ev::AdcIrq,
            30 => Wev7Ev::McuEv,
            29 => Wev7Ev::AclkRef,
            28 => Wev7Ev::Auxio15,
            27 => Wev7Ev::Auxio14,
            26 => Wev7Ev::Auxio13,
            25 => Wev7Ev::Auxio12,
            24 => Wev7Ev::Auxio11,
            23 => Wev7Ev::Auxio10,
            22 => Wev7Ev::Auxio9,
            21 => Wev7Ev::Auxio8,
            20 => Wev7Ev::Auxio7,
            19 => Wev7Ev::Auxio6,
            18 => Wev7Ev::Auxio5,
            17 => Wev7Ev::Auxio4,
            16 => Wev7Ev::Auxio3,
            15 => Wev7Ev::Auxio2,
            14 => Wev7Ev::Auxio1,
            13 => Wev7Ev::Auxio0,
            12 => Wev7Ev::AonProgWu,
            11 => Wev7Ev::AonSw,
            10 => Wev7Ev::Obsmux1,
            9 => Wev7Ev::Obsmux0,
            8 => Wev7Ev::AdcFifoAlmostFull,
            7 => Wev7Ev::AdcDone,
            6 => Wev7Ev::SmphAutotakeDone,
            5 => Wev7Ev::Timer1Ev,
            4 => Wev7Ev::Timer0Ev,
            3 => Wev7Ev::TdcDone,
            2 => Wev7Ev::AuxCompb,
            1 => Wev7Ev::AuxCompa,
            0 => Wev7Ev::AonRtcCh2,
            _ => unreachable!(),
        }
    }
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn is_adc_irq(&self) -> bool {
        *self == Wev7Ev::AdcIrq
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == Wev7Ev::McuEv
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == Wev7Ev::AclkRef
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == Wev7Ev::Auxio15
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == Wev7Ev::Auxio14
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == Wev7Ev::Auxio13
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == Wev7Ev::Auxio12
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == Wev7Ev::Auxio11
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == Wev7Ev::Auxio10
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == Wev7Ev::Auxio9
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == Wev7Ev::Auxio8
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == Wev7Ev::Auxio7
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == Wev7Ev::Auxio6
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == Wev7Ev::Auxio5
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == Wev7Ev::Auxio4
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == Wev7Ev::Auxio3
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == Wev7Ev::Auxio2
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == Wev7Ev::Auxio1
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == Wev7Ev::Auxio0
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == Wev7Ev::AonProgWu
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn is_aon_sw(&self) -> bool {
        *self == Wev7Ev::AonSw
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn is_obsmux1(&self) -> bool {
        *self == Wev7Ev::Obsmux1
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn is_obsmux0(&self) -> bool {
        *self == Wev7Ev::Obsmux0
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn is_adc_fifo_almost_full(&self) -> bool {
        *self == Wev7Ev::AdcFifoAlmostFull
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn is_adc_done(&self) -> bool {
        *self == Wev7Ev::AdcDone
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == Wev7Ev::SmphAutotakeDone
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn is_timer1_ev(&self) -> bool {
        *self == Wev7Ev::Timer1Ev
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn is_timer0_ev(&self) -> bool {
        *self == Wev7Ev::Timer0Ev
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn is_tdc_done(&self) -> bool {
        *self == Wev7Ev::TdcDone
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Wev7Ev::AuxCompb
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Wev7Ev::AuxCompa
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == Wev7Ev::AonRtcCh2
    }
}
#[doc = "Field `WEV7_EV` writer - 4:0\\]
Select event source to connect to AUX_SCE:WUSTAT.EV_SIGNALS bit 7."]
pub type Wev7EvW<'a, REG> = crate::FieldWriter<'a, REG, 5, Wev7Ev, crate::Safe>;
impl<'a, REG> Wev7EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline(always)]
    pub fn adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::AdcIrq)
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::McuEv)
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::AclkRef)
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio15)
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio14)
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio13)
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio12)
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio11)
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio10)
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio9)
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio8)
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio7)
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio6)
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio5)
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio4)
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Auxio0)
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::AonProgWu)
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline(always)]
    pub fn aon_sw(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::AonSw)
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline(always)]
    pub fn obsmux1(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Obsmux1)
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline(always)]
    pub fn obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Obsmux0)
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::AdcFifoAlmostFull)
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline(always)]
    pub fn adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::AdcDone)
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::SmphAutotakeDone)
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Timer1Ev)
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline(always)]
    pub fn timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::Timer0Ev)
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::TdcDone)
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::AuxCompb)
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::AuxCompa)
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Wev7Ev::AonRtcCh2)
    }
}
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Select event source to connect to AUX_SCE:WUSTAT.EV_SIGNALS bit 7."]
    #[inline(always)]
    pub fn wev7_ev(&self) -> Wev7EvR {
        Wev7EvR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Select event source to connect to AUX_SCE:WUSTAT.EV_SIGNALS bit 7."]
    #[inline(always)]
    #[must_use]
    pub fn wev7_ev(&mut self) -> Wev7EvW<ScewevselSpec> {
        Wev7EvW::new(self, 0)
    }
}
#[doc = "Sensor Controller Engine Wait Event Selection Configuration of this register controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scewevsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scewevsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScewevselSpec;
impl crate::RegisterSpec for ScewevselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scewevsel::R`](R) reader structure"]
impl crate::Readable for ScewevselSpec {}
#[doc = "`write(|w| ..)` method takes [`scewevsel::W`](W) writer structure"]
impl crate::Writable for ScewevselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCEWEVSEL to value 0"]
impl crate::Resettable for ScewevselSpec {
    const RESET_VALUE: u32 = 0;
}

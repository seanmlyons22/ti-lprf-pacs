#[doc = "Register `CPUIRQSEL30` reader"]
pub struct R(crate::R<CPUIRQSEL30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUIRQSEL30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUIRQSEL30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUIRQSEL30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUIRQSEL30` writer"]
pub struct W(crate::W<CPUIRQSEL30_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUIRQSEL30_SPEC>;
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
impl From<crate::W<CPUIRQSEL30_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUIRQSEL30_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV` reader - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EV_R = crate::FieldReader<u8, EV_A>;
#[doc = "6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "121: Always asserted"]
    ALWAYS_ACTIVE = 121,
    #[doc = "119: RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    AON_RTC_UPD = 119,
    #[doc = "114: Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.MCU_OBSMUX0"]
    AUX_OBSMUX0 = 114,
    #[doc = "113: AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL"]
    AUX_ADC_FIFO_ALMOST_FULL = 113,
    #[doc = "112: AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_DONE"]
    AUX_ADC_DONE = 112,
    #[doc = "111: Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    AUX_SMPH_AUTOTAKE_DONE = 111,
    #[doc = "110: AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER1_EV"]
    AUX_TIMER1_EV = 110,
    #[doc = "109: AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER0_EV"]
    AUX_TIMER0_EV = 109,
    #[doc = "108: AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
    AUX_TDC_DONE = 108,
    #[doc = "107: AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB"]
    AUX_COMPB = 107,
    #[doc = "105: AON wakeup event, the corresponding flag is here AUX_EVCTL:EVTOMCUFLAGS.AUX_WU_EV"]
    AUX_AON_WU_EV = 105,
    #[doc = "94: CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    CRYPTO_DMA_DONE_IRQ = 94,
    #[doc = "60: AUX Timer2 pulse, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_PULSE"]
    AUX_TIMER2_PULSE = 60,
    #[doc = "59: AUX Timer2 event 3, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV3"]
    AUX_TIMER2_EV3 = 59,
    #[doc = "58: AUX Timer2 event 2, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV2"]
    AUX_TIMER2_EV2 = 58,
    #[doc = "57: AUX Timer2 event 1, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV1"]
    AUX_TIMER2_EV1 = 57,
    #[doc = "56: AUX Timer2 event 0, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV0"]
    AUX_TIMER2_EV0 = 56,
    #[doc = "22: DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    DMA_CH18_DONE = 22,
    #[doc = "20: DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    DMA_CH0_DONE = 20,
    #[doc = "10: AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    AON_AUX_SWEV0 = 10,
    #[doc = "8: Interrupt event from I2S"]
    I2S_IRQ = 8,
    #[doc = "3: AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    AON_PROG2 = 3,
    #[doc = "2: AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    AON_PROG1 = 2,
    #[doc = "0: Always inactive"]
    NONE = 0,
}
impl From<EV_A> for u8 {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        variant as _
    }
}
impl EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EV_A> {
        match self.bits {
            121 => Some(EV_A::ALWAYS_ACTIVE),
            119 => Some(EV_A::AON_RTC_UPD),
            114 => Some(EV_A::AUX_OBSMUX0),
            113 => Some(EV_A::AUX_ADC_FIFO_ALMOST_FULL),
            112 => Some(EV_A::AUX_ADC_DONE),
            111 => Some(EV_A::AUX_SMPH_AUTOTAKE_DONE),
            110 => Some(EV_A::AUX_TIMER1_EV),
            109 => Some(EV_A::AUX_TIMER0_EV),
            108 => Some(EV_A::AUX_TDC_DONE),
            107 => Some(EV_A::AUX_COMPB),
            105 => Some(EV_A::AUX_AON_WU_EV),
            94 => Some(EV_A::CRYPTO_DMA_DONE_IRQ),
            60 => Some(EV_A::AUX_TIMER2_PULSE),
            59 => Some(EV_A::AUX_TIMER2_EV3),
            58 => Some(EV_A::AUX_TIMER2_EV2),
            57 => Some(EV_A::AUX_TIMER2_EV1),
            56 => Some(EV_A::AUX_TIMER2_EV0),
            22 => Some(EV_A::DMA_CH18_DONE),
            20 => Some(EV_A::DMA_CH0_DONE),
            10 => Some(EV_A::AON_AUX_SWEV0),
            8 => Some(EV_A::I2S_IRQ),
            3 => Some(EV_A::AON_PROG2),
            2 => Some(EV_A::AON_PROG1),
            0 => Some(EV_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        *self == EV_A::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `AON_RTC_UPD`"]
    #[inline(always)]
    pub fn is_aon_rtc_upd(&self) -> bool {
        *self == EV_A::AON_RTC_UPD
    }
    #[doc = "Checks if the value of the field is `AUX_OBSMUX0`"]
    #[inline(always)]
    pub fn is_aux_obsmux0(&self) -> bool {
        *self == EV_A::AUX_OBSMUX0
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_ALMOST_FULL`"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == EV_A::AUX_ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_SMPH_AUTOTAKE_DONE`"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == EV_A::AUX_SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_AON_WU_EV`"]
    #[inline(always)]
    pub fn is_aux_aon_wu_ev(&self) -> bool {
        *self == EV_A::AUX_AON_WU_EV
    }
    #[doc = "Checks if the value of the field is `CRYPTO_DMA_DONE_IRQ`"]
    #[inline(always)]
    pub fn is_crypto_dma_done_irq(&self) -> bool {
        *self == EV_A::CRYPTO_DMA_DONE_IRQ
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_PULSE`"]
    #[inline(always)]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == EV_A::AUX_TIMER2_PULSE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `DMA_CH18_DONE`"]
    #[inline(always)]
    pub fn is_dma_ch18_done(&self) -> bool {
        *self == EV_A::DMA_CH18_DONE
    }
    #[doc = "Checks if the value of the field is `DMA_CH0_DONE`"]
    #[inline(always)]
    pub fn is_dma_ch0_done(&self) -> bool {
        *self == EV_A::DMA_CH0_DONE
    }
    #[doc = "Checks if the value of the field is `AON_AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aon_aux_swev0(&self) -> bool {
        *self == EV_A::AON_AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `I2S_IRQ`"]
    #[inline(always)]
    pub fn is_i2s_irq(&self) -> bool {
        *self == EV_A::I2S_IRQ
    }
    #[doc = "Checks if the value of the field is `AON_PROG2`"]
    #[inline(always)]
    pub fn is_aon_prog2(&self) -> bool {
        *self == EV_A::AON_PROG2
    }
    #[doc = "Checks if the value of the field is `AON_PROG1`"]
    #[inline(always)]
    pub fn is_aon_prog1(&self) -> bool {
        *self == EV_A::AON_PROG1
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EV_A::NONE
    }
}
#[doc = "Field `EV` writer - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPUIRQSEL30_SPEC, u8, EV_A, 7, O>;
impl<'a, const O: u8> EV_W<'a, O> {
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn always_active(self) -> &'a mut W {
        self.variant(EV_A::ALWAYS_ACTIVE)
    }
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    #[inline(always)]
    pub fn aon_rtc_upd(self) -> &'a mut W {
        self.variant(EV_A::AON_RTC_UPD)
    }
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn aux_obsmux0(self) -> &'a mut W {
        self.variant(EV_A::AUX_OBSMUX0)
    }
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(EV_A::AUX_ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(EV_A::AUX_ADC_DONE)
    }
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut W {
        self.variant(EV_A::AUX_SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER0_EV)
    }
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(EV_A::AUX_TDC_DONE)
    }
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(EV_A::AUX_COMPB)
    }
    #[doc = "AON wakeup event, the corresponding flag is here AUX_EVCTL:EVTOMCUFLAGS.AUX_WU_EV"]
    #[inline(always)]
    pub fn aux_aon_wu_ev(self) -> &'a mut W {
        self.variant(EV_A::AUX_AON_WU_EV)
    }
    #[doc = "CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    #[inline(always)]
    pub fn crypto_dma_done_irq(self) -> &'a mut W {
        self.variant(EV_A::CRYPTO_DMA_DONE_IRQ)
    }
    #[doc = "AUX Timer2 pulse, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_pulse(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_PULSE)
    }
    #[doc = "AUX Timer2 event 3, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "AUX Timer2 event 2, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "AUX Timer2 event 1, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "AUX Timer2 event 0, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    #[inline(always)]
    pub fn dma_ch18_done(self) -> &'a mut W {
        self.variant(EV_A::DMA_CH18_DONE)
    }
    #[doc = "DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    #[inline(always)]
    pub fn dma_ch0_done(self) -> &'a mut W {
        self.variant(EV_A::DMA_CH0_DONE)
    }
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aon_aux_swev0(self) -> &'a mut W {
        self.variant(EV_A::AON_AUX_SWEV0)
    }
    #[doc = "Interrupt event from I2S"]
    #[inline(always)]
    pub fn i2s_irq(self) -> &'a mut W {
        self.variant(EV_A::I2S_IRQ)
    }
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    #[inline(always)]
    pub fn aon_prog2(self) -> &'a mut W {
        self.variant(EV_A::AON_PROG2)
    }
    #[doc = "AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    #[inline(always)]
    pub fn aon_prog1(self) -> &'a mut W {
        self.variant(EV_A::AON_PROG1)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(EV_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EV_W<0> {
        EV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Selection for CPU Interrupt 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel30](index.html) module"]
pub struct CPUIRQSEL30_SPEC;
impl crate::RegisterSpec for CPUIRQSEL30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuirqsel30::R](R) reader structure"]
impl crate::Readable for CPUIRQSEL30_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpuirqsel30::W](W) writer structure"]
impl crate::Writable for CPUIRQSEL30_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL30 to value 0"]
impl crate::Resettable for CPUIRQSEL30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

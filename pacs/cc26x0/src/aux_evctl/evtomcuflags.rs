#[doc = "Register `EVTOMCUFLAGS` reader"]
pub type R = crate::R<EvtomcuflagsSpec>;
#[doc = "Register `EVTOMCUFLAGS` writer"]
pub type W = crate::W<EvtomcuflagsSpec>;
#[doc = "Field `AON_WU_EV` reader - 0:0\\]
This event flag is set when level selected by EVTOMCUPOL.AON_WU_EV occurs on the reduction-OR of the AUX_EVCTL:EVSTAT0.RTC_CH2_EV, AUX_EVCTL:EVSTAT0.AON_SW, and AUX_EVCTL:EVSTAT0.AON_PROG_WU events."]
pub type AonWuEvR = crate::BitReader;
#[doc = "Field `AON_WU_EV` writer - 0:0\\]
This event flag is set when level selected by EVTOMCUPOL.AON_WU_EV occurs on the reduction-OR of the AUX_EVCTL:EVSTAT0.RTC_CH2_EV, AUX_EVCTL:EVSTAT0.AON_SW, and AUX_EVCTL:EVSTAT0.AON_PROG_WU events."]
pub type AonWuEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_COMPA` reader - 1:1\\]
This event flag is set when edge selected by EVTOMCUPOL.AUX_COMPA occurs on EVSTAT0.AUX_COMPA."]
pub type AuxCompaR = crate::BitReader;
#[doc = "Field `AUX_COMPA` writer - 1:1\\]
This event flag is set when edge selected by EVTOMCUPOL.AUX_COMPA occurs on EVSTAT0.AUX_COMPA."]
pub type AuxCompaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_COMPB` reader - 2:2\\]
This event flag is set when edge selected by EVTOMCUPOL.AUX_COMPB occurs on EVSTAT0.AUX_COMPB."]
pub type AuxCompbR = crate::BitReader;
#[doc = "Field `AUX_COMPB` writer - 2:2\\]
This event flag is set when edge selected by EVTOMCUPOL.AUX_COMPB occurs on EVSTAT0.AUX_COMPB."]
pub type AuxCompbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDC_DONE` reader - 3:3\\]
This event flag is set when level selected by EVTOMCUPOL.TDC_DONE occurs on EVSTAT0.TDC_DONE."]
pub type TdcDoneR = crate::BitReader;
#[doc = "Field `TDC_DONE` writer - 3:3\\]
This event flag is set when level selected by EVTOMCUPOL.TDC_DONE occurs on EVSTAT0.TDC_DONE."]
pub type TdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_EV` reader - 4:4\\]
This event flag is set when level selected by EVTOMCUPOL.TIMER0_EV occurs on EVSTAT0.TIMER0_EV."]
pub type Timer0EvR = crate::BitReader;
#[doc = "Field `TIMER0_EV` writer - 4:4\\]
This event flag is set when level selected by EVTOMCUPOL.TIMER0_EV occurs on EVSTAT0.TIMER0_EV."]
pub type Timer0EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_EV` reader - 5:5\\]
This event flag is set when level selected by EVTOMCUPOL.TIMER1_EV occurs on EVSTAT0.TIMER1_EV."]
pub type Timer1EvR = crate::BitReader;
#[doc = "Field `TIMER1_EV` writer - 5:5\\]
This event flag is set when level selected by EVTOMCUPOL.TIMER1_EV occurs on EVSTAT0.TIMER1_EV."]
pub type Timer1EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPH_AUTOTAKE_DONE` reader - 6:6\\]
This event flag is set when level selected by EVTOMCUPOL.SMPH_AUTOTAKE_DONE occurs on EVSTAT0.SMPH_AUTOTAKE_DONE."]
pub type SmphAutotakeDoneR = crate::BitReader;
#[doc = "Field `SMPH_AUTOTAKE_DONE` writer - 6:6\\]
This event flag is set when level selected by EVTOMCUPOL.SMPH_AUTOTAKE_DONE occurs on EVSTAT0.SMPH_AUTOTAKE_DONE."]
pub type SmphAutotakeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_DONE` reader - 7:7\\]
This event flag is set when level selected by EVTOMCUPOL.ADC_DONE occurs on EVSTAT0.ADC_DONE."]
pub type AdcDoneR = crate::BitReader;
#[doc = "Field `ADC_DONE` writer - 7:7\\]
This event flag is set when level selected by EVTOMCUPOL.ADC_DONE occurs on EVSTAT0.ADC_DONE."]
pub type AdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_FIFO_ALMOST_FULL` reader - 8:8\\]
This event flag is set when level selected by EVTOMCUPOL.ADC_FIFO_ALMOST_FULL occurs on EVSTAT0.ADC_FIFO_ALMOST_FULL."]
pub type AdcFifoAlmostFullR = crate::BitReader;
#[doc = "Field `ADC_FIFO_ALMOST_FULL` writer - 8:8\\]
This event flag is set when level selected by EVTOMCUPOL.ADC_FIFO_ALMOST_FULL occurs on EVSTAT0.ADC_FIFO_ALMOST_FULL."]
pub type AdcFifoAlmostFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBSMUX0` reader - 9:9\\]
This event flag is set when level selected by EVTOMCUPOL.MCU_OBSMUX0 occurs on EVSTAT0.MCU_OBSMUX0."]
pub type Obsmux0R = crate::BitReader;
#[doc = "Field `OBSMUX0` writer - 9:9\\]
This event flag is set when level selected by EVTOMCUPOL.MCU_OBSMUX0 occurs on EVSTAT0.MCU_OBSMUX0."]
pub type Obsmux0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_IRQ` reader - 10:10\\]
This event flag is set when level selected by EVTOMCUPOL.ADC_IRQ occurs on EVSTAT0.ADC_IRQ."]
pub type AdcIrqR = crate::BitReader;
#[doc = "Field `ADC_IRQ` writer - 10:10\\]
This event flag is set when level selected by EVTOMCUPOL.ADC_IRQ occurs on EVSTAT0.ADC_IRQ."]
pub type AdcIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This event flag is set when level selected by EVTOMCUPOL.AON_WU_EV occurs on the reduction-OR of the AUX_EVCTL:EVSTAT0.RTC_CH2_EV, AUX_EVCTL:EVSTAT0.AON_SW, and AUX_EVCTL:EVSTAT0.AON_PROG_WU events."]
    #[inline(always)]
    pub fn aon_wu_ev(&self) -> AonWuEvR {
        AonWuEvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This event flag is set when edge selected by EVTOMCUPOL.AUX_COMPA occurs on EVSTAT0.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AuxCompaR {
        AuxCompaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This event flag is set when edge selected by EVTOMCUPOL.AUX_COMPB occurs on EVSTAT0.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AuxCompbR {
        AuxCompbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This event flag is set when level selected by EVTOMCUPOL.TDC_DONE occurs on EVSTAT0.TDC_DONE."]
    #[inline(always)]
    pub fn tdc_done(&self) -> TdcDoneR {
        TdcDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This event flag is set when level selected by EVTOMCUPOL.TIMER0_EV occurs on EVSTAT0.TIMER0_EV."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> Timer0EvR {
        Timer0EvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This event flag is set when level selected by EVTOMCUPOL.TIMER1_EV occurs on EVSTAT0.TIMER1_EV."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> Timer1EvR {
        Timer1EvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
This event flag is set when level selected by EVTOMCUPOL.SMPH_AUTOTAKE_DONE occurs on EVSTAT0.SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    pub fn smph_autotake_done(&self) -> SmphAutotakeDoneR {
        SmphAutotakeDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
This event flag is set when level selected by EVTOMCUPOL.ADC_DONE occurs on EVSTAT0.ADC_DONE."]
    #[inline(always)]
    pub fn adc_done(&self) -> AdcDoneR {
        AdcDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
This event flag is set when level selected by EVTOMCUPOL.ADC_FIFO_ALMOST_FULL occurs on EVSTAT0.ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    pub fn adc_fifo_almost_full(&self) -> AdcFifoAlmostFullR {
        AdcFifoAlmostFullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
This event flag is set when level selected by EVTOMCUPOL.MCU_OBSMUX0 occurs on EVSTAT0.MCU_OBSMUX0."]
    #[inline(always)]
    pub fn obsmux0(&self) -> Obsmux0R {
        Obsmux0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
This event flag is set when level selected by EVTOMCUPOL.ADC_IRQ occurs on EVSTAT0.ADC_IRQ."]
    #[inline(always)]
    pub fn adc_irq(&self) -> AdcIrqR {
        AdcIrqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This event flag is set when level selected by EVTOMCUPOL.AON_WU_EV occurs on the reduction-OR of the AUX_EVCTL:EVSTAT0.RTC_CH2_EV, AUX_EVCTL:EVSTAT0.AON_SW, and AUX_EVCTL:EVSTAT0.AON_PROG_WU events."]
    #[inline(always)]
    #[must_use]
    pub fn aon_wu_ev(&mut self) -> AonWuEvW<EvtomcuflagsSpec> {
        AonWuEvW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This event flag is set when edge selected by EVTOMCUPOL.AUX_COMPA occurs on EVSTAT0.AUX_COMPA."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AuxCompaW<EvtomcuflagsSpec> {
        AuxCompaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This event flag is set when edge selected by EVTOMCUPOL.AUX_COMPB occurs on EVSTAT0.AUX_COMPB."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AuxCompbW<EvtomcuflagsSpec> {
        AuxCompbW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
This event flag is set when level selected by EVTOMCUPOL.TDC_DONE occurs on EVSTAT0.TDC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn tdc_done(&mut self) -> TdcDoneW<EvtomcuflagsSpec> {
        TdcDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
This event flag is set when level selected by EVTOMCUPOL.TIMER0_EV occurs on EVSTAT0.TIMER0_EV."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_ev(&mut self) -> Timer0EvW<EvtomcuflagsSpec> {
        Timer0EvW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
This event flag is set when level selected by EVTOMCUPOL.TIMER1_EV occurs on EVSTAT0.TIMER1_EV."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ev(&mut self) -> Timer1EvW<EvtomcuflagsSpec> {
        Timer1EvW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
This event flag is set when level selected by EVTOMCUPOL.SMPH_AUTOTAKE_DONE occurs on EVSTAT0.SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn smph_autotake_done(&mut self) -> SmphAutotakeDoneW<EvtomcuflagsSpec> {
        SmphAutotakeDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
This event flag is set when level selected by EVTOMCUPOL.ADC_DONE occurs on EVSTAT0.ADC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn adc_done(&mut self) -> AdcDoneW<EvtomcuflagsSpec> {
        AdcDoneW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
This event flag is set when level selected by EVTOMCUPOL.ADC_FIFO_ALMOST_FULL occurs on EVSTAT0.ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    #[must_use]
    pub fn adc_fifo_almost_full(&mut self) -> AdcFifoAlmostFullW<EvtomcuflagsSpec> {
        AdcFifoAlmostFullW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
This event flag is set when level selected by EVTOMCUPOL.MCU_OBSMUX0 occurs on EVSTAT0.MCU_OBSMUX0."]
    #[inline(always)]
    #[must_use]
    pub fn obsmux0(&mut self) -> Obsmux0W<EvtomcuflagsSpec> {
        Obsmux0W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
This event flag is set when level selected by EVTOMCUPOL.ADC_IRQ occurs on EVSTAT0.ADC_IRQ."]
    #[inline(always)]
    #[must_use]
    pub fn adc_irq(&mut self) -> AdcIrqW<EvtomcuflagsSpec> {
        AdcIrqW::new(self, 10)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<EvtomcuflagsSpec> {
        Reserved11W::new(self, 11)
    }
}
#[doc = "Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtomcuflags::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtomcuflags::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtomcuflagsSpec;
impl crate::RegisterSpec for EvtomcuflagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtomcuflags::R`](R) reader structure"]
impl crate::Readable for EvtomcuflagsSpec {}
#[doc = "`write(|w| ..)` method takes [`evtomcuflags::W`](W) writer structure"]
impl crate::Writable for EvtomcuflagsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVTOMCUFLAGS to value 0"]
impl crate::Resettable for EvtomcuflagsSpec {
    const RESET_VALUE: u32 = 0;
}

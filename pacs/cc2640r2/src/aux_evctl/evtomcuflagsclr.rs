#[doc = "Register `EVTOMCUFLAGSCLR` reader"]
pub type R = crate::R<EvtomcuflagsclrSpec>;
#[doc = "Register `EVTOMCUFLAGSCLR` writer"]
pub type W = crate::W<EvtomcuflagsclrSpec>;
#[doc = "Field `AON_WU_EV` writer - 0:0\\]
Write 1 to clear EVTOMCUFLAGS.AON_WU_EV. Read value is 0."]
pub type AonWuEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_COMPA` writer - 1:1\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPA. Read value is 0."]
pub type AuxCompaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_COMPB` writer - 2:2\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPB. Read value is 0."]
pub type AuxCompbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDC_DONE` writer - 3:3\\]
Write 1 to clear EVTOMCUFLAGS.TDC_DONE. Read value is 0."]
pub type TdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_EV` writer - 4:4\\]
Write 1 to clear EVTOMCUFLAGS.TIMER0_EV. Read value is 0."]
pub type Timer0EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_EV` writer - 5:5\\]
Write 1 to clear EVTOMCUFLAGS.TIMER1_EV. Read value is 0."]
pub type Timer1EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPH_AUTOTAKE_DONE` writer - 6:6\\]
Write 1 to clear EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE. Read value is 0."]
pub type SmphAutotakeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_DONE` writer - 7:7\\]
Write 1 to clear EVTOMCUFLAGS.ADC_DONE. Read value is 0."]
pub type AdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_FIFO_ALMOST_FULL` writer - 8:8\\]
Write 1 to clear EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL. Read value is 0."]
pub type AdcFifoAlmostFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBSMUX0` writer - 9:9\\]
Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
pub type Obsmux0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_IRQ` writer - 10:10\\]
Write 1 to clear EVTOMCUFLAGS.ADC_IRQ. Read value is 0."]
pub type AdcIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write 1 to clear EVTOMCUFLAGS.AON_WU_EV. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aon_wu_ev(&mut self) -> AonWuEvW<EvtomcuflagsclrSpec> {
        AonWuEvW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPA. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AuxCompaW<EvtomcuflagsclrSpec> {
        AuxCompaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPB. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AuxCompbW<EvtomcuflagsclrSpec> {
        AuxCompbW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Write 1 to clear EVTOMCUFLAGS.TDC_DONE. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn tdc_done(&mut self) -> TdcDoneW<EvtomcuflagsclrSpec> {
        TdcDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Write 1 to clear EVTOMCUFLAGS.TIMER0_EV. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_ev(&mut self) -> Timer0EvW<EvtomcuflagsclrSpec> {
        Timer0EvW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Write 1 to clear EVTOMCUFLAGS.TIMER1_EV. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ev(&mut self) -> Timer1EvW<EvtomcuflagsclrSpec> {
        Timer1EvW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Write 1 to clear EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn smph_autotake_done(&mut self) -> SmphAutotakeDoneW<EvtomcuflagsclrSpec> {
        SmphAutotakeDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Write 1 to clear EVTOMCUFLAGS.ADC_DONE. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn adc_done(&mut self) -> AdcDoneW<EvtomcuflagsclrSpec> {
        AdcDoneW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Write 1 to clear EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn adc_fifo_almost_full(&mut self) -> AdcFifoAlmostFullW<EvtomcuflagsclrSpec> {
        AdcFifoAlmostFullW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn obsmux0(&mut self) -> Obsmux0W<EvtomcuflagsclrSpec> {
        Obsmux0W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Write 1 to clear EVTOMCUFLAGS.ADC_IRQ. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn adc_irq(&mut self) -> AdcIrqW<EvtomcuflagsclrSpec> {
        AdcIrqW::new(self, 10)
    }
}
#[doc = "Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtomcuflagsclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtomcuflagsclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtomcuflagsclrSpec;
impl crate::RegisterSpec for EvtomcuflagsclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtomcuflagsclr::R`](R) reader structure"]
impl crate::Readable for EvtomcuflagsclrSpec {}
#[doc = "`write(|w| ..)` method takes [`evtomcuflagsclr::W`](W) writer structure"]
impl crate::Writable for EvtomcuflagsclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVTOMCUFLAGSCLR to value 0"]
impl crate::Resettable for EvtomcuflagsclrSpec {
    const RESET_VALUE: u32 = 0;
}

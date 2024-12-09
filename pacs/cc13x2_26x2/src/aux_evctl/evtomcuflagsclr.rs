#[doc = "Register `EVTOMCUFLAGSCLR` reader"]
pub type R = crate::R<EvtomcuflagsclrSpec>;
#[doc = "Register `EVTOMCUFLAGSCLR` writer"]
pub type W = crate::W<EvtomcuflagsclrSpec>;
#[doc = "Field `AUX_WU_EV` writer - 0:0\\]
Write 1 to clear EVTOMCUFLAGS.AUX_WU_EV. Read value is 0."]
pub type AuxWuEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_COMPA` writer - 1:1\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPA. Read value is 0."]
pub type AuxCompaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_COMPB` writer - 2:2\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPB. Read value is 0."]
pub type AuxCompbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TDC_DONE` writer - 3:3\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TDC_DONE. Read value is 0."]
pub type AuxTdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER0_EV` writer - 4:4\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER0_EV. Read value is 0."]
pub type AuxTimer0EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER1_EV` writer - 5:5\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER1_EV. Read value is 0."]
pub type AuxTimer1EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_SMPH_AUTOTAKE_DONE` writer - 6:6\\]
Write 1 to clear EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE. Read value is 0."]
pub type AuxSmphAutotakeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_ADC_DONE` writer - 7:7\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_DONE. Read value is 0."]
pub type AuxAdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_ADC_FIFO_ALMOST_FULL` writer - 8:8\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL. Read value is 0."]
pub type AuxAdcFifoAlmostFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_OBSMUX0` writer - 9:9\\]
Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
pub type McuObsmux0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_ADC_IRQ` writer - 10:10\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_IRQ. Read value is 0."]
pub type AuxAdcIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_EV0` writer - 11:11\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV0. Read value is 0."]
pub type AuxTimer2Ev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_EV1` writer - 12:12\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV1. Read value is 0."]
pub type AuxTimer2Ev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_EV2` writer - 13:13\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV2. Read value is 0."]
pub type AuxTimer2Ev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_EV3` writer - 14:14\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV3. Read value is 0."]
pub type AuxTimer2Ev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_PULSE` writer - 15:15\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_PULSE. Read value is 0."]
pub type AuxTimer2PulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write 1 to clear EVTOMCUFLAGS.AUX_WU_EV. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_wu_ev(&mut self) -> AuxWuEvW<EvtomcuflagsclrSpec> {
        AuxWuEvW::new(self, 0)
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
Write 1 to clear EVTOMCUFLAGS.AUX_TDC_DONE. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_tdc_done(&mut self) -> AuxTdcDoneW<EvtomcuflagsclrSpec> {
        AuxTdcDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER0_EV. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer0_ev(&mut self) -> AuxTimer0EvW<EvtomcuflagsclrSpec> {
        AuxTimer0EvW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER1_EV. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer1_ev(&mut self) -> AuxTimer1EvW<EvtomcuflagsclrSpec> {
        AuxTimer1EvW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Write 1 to clear EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_smph_autotake_done(&mut self) -> AuxSmphAutotakeDoneW<EvtomcuflagsclrSpec> {
        AuxSmphAutotakeDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_DONE. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_done(&mut self) -> AuxAdcDoneW<EvtomcuflagsclrSpec> {
        AuxAdcDoneW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_fifo_almost_full(&mut self) -> AuxAdcFifoAlmostFullW<EvtomcuflagsclrSpec> {
        AuxAdcFifoAlmostFullW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsmux0(&mut self) -> McuObsmux0W<EvtomcuflagsclrSpec> {
        McuObsmux0W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_IRQ. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_irq(&mut self) -> AuxAdcIrqW<EvtomcuflagsclrSpec> {
        AuxAdcIrqW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV0. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev0(&mut self) -> AuxTimer2Ev0W<EvtomcuflagsclrSpec> {
        AuxTimer2Ev0W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV1. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev1(&mut self) -> AuxTimer2Ev1W<EvtomcuflagsclrSpec> {
        AuxTimer2Ev1W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV2. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev2(&mut self) -> AuxTimer2Ev2W<EvtomcuflagsclrSpec> {
        AuxTimer2Ev2W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV3. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev3(&mut self) -> AuxTimer2Ev3W<EvtomcuflagsclrSpec> {
        AuxTimer2Ev3W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_PULSE. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_pulse(&mut self) -> AuxTimer2PulseW<EvtomcuflagsclrSpec> {
        AuxTimer2PulseW::new(self, 15)
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

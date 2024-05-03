#[doc = "Register `EVSTAT3` reader"]
pub type R = crate::R<Evstat3Spec>;
#[doc = "Register `EVSTAT3` writer"]
pub type W = crate::W<Evstat3Spec>;
#[doc = "Field `AUX_TIMER2_EV0` reader - 0:0\\]
AUX_TIMER2 event output 0. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxTimer2Ev0R = crate::BitReader;
#[doc = "Field `AUX_TIMER2_EV0` writer - 0:0\\]
AUX_TIMER2 event output 0. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxTimer2Ev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_EV1` reader - 1:1\\]
AUX_TIMER2 event output 1. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxTimer2Ev1R = crate::BitReader;
#[doc = "Field `AUX_TIMER2_EV1` writer - 1:1\\]
AUX_TIMER2 event output 1. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxTimer2Ev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_EV2` reader - 2:2\\]
AUX_TIMER2 event output 2. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxTimer2Ev2R = crate::BitReader;
#[doc = "Field `AUX_TIMER2_EV2` writer - 2:2\\]
AUX_TIMER2 event output 2. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxTimer2Ev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_EV3` reader - 3:3\\]
AUX_TIMER2 event output 3. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxTimer2Ev3R = crate::BitReader;
#[doc = "Field `AUX_TIMER2_EV3` writer - 3:3\\]
AUX_TIMER2 event output 3. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxTimer2Ev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_PULSE` reader - 4:4\\]
AUX_TIMER2 pulse event. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxTimer2PulseR = crate::BitReader;
#[doc = "Field `AUX_TIMER2_PULSE` writer - 4:4\\]
AUX_TIMER2 pulse event. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxTimer2PulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER1_EV` reader - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER01:T1TARGET for description."]
pub type AuxTimer1EvR = crate::BitReader;
#[doc = "Field `AUX_TIMER1_EV` writer - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER01:T1TARGET for description."]
pub type AuxTimer1EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER0_EV` reader - 6:6\\]
AUX_TIMER0_EV event, see AUX_TIMER01:T0TARGET for description."]
pub type AuxTimer0EvR = crate::BitReader;
#[doc = "Field `AUX_TIMER0_EV` writer - 6:6\\]
AUX_TIMER0_EV event, see AUX_TIMER01:T0TARGET for description."]
pub type AuxTimer0EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TDC_DONE` reader - 7:7\\]
AUX_TDC:STAT.DONE"]
pub type AuxTdcDoneR = crate::BitReader;
#[doc = "Field `AUX_TDC_DONE` writer - 7:7\\]
AUX_TDC:STAT.DONE"]
pub type AuxTdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_ISRC_RESET_N` reader - 8:8\\]
AUX_ANAIF:ISRCCTL.RESET_N"]
pub type AuxIsrcResetNR = crate::BitReader;
#[doc = "Field `AUX_ISRC_RESET_N` writer - 8:8\\]
AUX_ANAIF:ISRCCTL.RESET_N"]
pub type AuxIsrcResetNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_ADC_DONE` reader - 9:9\\]
AUX_ANAIF ADC conversion done event. Event is synchronized at AUX bus rate."]
pub type AuxAdcDoneR = crate::BitReader;
#[doc = "Field `AUX_ADC_DONE` writer - 9:9\\]
AUX_ANAIF ADC conversion done event. Event is synchronized at AUX bus rate."]
pub type AuxAdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_ADC_IRQ` reader - 10:10\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
pub type AuxAdcIrqR = crate::BitReader;
#[doc = "Field `AUX_ADC_IRQ` writer - 10:10\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
pub type AuxAdcIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_ADC_FIFO_ALMOST_FULL` reader - 11:11\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
pub type AuxAdcFifoAlmostFullR = crate::BitReader;
#[doc = "Field `AUX_ADC_FIFO_ALMOST_FULL` writer - 11:11\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
pub type AuxAdcFifoAlmostFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_ADC_FIFO_NOT_EMPTY` reader - 12:12\\]
AUX_ANAIF:ADCFIFOSTAT.EMPTY negated"]
pub type AuxAdcFifoNotEmptyR = crate::BitReader;
#[doc = "Field `AUX_ADC_FIFO_NOT_EMPTY` writer - 12:12\\]
AUX_ANAIF:ADCFIFOSTAT.EMPTY negated"]
pub type AuxAdcFifoNotEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_SMPH_AUTOTAKE_DONE` reader - 13:13\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
pub type AuxSmphAutotakeDoneR = crate::BitReader;
#[doc = "Field `AUX_SMPH_AUTOTAKE_DONE` writer - 13:13\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
pub type AuxSmphAutotakeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_DAC_HOLD_ACTIVE` reader - 14:14\\]
AUX_ANAIF:DACSTAT.HOLD_ACTIVE"]
pub type AuxDacHoldActiveR = crate::BitReader;
#[doc = "Field `AUX_DAC_HOLD_ACTIVE` writer - 14:14\\]
AUX_ANAIF:DACSTAT.HOLD_ACTIVE"]
pub type AuxDacHoldActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_CLKSWITCH_RDY` reader - 15:15\\]
AUX_SYSIF:TIMER2CLKSWITCH.RDY"]
pub type AuxTimer2ClkswitchRdyR = crate::BitReader;
#[doc = "Field `AUX_TIMER2_CLKSWITCH_RDY` writer - 15:15\\]
AUX_SYSIF:TIMER2CLKSWITCH.RDY"]
pub type AuxTimer2ClkswitchRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
AUX_TIMER2 event output 0. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_ev0(&self) -> AuxTimer2Ev0R {
        AuxTimer2Ev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AUX_TIMER2 event output 1. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_ev1(&self) -> AuxTimer2Ev1R {
        AuxTimer2Ev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AUX_TIMER2 event output 2. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_ev2(&self) -> AuxTimer2Ev2R {
        AuxTimer2Ev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUX_TIMER2 event output 3. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_ev3(&self) -> AuxTimer2Ev3R {
        AuxTimer2Ev3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUX_TIMER2 pulse event. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_pulse(&self) -> AuxTimer2PulseR {
        AuxTimer2PulseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER01:T1TARGET for description."]
    #[inline(always)]
    pub fn aux_timer1_ev(&self) -> AuxTimer1EvR {
        AuxTimer1EvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AUX_TIMER0_EV event, see AUX_TIMER01:T0TARGET for description."]
    #[inline(always)]
    pub fn aux_timer0_ev(&self) -> AuxTimer0EvR {
        AuxTimer0EvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUX_TDC:STAT.DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(&self) -> AuxTdcDoneR {
        AuxTdcDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUX_ANAIF:ISRCCTL.RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(&self) -> AuxIsrcResetNR {
        AuxIsrcResetNR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
AUX_ANAIF ADC conversion done event. Event is synchronized at AUX bus rate."]
    #[inline(always)]
    pub fn aux_adc_done(&self) -> AuxAdcDoneR {
        AuxAdcDoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline(always)]
    pub fn aux_adc_irq(&self) -> AuxAdcIrqR {
        AuxAdcIrqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(&self) -> AuxAdcFifoAlmostFullR {
        AuxAdcFifoAlmostFullR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AUX_ANAIF:ADCFIFOSTAT.EMPTY negated"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(&self) -> AuxAdcFifoNotEmptyR {
        AuxAdcFifoNotEmptyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline(always)]
    pub fn aux_smph_autotake_done(&self) -> AuxSmphAutotakeDoneR {
        AuxSmphAutotakeDoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
AUX_ANAIF:DACSTAT.HOLD_ACTIVE"]
    #[inline(always)]
    pub fn aux_dac_hold_active(&self) -> AuxDacHoldActiveR {
        AuxDacHoldActiveR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
AUX_SYSIF:TIMER2CLKSWITCH.RDY"]
    #[inline(always)]
    pub fn aux_timer2_clkswitch_rdy(&self) -> AuxTimer2ClkswitchRdyR {
        AuxTimer2ClkswitchRdyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
AUX_TIMER2 event output 0. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev0(&mut self) -> AuxTimer2Ev0W<Evstat3Spec> {
        AuxTimer2Ev0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AUX_TIMER2 event output 1. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev1(&mut self) -> AuxTimer2Ev1W<Evstat3Spec> {
        AuxTimer2Ev1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
AUX_TIMER2 event output 2. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev2(&mut self) -> AuxTimer2Ev2W<Evstat3Spec> {
        AuxTimer2Ev2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
AUX_TIMER2 event output 3. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev3(&mut self) -> AuxTimer2Ev3W<Evstat3Spec> {
        AuxTimer2Ev3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
AUX_TIMER2 pulse event. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_pulse(&mut self) -> AuxTimer2PulseW<Evstat3Spec> {
        AuxTimer2PulseW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER01:T1TARGET for description."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer1_ev(&mut self) -> AuxTimer1EvW<Evstat3Spec> {
        AuxTimer1EvW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
AUX_TIMER0_EV event, see AUX_TIMER01:T0TARGET for description."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer0_ev(&mut self) -> AuxTimer0EvW<Evstat3Spec> {
        AuxTimer0EvW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
AUX_TDC:STAT.DONE"]
    #[inline(always)]
    #[must_use]
    pub fn aux_tdc_done(&mut self) -> AuxTdcDoneW<Evstat3Spec> {
        AuxTdcDoneW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
AUX_ANAIF:ISRCCTL.RESET_N"]
    #[inline(always)]
    #[must_use]
    pub fn aux_isrc_reset_n(&mut self) -> AuxIsrcResetNW<Evstat3Spec> {
        AuxIsrcResetNW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
AUX_ANAIF ADC conversion done event. Event is synchronized at AUX bus rate."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_done(&mut self) -> AuxAdcDoneW<Evstat3Spec> {
        AuxAdcDoneW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_irq(&mut self) -> AuxAdcIrqW<Evstat3Spec> {
        AuxAdcIrqW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_fifo_almost_full(&mut self) -> AuxAdcFifoAlmostFullW<Evstat3Spec> {
        AuxAdcFifoAlmostFullW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
AUX_ANAIF:ADCFIFOSTAT.EMPTY negated"]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_fifo_not_empty(&mut self) -> AuxAdcFifoNotEmptyW<Evstat3Spec> {
        AuxAdcFifoNotEmptyW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline(always)]
    #[must_use]
    pub fn aux_smph_autotake_done(&mut self) -> AuxSmphAutotakeDoneW<Evstat3Spec> {
        AuxSmphAutotakeDoneW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
AUX_ANAIF:DACSTAT.HOLD_ACTIVE"]
    #[inline(always)]
    #[must_use]
    pub fn aux_dac_hold_active(&mut self) -> AuxDacHoldActiveW<Evstat3Spec> {
        AuxDacHoldActiveW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
AUX_SYSIF:TIMER2CLKSWITCH.RDY"]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_clkswitch_rdy(&mut self) -> AuxTimer2ClkswitchRdyW<Evstat3Spec> {
        AuxTimer2ClkswitchRdyW::new(self, 15)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Evstat3Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Event Status 3 Register holds events 48 thru 63 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC . - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Evstat3Spec;
impl crate::RegisterSpec for Evstat3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evstat3::R`](R) reader structure"]
impl crate::Readable for Evstat3Spec {}
#[doc = "`write(|w| ..)` method takes [`evstat3::W`](W) writer structure"]
impl crate::Writable for Evstat3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVSTAT3 to value 0"]
impl crate::Resettable for Evstat3Spec {
    const RESET_VALUE: u32 = 0;
}

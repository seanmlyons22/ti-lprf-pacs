#[doc = "Register `COMBEVTOMCUMASK` reader"]
pub type R = crate::R<CombevtomcumaskSpec>;
#[doc = "Register `COMBEVTOMCUMASK` writer"]
pub type W = crate::W<CombevtomcumaskSpec>;
#[doc = "Field `AUX_WU_EV` reader - 0:0\\]
EVTOMCUFLAGS.AUX_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxWuEvR = crate::BitReader;
#[doc = "Field `AUX_WU_EV` writer - 0:0\\]
EVTOMCUFLAGS.AUX_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxWuEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_COMPA` reader - 1:1\\]
EVTOMCUFLAGS.AUX_COMPA contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxCompaR = crate::BitReader;
#[doc = "Field `AUX_COMPA` writer - 1:1\\]
EVTOMCUFLAGS.AUX_COMPA contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxCompaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_COMPB` reader - 2:2\\]
EVTOMCUFLAGS.AUX_COMPB contribution to the AUX_COMB event. 0: Exclude 1: Include."]
pub type AuxCompbR = crate::BitReader;
#[doc = "Field `AUX_COMPB` writer - 2:2\\]
EVTOMCUFLAGS.AUX_COMPB contribution to the AUX_COMB event. 0: Exclude 1: Include."]
pub type AuxCompbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TDC_DONE` reader - 3:3\\]
EVTOMCUFLAGS.AUX_TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTdcDoneR = crate::BitReader;
#[doc = "Field `AUX_TDC_DONE` writer - 3:3\\]
EVTOMCUFLAGS.AUX_TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER0_EV` reader - 4:4\\]
EVTOMCUFLAGS.AUX_TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer0EvR = crate::BitReader;
#[doc = "Field `AUX_TIMER0_EV` writer - 4:4\\]
EVTOMCUFLAGS.AUX_TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer0EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER1_EV` reader - 5:5\\]
EVTOMCUFLAGS.AUX_TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer1EvR = crate::BitReader;
#[doc = "Field `AUX_TIMER1_EV` writer - 5:5\\]
EVTOMCUFLAGS.AUX_TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer1EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_SMPH_AUTOTAKE_DONE` reader - 6:6\\]
EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxSmphAutotakeDoneR = crate::BitReader;
#[doc = "Field `AUX_SMPH_AUTOTAKE_DONE` writer - 6:6\\]
EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxSmphAutotakeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_ADC_DONE` reader - 7:7\\]
EVTOMCUFLAGS.AUX_ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxAdcDoneR = crate::BitReader;
#[doc = "Field `AUX_ADC_DONE` writer - 7:7\\]
EVTOMCUFLAGS.AUX_ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxAdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_ADC_FIFO_ALMOST_FULL` reader - 8:8\\]
EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxAdcFifoAlmostFullR = crate::BitReader;
#[doc = "Field `AUX_ADC_FIFO_ALMOST_FULL` writer - 8:8\\]
EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxAdcFifoAlmostFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_OBSMUX0` reader - 9:9\\]
EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type McuObsmux0R = crate::BitReader;
#[doc = "Field `MCU_OBSMUX0` writer - 9:9\\]
EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type McuObsmux0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_ADC_IRQ` reader - 10:10\\]
EVTOMCUFLAGS.AUX_ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxAdcIrqR = crate::BitReader;
#[doc = "Field `AUX_ADC_IRQ` writer - 10:10\\]
EVTOMCUFLAGS.AUX_ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxAdcIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_EV0` reader - 11:11\\]
EVTOMCUFLAGS.AUX_TIMER2_EV0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer2Ev0R = crate::BitReader;
#[doc = "Field `AUX_TIMER2_EV0` writer - 11:11\\]
EVTOMCUFLAGS.AUX_TIMER2_EV0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer2Ev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_EV1` reader - 12:12\\]
EVTOMCUFLAGS.AUX_TIMER2_EV1 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer2Ev1R = crate::BitReader;
#[doc = "Field `AUX_TIMER2_EV1` writer - 12:12\\]
EVTOMCUFLAGS.AUX_TIMER2_EV1 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer2Ev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_EV2` reader - 13:13\\]
EVTOMCUFLAGS.AUX_TIMER2_EV2 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer2Ev2R = crate::BitReader;
#[doc = "Field `AUX_TIMER2_EV2` writer - 13:13\\]
EVTOMCUFLAGS.AUX_TIMER2_EV2 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer2Ev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_EV3` reader - 14:14\\]
EVTOMCUFLAGS.AUX_TIMER2_EV3 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer2Ev3R = crate::BitReader;
#[doc = "Field `AUX_TIMER2_EV3` writer - 14:14\\]
EVTOMCUFLAGS.AUX_TIMER2_EV3 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer2Ev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER2_PULSE` reader - 15:15\\]
EVTOMCUFLAGS.AUX_TIMER2_PULSE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer2PulseR = crate::BitReader;
#[doc = "Field `AUX_TIMER2_PULSE` writer - 15:15\\]
EVTOMCUFLAGS.AUX_TIMER2_PULSE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AuxTimer2PulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
EVTOMCUFLAGS.AUX_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_wu_ev(&self) -> AuxWuEvR {
        AuxWuEvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
EVTOMCUFLAGS.AUX_COMPA contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AuxCompaR {
        AuxCompaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
EVTOMCUFLAGS.AUX_COMPB contribution to the AUX_COMB event. 0: Exclude 1: Include."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AuxCompbR {
        AuxCompbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
EVTOMCUFLAGS.AUX_TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_tdc_done(&self) -> AuxTdcDoneR {
        AuxTdcDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
EVTOMCUFLAGS.AUX_TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_timer0_ev(&self) -> AuxTimer0EvR {
        AuxTimer0EvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
EVTOMCUFLAGS.AUX_TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_timer1_ev(&self) -> AuxTimer1EvR {
        AuxTimer1EvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_smph_autotake_done(&self) -> AuxSmphAutotakeDoneR {
        AuxSmphAutotakeDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
EVTOMCUFLAGS.AUX_ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_adc_done(&self) -> AuxAdcDoneR {
        AuxAdcDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(&self) -> AuxAdcFifoAlmostFullR {
        AuxAdcFifoAlmostFullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn mcu_obsmux0(&self) -> McuObsmux0R {
        McuObsmux0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
EVTOMCUFLAGS.AUX_ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_adc_irq(&self) -> AuxAdcIrqR {
        AuxAdcIrqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
EVTOMCUFLAGS.AUX_TIMER2_EV0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_timer2_ev0(&self) -> AuxTimer2Ev0R {
        AuxTimer2Ev0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
EVTOMCUFLAGS.AUX_TIMER2_EV1 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_timer2_ev1(&self) -> AuxTimer2Ev1R {
        AuxTimer2Ev1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
EVTOMCUFLAGS.AUX_TIMER2_EV2 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_timer2_ev2(&self) -> AuxTimer2Ev2R {
        AuxTimer2Ev2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
EVTOMCUFLAGS.AUX_TIMER2_EV3 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_timer2_ev3(&self) -> AuxTimer2Ev3R {
        AuxTimer2Ev3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
EVTOMCUFLAGS.AUX_TIMER2_PULSE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_timer2_pulse(&self) -> AuxTimer2PulseR {
        AuxTimer2PulseR::new(((self.bits >> 15) & 1) != 0)
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
EVTOMCUFLAGS.AUX_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_wu_ev(&mut self) -> AuxWuEvW<CombevtomcumaskSpec> {
        AuxWuEvW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
EVTOMCUFLAGS.AUX_COMPA contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AuxCompaW<CombevtomcumaskSpec> {
        AuxCompaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
EVTOMCUFLAGS.AUX_COMPB contribution to the AUX_COMB event. 0: Exclude 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AuxCompbW<CombevtomcumaskSpec> {
        AuxCompbW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
EVTOMCUFLAGS.AUX_TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_tdc_done(&mut self) -> AuxTdcDoneW<CombevtomcumaskSpec> {
        AuxTdcDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
EVTOMCUFLAGS.AUX_TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer0_ev(&mut self) -> AuxTimer0EvW<CombevtomcumaskSpec> {
        AuxTimer0EvW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
EVTOMCUFLAGS.AUX_TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer1_ev(&mut self) -> AuxTimer1EvW<CombevtomcumaskSpec> {
        AuxTimer1EvW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_smph_autotake_done(&mut self) -> AuxSmphAutotakeDoneW<CombevtomcumaskSpec> {
        AuxSmphAutotakeDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
EVTOMCUFLAGS.AUX_ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_done(&mut self) -> AuxAdcDoneW<CombevtomcumaskSpec> {
        AuxAdcDoneW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_fifo_almost_full(&mut self) -> AuxAdcFifoAlmostFullW<CombevtomcumaskSpec> {
        AuxAdcFifoAlmostFullW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsmux0(&mut self) -> McuObsmux0W<CombevtomcumaskSpec> {
        McuObsmux0W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
EVTOMCUFLAGS.AUX_ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_irq(&mut self) -> AuxAdcIrqW<CombevtomcumaskSpec> {
        AuxAdcIrqW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
EVTOMCUFLAGS.AUX_TIMER2_EV0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev0(&mut self) -> AuxTimer2Ev0W<CombevtomcumaskSpec> {
        AuxTimer2Ev0W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
EVTOMCUFLAGS.AUX_TIMER2_EV1 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev1(&mut self) -> AuxTimer2Ev1W<CombevtomcumaskSpec> {
        AuxTimer2Ev1W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
EVTOMCUFLAGS.AUX_TIMER2_EV2 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev2(&mut self) -> AuxTimer2Ev2W<CombevtomcumaskSpec> {
        AuxTimer2Ev2W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
EVTOMCUFLAGS.AUX_TIMER2_EV3 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev3(&mut self) -> AuxTimer2Ev3W<CombevtomcumaskSpec> {
        AuxTimer2Ev3W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
EVTOMCUFLAGS.AUX_TIMER2_PULSE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_pulse(&mut self) -> AuxTimer2PulseW<CombevtomcumaskSpec> {
        AuxTimer2PulseW::new(self, 15)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<CombevtomcumaskSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`combevtomcumask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`combevtomcumask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CombevtomcumaskSpec;
impl crate::RegisterSpec for CombevtomcumaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`combevtomcumask::R`](R) reader structure"]
impl crate::Readable for CombevtomcumaskSpec {}
#[doc = "`write(|w| ..)` method takes [`combevtomcumask::W`](W) writer structure"]
impl crate::Writable for CombevtomcumaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMBEVTOMCUMASK to value 0"]
impl crate::Resettable for CombevtomcumaskSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `COMBEVTOMCUMASK` reader"]
pub type R = crate::R<CombevtomcumaskSpec>;
#[doc = "Register `COMBEVTOMCUMASK` writer"]
pub type W = crate::W<CombevtomcumaskSpec>;
#[doc = "Field `AON_WU_EV` reader - 0:0\\]
EVTOMCUFLAGS.AON_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AonWuEvR = crate::BitReader;
#[doc = "Field `AON_WU_EV` writer - 0:0\\]
EVTOMCUFLAGS.AON_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AonWuEvW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `TDC_DONE` reader - 3:3\\]
EVTOMCUFLAGS.TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type TdcDoneR = crate::BitReader;
#[doc = "Field `TDC_DONE` writer - 3:3\\]
EVTOMCUFLAGS.TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type TdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_EV` reader - 4:4\\]
EVTOMCUFLAGS.TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type Timer0EvR = crate::BitReader;
#[doc = "Field `TIMER0_EV` writer - 4:4\\]
EVTOMCUFLAGS.TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type Timer0EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_EV` reader - 5:5\\]
EVTOMCUFLAGS.TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type Timer1EvR = crate::BitReader;
#[doc = "Field `TIMER1_EV` writer - 5:5\\]
EVTOMCUFLAGS.TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type Timer1EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPH_AUTOTAKE_DONE` reader - 6:6\\]
EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type SmphAutotakeDoneR = crate::BitReader;
#[doc = "Field `SMPH_AUTOTAKE_DONE` writer - 6:6\\]
EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type SmphAutotakeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_DONE` reader - 7:7\\]
EVTOMCUFLAGS.ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AdcDoneR = crate::BitReader;
#[doc = "Field `ADC_DONE` writer - 7:7\\]
EVTOMCUFLAGS.ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_FIFO_ALMOST_FULL` reader - 8:8\\]
EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AdcFifoAlmostFullR = crate::BitReader;
#[doc = "Field `ADC_FIFO_ALMOST_FULL` writer - 8:8\\]
EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AdcFifoAlmostFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBSMUX0` reader - 9:9\\]
EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type Obsmux0R = crate::BitReader;
#[doc = "Field `OBSMUX0` writer - 9:9\\]
EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type Obsmux0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_IRQ` reader - 10:10\\]
EVTOMCUFLAGS.ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AdcIrqR = crate::BitReader;
#[doc = "Field `ADC_IRQ` writer - 10:10\\]
EVTOMCUFLAGS.ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AdcIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
EVTOMCUFLAGS.AON_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aon_wu_ev(&self) -> AonWuEvR {
        AonWuEvR::new((self.bits & 1) != 0)
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
EVTOMCUFLAGS.TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn tdc_done(&self) -> TdcDoneR {
        TdcDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
EVTOMCUFLAGS.TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> Timer0EvR {
        Timer0EvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
EVTOMCUFLAGS.TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> Timer1EvR {
        Timer1EvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn smph_autotake_done(&self) -> SmphAutotakeDoneR {
        SmphAutotakeDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
EVTOMCUFLAGS.ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn adc_done(&self) -> AdcDoneR {
        AdcDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn adc_fifo_almost_full(&self) -> AdcFifoAlmostFullR {
        AdcFifoAlmostFullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn obsmux0(&self) -> Obsmux0R {
        Obsmux0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
EVTOMCUFLAGS.ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
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
EVTOMCUFLAGS.AON_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aon_wu_ev(&mut self) -> AonWuEvW<CombevtomcumaskSpec> {
        AonWuEvW::new(self, 0)
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
EVTOMCUFLAGS.TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn tdc_done(&mut self) -> TdcDoneW<CombevtomcumaskSpec> {
        TdcDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
EVTOMCUFLAGS.TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_ev(&mut self) -> Timer0EvW<CombevtomcumaskSpec> {
        Timer0EvW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
EVTOMCUFLAGS.TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ev(&mut self) -> Timer1EvW<CombevtomcumaskSpec> {
        Timer1EvW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn smph_autotake_done(&mut self) -> SmphAutotakeDoneW<CombevtomcumaskSpec> {
        SmphAutotakeDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
EVTOMCUFLAGS.ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn adc_done(&mut self) -> AdcDoneW<CombevtomcumaskSpec> {
        AdcDoneW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn adc_fifo_almost_full(&mut self) -> AdcFifoAlmostFullW<CombevtomcumaskSpec> {
        AdcFifoAlmostFullW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn obsmux0(&mut self) -> Obsmux0W<CombevtomcumaskSpec> {
        Obsmux0W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
EVTOMCUFLAGS.ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn adc_irq(&mut self) -> AdcIrqW<CombevtomcumaskSpec> {
        AdcIrqW::new(self, 10)
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

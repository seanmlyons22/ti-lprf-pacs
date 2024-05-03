#[doc = "Register `EVTOAONFLAGSCLR` reader"]
pub type R = crate::R<EvtoaonflagsclrSpec>;
#[doc = "Register `EVTOAONFLAGSCLR` writer"]
pub type W = crate::W<EvtoaonflagsclrSpec>;
#[doc = "Field `SWEV0` reader - 0:0\\]
Write 1 to clear EVTOAONFLAGS.SWEV0. Read value is 0."]
pub type Swev0R = crate::BitReader;
#[doc = "Field `SWEV0` writer - 0:0\\]
Write 1 to clear EVTOAONFLAGS.SWEV0. Read value is 0."]
pub type Swev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWEV1` reader - 1:1\\]
Write 1 to clear EVTOAONFLAGS.SWEV1. Read value is 0."]
pub type Swev1R = crate::BitReader;
#[doc = "Field `SWEV1` writer - 1:1\\]
Write 1 to clear EVTOAONFLAGS.SWEV1. Read value is 0."]
pub type Swev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWEV2` reader - 2:2\\]
Write 1 to clear EVTOAONFLAGS.SWEV2. Read value is 0."]
pub type Swev2R = crate::BitReader;
#[doc = "Field `SWEV2` writer - 2:2\\]
Write 1 to clear EVTOAONFLAGS.SWEV2. Read value is 0."]
pub type Swev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_COMPA` reader - 3:3\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPA. Read value is 0."]
pub type AuxCompaR = crate::BitReader;
#[doc = "Field `AUX_COMPA` writer - 3:3\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPA. Read value is 0."]
pub type AuxCompaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_COMPB` reader - 4:4\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPB. Read value is 0."]
pub type AuxCompbR = crate::BitReader;
#[doc = "Field `AUX_COMPB` writer - 4:4\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPB. Read value is 0."]
pub type AuxCompbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_ADC_DONE` reader - 5:5\\]
Write 1 to clear EVTOAONFLAGS.AUX_ADC_DONE. Read value is 0."]
pub type AuxAdcDoneR = crate::BitReader;
#[doc = "Field `AUX_ADC_DONE` writer - 5:5\\]
Write 1 to clear EVTOAONFLAGS.AUX_ADC_DONE. Read value is 0."]
pub type AuxAdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TDC_DONE` reader - 6:6\\]
Write 1 to clear EVTOAONFLAGS.AUX_TDC_DONE. Read value is 0."]
pub type AuxTdcDoneR = crate::BitReader;
#[doc = "Field `AUX_TDC_DONE` writer - 6:6\\]
Write 1 to clear EVTOAONFLAGS.AUX_TDC_DONE. Read value is 0."]
pub type AuxTdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER0_EV` reader - 7:7\\]
Write 1 to clear EVTOAONFLAGS.AUX_TIMER0_EV. Read value is 0."]
pub type AuxTimer0EvR = crate::BitReader;
#[doc = "Field `AUX_TIMER0_EV` writer - 7:7\\]
Write 1 to clear EVTOAONFLAGS.AUX_TIMER0_EV. Read value is 0."]
pub type AuxTimer0EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_TIMER1_EV` reader - 8:8\\]
Write 1 to clear EVTOAONFLAGS.AUX_TIMER1_EV. Read value is 0."]
pub type AuxTimer1EvR = crate::BitReader;
#[doc = "Field `AUX_TIMER1_EV` writer - 8:8\\]
Write 1 to clear EVTOAONFLAGS.AUX_TIMER1_EV. Read value is 0."]
pub type AuxTimer1EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write 1 to clear EVTOAONFLAGS.SWEV0. Read value is 0."]
    #[inline(always)]
    pub fn swev0(&self) -> Swev0R {
        Swev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to clear EVTOAONFLAGS.SWEV1. Read value is 0."]
    #[inline(always)]
    pub fn swev1(&self) -> Swev1R {
        Swev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Write 1 to clear EVTOAONFLAGS.SWEV2. Read value is 0."]
    #[inline(always)]
    pub fn swev2(&self) -> Swev2R {
        Swev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPA. Read value is 0."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AuxCompaR {
        AuxCompaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPB. Read value is 0."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AuxCompbR {
        AuxCompbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Write 1 to clear EVTOAONFLAGS.AUX_ADC_DONE. Read value is 0."]
    #[inline(always)]
    pub fn aux_adc_done(&self) -> AuxAdcDoneR {
        AuxAdcDoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Write 1 to clear EVTOAONFLAGS.AUX_TDC_DONE. Read value is 0."]
    #[inline(always)]
    pub fn aux_tdc_done(&self) -> AuxTdcDoneR {
        AuxTdcDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Write 1 to clear EVTOAONFLAGS.AUX_TIMER0_EV. Read value is 0."]
    #[inline(always)]
    pub fn aux_timer0_ev(&self) -> AuxTimer0EvR {
        AuxTimer0EvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Write 1 to clear EVTOAONFLAGS.AUX_TIMER1_EV. Read value is 0."]
    #[inline(always)]
    pub fn aux_timer1_ev(&self) -> AuxTimer1EvR {
        AuxTimer1EvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write 1 to clear EVTOAONFLAGS.SWEV0. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn swev0(&mut self) -> Swev0W<EvtoaonflagsclrSpec> {
        Swev0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to clear EVTOAONFLAGS.SWEV1. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn swev1(&mut self) -> Swev1W<EvtoaonflagsclrSpec> {
        Swev1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Write 1 to clear EVTOAONFLAGS.SWEV2. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn swev2(&mut self) -> Swev2W<EvtoaonflagsclrSpec> {
        Swev2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPA. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AuxCompaW<EvtoaonflagsclrSpec> {
        AuxCompaW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPB. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AuxCompbW<EvtoaonflagsclrSpec> {
        AuxCompbW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Write 1 to clear EVTOAONFLAGS.AUX_ADC_DONE. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_done(&mut self) -> AuxAdcDoneW<EvtoaonflagsclrSpec> {
        AuxAdcDoneW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Write 1 to clear EVTOAONFLAGS.AUX_TDC_DONE. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_tdc_done(&mut self) -> AuxTdcDoneW<EvtoaonflagsclrSpec> {
        AuxTdcDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Write 1 to clear EVTOAONFLAGS.AUX_TIMER0_EV. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer0_ev(&mut self) -> AuxTimer0EvW<EvtoaonflagsclrSpec> {
        AuxTimer0EvW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Write 1 to clear EVTOAONFLAGS.AUX_TIMER1_EV. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer1_ev(&mut self) -> AuxTimer1EvW<EvtoaonflagsclrSpec> {
        AuxTimer1EvW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<EvtoaonflagsclrSpec> {
        Reserved9W::new(self, 9)
    }
}
#[doc = "Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtoaonflagsclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtoaonflagsclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtoaonflagsclrSpec;
impl crate::RegisterSpec for EvtoaonflagsclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtoaonflagsclr::R`](R) reader structure"]
impl crate::Readable for EvtoaonflagsclrSpec {}
#[doc = "`write(|w| ..)` method takes [`evtoaonflagsclr::W`](W) writer structure"]
impl crate::Writable for EvtoaonflagsclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVTOAONFLAGSCLR to value 0"]
impl crate::Resettable for EvtoaonflagsclrSpec {
    const RESET_VALUE: u32 = 0;
}

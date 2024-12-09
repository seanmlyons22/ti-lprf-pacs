#[doc = "Register `EVTOAONPOL` reader"]
pub type R = crate::R<EvtoaonpolSpec>;
#[doc = "Register `EVTOAONPOL` writer"]
pub type W = crate::W<EvtoaonpolSpec>;
#[doc = "Field `RESERVED2` reader - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "3:3\\]
Select the edge of EVSTAT2.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxCompa {
    #[doc = "1: Falling edge"]
    Fall = 1,
    #[doc = "0: Rising edge"]
    Rise = 0,
}
impl From<AuxCompa> for bool {
    #[inline(always)]
    fn from(variant: AuxCompa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPA` reader - 3:3\\]
Select the edge of EVSTAT2.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
pub type AuxCompaR = crate::BitReader<AuxCompa>;
impl AuxCompaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxCompa {
        match self.bits {
            true => AuxCompa::Fall,
            false => AuxCompa::Rise,
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == AuxCompa::Fall
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == AuxCompa::Rise
    }
}
#[doc = "Field `AUX_COMPA` writer - 3:3\\]
Select the edge of EVSTAT2.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
pub type AuxCompaW<'a, REG> = crate::BitWriter<'a, REG, AuxCompa>;
impl<'a, REG> AuxCompaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompa::Fall)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompa::Rise)
    }
}
#[doc = "4:4\\]
Select the edge of EVSTAT2.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxCompb {
    #[doc = "1: Falling edge"]
    Fall = 1,
    #[doc = "0: Rising edge"]
    Rise = 0,
}
impl From<AuxCompb> for bool {
    #[inline(always)]
    fn from(variant: AuxCompb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPB` reader - 4:4\\]
Select the edge of EVSTAT2.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
pub type AuxCompbR = crate::BitReader<AuxCompb>;
impl AuxCompbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxCompb {
        match self.bits {
            true => AuxCompb::Fall,
            false => AuxCompb::Rise,
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == AuxCompb::Fall
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == AuxCompb::Rise
    }
}
#[doc = "Field `AUX_COMPB` writer - 4:4\\]
Select the edge of EVSTAT2.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
pub type AuxCompbW<'a, REG> = crate::BitWriter<'a, REG, AuxCompb>;
impl<'a, REG> AuxCompbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompb::Fall)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompb::Rise)
    }
}
#[doc = "5:5\\]
Select the level of EVSTAT3.AUX_ADC_DONE that sets EVTOAONFLAGS.AUX_ADC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxAdcDone {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AuxAdcDone> for bool {
    #[inline(always)]
    fn from(variant: AuxAdcDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_ADC_DONE` reader - 5:5\\]
Select the level of EVSTAT3.AUX_ADC_DONE that sets EVTOAONFLAGS.AUX_ADC_DONE."]
pub type AuxAdcDoneR = crate::BitReader<AuxAdcDone>;
impl AuxAdcDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxAdcDone {
        match self.bits {
            true => AuxAdcDone::Low,
            false => AuxAdcDone::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxAdcDone::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxAdcDone::High
    }
}
#[doc = "Field `AUX_ADC_DONE` writer - 5:5\\]
Select the level of EVSTAT3.AUX_ADC_DONE that sets EVTOAONFLAGS.AUX_ADC_DONE."]
pub type AuxAdcDoneW<'a, REG> = crate::BitWriter<'a, REG, AuxAdcDone>;
impl<'a, REG> AuxAdcDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxAdcDone::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxAdcDone::High)
    }
}
#[doc = "6:6\\]
Select level of EVSTAT3.AUX_TDC_DONE that sets EVTOAONFLAGS.AUX_TDC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxTdcDone {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AuxTdcDone> for bool {
    #[inline(always)]
    fn from(variant: AuxTdcDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_TDC_DONE` reader - 6:6\\]
Select level of EVSTAT3.AUX_TDC_DONE that sets EVTOAONFLAGS.AUX_TDC_DONE."]
pub type AuxTdcDoneR = crate::BitReader<AuxTdcDone>;
impl AuxTdcDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxTdcDone {
        match self.bits {
            true => AuxTdcDone::Low,
            false => AuxTdcDone::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxTdcDone::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxTdcDone::High
    }
}
#[doc = "Field `AUX_TDC_DONE` writer - 6:6\\]
Select level of EVSTAT3.AUX_TDC_DONE that sets EVTOAONFLAGS.AUX_TDC_DONE."]
pub type AuxTdcDoneW<'a, REG> = crate::BitWriter<'a, REG, AuxTdcDone>;
impl<'a, REG> AuxTdcDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTdcDone::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTdcDone::High)
    }
}
#[doc = "7:7\\]
Select the level of EVSTAT3.AUX_TIMER0_EV that sets EVTOAONFLAGS.AUX_TIMER0_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxTimer0Ev {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AuxTimer0Ev> for bool {
    #[inline(always)]
    fn from(variant: AuxTimer0Ev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_TIMER0_EV` reader - 7:7\\]
Select the level of EVSTAT3.AUX_TIMER0_EV that sets EVTOAONFLAGS.AUX_TIMER0_EV."]
pub type AuxTimer0EvR = crate::BitReader<AuxTimer0Ev>;
impl AuxTimer0EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxTimer0Ev {
        match self.bits {
            true => AuxTimer0Ev::Low,
            false => AuxTimer0Ev::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxTimer0Ev::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxTimer0Ev::High
    }
}
#[doc = "Field `AUX_TIMER0_EV` writer - 7:7\\]
Select the level of EVSTAT3.AUX_TIMER0_EV that sets EVTOAONFLAGS.AUX_TIMER0_EV."]
pub type AuxTimer0EvW<'a, REG> = crate::BitWriter<'a, REG, AuxTimer0Ev>;
impl<'a, REG> AuxTimer0EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTimer0Ev::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTimer0Ev::High)
    }
}
#[doc = "8:8\\]
Select the level of EVSTAT3.AUX_TIMER1_EV that sets EVTOAONFLAGS.AUX_TIMER1_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxTimer1Ev {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AuxTimer1Ev> for bool {
    #[inline(always)]
    fn from(variant: AuxTimer1Ev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_TIMER1_EV` reader - 8:8\\]
Select the level of EVSTAT3.AUX_TIMER1_EV that sets EVTOAONFLAGS.AUX_TIMER1_EV."]
pub type AuxTimer1EvR = crate::BitReader<AuxTimer1Ev>;
impl AuxTimer1EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxTimer1Ev {
        match self.bits {
            true => AuxTimer1Ev::Low,
            false => AuxTimer1Ev::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxTimer1Ev::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxTimer1Ev::High
    }
}
#[doc = "Field `AUX_TIMER1_EV` writer - 8:8\\]
Select the level of EVSTAT3.AUX_TIMER1_EV that sets EVTOAONFLAGS.AUX_TIMER1_EV."]
pub type AuxTimer1EvW<'a, REG> = crate::BitWriter<'a, REG, AuxTimer1Ev>;
impl<'a, REG> AuxTimer1EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTimer1Ev::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTimer1Ev::High)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Select the edge of EVSTAT2.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AuxCompaR {
        AuxCompaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the edge of EVSTAT2.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AuxCompbR {
        AuxCompbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the level of EVSTAT3.AUX_ADC_DONE that sets EVTOAONFLAGS.AUX_ADC_DONE."]
    #[inline(always)]
    pub fn aux_adc_done(&self) -> AuxAdcDoneR {
        AuxAdcDoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select level of EVSTAT3.AUX_TDC_DONE that sets EVTOAONFLAGS.AUX_TDC_DONE."]
    #[inline(always)]
    pub fn aux_tdc_done(&self) -> AuxTdcDoneR {
        AuxTdcDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the level of EVSTAT3.AUX_TIMER0_EV that sets EVTOAONFLAGS.AUX_TIMER0_EV."]
    #[inline(always)]
    pub fn aux_timer0_ev(&self) -> AuxTimer0EvR {
        AuxTimer0EvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the level of EVSTAT3.AUX_TIMER1_EV that sets EVTOAONFLAGS.AUX_TIMER1_EV."]
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
    #[doc = "Bit 3 - 3:3\\]
Select the edge of EVSTAT2.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AuxCompaW<EvtoaonpolSpec> {
        AuxCompaW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the edge of EVSTAT2.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AuxCompbW<EvtoaonpolSpec> {
        AuxCompbW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the level of EVSTAT3.AUX_ADC_DONE that sets EVTOAONFLAGS.AUX_ADC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_done(&mut self) -> AuxAdcDoneW<EvtoaonpolSpec> {
        AuxAdcDoneW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Select level of EVSTAT3.AUX_TDC_DONE that sets EVTOAONFLAGS.AUX_TDC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn aux_tdc_done(&mut self) -> AuxTdcDoneW<EvtoaonpolSpec> {
        AuxTdcDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the level of EVSTAT3.AUX_TIMER0_EV that sets EVTOAONFLAGS.AUX_TIMER0_EV."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer0_ev(&mut self) -> AuxTimer0EvW<EvtoaonpolSpec> {
        AuxTimer0EvW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the level of EVSTAT3.AUX_TIMER1_EV that sets EVTOAONFLAGS.AUX_TIMER1_EV."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer1_ev(&mut self) -> AuxTimer1EvW<EvtoaonpolSpec> {
        AuxTimer1EvW::new(self, 8)
    }
}
#[doc = "Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtoaonpol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtoaonpol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtoaonpolSpec;
impl crate::RegisterSpec for EvtoaonpolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtoaonpol::R`](R) reader structure"]
impl crate::Readable for EvtoaonpolSpec {}
#[doc = "`write(|w| ..)` method takes [`evtoaonpol::W`](W) writer structure"]
impl crate::Writable for EvtoaonpolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVTOAONPOL to value 0"]
impl crate::Resettable for EvtoaonpolSpec {
    const RESET_VALUE: u32 = 0;
}

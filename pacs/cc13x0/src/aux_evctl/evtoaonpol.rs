#[doc = "Register `EVTOAONPOL` reader"]
pub type R = crate::R<EvtoaonpolSpec>;
#[doc = "Register `EVTOAONPOL` writer"]
pub type W = crate::W<EvtoaonpolSpec>;
#[doc = "Field `RESERVED2` reader - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "3:3\\]
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxCompa {
    #[doc = "1: Falling edge"]
    Low = 1,
    #[doc = "0: Rising edge"]
    High = 0,
}
impl From<AuxCompa> for bool {
    #[inline(always)]
    fn from(variant: AuxCompa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPA` reader - 3:3\\]
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
pub type AuxCompaR = crate::BitReader<AuxCompa>;
impl AuxCompaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxCompa {
        match self.bits {
            true => AuxCompa::Low,
            false => AuxCompa::High,
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxCompa::Low
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxCompa::High
    }
}
#[doc = "Field `AUX_COMPA` writer - 3:3\\]
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
pub type AuxCompaW<'a, REG> = crate::BitWriter<'a, REG, AuxCompa>;
impl<'a, REG> AuxCompaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompa::Low)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompa::High)
    }
}
#[doc = "4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxCompb {
    #[doc = "1: Falling edge"]
    Low = 1,
    #[doc = "0: Rising edge"]
    High = 0,
}
impl From<AuxCompb> for bool {
    #[inline(always)]
    fn from(variant: AuxCompb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPB` reader - 4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
pub type AuxCompbR = crate::BitReader<AuxCompb>;
impl AuxCompbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxCompb {
        match self.bits {
            true => AuxCompb::Low,
            false => AuxCompb::High,
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxCompb::Low
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxCompb::High
    }
}
#[doc = "Field `AUX_COMPB` writer - 4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
pub type AuxCompbW<'a, REG> = crate::BitWriter<'a, REG, AuxCompb>;
impl<'a, REG> AuxCompbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompb::Low)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompb::High)
    }
}
#[doc = "5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcDone {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AdcDone> for bool {
    #[inline(always)]
    fn from(variant: AdcDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_DONE` reader - 5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
pub type AdcDoneR = crate::BitReader<AdcDone>;
impl AdcDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcDone {
        match self.bits {
            true => AdcDone::Low,
            false => AdcDone::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AdcDone::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AdcDone::High
    }
}
#[doc = "Field `ADC_DONE` writer - 5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
pub type AdcDoneW<'a, REG> = crate::BitWriter<'a, REG, AdcDone>;
impl<'a, REG> AdcDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AdcDone::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AdcDone::High)
    }
}
#[doc = "6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdcDone {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<TdcDone> for bool {
    #[inline(always)]
    fn from(variant: TdcDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC_DONE` reader - 6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
pub type TdcDoneR = crate::BitReader<TdcDone>;
impl TdcDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TdcDone {
        match self.bits {
            true => TdcDone::Low,
            false => TdcDone::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TdcDone::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TdcDone::High
    }
}
#[doc = "Field `TDC_DONE` writer - 6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
pub type TdcDoneW<'a, REG> = crate::BitWriter<'a, REG, TdcDone>;
impl<'a, REG> TdcDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(TdcDone::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(TdcDone::High)
    }
}
#[doc = "7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0Ev {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<Timer0Ev> for bool {
    #[inline(always)]
    fn from(variant: Timer0Ev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER0_EV` reader - 7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
pub type Timer0EvR = crate::BitReader<Timer0Ev>;
impl Timer0EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0Ev {
        match self.bits {
            true => Timer0Ev::Low,
            false => Timer0Ev::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Timer0Ev::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Timer0Ev::High
    }
}
#[doc = "Field `TIMER0_EV` writer - 7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
pub type Timer0EvW<'a, REG> = crate::BitWriter<'a, REG, Timer0Ev>;
impl<'a, REG> Timer0EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0Ev::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0Ev::High)
    }
}
#[doc = "8:8\\]
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer1Ev {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<Timer1Ev> for bool {
    #[inline(always)]
    fn from(variant: Timer1Ev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER1_EV` reader - 8:8\\]
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
pub type Timer1EvR = crate::BitReader<Timer1Ev>;
impl Timer1EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer1Ev {
        match self.bits {
            true => Timer1Ev::Low,
            false => Timer1Ev::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Timer1Ev::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Timer1Ev::High
    }
}
#[doc = "Field `TIMER1_EV` writer - 8:8\\]
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
pub type Timer1EvW<'a, REG> = crate::BitWriter<'a, REG, Timer1Ev>;
impl<'a, REG> Timer1EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1Ev::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1Ev::High)
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
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AuxCompaR {
        AuxCompaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AuxCompbR {
        AuxCompbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
    #[inline(always)]
    pub fn adc_done(&self) -> AdcDoneR {
        AdcDoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
    #[inline(always)]
    pub fn tdc_done(&self) -> TdcDoneR {
        TdcDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> Timer0EvR {
        Timer0EvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> Timer1EvR {
        Timer1EvR::new(((self.bits >> 8) & 1) != 0)
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
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AuxCompaW<EvtoaonpolSpec> {
        AuxCompaW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AuxCompbW<EvtoaonpolSpec> {
        AuxCompbW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn adc_done(&mut self) -> AdcDoneW<EvtoaonpolSpec> {
        AdcDoneW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn tdc_done(&mut self) -> TdcDoneW<EvtoaonpolSpec> {
        TdcDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_ev(&mut self) -> Timer0EvW<EvtoaonpolSpec> {
        Timer0EvW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ev(&mut self) -> Timer1EvW<EvtoaonpolSpec> {
        Timer1EvW::new(self, 8)
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

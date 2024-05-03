#[doc = "Register `MODCLKEN0` reader"]
pub type R = crate::R<Modclken0Spec>;
#[doc = "Register `MODCLKEN0` writer"]
pub type W = crate::W<Modclken0Spec>;
#[doc = "0:0\\]
Enables (1) or disables (0) clock for AUX_SMPH.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smph {
    #[doc = "1: System CPU has requested clock for SMPH"]
    En = 1,
    #[doc = "0: System CPU has not requested clock for SMPH"]
    Dis = 0,
}
impl From<Smph> for bool {
    #[inline(always)]
    fn from(variant: Smph) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPH` reader - 0:0\\]
Enables (1) or disables (0) clock for AUX_SMPH."]
pub type SmphR = crate::BitReader<Smph>;
impl SmphR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smph {
        match self.bits {
            true => Smph::En,
            false => Smph::Dis,
        }
    }
    #[doc = "System CPU has requested clock for SMPH"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Smph::En
    }
    #[doc = "System CPU has not requested clock for SMPH"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Smph::Dis
    }
}
#[doc = "Field `SMPH` writer - 0:0\\]
Enables (1) or disables (0) clock for AUX_SMPH."]
pub type SmphW<'a, REG> = crate::BitWriter<'a, REG, Smph>;
impl<'a, REG> SmphW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "System CPU has requested clock for SMPH"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Smph::En)
    }
    #[doc = "System CPU has not requested clock for SMPH"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Smph::Dis)
    }
}
#[doc = "1:1\\]
Enables (1) or disables (0) clock for AUX_AIODIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aiodio0 {
    #[doc = "1: System CPU has requested clock for AIODIO0"]
    En = 1,
    #[doc = "0: System CPU has not requested clock for AIODIO0"]
    Dis = 0,
}
impl From<Aiodio0> for bool {
    #[inline(always)]
    fn from(variant: Aiodio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIODIO0` reader - 1:1\\]
Enables (1) or disables (0) clock for AUX_AIODIO0."]
pub type Aiodio0R = crate::BitReader<Aiodio0>;
impl Aiodio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aiodio0 {
        match self.bits {
            true => Aiodio0::En,
            false => Aiodio0::Dis,
        }
    }
    #[doc = "System CPU has requested clock for AIODIO0"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Aiodio0::En
    }
    #[doc = "System CPU has not requested clock for AIODIO0"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Aiodio0::Dis
    }
}
#[doc = "Field `AIODIO0` writer - 1:1\\]
Enables (1) or disables (0) clock for AUX_AIODIO0."]
pub type Aiodio0W<'a, REG> = crate::BitWriter<'a, REG, Aiodio0>;
impl<'a, REG> Aiodio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "System CPU has requested clock for AIODIO0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Aiodio0::En)
    }
    #[doc = "System CPU has not requested clock for AIODIO0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Aiodio0::Dis)
    }
}
#[doc = "2:2\\]
Enables (1) or disables (0) clock for AUX_AIODIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aiodio1 {
    #[doc = "1: System CPU has requested clock for AIODIO1"]
    En = 1,
    #[doc = "0: System CPU has not requested clock for AIODIO1"]
    Dis = 0,
}
impl From<Aiodio1> for bool {
    #[inline(always)]
    fn from(variant: Aiodio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIODIO1` reader - 2:2\\]
Enables (1) or disables (0) clock for AUX_AIODIO1."]
pub type Aiodio1R = crate::BitReader<Aiodio1>;
impl Aiodio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aiodio1 {
        match self.bits {
            true => Aiodio1::En,
            false => Aiodio1::Dis,
        }
    }
    #[doc = "System CPU has requested clock for AIODIO1"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Aiodio1::En
    }
    #[doc = "System CPU has not requested clock for AIODIO1"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Aiodio1::Dis
    }
}
#[doc = "Field `AIODIO1` writer - 2:2\\]
Enables (1) or disables (0) clock for AUX_AIODIO1."]
pub type Aiodio1W<'a, REG> = crate::BitWriter<'a, REG, Aiodio1>;
impl<'a, REG> Aiodio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "System CPU has requested clock for AIODIO1"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Aiodio1::En)
    }
    #[doc = "System CPU has not requested clock for AIODIO1"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Aiodio1::Dis)
    }
}
#[doc = "3:3\\]
Enables (1) or disables (0) clock for AUX_TIMER.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer {
    #[doc = "1: System CPU has requested clock for TIMER"]
    En = 1,
    #[doc = "0: System CPU has not requested clock for TIMER"]
    Dis = 0,
}
impl From<Timer> for bool {
    #[inline(always)]
    fn from(variant: Timer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER` reader - 3:3\\]
Enables (1) or disables (0) clock for AUX_TIMER."]
pub type TimerR = crate::BitReader<Timer>;
impl TimerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer {
        match self.bits {
            true => Timer::En,
            false => Timer::Dis,
        }
    }
    #[doc = "System CPU has requested clock for TIMER"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Timer::En
    }
    #[doc = "System CPU has not requested clock for TIMER"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Timer::Dis
    }
}
#[doc = "Field `TIMER` writer - 3:3\\]
Enables (1) or disables (0) clock for AUX_TIMER."]
pub type TimerW<'a, REG> = crate::BitWriter<'a, REG, Timer>;
impl<'a, REG> TimerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "System CPU has requested clock for TIMER"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Timer::En)
    }
    #[doc = "System CPU has not requested clock for TIMER"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Timer::Dis)
    }
}
#[doc = "4:4\\]
Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anaif {
    #[doc = "1: System CPU has requested clock for ANAIF"]
    En = 1,
    #[doc = "0: System CPU has not requested clock for ANAIF"]
    Dis = 0,
}
impl From<Anaif> for bool {
    #[inline(always)]
    fn from(variant: Anaif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANAIF` reader - 4:4\\]
Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL."]
pub type AnaifR = crate::BitReader<Anaif>;
impl AnaifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anaif {
        match self.bits {
            true => Anaif::En,
            false => Anaif::Dis,
        }
    }
    #[doc = "System CPU has requested clock for ANAIF"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Anaif::En
    }
    #[doc = "System CPU has not requested clock for ANAIF"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Anaif::Dis
    }
}
#[doc = "Field `ANAIF` writer - 4:4\\]
Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL."]
pub type AnaifW<'a, REG> = crate::BitWriter<'a, REG, Anaif>;
impl<'a, REG> AnaifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "System CPU has requested clock for ANAIF"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Anaif::En)
    }
    #[doc = "System CPU has not requested clock for ANAIF"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Anaif::Dis)
    }
}
#[doc = "5:5\\]
Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdc {
    #[doc = "1: System CPU has requested clock for TDC"]
    En = 1,
    #[doc = "0: System CPU has not requested clock for TDC"]
    Dis = 0,
}
impl From<Tdc> for bool {
    #[inline(always)]
    fn from(variant: Tdc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC` reader - 5:5\\]
Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively."]
pub type TdcR = crate::BitReader<Tdc>;
impl TdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdc {
        match self.bits {
            true => Tdc::En,
            false => Tdc::Dis,
        }
    }
    #[doc = "System CPU has requested clock for TDC"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tdc::En
    }
    #[doc = "System CPU has not requested clock for TDC"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tdc::Dis
    }
}
#[doc = "Field `TDC` writer - 5:5\\]
Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively."]
pub type TdcW<'a, REG> = crate::BitWriter<'a, REG, Tdc>;
impl<'a, REG> TdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "System CPU has requested clock for TDC"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tdc::En)
    }
    #[doc = "System CPU has not requested clock for TDC"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tdc::Dis)
    }
}
#[doc = "6:6\\]
Enables (1) or disables (0) clock for AUX_DDI0_OSC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxDdi0Osc {
    #[doc = "1: System CPU has requested clock for AUX_DDI0_OSC"]
    En = 1,
    #[doc = "0: System CPU has not requested clock for AUX_DDI0_OSC"]
    Dis = 0,
}
impl From<AuxDdi0Osc> for bool {
    #[inline(always)]
    fn from(variant: AuxDdi0Osc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_DDI0_OSC` reader - 6:6\\]
Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
pub type AuxDdi0OscR = crate::BitReader<AuxDdi0Osc>;
impl AuxDdi0OscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxDdi0Osc {
        match self.bits {
            true => AuxDdi0Osc::En,
            false => AuxDdi0Osc::Dis,
        }
    }
    #[doc = "System CPU has requested clock for AUX_DDI0_OSC"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == AuxDdi0Osc::En
    }
    #[doc = "System CPU has not requested clock for AUX_DDI0_OSC"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == AuxDdi0Osc::Dis
    }
}
#[doc = "Field `AUX_DDI0_OSC` writer - 6:6\\]
Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
pub type AuxDdi0OscW<'a, REG> = crate::BitWriter<'a, REG, AuxDdi0Osc>;
impl<'a, REG> AuxDdi0OscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "System CPU has requested clock for AUX_DDI0_OSC"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(AuxDdi0Osc::En)
    }
    #[doc = "System CPU has not requested clock for AUX_DDI0_OSC"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(AuxDdi0Osc::Dis)
    }
}
#[doc = "7:7\\]
Enables (1) or disables (0) clock for AUX_ADI4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxAdi4 {
    #[doc = "1: System CPU has requested clock for AUX_ADI4"]
    En = 1,
    #[doc = "0: System CPU has not requested clock for AUX_ADI4"]
    Dis = 0,
}
impl From<AuxAdi4> for bool {
    #[inline(always)]
    fn from(variant: AuxAdi4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_ADI4` reader - 7:7\\]
Enables (1) or disables (0) clock for AUX_ADI4."]
pub type AuxAdi4R = crate::BitReader<AuxAdi4>;
impl AuxAdi4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxAdi4 {
        match self.bits {
            true => AuxAdi4::En,
            false => AuxAdi4::Dis,
        }
    }
    #[doc = "System CPU has requested clock for AUX_ADI4"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == AuxAdi4::En
    }
    #[doc = "System CPU has not requested clock for AUX_ADI4"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == AuxAdi4::Dis
    }
}
#[doc = "Field `AUX_ADI4` writer - 7:7\\]
Enables (1) or disables (0) clock for AUX_ADI4."]
pub type AuxAdi4W<'a, REG> = crate::BitWriter<'a, REG, AuxAdi4>;
impl<'a, REG> AuxAdi4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "System CPU has requested clock for AUX_ADI4"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(AuxAdi4::En)
    }
    #[doc = "System CPU has not requested clock for AUX_ADI4"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(AuxAdi4::Dis)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables (1) or disables (0) clock for AUX_SMPH."]
    #[inline(always)]
    pub fn smph(&self) -> SmphR {
        SmphR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables (1) or disables (0) clock for AUX_AIODIO0."]
    #[inline(always)]
    pub fn aiodio0(&self) -> Aiodio0R {
        Aiodio0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables (1) or disables (0) clock for AUX_AIODIO1."]
    #[inline(always)]
    pub fn aiodio1(&self) -> Aiodio1R {
        Aiodio1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables (1) or disables (0) clock for AUX_TIMER."]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL."]
    #[inline(always)]
    pub fn anaif(&self) -> AnaifR {
        AnaifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively."]
    #[inline(always)]
    pub fn tdc(&self) -> TdcR {
        TdcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
    #[inline(always)]
    pub fn aux_ddi0_osc(&self) -> AuxDdi0OscR {
        AuxDdi0OscR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables (1) or disables (0) clock for AUX_ADI4."]
    #[inline(always)]
    pub fn aux_adi4(&self) -> AuxAdi4R {
        AuxAdi4R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables (1) or disables (0) clock for AUX_SMPH."]
    #[inline(always)]
    #[must_use]
    pub fn smph(&mut self) -> SmphW<Modclken0Spec> {
        SmphW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables (1) or disables (0) clock for AUX_AIODIO0."]
    #[inline(always)]
    #[must_use]
    pub fn aiodio0(&mut self) -> Aiodio0W<Modclken0Spec> {
        Aiodio0W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables (1) or disables (0) clock for AUX_AIODIO1."]
    #[inline(always)]
    #[must_use]
    pub fn aiodio1(&mut self) -> Aiodio1W<Modclken0Spec> {
        Aiodio1W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables (1) or disables (0) clock for AUX_TIMER."]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TimerW<Modclken0Spec> {
        TimerW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL."]
    #[inline(always)]
    #[must_use]
    pub fn anaif(&mut self) -> AnaifW<Modclken0Spec> {
        AnaifW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively."]
    #[inline(always)]
    #[must_use]
    pub fn tdc(&mut self) -> TdcW<Modclken0Spec> {
        TdcW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
    #[inline(always)]
    #[must_use]
    pub fn aux_ddi0_osc(&mut self) -> AuxDdi0OscW<Modclken0Spec> {
        AuxDdi0OscW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables (1) or disables (0) clock for AUX_ADI4."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adi4(&mut self) -> AuxAdi4W<Modclken0Spec> {
        AuxAdi4W::new(self, 7)
    }
}
#[doc = "Module Clock Enable Clock enable for each module in the AUX domain For use by the system CPU The settings in this register are OR'ed with the corresponding settings in MODCLKEN1. This allows the system CPU and AUX_SCE to request clocks independently. Settings take effect immediately.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Modclken0Spec;
impl crate::RegisterSpec for Modclken0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modclken0::R`](R) reader structure"]
impl crate::Readable for Modclken0Spec {}
#[doc = "`write(|w| ..)` method takes [`modclken0::W`](W) writer structure"]
impl crate::Writable for Modclken0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODCLKEN0 to value 0"]
impl crate::Resettable for Modclken0Spec {
    const RESET_VALUE: u32 = 0;
}

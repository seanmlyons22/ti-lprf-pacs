#[doc = "Register `IO7PSEL` reader"]
pub type R = crate::R<Io7pselSpec>;
#[doc = "Register `IO7PSEL` writer"]
pub type W = crate::W<Io7pselSpec>;
#[doc = "2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "7: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE."]
    AuxTimer2Pulse = 7,
    #[doc = "6: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    AuxTimer2Ev3 = 6,
    #[doc = "5: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2."]
    AuxTimer2Ev2 = 5,
    #[doc = "4: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1."]
    AuxTimer2Ev1 = 4,
    #[doc = "3: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0."]
    AuxTimer2Ev0 = 3,
    #[doc = "2: Peripheral output mux selects AUX_SPIM MOSI."]
    AuxSpimMosi = 2,
    #[doc = "1: Peripheral output mux selects AUX_SPIM SCLK."]
    AuxSpimSclk = 1,
    #[doc = "0: Peripheral output mux selects event selected by AUX_EVCTL:EVOBSCFG"]
    AuxEvObs = 0,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is set."]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src {
        match self.bits {
            7 => Src::AuxTimer2Pulse,
            6 => Src::AuxTimer2Ev3,
            5 => Src::AuxTimer2Ev2,
            4 => Src::AuxTimer2Ev1,
            3 => Src::AuxTimer2Ev0,
            2 => Src::AuxSpimMosi,
            1 => Src::AuxSpimSclk,
            0 => Src::AuxEvObs,
            _ => unreachable!(),
        }
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE."]
    #[inline(always)]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == Src::AuxTimer2Pulse
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == Src::AuxTimer2Ev3
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2."]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == Src::AuxTimer2Ev2
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1."]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == Src::AuxTimer2Ev1
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0."]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == Src::AuxTimer2Ev0
    }
    #[doc = "Peripheral output mux selects AUX_SPIM MOSI."]
    #[inline(always)]
    pub fn is_aux_spim_mosi(&self) -> bool {
        *self == Src::AuxSpimMosi
    }
    #[doc = "Peripheral output mux selects AUX_SPIM SCLK."]
    #[inline(always)]
    pub fn is_aux_spim_sclk(&self) -> bool {
        *self == Src::AuxSpimSclk
    }
    #[doc = "Peripheral output mux selects event selected by AUX_EVCTL:EVOBSCFG"]
    #[inline(always)]
    pub fn is_aux_ev_obs(&self) -> bool {
        *self == Src::AuxEvObs
    }
}
#[doc = "Field `SRC` writer - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is set."]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Src, crate::Safe>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE."]
    #[inline(always)]
    pub fn aux_timer2_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxTimer2Pulse)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxTimer2Ev3)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2."]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxTimer2Ev2)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1."]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxTimer2Ev1)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0."]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxTimer2Ev0)
    }
    #[doc = "Peripheral output mux selects AUX_SPIM MOSI."]
    #[inline(always)]
    pub fn aux_spim_mosi(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxSpimMosi)
    }
    #[doc = "Peripheral output mux selects AUX_SPIM SCLK."]
    #[inline(always)]
    pub fn aux_spim_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxSpimSclk)
    }
    #[doc = "Peripheral output mux selects event selected by AUX_EVCTL:EVOBSCFG"]
    #[inline(always)]
    pub fn aux_ev_obs(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxEvObs)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is set."]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is set."]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<Io7pselSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Io7pselSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Input Output 7 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is 1. To avoid glitches on AUXIO\\[8i+7\\]
you must configure this register while IOPOE bit 7 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io7psel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io7psel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Io7pselSpec;
impl crate::RegisterSpec for Io7pselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io7psel::R`](R) reader structure"]
impl crate::Readable for Io7pselSpec {}
#[doc = "`write(|w| ..)` method takes [`io7psel::W`](W) writer structure"]
impl crate::Writable for Io7pselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IO7PSEL to value 0"]
impl crate::Resettable for Io7pselSpec {
    const RESET_VALUE: u32 = 0;
}

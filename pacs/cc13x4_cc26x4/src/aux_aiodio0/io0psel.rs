#[doc = "Register `IO0PSEL` reader"]
pub struct R(crate::R<IO0PSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO0PSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO0PSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO0PSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IO0PSEL` writer"]
pub struct W(crate::W<IO0PSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IO0PSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IO0PSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IO0PSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC` reader - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is set."]
pub type SRC_R = crate::FieldReader<u8, SRC_A>;
#[doc = "2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "7: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE."]
    AUX_TIMER2_PULSE = 7,
    #[doc = "6: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    AUX_TIMER2_EV3 = 6,
    #[doc = "5: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2."]
    AUX_TIMER2_EV2 = 5,
    #[doc = "4: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1."]
    AUX_TIMER2_EV1 = 4,
    #[doc = "3: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0."]
    AUX_TIMER2_EV0 = 3,
    #[doc = "2: Peripheral output mux selects AUX_SPIM MOSI."]
    AUX_SPIM_MOSI = 2,
    #[doc = "1: Peripheral output mux selects AUX_SPIM SCLK."]
    AUX_SPIM_SCLK = 1,
    #[doc = "0: Peripheral output mux selects event selected by AUX_EVCTL:EVOBSCFG"]
    AUX_EV_OBS = 0,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            7 => SRC_A::AUX_TIMER2_PULSE,
            6 => SRC_A::AUX_TIMER2_EV3,
            5 => SRC_A::AUX_TIMER2_EV2,
            4 => SRC_A::AUX_TIMER2_EV1,
            3 => SRC_A::AUX_TIMER2_EV0,
            2 => SRC_A::AUX_SPIM_MOSI,
            1 => SRC_A::AUX_SPIM_SCLK,
            0 => SRC_A::AUX_EV_OBS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_PULSE`"]
    #[inline(always)]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_PULSE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `AUX_SPIM_MOSI`"]
    #[inline(always)]
    pub fn is_aux_spim_mosi(&self) -> bool {
        *self == SRC_A::AUX_SPIM_MOSI
    }
    #[doc = "Checks if the value of the field is `AUX_SPIM_SCLK`"]
    #[inline(always)]
    pub fn is_aux_spim_sclk(&self) -> bool {
        *self == SRC_A::AUX_SPIM_SCLK
    }
    #[doc = "Checks if the value of the field is `AUX_EV_OBS`"]
    #[inline(always)]
    pub fn is_aux_ev_obs(&self) -> bool {
        *self == SRC_A::AUX_EV_OBS
    }
}
#[doc = "Field `SRC` writer - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is set."]
pub type SRC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IO0PSEL_SPEC, u8, SRC_A, 3, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE."]
    #[inline(always)]
    pub fn aux_timer2_pulse(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_PULSE)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_EV3)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2."]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_EV2)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1."]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_EV1)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0."]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_EV0)
    }
    #[doc = "Peripheral output mux selects AUX_SPIM MOSI."]
    #[inline(always)]
    pub fn aux_spim_mosi(self) -> &'a mut W {
        self.variant(SRC_A::AUX_SPIM_MOSI)
    }
    #[doc = "Peripheral output mux selects AUX_SPIM SCLK."]
    #[inline(always)]
    pub fn aux_spim_sclk(self) -> &'a mut W {
        self.variant(SRC_A::AUX_SPIM_SCLK)
    }
    #[doc = "Peripheral output mux selects event selected by AUX_EVCTL:EVOBSCFG"]
    #[inline(always)]
    pub fn aux_ev_obs(self) -> &'a mut W {
        self.variant(SRC_A::AUX_EV_OBS)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IO0PSEL_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is set."]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is set."]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Output 0 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is 1. To avoid glitches on AUXIO\\[8i+0\\]
you must configure this register while IOPOE bit 0 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io0psel](index.html) module"]
pub struct IO0PSEL_SPEC;
impl crate::RegisterSpec for IO0PSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io0psel::R](R) reader structure"]
impl crate::Readable for IO0PSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io0psel::W](W) writer structure"]
impl crate::Writable for IO0PSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IO0PSEL to value 0"]
impl crate::Resettable for IO0PSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

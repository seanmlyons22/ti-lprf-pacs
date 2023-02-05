#[doc = "Register `IO3PSEL` reader"]
pub struct R(crate::R<IO3PSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO3PSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO3PSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO3PSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IO3PSEL` writer"]
pub struct W(crate::W<IO3PSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IO3PSEL_SPEC>;
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
impl From<crate::W<IO3PSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IO3PSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC` reader - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is set."]
pub type SRC_R = crate::FieldReader<u8, SRC_A>;
#[doc = "2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRC_A {
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
    pub fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            0 => Some(SRC_A::AUX_EV_OBS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUX_EV_OBS`"]
    #[inline(always)]
    pub fn is_aux_ev_obs(&self) -> bool {
        *self == SRC_A::AUX_EV_OBS
    }
}
#[doc = "Field `SRC` writer - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is set."]
pub type SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IO3PSEL_SPEC, u8, SRC_A, 3, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
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
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IO3PSEL_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is set."]
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
Select a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is set."]
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
#[doc = "Input Output 3 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is 1. To avoid glitches on AUXIO\\[8i+3\\]
you must configure this register while IOPOE bit 3 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io3psel](index.html) module"]
pub struct IO3PSEL_SPEC;
impl crate::RegisterSpec for IO3PSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io3psel::R](R) reader structure"]
impl crate::Readable for IO3PSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io3psel::W](W) writer structure"]
impl crate::Writable for IO3PSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IO3PSEL to value 0"]
impl crate::Resettable for IO3PSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

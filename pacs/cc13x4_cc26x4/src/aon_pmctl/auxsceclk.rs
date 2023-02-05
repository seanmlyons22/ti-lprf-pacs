#[doc = "Register `AUXSCECLK` reader"]
pub struct R(crate::R<AUXSCECLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXSCECLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXSCECLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXSCECLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXSCECLK` writer"]
pub struct W(crate::W<AUXSCECLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXSCECLK_SPEC>;
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
impl From<crate::W<AUXSCECLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXSCECLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC` reader - 0:0\\]
Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free"]
pub type SRC_R = crate::BitReader<SRC_A>;
#[doc = "0:0\\]
Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRC_A {
    #[doc = "1: MF Clock (SCLK_MF)"]
    SCLK_MF = 1,
    #[doc = "0: HF Clock divided by 2 (SCLK_HFDIV2)"]
    SCLK_HFDIV2 = 0,
}
impl From<SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            true => SRC_A::SCLK_MF,
            false => SRC_A::SCLK_HFDIV2,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_MF`"]
    #[inline(always)]
    pub fn is_sclk_mf(&self) -> bool {
        *self == SRC_A::SCLK_MF
    }
    #[doc = "Checks if the value of the field is `SCLK_HFDIV2`"]
    #[inline(always)]
    pub fn is_sclk_hfdiv2(&self) -> bool {
        *self == SRC_A::SCLK_HFDIV2
    }
}
#[doc = "Field `SRC` writer - 0:0\\]
Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free"]
pub type SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUXSCECLK_SPEC, SRC_A, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
    #[doc = "MF Clock (SCLK_MF)"]
    #[inline(always)]
    pub fn sclk_mf(self) -> &'a mut W {
        self.variant(SRC_A::SCLK_MF)
    }
    #[doc = "HF Clock divided by 2 (SCLK_HFDIV2)"]
    #[inline(always)]
    pub fn sclk_hfdiv2(self) -> &'a mut W {
        self.variant(SRC_A::SCLK_HFDIV2)
    }
}
#[doc = "Field `RESERVED3` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED3` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUXSCECLK_SPEC, u8, u8, 7, O>;
#[doc = "Field `PD_SRC` reader - 8:8\\]
Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free"]
pub type PD_SRC_R = crate::BitReader<PD_SRC_A>;
#[doc = "8:8\\]
Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD_SRC_A {
    #[doc = "1: LF clock (SCLK_LF )"]
    SCLK_LF = 1,
    #[doc = "0: No clock"]
    NO_CLOCK = 0,
}
impl From<PD_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: PD_SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl PD_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_SRC_A {
        match self.bits {
            true => PD_SRC_A::SCLK_LF,
            false => PD_SRC_A::NO_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == PD_SRC_A::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == PD_SRC_A::NO_CLOCK
    }
}
#[doc = "Field `PD_SRC` writer - 8:8\\]
Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free"]
pub type PD_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUXSCECLK_SPEC, PD_SRC_A, O>;
impl<'a, const O: u8> PD_SRC_W<'a, O> {
    #[doc = "LF clock (SCLK_LF )"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(PD_SRC_A::SCLK_LF)
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(PD_SRC_A::NO_CLOCK)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUXSCECLK_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline(always)]
    pub fn pd_src(&self) -> PD_SRC_R {
        PD_SRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<1> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline(always)]
    #[must_use]
    pub fn pd_src(&mut self) -> PD_SRC_W<8> {
        PD_SRC_W::new(self)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX SCE Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxsceclk](index.html) module"]
pub struct AUXSCECLK_SPEC;
impl crate::RegisterSpec for AUXSCECLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxsceclk::R](R) reader structure"]
impl crate::Readable for AUXSCECLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxsceclk::W](W) writer structure"]
impl crate::Writable for AUXSCECLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUXSCECLK to value 0"]
impl crate::Resettable for AUXSCECLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

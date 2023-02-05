#[doc = "Register `TIMER2CLKSTAT` reader"]
pub struct R(crate::R<TIMER2CLKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2CLKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER2CLKSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER2CLKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2CLKSTAT` writer"]
pub struct W(crate::W<TIMER2CLKSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER2CLKSTAT_SPEC>;
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
impl From<crate::W<TIMER2CLKSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER2CLKSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STAT` reader - 2:0\\]
AUX_TIMER2 clock source status."]
pub type STAT_R = crate::FieldReader<u8, STAT_A>;
#[doc = "2:0\\]
AUX_TIMER2 clock source status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STAT_A {
    #[doc = "4: SCLK_HF / 2"]
    SCLK_HFDIV2 = 4,
    #[doc = "2: SCLK_MF"]
    SCLK_MF = 2,
    #[doc = "1: SCLK_LF"]
    SCLK_LF = 1,
    #[doc = "0: No clock"]
    NONE = 0,
}
impl From<STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: STAT_A) -> Self {
        variant as _
    }
}
impl STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STAT_A> {
        match self.bits {
            4 => Some(STAT_A::SCLK_HFDIV2),
            2 => Some(STAT_A::SCLK_MF),
            1 => Some(STAT_A::SCLK_LF),
            0 => Some(STAT_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_HFDIV2`"]
    #[inline(always)]
    pub fn is_sclk_hfdiv2(&self) -> bool {
        *self == STAT_A::SCLK_HFDIV2
    }
    #[doc = "Checks if the value of the field is `SCLK_MF`"]
    #[inline(always)]
    pub fn is_sclk_mf(&self) -> bool {
        *self == STAT_A::SCLK_MF
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == STAT_A::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == STAT_A::NONE
    }
}
#[doc = "Field `STAT` writer - 2:0\\]
AUX_TIMER2 clock source status."]
pub type STAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER2CLKSTAT_SPEC, u8, STAT_A, 3, O>;
impl<'a, const O: u8> STAT_W<'a, O> {
    #[doc = "SCLK_HF / 2"]
    #[inline(always)]
    pub fn sclk_hfdiv2(self) -> &'a mut W {
        self.variant(STAT_A::SCLK_HFDIV2)
    }
    #[doc = "SCLK_MF"]
    #[inline(always)]
    pub fn sclk_mf(self) -> &'a mut W {
        self.variant(STAT_A::SCLK_MF)
    }
    #[doc = "SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(STAT_A::SCLK_LF)
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(STAT_A::NONE)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER2CLKSTAT_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
AUX_TIMER2 clock source status."]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 7) as u8)
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
AUX_TIMER2 clock source status."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> STAT_W<0> {
        STAT_W::new(self)
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
#[doc = "AUX_TIMER2 Clock Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2clkstat](index.html) module"]
pub struct TIMER2CLKSTAT_SPEC;
impl crate::RegisterSpec for TIMER2CLKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2clkstat::R](R) reader structure"]
impl crate::Readable for TIMER2CLKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer2clkstat::W](W) writer structure"]
impl crate::Writable for TIMER2CLKSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER2CLKSTAT to value 0"]
impl crate::Resettable for TIMER2CLKSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

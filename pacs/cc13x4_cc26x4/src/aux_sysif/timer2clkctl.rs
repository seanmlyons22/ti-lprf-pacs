#[doc = "Register `TIMER2CLKCTL` reader"]
pub struct R(crate::R<TIMER2CLKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2CLKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER2CLKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER2CLKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2CLKCTL` writer"]
pub struct W(crate::W<TIMER2CLKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER2CLKCTL_SPEC>;
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
impl From<crate::W<TIMER2CLKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER2CLKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC` reader - 2:0\\]
Select clock source for AUX_TIMER2. Update is only accepted if SRC equals TIMER2CLKSTAT.STAT or TIMER2CLKSWITCH.RDY is 1. It is recommended to select NONE only when TIMER2BRIDGE.BUSY is 0. A non-enumerated value is ignored."]
pub type SRC_R = crate::FieldReader<u8, SRC_A>;
#[doc = "2:0\\]
Select clock source for AUX_TIMER2. Update is only accepted if SRC equals TIMER2CLKSTAT.STAT or TIMER2CLKSWITCH.RDY is 1. It is recommended to select NONE only when TIMER2BRIDGE.BUSY is 0. A non-enumerated value is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "4: SCLK_HF / 2"]
    SCLK_HFDIV2 = 4,
    #[doc = "2: SCLK_MF"]
    SCLK_MF = 2,
    #[doc = "1: SCLK_LF"]
    SCLK_LF = 1,
    #[doc = "0: no clock"]
    NONE = 0,
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
            4 => Some(SRC_A::SCLK_HFDIV2),
            2 => Some(SRC_A::SCLK_MF),
            1 => Some(SRC_A::SCLK_LF),
            0 => Some(SRC_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_HFDIV2`"]
    #[inline(always)]
    pub fn is_sclk_hfdiv2(&self) -> bool {
        *self == SRC_A::SCLK_HFDIV2
    }
    #[doc = "Checks if the value of the field is `SCLK_MF`"]
    #[inline(always)]
    pub fn is_sclk_mf(&self) -> bool {
        *self == SRC_A::SCLK_MF
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == SRC_A::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SRC_A::NONE
    }
}
#[doc = "Field `SRC` writer - 2:0\\]
Select clock source for AUX_TIMER2. Update is only accepted if SRC equals TIMER2CLKSTAT.STAT or TIMER2CLKSWITCH.RDY is 1. It is recommended to select NONE only when TIMER2BRIDGE.BUSY is 0. A non-enumerated value is ignored."]
pub type SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER2CLKCTL_SPEC, u8, SRC_A, 3, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
    #[doc = "SCLK_HF / 2"]
    #[inline(always)]
    pub fn sclk_hfdiv2(self) -> &'a mut W {
        self.variant(SRC_A::SCLK_HFDIV2)
    }
    #[doc = "SCLK_MF"]
    #[inline(always)]
    pub fn sclk_mf(self) -> &'a mut W {
        self.variant(SRC_A::SCLK_MF)
    }
    #[doc = "SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(SRC_A::SCLK_LF)
    }
    #[doc = "no clock"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SRC_A::NONE)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER2CLKCTL_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Select clock source for AUX_TIMER2. Update is only accepted if SRC equals TIMER2CLKSTAT.STAT or TIMER2CLKSWITCH.RDY is 1. It is recommended to select NONE only when TIMER2BRIDGE.BUSY is 0. A non-enumerated value is ignored."]
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
Select clock source for AUX_TIMER2. Update is only accepted if SRC equals TIMER2CLKSTAT.STAT or TIMER2CLKSWITCH.RDY is 1. It is recommended to select NONE only when TIMER2BRIDGE.BUSY is 0. A non-enumerated value is ignored."]
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
#[doc = "AUX_TIMER2 Clock Control Access to AUX_TIMER2 is only possible when TIMER2CLKSTAT.STAT is different from NONE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2clkctl](index.html) module"]
pub struct TIMER2CLKCTL_SPEC;
impl crate::RegisterSpec for TIMER2CLKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2clkctl::R](R) reader structure"]
impl crate::Readable for TIMER2CLKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer2clkctl::W](W) writer structure"]
impl crate::Writable for TIMER2CLKCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER2CLKCTL to value 0"]
impl crate::Resettable for TIMER2CLKCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `LFOSCCTL` reader"]
pub struct R(crate::R<LFOSCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFOSCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFOSCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFOSCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFOSCCTL` writer"]
pub struct W(crate::W<LFOSCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFOSCCTL_SPEC>;
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
impl From<crate::W<LFOSCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFOSCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCOSCLF_CTUNE_TRIM` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCLF_CTUNE_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSCLF_CTUNE_TRIM` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCLF_CTUNE_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LFOSCCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `RCOSCLF_RTUNE_TRIM` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCLF_RTUNE_TRIM_R = crate::FieldReader<u8, RCOSCLF_RTUNE_TRIM_A>;
#[doc = "9:8\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RCOSCLF_RTUNE_TRIM_A {
    #[doc = "3: Internal. Only to be used through TI provided API."]
    _6P0MEG = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    _6P5MEG = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    _7P0MEG = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    _7P5MEG = 0,
}
impl From<RCOSCLF_RTUNE_TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: RCOSCLF_RTUNE_TRIM_A) -> Self {
        variant as _
    }
}
impl RCOSCLF_RTUNE_TRIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCOSCLF_RTUNE_TRIM_A {
        match self.bits {
            3 => RCOSCLF_RTUNE_TRIM_A::_6P0MEG,
            2 => RCOSCLF_RTUNE_TRIM_A::_6P5MEG,
            1 => RCOSCLF_RTUNE_TRIM_A::_7P0MEG,
            0 => RCOSCLF_RTUNE_TRIM_A::_7P5MEG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_6P0MEG`"]
    #[inline(always)]
    pub fn is_6p0meg(&self) -> bool {
        *self == RCOSCLF_RTUNE_TRIM_A::_6P0MEG
    }
    #[doc = "Checks if the value of the field is `_6P5MEG`"]
    #[inline(always)]
    pub fn is_6p5meg(&self) -> bool {
        *self == RCOSCLF_RTUNE_TRIM_A::_6P5MEG
    }
    #[doc = "Checks if the value of the field is `_7P0MEG`"]
    #[inline(always)]
    pub fn is_7p0meg(&self) -> bool {
        *self == RCOSCLF_RTUNE_TRIM_A::_7P0MEG
    }
    #[doc = "Checks if the value of the field is `_7P5MEG`"]
    #[inline(always)]
    pub fn is_7p5meg(&self) -> bool {
        *self == RCOSCLF_RTUNE_TRIM_A::_7P5MEG
    }
}
#[doc = "Field `RCOSCLF_RTUNE_TRIM` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCLF_RTUNE_TRIM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LFOSCCTL_SPEC, u8, RCOSCLF_RTUNE_TRIM_A, 2, O>;
impl<'a, const O: u8> RCOSCLF_RTUNE_TRIM_W<'a, O> {
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _6p0meg(self) -> &'a mut W {
        self.variant(RCOSCLF_RTUNE_TRIM_A::_6P0MEG)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _6p5meg(self) -> &'a mut W {
        self.variant(RCOSCLF_RTUNE_TRIM_A::_6P5MEG)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _7p0meg(self) -> &'a mut W {
        self.variant(RCOSCLF_RTUNE_TRIM_A::_7P0MEG)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _7p5meg(self) -> &'a mut W {
        self.variant(RCOSCLF_RTUNE_TRIM_A::_7P5MEG)
    }
}
#[doc = "Field `RESERVED10` reader - 17:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED10` writer - 17:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LFOSCCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `XOSCLF_CMIRRWR_RATIO` reader - 21:18\\]
Internal. Only to be used through TI provided API."]
pub type XOSCLF_CMIRRWR_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XOSCLF_CMIRRWR_RATIO` writer - 21:18\\]
Internal. Only to be used through TI provided API."]
pub type XOSCLF_CMIRRWR_RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LFOSCCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `XOSCLF_REGULATOR_TRIM` reader - 23:22\\]
Internal. Only to be used through TI provided API."]
pub type XOSCLF_REGULATOR_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XOSCLF_REGULATOR_TRIM` writer - 23:22\\]
Internal. Only to be used through TI provided API."]
pub type XOSCLF_REGULATOR_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LFOSCCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LFOSCCTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_ctune_trim(&self) -> RCOSCLF_CTUNE_TRIM_R {
        RCOSCLF_CTUNE_TRIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_rtune_trim(&self) -> RCOSCLF_RTUNE_TRIM_R {
        RCOSCLF_RTUNE_TRIM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:17 - 17:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosclf_cmirrwr_ratio(&self) -> XOSCLF_CMIRRWR_RATIO_R {
        XOSCLF_CMIRRWR_RATIO_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosclf_regulator_trim(&self) -> XOSCLF_REGULATOR_TRIM_R {
        XOSCLF_REGULATOR_TRIM_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcosclf_ctune_trim(&mut self) -> RCOSCLF_CTUNE_TRIM_W<0> {
        RCOSCLF_CTUNE_TRIM_W::new(self)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcosclf_rtune_trim(&mut self) -> RCOSCLF_RTUNE_TRIM_W<8> {
        RCOSCLF_RTUNE_TRIM_W::new(self)
    }
    #[doc = "Bits 10:17 - 17:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosclf_cmirrwr_ratio(&mut self) -> XOSCLF_CMIRRWR_RATIO_W<18> {
        XOSCLF_CMIRRWR_RATIO_W::new(self)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosclf_regulator_trim(&mut self) -> XOSCLF_REGULATOR_TRIM_W<22> {
        XOSCLF_REGULATOR_TRIM_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfoscctl](index.html) module"]
pub struct LFOSCCTL_SPEC;
impl crate::RegisterSpec for LFOSCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfoscctl::R](R) reader structure"]
impl crate::Readable for LFOSCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfoscctl::W](W) writer structure"]
impl crate::Writable for LFOSCCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFOSCCTL to value 0"]
impl crate::Resettable for LFOSCCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

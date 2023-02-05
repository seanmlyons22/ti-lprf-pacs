#[doc = "Register `AUXCLK` reader"]
pub struct R(crate::R<AUXCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXCLK` writer"]
pub struct W(crate::W<AUXCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXCLK_SPEC>;
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
impl From<crate::W<AUXCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC` reader - 2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
pub type SRC_R = crate::FieldReader<u8, SRC_A>;
#[doc = "2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "4: LF Clock (SCLK_LF)"]
    SCLK_LF = 4,
    #[doc = "1: HF Clock (SCLK_HF)"]
    SCLK_HF = 1,
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
            4 => Some(SRC_A::SCLK_LF),
            1 => Some(SRC_A::SCLK_HF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == SRC_A::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `SCLK_HF`"]
    #[inline(always)]
    pub fn is_sclk_hf(&self) -> bool {
        *self == SRC_A::SCLK_HF
    }
}
#[doc = "Field `SRC` writer - 2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
pub type SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUXCLK_SPEC, u8, SRC_A, 3, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
    #[doc = "LF Clock (SCLK_LF)"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(SRC_A::SCLK_LF)
    }
    #[doc = "HF Clock (SCLK_HF)"]
    #[inline(always)]
    pub fn sclk_hf(self) -> &'a mut W {
        self.variant(SRC_A::SCLK_HF)
    }
}
#[doc = "Field `RESERVED3` reader - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED3` writer - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUXCLK_SPEC, u8, u8, 5, O>;
#[doc = "Field `SCLK_HF_DIV` reader - 10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
pub type SCLK_HF_DIV_R = crate::FieldReader<u8, SCLK_HF_DIV_A>;
#[doc = "10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCLK_HF_DIV_A {
    #[doc = "7: Divide by 256"]
    DIV256 = 7,
    #[doc = "6: Divide by 128"]
    DIV128 = 6,
    #[doc = "5: Divide by 64"]
    DIV64 = 5,
    #[doc = "4: Divide by 32"]
    DIV32 = 4,
    #[doc = "3: Divide by 16"]
    DIV16 = 3,
    #[doc = "2: Divide by 8"]
    DIV8 = 2,
    #[doc = "1: Divide by 4"]
    DIV4 = 1,
    #[doc = "0: Divide by 2"]
    DIV2 = 0,
}
impl From<SCLK_HF_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLK_HF_DIV_A) -> Self {
        variant as _
    }
}
impl SCLK_HF_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_HF_DIV_A {
        match self.bits {
            7 => SCLK_HF_DIV_A::DIV256,
            6 => SCLK_HF_DIV_A::DIV128,
            5 => SCLK_HF_DIV_A::DIV64,
            4 => SCLK_HF_DIV_A::DIV32,
            3 => SCLK_HF_DIV_A::DIV16,
            2 => SCLK_HF_DIV_A::DIV8,
            1 => SCLK_HF_DIV_A::DIV4,
            0 => SCLK_HF_DIV_A::DIV2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == SCLK_HF_DIV_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == SCLK_HF_DIV_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == SCLK_HF_DIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == SCLK_HF_DIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == SCLK_HF_DIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == SCLK_HF_DIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == SCLK_HF_DIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SCLK_HF_DIV_A::DIV2
    }
}
#[doc = "Field `SCLK_HF_DIV` writer - 10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
pub type SCLK_HF_DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AUXCLK_SPEC, u8, SCLK_HF_DIV_A, 3, O>;
impl<'a, const O: u8> SCLK_HF_DIV_W<'a, O> {
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV256)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV128)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV64)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV32)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV16)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV8)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV4)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV2)
    }
}
#[doc = "Field `PWR_DWN_SRC` reader - 12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
pub type PWR_DWN_SRC_R = crate::FieldReader<u8, PWR_DWN_SRC_A>;
#[doc = "12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWR_DWN_SRC_A {
    #[doc = "1: Use SCLK_LF in Powerdown"]
    SCLK_LF = 1,
    #[doc = "0: No clock in Powerdown"]
    NONE = 0,
}
impl From<PWR_DWN_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PWR_DWN_SRC_A) -> Self {
        variant as _
    }
}
impl PWR_DWN_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWR_DWN_SRC_A> {
        match self.bits {
            1 => Some(PWR_DWN_SRC_A::SCLK_LF),
            0 => Some(PWR_DWN_SRC_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == PWR_DWN_SRC_A::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PWR_DWN_SRC_A::NONE
    }
}
#[doc = "Field `PWR_DWN_SRC` writer - 12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
pub type PWR_DWN_SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUXCLK_SPEC, u8, PWR_DWN_SRC_A, 2, O>;
impl<'a, const O: u8> PWR_DWN_SRC_W<'a, O> {
    #[doc = "Use SCLK_LF in Powerdown"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(PWR_DWN_SRC_A::SCLK_LF)
    }
    #[doc = "No clock in Powerdown"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PWR_DWN_SRC_A::NONE)
    }
}
#[doc = "Field `RESERVED13` reader - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED13` writer - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUXCLK_SPEC, u32, u32, 19, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
    #[inline(always)]
    pub fn sclk_hf_div(&self) -> SCLK_HF_DIV_R {
        SCLK_HF_DIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
    #[inline(always)]
    pub fn pwr_dwn_src(&self) -> PWR_DWN_SRC_R {
        PWR_DWN_SRC_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_hf_div(&mut self) -> SCLK_HF_DIV_W<8> {
        SCLK_HF_DIV_W::new(self)
    }
    #[doc = "Bits 11:12 - 12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_dwn_src(&mut self) -> PWR_DWN_SRC_W<11> {
        PWR_DWN_SRC_W::new(self)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> RESERVED13_W<13> {
        RESERVED13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxclk](index.html) module"]
pub struct AUXCLK_SPEC;
impl crate::RegisterSpec for AUXCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxclk::R](R) reader structure"]
impl crate::Readable for AUXCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxclk::W](W) writer structure"]
impl crate::Writable for AUXCLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUXCLK to value 0x01"]
impl crate::Resettable for AUXCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

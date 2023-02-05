#[doc = "Register `BANK0INFO1` reader"]
pub struct R(crate::R<BANK0INFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BANK0INFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BANK0INFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BANK0INFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BANK0INFO1` writer"]
pub struct W(crate::W<BANK0INFO1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BANK0INFO1_SPEC>;
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
impl From<crate::W<BANK0INFO1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BANK0INFO1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONMAINSIZE` reader - 7:0\\]
Non-main region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
pub type NONMAINSIZE_R = crate::FieldReader<u8, NONMAINSIZE_A>;
#[doc = "7:0\\]
Non-main region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NONMAINSIZE_A {
    #[doc = "32: Maximum value of NONMAINSIZE"]
    MAXSECTORS = 32,
    #[doc = "0: Minimum value of NONMAINSIZE"]
    MINSECTORS = 0,
}
impl From<NONMAINSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: NONMAINSIZE_A) -> Self {
        variant as _
    }
}
impl NONMAINSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NONMAINSIZE_A> {
        match self.bits {
            32 => Some(NONMAINSIZE_A::MAXSECTORS),
            0 => Some(NONMAINSIZE_A::MINSECTORS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXSECTORS`"]
    #[inline(always)]
    pub fn is_maxsectors(&self) -> bool {
        *self == NONMAINSIZE_A::MAXSECTORS
    }
    #[doc = "Checks if the value of the field is `MINSECTORS`"]
    #[inline(always)]
    pub fn is_minsectors(&self) -> bool {
        *self == NONMAINSIZE_A::MINSECTORS
    }
}
#[doc = "Field `NONMAINSIZE` writer - 7:0\\]
Non-main region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
pub type NONMAINSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BANK0INFO1_SPEC, u8, NONMAINSIZE_A, 8, O>;
impl<'a, const O: u8> NONMAINSIZE_W<'a, O> {
    #[doc = "Maximum value of NONMAINSIZE"]
    #[inline(always)]
    pub fn maxsectors(self) -> &'a mut W {
        self.variant(NONMAINSIZE_A::MAXSECTORS)
    }
    #[doc = "Minimum value of NONMAINSIZE"]
    #[inline(always)]
    pub fn minsectors(self) -> &'a mut W {
        self.variant(NONMAINSIZE_A::MINSECTORS)
    }
}
#[doc = "Field `TRIMSIZE` reader - 15:8\\]
Trim region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
pub type TRIMSIZE_R = crate::FieldReader<u8, TRIMSIZE_A>;
#[doc = "15:8\\]
Trim region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIMSIZE_A {
    #[doc = "32: Maximum value of TRIMSIZE"]
    MAXSECTORS = 32,
    #[doc = "0: Minimum value of TRIMSIZE"]
    MINSECTORS = 0,
}
impl From<TRIMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIMSIZE_A) -> Self {
        variant as _
    }
}
impl TRIMSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIMSIZE_A> {
        match self.bits {
            32 => Some(TRIMSIZE_A::MAXSECTORS),
            0 => Some(TRIMSIZE_A::MINSECTORS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXSECTORS`"]
    #[inline(always)]
    pub fn is_maxsectors(&self) -> bool {
        *self == TRIMSIZE_A::MAXSECTORS
    }
    #[doc = "Checks if the value of the field is `MINSECTORS`"]
    #[inline(always)]
    pub fn is_minsectors(&self) -> bool {
        *self == TRIMSIZE_A::MINSECTORS
    }
}
#[doc = "Field `TRIMSIZE` writer - 15:8\\]
Trim region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
pub type TRIMSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BANK0INFO1_SPEC, u8, TRIMSIZE_A, 8, O>;
impl<'a, const O: u8> TRIMSIZE_W<'a, O> {
    #[doc = "Maximum value of TRIMSIZE"]
    #[inline(always)]
    pub fn maxsectors(self) -> &'a mut W {
        self.variant(TRIMSIZE_A::MAXSECTORS)
    }
    #[doc = "Minimum value of TRIMSIZE"]
    #[inline(always)]
    pub fn minsectors(self) -> &'a mut W {
        self.variant(TRIMSIZE_A::MINSECTORS)
    }
}
#[doc = "Field `ENGRSIZE` reader - 23:16\\]
Engr region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
pub type ENGRSIZE_R = crate::FieldReader<u8, ENGRSIZE_A>;
#[doc = "23:16\\]
Engr region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENGRSIZE_A {
    #[doc = "32: Maximum value of ENGRSIZE"]
    MAXSECTORS = 32,
    #[doc = "0: Minimum value of ENGRSIZE"]
    MINSECTORS = 0,
}
impl From<ENGRSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: ENGRSIZE_A) -> Self {
        variant as _
    }
}
impl ENGRSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENGRSIZE_A> {
        match self.bits {
            32 => Some(ENGRSIZE_A::MAXSECTORS),
            0 => Some(ENGRSIZE_A::MINSECTORS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXSECTORS`"]
    #[inline(always)]
    pub fn is_maxsectors(&self) -> bool {
        *self == ENGRSIZE_A::MAXSECTORS
    }
    #[doc = "Checks if the value of the field is `MINSECTORS`"]
    #[inline(always)]
    pub fn is_minsectors(&self) -> bool {
        *self == ENGRSIZE_A::MINSECTORS
    }
}
#[doc = "Field `ENGRSIZE` writer - 23:16\\]
Engr region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
pub type ENGRSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BANK0INFO1_SPEC, u8, ENGRSIZE_A, 8, O>;
impl<'a, const O: u8> ENGRSIZE_W<'a, O> {
    #[doc = "Maximum value of ENGRSIZE"]
    #[inline(always)]
    pub fn maxsectors(self) -> &'a mut W {
        self.variant(ENGRSIZE_A::MAXSECTORS)
    }
    #[doc = "Minimum value of ENGRSIZE"]
    #[inline(always)]
    pub fn minsectors(self) -> &'a mut W {
        self.variant(ENGRSIZE_A::MINSECTORS)
    }
}
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BANK0INFO1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Non-main region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
    #[inline(always)]
    pub fn nonmainsize(&self) -> NONMAINSIZE_R {
        NONMAINSIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Trim region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
    #[inline(always)]
    pub fn trimsize(&self) -> TRIMSIZE_R {
        TRIMSIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Engr region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
    #[inline(always)]
    pub fn engrsize(&self) -> ENGRSIZE_R {
        ENGRSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
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
Non-main region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
    #[inline(always)]
    #[must_use]
    pub fn nonmainsize(&mut self) -> NONMAINSIZE_W<0> {
        NONMAINSIZE_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Trim region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
    #[inline(always)]
    #[must_use]
    pub fn trimsize(&mut self) -> TRIMSIZE_W<8> {
        TRIMSIZE_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Engr region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
    #[inline(always)]
    #[must_use]
    pub fn engrsize(&mut self) -> ENGRSIZE_W<16> {
        ENGRSIZE_W::new(self)
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
#[doc = "Bank Info1 Register for bank 0. Read only register detailing information about Non-Main, Trim, and Engr region sizes in the bank.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bank0info1](index.html) module"]
pub struct BANK0INFO1_SPEC;
impl crate::RegisterSpec for BANK0INFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bank0info1::R](R) reader structure"]
impl crate::Readable for BANK0INFO1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bank0info1::W](W) writer structure"]
impl crate::Writable for BANK0INFO1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BANK0INFO1 to value 0x0001_0101"]
impl crate::Resettable for BANK0INFO1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0101;
}

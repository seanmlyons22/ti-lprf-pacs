#[doc = "Register `DESC` reader"]
pub struct R(crate::R<DESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESC` writer"]
pub struct W(crate::W<DESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESC_SPEC>;
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
impl From<crate::W<DESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MINREV` reader - 3:0\\]
Minor Revision"]
pub type MINREV_R = crate::FieldReader<u8, MINREV_A>;
#[doc = "3:0\\]
Minor Revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MINREV_A {
    #[doc = "15: Highest possible value"]
    MAXIMUM = 15,
    #[doc = "0: Smallest value"]
    MINIMUM = 0,
}
impl From<MINREV_A> for u8 {
    #[inline(always)]
    fn from(variant: MINREV_A) -> Self {
        variant as _
    }
}
impl MINREV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MINREV_A> {
        match self.bits {
            15 => Some(MINREV_A::MAXIMUM),
            0 => Some(MINREV_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == MINREV_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == MINREV_A::MINIMUM
    }
}
#[doc = "Field `MINREV` writer - 3:0\\]
Minor Revision"]
pub type MINREV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESC_SPEC, u8, MINREV_A, 4, O>;
impl<'a, const O: u8> MINREV_W<'a, O> {
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MINREV_A::MAXIMUM)
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(MINREV_A::MINIMUM)
    }
}
#[doc = "Field `MAJREV` reader - 7:4\\]
Major Revision"]
pub type MAJREV_R = crate::FieldReader<u8, MAJREV_A>;
#[doc = "7:4\\]
Major Revision\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAJREV_A {
    #[doc = "15: Highest possible value"]
    MAXIMUM = 15,
    #[doc = "0: Smallest value"]
    MINIMUM = 0,
}
impl From<MAJREV_A> for u8 {
    #[inline(always)]
    fn from(variant: MAJREV_A) -> Self {
        variant as _
    }
}
impl MAJREV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAJREV_A> {
        match self.bits {
            15 => Some(MAJREV_A::MAXIMUM),
            0 => Some(MAJREV_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == MAJREV_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == MAJREV_A::MINIMUM
    }
}
#[doc = "Field `MAJREV` writer - 7:4\\]
Major Revision"]
pub type MAJREV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESC_SPEC, u8, MAJREV_A, 4, O>;
impl<'a, const O: u8> MAJREV_W<'a, O> {
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAJREV_A::MAXIMUM)
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(MAJREV_A::MINIMUM)
    }
}
#[doc = "Field `INSTNUM` reader - 11:8\\]
Instance number"]
pub type INSTNUM_R = crate::FieldReader<u8, INSTNUM_A>;
#[doc = "11:8\\]
Instance number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INSTNUM_A {
    #[doc = "15: Highest possible value"]
    MAXIMUM = 15,
    #[doc = "0: Smallest value"]
    MINIMUM = 0,
}
impl From<INSTNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: INSTNUM_A) -> Self {
        variant as _
    }
}
impl INSTNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INSTNUM_A> {
        match self.bits {
            15 => Some(INSTNUM_A::MAXIMUM),
            0 => Some(INSTNUM_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == INSTNUM_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == INSTNUM_A::MINIMUM
    }
}
#[doc = "Field `INSTNUM` writer - 11:8\\]
Instance number"]
pub type INSTNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESC_SPEC, u8, INSTNUM_A, 4, O>;
impl<'a, const O: u8> INSTNUM_W<'a, O> {
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(INSTNUM_A::MAXIMUM)
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(INSTNUM_A::MINIMUM)
    }
}
#[doc = "Field `FEATUREVER` reader - 15:12\\]
Feature set"]
pub type FEATUREVER_R = crate::FieldReader<u8, FEATUREVER_A>;
#[doc = "15:12\\]
Feature set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FEATUREVER_A {
    #[doc = "15: Maximum Value"]
    MAXIMUM = 15,
    #[doc = "0: Minimum Value"]
    MINIMUM = 0,
}
impl From<FEATUREVER_A> for u8 {
    #[inline(always)]
    fn from(variant: FEATUREVER_A) -> Self {
        variant as _
    }
}
impl FEATUREVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FEATUREVER_A> {
        match self.bits {
            15 => Some(FEATUREVER_A::MAXIMUM),
            0 => Some(FEATUREVER_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == FEATUREVER_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == FEATUREVER_A::MINIMUM
    }
}
#[doc = "Field `FEATUREVER` writer - 15:12\\]
Feature set"]
pub type FEATUREVER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESC_SPEC, u8, FEATUREVER_A, 4, O>;
impl<'a, const O: u8> FEATUREVER_W<'a, O> {
    #[doc = "Maximum Value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(FEATUREVER_A::MAXIMUM)
    }
    #[doc = "Minimum Value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(FEATUREVER_A::MINIMUM)
    }
}
#[doc = "Field `MODULEID` reader - 31:16\\]
Module ID"]
pub type MODULEID_R = crate::FieldReader<u16, MODULEID_A>;
#[doc = "31:16\\]
Module ID\n\nValue on reset: 2880"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum MODULEID_A {
    #[doc = "65535: Highest possible value"]
    MAXIMUM = 65535,
    #[doc = "0: Smallest value"]
    MINIMUM = 0,
}
impl From<MODULEID_A> for u16 {
    #[inline(always)]
    fn from(variant: MODULEID_A) -> Self {
        variant as _
    }
}
impl MODULEID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODULEID_A> {
        match self.bits {
            65535 => Some(MODULEID_A::MAXIMUM),
            0 => Some(MODULEID_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == MODULEID_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == MODULEID_A::MINIMUM
    }
}
#[doc = "Field `MODULEID` writer - 31:16\\]
Module ID"]
pub type MODULEID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESC_SPEC, u16, MODULEID_A, 16, O>;
impl<'a, const O: u8> MODULEID_W<'a, O> {
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MODULEID_A::MAXIMUM)
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(MODULEID_A::MINIMUM)
    }
}
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Minor Revision"]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major Revision"]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Instance number"]
    #[inline(always)]
    pub fn instnum(&self) -> INSTNUM_R {
        INSTNUM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Feature set"]
    #[inline(always)]
    pub fn featurever(&self) -> FEATUREVER_R {
        FEATUREVER_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module ID"]
    #[inline(always)]
    pub fn moduleid(&self) -> MODULEID_R {
        MODULEID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Minor Revision"]
    #[inline(always)]
    #[must_use]
    pub fn minrev(&mut self) -> MINREV_W<0> {
        MINREV_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major Revision"]
    #[inline(always)]
    #[must_use]
    pub fn majrev(&mut self) -> MAJREV_W<4> {
        MAJREV_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Instance number"]
    #[inline(always)]
    #[must_use]
    pub fn instnum(&mut self) -> INSTNUM_W<8> {
        INSTNUM_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Feature set"]
    #[inline(always)]
    #[must_use]
    pub fn featurever(&mut self) -> FEATUREVER_W<12> {
        FEATUREVER_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module ID"]
    #[inline(always)]
    #[must_use]
    pub fn moduleid(&mut self) -> MODULEID_W<16> {
        MODULEID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware Version Description Register: This register identifies the NoWrapper hardware version and feature set used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [desc](index.html) module"]
pub struct DESC_SPEC;
impl crate::RegisterSpec for DESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [desc::R](R) reader structure"]
impl crate::Readable for DESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [desc::W](W) writer structure"]
impl crate::Writable for DESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESC to value 0x0b40_0010"]
impl crate::Resettable for DESC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b40_0010;
}

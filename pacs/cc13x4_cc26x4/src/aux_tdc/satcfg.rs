#[doc = "Register `SATCFG` reader"]
pub struct R(crate::R<SATCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SATCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SATCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SATCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SATCFG` writer"]
pub struct W(crate::W<SATCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SATCFG_SPEC>;
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
impl From<crate::W<SATCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SATCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIMIT` reader - 3:0\\]
Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported"]
pub type LIMIT_R = crate::FieldReader<u8, LIMIT_A>;
#[doc = "3:0\\]
Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LIMIT_A {
    #[doc = "15: Result bit 24: TDC conversion saturates and stops when RESULT.VALUE\\[24\\]
is set."]
    R24 = 15,
    #[doc = "14: Result bit 23: TDC conversion saturates and stops when RESULT.VALUE\\[23\\]
is set."]
    R23 = 14,
    #[doc = "13: Result bit 22: TDC conversion saturates and stops when RESULT.VALUE\\[22\\]
is set."]
    R22 = 13,
    #[doc = "12: Result bit 21: TDC conversion saturates and stops when RESULT.VALUE\\[21\\]
is set."]
    R21 = 12,
    #[doc = "11: Result bit 20: TDC conversion saturates and stops when RESULT.VALUE\\[20\\]
is set."]
    R20 = 11,
    #[doc = "10: Result bit 19: TDC conversion saturates and stops when RESULT.VALUE\\[19\\]
is set."]
    R19 = 10,
    #[doc = "9: Result bit 18: TDC conversion saturates and stops when RESULT.VALUE\\[18\\]
is set."]
    R18 = 9,
    #[doc = "8: Result bit 17: TDC conversion saturates and stops when RESULT.VALUE\\[17\\]
is set."]
    R17 = 8,
    #[doc = "7: Result bit 16: TDC conversion saturates and stops when RESULT.VALUE\\[16\\]
is set."]
    R16 = 7,
    #[doc = "6: Result bit 15: TDC conversion saturates and stops when RESULT.VALUE\\[15\\]
is set."]
    R15 = 6,
    #[doc = "5: Result bit 14: TDC conversion saturates and stops when RESULT.VALUE\\[14\\]
is set."]
    R14 = 5,
    #[doc = "4: Result bit 13: TDC conversion saturates and stops when RESULT.VALUE\\[13\\]
is set."]
    R13 = 4,
    #[doc = "3: Result bit 12: TDC conversion saturates and stops when RESULT.VALUE\\[12\\]
is set."]
    R12 = 3,
}
impl From<LIMIT_A> for u8 {
    #[inline(always)]
    fn from(variant: LIMIT_A) -> Self {
        variant as _
    }
}
impl LIMIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LIMIT_A> {
        match self.bits {
            15 => Some(LIMIT_A::R24),
            14 => Some(LIMIT_A::R23),
            13 => Some(LIMIT_A::R22),
            12 => Some(LIMIT_A::R21),
            11 => Some(LIMIT_A::R20),
            10 => Some(LIMIT_A::R19),
            9 => Some(LIMIT_A::R18),
            8 => Some(LIMIT_A::R17),
            7 => Some(LIMIT_A::R16),
            6 => Some(LIMIT_A::R15),
            5 => Some(LIMIT_A::R14),
            4 => Some(LIMIT_A::R13),
            3 => Some(LIMIT_A::R12),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `R24`"]
    #[inline(always)]
    pub fn is_r24(&self) -> bool {
        *self == LIMIT_A::R24
    }
    #[doc = "Checks if the value of the field is `R23`"]
    #[inline(always)]
    pub fn is_r23(&self) -> bool {
        *self == LIMIT_A::R23
    }
    #[doc = "Checks if the value of the field is `R22`"]
    #[inline(always)]
    pub fn is_r22(&self) -> bool {
        *self == LIMIT_A::R22
    }
    #[doc = "Checks if the value of the field is `R21`"]
    #[inline(always)]
    pub fn is_r21(&self) -> bool {
        *self == LIMIT_A::R21
    }
    #[doc = "Checks if the value of the field is `R20`"]
    #[inline(always)]
    pub fn is_r20(&self) -> bool {
        *self == LIMIT_A::R20
    }
    #[doc = "Checks if the value of the field is `R19`"]
    #[inline(always)]
    pub fn is_r19(&self) -> bool {
        *self == LIMIT_A::R19
    }
    #[doc = "Checks if the value of the field is `R18`"]
    #[inline(always)]
    pub fn is_r18(&self) -> bool {
        *self == LIMIT_A::R18
    }
    #[doc = "Checks if the value of the field is `R17`"]
    #[inline(always)]
    pub fn is_r17(&self) -> bool {
        *self == LIMIT_A::R17
    }
    #[doc = "Checks if the value of the field is `R16`"]
    #[inline(always)]
    pub fn is_r16(&self) -> bool {
        *self == LIMIT_A::R16
    }
    #[doc = "Checks if the value of the field is `R15`"]
    #[inline(always)]
    pub fn is_r15(&self) -> bool {
        *self == LIMIT_A::R15
    }
    #[doc = "Checks if the value of the field is `R14`"]
    #[inline(always)]
    pub fn is_r14(&self) -> bool {
        *self == LIMIT_A::R14
    }
    #[doc = "Checks if the value of the field is `R13`"]
    #[inline(always)]
    pub fn is_r13(&self) -> bool {
        *self == LIMIT_A::R13
    }
    #[doc = "Checks if the value of the field is `R12`"]
    #[inline(always)]
    pub fn is_r12(&self) -> bool {
        *self == LIMIT_A::R12
    }
}
#[doc = "Field `LIMIT` writer - 3:0\\]
Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported"]
pub type LIMIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SATCFG_SPEC, u8, LIMIT_A, 4, O>;
impl<'a, const O: u8> LIMIT_W<'a, O> {
    #[doc = "Result bit 24: TDC conversion saturates and stops when RESULT.VALUE\\[24\\]
is set."]
    #[inline(always)]
    pub fn r24(self) -> &'a mut W {
        self.variant(LIMIT_A::R24)
    }
    #[doc = "Result bit 23: TDC conversion saturates and stops when RESULT.VALUE\\[23\\]
is set."]
    #[inline(always)]
    pub fn r23(self) -> &'a mut W {
        self.variant(LIMIT_A::R23)
    }
    #[doc = "Result bit 22: TDC conversion saturates and stops when RESULT.VALUE\\[22\\]
is set."]
    #[inline(always)]
    pub fn r22(self) -> &'a mut W {
        self.variant(LIMIT_A::R22)
    }
    #[doc = "Result bit 21: TDC conversion saturates and stops when RESULT.VALUE\\[21\\]
is set."]
    #[inline(always)]
    pub fn r21(self) -> &'a mut W {
        self.variant(LIMIT_A::R21)
    }
    #[doc = "Result bit 20: TDC conversion saturates and stops when RESULT.VALUE\\[20\\]
is set."]
    #[inline(always)]
    pub fn r20(self) -> &'a mut W {
        self.variant(LIMIT_A::R20)
    }
    #[doc = "Result bit 19: TDC conversion saturates and stops when RESULT.VALUE\\[19\\]
is set."]
    #[inline(always)]
    pub fn r19(self) -> &'a mut W {
        self.variant(LIMIT_A::R19)
    }
    #[doc = "Result bit 18: TDC conversion saturates and stops when RESULT.VALUE\\[18\\]
is set."]
    #[inline(always)]
    pub fn r18(self) -> &'a mut W {
        self.variant(LIMIT_A::R18)
    }
    #[doc = "Result bit 17: TDC conversion saturates and stops when RESULT.VALUE\\[17\\]
is set."]
    #[inline(always)]
    pub fn r17(self) -> &'a mut W {
        self.variant(LIMIT_A::R17)
    }
    #[doc = "Result bit 16: TDC conversion saturates and stops when RESULT.VALUE\\[16\\]
is set."]
    #[inline(always)]
    pub fn r16(self) -> &'a mut W {
        self.variant(LIMIT_A::R16)
    }
    #[doc = "Result bit 15: TDC conversion saturates and stops when RESULT.VALUE\\[15\\]
is set."]
    #[inline(always)]
    pub fn r15(self) -> &'a mut W {
        self.variant(LIMIT_A::R15)
    }
    #[doc = "Result bit 14: TDC conversion saturates and stops when RESULT.VALUE\\[14\\]
is set."]
    #[inline(always)]
    pub fn r14(self) -> &'a mut W {
        self.variant(LIMIT_A::R14)
    }
    #[doc = "Result bit 13: TDC conversion saturates and stops when RESULT.VALUE\\[13\\]
is set."]
    #[inline(always)]
    pub fn r13(self) -> &'a mut W {
        self.variant(LIMIT_A::R13)
    }
    #[doc = "Result bit 12: TDC conversion saturates and stops when RESULT.VALUE\\[12\\]
is set."]
    #[inline(always)]
    pub fn r12(self) -> &'a mut W {
        self.variant(LIMIT_A::R12)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SATCFG_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported"]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported"]
    #[inline(always)]
    #[must_use]
    pub fn limit(&mut self) -> LIMIT_W<0> {
        LIMIT_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Saturation Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [satcfg](index.html) module"]
pub struct SATCFG_SPEC;
impl crate::RegisterSpec for SATCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [satcfg::R](R) reader structure"]
impl crate::Readable for SATCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [satcfg::W](W) writer structure"]
impl crate::Writable for SATCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SATCFG to value 0x0f"]
impl crate::Resettable for SATCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}

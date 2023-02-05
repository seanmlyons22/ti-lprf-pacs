#[doc = "Register `PERDMACLKDIV` reader"]
pub struct R(crate::R<PERDMACLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERDMACLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERDMACLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERDMACLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERDMACLKDIV` writer"]
pub struct W(crate::W<PERDMACLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERDMACLKDIV_SPEC>;
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
impl From<crate::W<PERDMACLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERDMACLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RATIO` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type RATIO_R = crate::FieldReader<u8, RATIO_A>;
#[doc = "3:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RATIO_A {
    #[doc = "8: Internal. Only to be used through TI provided API."]
    DIV256 = 8,
    #[doc = "7: Internal. Only to be used through TI provided API."]
    DIV128 = 7,
    #[doc = "6: Internal. Only to be used through TI provided API."]
    DIV64 = 6,
    #[doc = "5: Internal. Only to be used through TI provided API."]
    DIV32 = 5,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    DIV16 = 4,
    #[doc = "3: Internal. Only to be used through TI provided API."]
    DIV8 = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    DIV4 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    DIV2 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    DIV1 = 0,
}
impl From<RATIO_A> for u8 {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as _
    }
}
impl RATIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RATIO_A> {
        match self.bits {
            8 => Some(RATIO_A::DIV256),
            7 => Some(RATIO_A::DIV128),
            6 => Some(RATIO_A::DIV64),
            5 => Some(RATIO_A::DIV32),
            4 => Some(RATIO_A::DIV16),
            3 => Some(RATIO_A::DIV8),
            2 => Some(RATIO_A::DIV4),
            1 => Some(RATIO_A::DIV2),
            0 => Some(RATIO_A::DIV1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == RATIO_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == RATIO_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == RATIO_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == RATIO_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == RATIO_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == RATIO_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == RATIO_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RATIO_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == RATIO_A::DIV1
    }
}
#[doc = "Field `RATIO` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERDMACLKDIV_SPEC, u8, RATIO_A, 4, O>;
impl<'a, const O: u8> RATIO_W<'a, O> {
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(RATIO_A::DIV256)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(RATIO_A::DIV128)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(RATIO_A::DIV64)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(RATIO_A::DIV32)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(RATIO_A::DIV16)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(RATIO_A::DIV8)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(RATIO_A::DIV4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RATIO_A::DIV2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(RATIO_A::DIV1)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERDMACLKDIV_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RATIO_W<0> {
        RATIO_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perdmaclkdiv](index.html) module"]
pub struct PERDMACLKDIV_SPEC;
impl crate::RegisterSpec for PERDMACLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perdmaclkdiv::R](R) reader structure"]
impl crate::Readable for PERDMACLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perdmaclkdiv::W](W) writer structure"]
impl crate::Writable for PERDMACLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERDMACLKDIV to value 0"]
impl crate::Resettable for PERDMACLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

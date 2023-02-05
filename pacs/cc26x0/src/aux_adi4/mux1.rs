#[doc = "Register `MUX1` reader"]
pub struct R(crate::R<MUX1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX1` writer"]
pub struct W(crate::W<MUX1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX1_SPEC>;
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
impl From<crate::W<MUX1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPA_IN` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type COMPA_IN_R = crate::FieldReader<u8, COMPA_IN_A>;
#[doc = "7:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMPA_IN_A {
    #[doc = "128: Internal. Only to be used through TI provided API."]
    AUXIO0 = 128,
    #[doc = "64: Internal. Only to be used through TI provided API."]
    AUXIO1 = 64,
    #[doc = "32: Internal. Only to be used through TI provided API."]
    AUXIO2 = 32,
    #[doc = "16: Internal. Only to be used through TI provided API."]
    AUXIO3 = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    AUXIO4 = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    AUXIO5 = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    AUXIO6 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    AUXIO7 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    NC = 0,
}
impl From<COMPA_IN_A> for u8 {
    #[inline(always)]
    fn from(variant: COMPA_IN_A) -> Self {
        variant as _
    }
}
impl COMPA_IN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPA_IN_A> {
        match self.bits {
            128 => Some(COMPA_IN_A::AUXIO0),
            64 => Some(COMPA_IN_A::AUXIO1),
            32 => Some(COMPA_IN_A::AUXIO2),
            16 => Some(COMPA_IN_A::AUXIO3),
            8 => Some(COMPA_IN_A::AUXIO4),
            4 => Some(COMPA_IN_A::AUXIO5),
            2 => Some(COMPA_IN_A::AUXIO6),
            1 => Some(COMPA_IN_A::AUXIO7),
            0 => Some(COMPA_IN_A::NC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == COMPA_IN_A::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == COMPA_IN_A::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == COMPA_IN_A::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == COMPA_IN_A::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == COMPA_IN_A::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == COMPA_IN_A::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == COMPA_IN_A::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == COMPA_IN_A::AUXIO7
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == COMPA_IN_A::NC
    }
}
#[doc = "Field `COMPA_IN` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type COMPA_IN_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MUX1_SPEC, u8, COMPA_IN_A, 8, O>;
impl<'a, const O: u8> COMPA_IN_W<'a, O> {
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(COMPA_IN_A::AUXIO0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(COMPA_IN_A::AUXIO1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(COMPA_IN_A::AUXIO2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(COMPA_IN_A::AUXIO3)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(COMPA_IN_A::AUXIO4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(COMPA_IN_A::AUXIO5)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(COMPA_IN_A::AUXIO6)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(COMPA_IN_A::AUXIO7)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(COMPA_IN_A::NC)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn compa_in(&self) -> COMPA_IN_R {
        COMPA_IN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn compa_in(&mut self) -> COMPA_IN_W<0> {
        COMPA_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux1](index.html) module"]
pub struct MUX1_SPEC;
impl crate::RegisterSpec for MUX1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mux1::R](R) reader structure"]
impl crate::Readable for MUX1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux1::W](W) writer structure"]
impl crate::Writable for MUX1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUX1 to value 0"]
impl crate::Resettable for MUX1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

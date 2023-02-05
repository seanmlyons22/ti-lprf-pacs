#[doc = "Register `MUX4` reader"]
pub struct R(crate::R<MUX4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX4` writer"]
pub struct W(crate::W<MUX4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX4_SPEC>;
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
impl From<crate::W<MUX4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPA_REF` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type COMPA_REF_R = crate::FieldReader<u8, COMPA_REF_A>;
#[doc = "7:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMPA_REF_A {
    #[doc = "128: Internal. Only to be used through TI provided API."]
    AUXIO19 = 128,
    #[doc = "64: Internal. Only to be used through TI provided API."]
    AUXIO20 = 64,
    #[doc = "32: Internal. Only to be used through TI provided API."]
    AUXIO21 = 32,
    #[doc = "16: Internal. Only to be used through TI provided API."]
    AUXIO22 = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    AUXIO23 = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    AUXIO24 = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    AUXIO25 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    AUXIO26 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    NC = 0,
}
impl From<COMPA_REF_A> for u8 {
    #[inline(always)]
    fn from(variant: COMPA_REF_A) -> Self {
        variant as _
    }
}
impl COMPA_REF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPA_REF_A> {
        match self.bits {
            128 => Some(COMPA_REF_A::AUXIO19),
            64 => Some(COMPA_REF_A::AUXIO20),
            32 => Some(COMPA_REF_A::AUXIO21),
            16 => Some(COMPA_REF_A::AUXIO22),
            8 => Some(COMPA_REF_A::AUXIO23),
            4 => Some(COMPA_REF_A::AUXIO24),
            2 => Some(COMPA_REF_A::AUXIO25),
            1 => Some(COMPA_REF_A::AUXIO26),
            0 => Some(COMPA_REF_A::NC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUXIO19`"]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == COMPA_REF_A::AUXIO19
    }
    #[doc = "Checks if the value of the field is `AUXIO20`"]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == COMPA_REF_A::AUXIO20
    }
    #[doc = "Checks if the value of the field is `AUXIO21`"]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == COMPA_REF_A::AUXIO21
    }
    #[doc = "Checks if the value of the field is `AUXIO22`"]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == COMPA_REF_A::AUXIO22
    }
    #[doc = "Checks if the value of the field is `AUXIO23`"]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == COMPA_REF_A::AUXIO23
    }
    #[doc = "Checks if the value of the field is `AUXIO24`"]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == COMPA_REF_A::AUXIO24
    }
    #[doc = "Checks if the value of the field is `AUXIO25`"]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == COMPA_REF_A::AUXIO25
    }
    #[doc = "Checks if the value of the field is `AUXIO26`"]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == COMPA_REF_A::AUXIO26
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == COMPA_REF_A::NC
    }
}
#[doc = "Field `COMPA_REF` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type COMPA_REF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, MUX4_SPEC, u8, COMPA_REF_A, 8, O>;
impl<'a, const O: u8> COMPA_REF_W<'a, O> {
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut W {
        self.variant(COMPA_REF_A::AUXIO19)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut W {
        self.variant(COMPA_REF_A::AUXIO20)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut W {
        self.variant(COMPA_REF_A::AUXIO21)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut W {
        self.variant(COMPA_REF_A::AUXIO22)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut W {
        self.variant(COMPA_REF_A::AUXIO23)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut W {
        self.variant(COMPA_REF_A::AUXIO24)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut W {
        self.variant(COMPA_REF_A::AUXIO25)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut W {
        self.variant(COMPA_REF_A::AUXIO26)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(COMPA_REF_A::NC)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn compa_ref(&self) -> COMPA_REF_R {
        COMPA_REF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn compa_ref(&mut self) -> COMPA_REF_W<0> {
        COMPA_REF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux4](index.html) module"]
pub struct MUX4_SPEC;
impl crate::RegisterSpec for MUX4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mux4::R](R) reader structure"]
impl crate::Readable for MUX4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux4::W](W) writer structure"]
impl crate::Writable for MUX4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUX4 to value 0"]
impl crate::Resettable for MUX4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

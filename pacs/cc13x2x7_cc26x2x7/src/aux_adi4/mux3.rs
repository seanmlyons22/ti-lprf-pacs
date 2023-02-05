#[doc = "Register `MUX3` reader"]
pub struct R(crate::R<MUX3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX3` writer"]
pub struct W(crate::W<MUX3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX3_SPEC>;
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
impl From<crate::W<MUX3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCCOMPB_IN` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type ADCCOMPB_IN_R = crate::FieldReader<u8, ADCCOMPB_IN_A>;
#[doc = "7:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCCOMPB_IN_A {
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
impl From<ADCCOMPB_IN_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCCOMPB_IN_A) -> Self {
        variant as _
    }
}
impl ADCCOMPB_IN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCCOMPB_IN_A> {
        match self.bits {
            128 => Some(ADCCOMPB_IN_A::AUXIO19),
            64 => Some(ADCCOMPB_IN_A::AUXIO20),
            32 => Some(ADCCOMPB_IN_A::AUXIO21),
            16 => Some(ADCCOMPB_IN_A::AUXIO22),
            8 => Some(ADCCOMPB_IN_A::AUXIO23),
            4 => Some(ADCCOMPB_IN_A::AUXIO24),
            2 => Some(ADCCOMPB_IN_A::AUXIO25),
            1 => Some(ADCCOMPB_IN_A::AUXIO26),
            0 => Some(ADCCOMPB_IN_A::NC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUXIO19`"]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO19
    }
    #[doc = "Checks if the value of the field is `AUXIO20`"]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO20
    }
    #[doc = "Checks if the value of the field is `AUXIO21`"]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO21
    }
    #[doc = "Checks if the value of the field is `AUXIO22`"]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO22
    }
    #[doc = "Checks if the value of the field is `AUXIO23`"]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO23
    }
    #[doc = "Checks if the value of the field is `AUXIO24`"]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO24
    }
    #[doc = "Checks if the value of the field is `AUXIO25`"]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO25
    }
    #[doc = "Checks if the value of the field is `AUXIO26`"]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO26
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == ADCCOMPB_IN_A::NC
    }
}
#[doc = "Field `ADCCOMPB_IN` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type ADCCOMPB_IN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, MUX3_SPEC, u8, ADCCOMPB_IN_A, 8, O>;
impl<'a, const O: u8> ADCCOMPB_IN_W<'a, O> {
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO19)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO20)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO21)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO22)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO23)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO24)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO25)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO26)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::NC)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&self) -> ADCCOMPB_IN_R {
        ADCCOMPB_IN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adccompb_in(&mut self) -> ADCCOMPB_IN_W<0> {
        ADCCOMPB_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux3](index.html) module"]
pub struct MUX3_SPEC;
impl crate::RegisterSpec for MUX3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mux3::R](R) reader structure"]
impl crate::Readable for MUX3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux3::W](W) writer structure"]
impl crate::Writable for MUX3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUX3 to value 0"]
impl crate::Resettable for MUX3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

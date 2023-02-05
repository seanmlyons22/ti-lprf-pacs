#[doc = "Register `TRACECLKMUX` reader"]
pub struct R(crate::R<TRACECLKMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACECLKMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACECLKMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACECLKMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACECLKMUX` writer"]
pub struct W(crate::W<TRACECLKMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACECLKMUX_SPEC>;
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
impl From<crate::W<TRACECLKMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACECLKMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRACECLK_N_SWV` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type TRACECLK_N_SWV_R = crate::BitReader<TRACECLK_N_SWV_A>;
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRACECLK_N_SWV_A {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    TRACECLK = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    SWV = 0,
}
impl From<TRACECLK_N_SWV_A> for bool {
    #[inline(always)]
    fn from(variant: TRACECLK_N_SWV_A) -> Self {
        variant as u8 != 0
    }
}
impl TRACECLK_N_SWV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACECLK_N_SWV_A {
        match self.bits {
            true => TRACECLK_N_SWV_A::TRACECLK,
            false => TRACECLK_N_SWV_A::SWV,
        }
    }
    #[doc = "Checks if the value of the field is `TRACECLK`"]
    #[inline(always)]
    pub fn is_traceclk(&self) -> bool {
        *self == TRACECLK_N_SWV_A::TRACECLK
    }
    #[doc = "Checks if the value of the field is `SWV`"]
    #[inline(always)]
    pub fn is_swv(&self) -> bool {
        *self == TRACECLK_N_SWV_A::SWV
    }
}
#[doc = "Field `TRACECLK_N_SWV` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type TRACECLK_N_SWV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRACECLKMUX_SPEC, TRACECLK_N_SWV_A, O>;
impl<'a, const O: u8> TRACECLK_N_SWV_W<'a, O> {
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn traceclk(self) -> &'a mut W {
        self.variant(TRACECLK_N_SWV_A::TRACECLK)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn swv(self) -> &'a mut W {
        self.variant(TRACECLK_N_SWV_A::SWV)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRACECLKMUX_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn traceclk_n_swv(&self) -> TRACECLK_N_SWV_R {
        TRACECLK_N_SWV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn traceclk_n_swv(&mut self) -> TRACECLK_N_SWV_W<0> {
        TRACECLK_N_SWV_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceclkmux](index.html) module"]
pub struct TRACECLKMUX_SPEC;
impl crate::RegisterSpec for TRACECLKMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [traceclkmux::R](R) reader structure"]
impl crate::Readable for TRACECLKMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [traceclkmux::W](W) writer structure"]
impl crate::Writable for TRACECLKMUX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRACECLKMUX to value 0"]
impl crate::Resettable for TRACECLKMUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

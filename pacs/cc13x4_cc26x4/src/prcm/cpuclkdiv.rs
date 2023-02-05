#[doc = "Register `CPUCLKDIV` reader"]
pub struct R(crate::R<CPUCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUCLKDIV` writer"]
pub struct W(crate::W<CPUCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUCLKDIV_SPEC>;
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
impl From<crate::W<CPUCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RATIO` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type RATIO_R = crate::BitReader<RATIO_A>;
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RATIO_A {
    #[doc = "0: Internal. Only to be used through TI provided API."]
    DIV1 = 0,
}
impl From<RATIO_A> for bool {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as u8 != 0
    }
}
impl RATIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RATIO_A> {
        match self.bits {
            false => Some(RATIO_A::DIV1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == RATIO_A::DIV1
    }
}
#[doc = "Field `RATIO` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type RATIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUCLKDIV_SPEC, RATIO_A, O>;
impl<'a, const O: u8> RATIO_W<'a, O> {
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(RATIO_A::DIV1)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPUCLKDIV_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 1) != 0)
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
    pub fn ratio(&mut self) -> RATIO_W<0> {
        RATIO_W::new(self)
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuclkdiv](index.html) module"]
pub struct CPUCLKDIV_SPEC;
impl crate::RegisterSpec for CPUCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuclkdiv::R](R) reader structure"]
impl crate::Readable for CPUCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpuclkdiv::W](W) writer structure"]
impl crate::Writable for CPUCLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUCLKDIV to value 0"]
impl crate::Resettable for CPUCLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

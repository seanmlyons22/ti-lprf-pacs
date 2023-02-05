#[doc = "Register `SYSBUSCLKDIV` reader"]
pub struct R(crate::R<SYSBUSCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSBUSCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSBUSCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSBUSCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSBUSCLKDIV` writer"]
pub struct W(crate::W<SYSBUSCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSBUSCLKDIV_SPEC>;
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
impl From<crate::W<SYSBUSCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSBUSCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RATIO` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type RATIO_R = crate::FieldReader<u8, RATIO_A>;
#[doc = "2:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RATIO_A {
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
            1 => Some(RATIO_A::DIV2),
            0 => Some(RATIO_A::DIV1),
            _ => None,
        }
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
#[doc = "Field `RATIO` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSBUSCLKDIV_SPEC, u8, RATIO_A, 3, O>;
impl<'a, const O: u8> RATIO_W<'a, O> {
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
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSBUSCLKDIV_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RATIO_W<0> {
        RATIO_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysbusclkdiv](index.html) module"]
pub struct SYSBUSCLKDIV_SPEC;
impl crate::RegisterSpec for SYSBUSCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysbusclkdiv::R](R) reader structure"]
impl crate::Readable for SYSBUSCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysbusclkdiv::W](W) writer structure"]
impl crate::Writable for SYSBUSCLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSBUSCLKDIV to value 0"]
impl crate::Resettable for SYSBUSCLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

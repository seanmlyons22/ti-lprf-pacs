#[doc = "Register `WEPROT_B0_31_0_BY1` reader"]
pub struct R(crate::R<WEPROT_B0_31_0_BY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WEPROT_B0_31_0_BY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WEPROT_B0_31_0_BY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WEPROT_B0_31_0_BY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WEPROT_B0_31_0_BY1` writer"]
pub struct W(crate::W<WEPROT_B0_31_0_BY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WEPROT_B0_31_0_BY1_SPEC>;
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
impl From<crate::W<WEPROT_B0_31_0_BY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WEPROT_B0_31_0_BY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WEPROT_B0_31_0_BY1` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B0_31_0_BY1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WEPROT_B0_31_0_BY1` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B0_31_0_BY1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WEPROT_B0_31_0_BY1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b0_31_0_by1(&self) -> WEPROT_B0_31_0_BY1_R {
        WEPROT_B0_31_0_BY1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b0_31_0_by1(&mut self) -> WEPROT_B0_31_0_BY1_W<0> {
        WEPROT_B0_31_0_BY1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [weprot_b0_31_0_by1](index.html) module"]
pub struct WEPROT_B0_31_0_BY1_SPEC;
impl crate::RegisterSpec for WEPROT_B0_31_0_BY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [weprot_b0_31_0_by1::R](R) reader structure"]
impl crate::Readable for WEPROT_B0_31_0_BY1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [weprot_b0_31_0_by1::W](W) writer structure"]
impl crate::Writable for WEPROT_B0_31_0_BY1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WEPROT_B0_31_0_BY1 to value 0xffff_ffff"]
impl crate::Resettable for WEPROT_B0_31_0_BY1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

#[doc = "Register `FPRIM_ADD_TAG` reader"]
pub struct R(crate::R<FPRIM_ADD_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPRIM_ADD_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPRIM_ADD_TAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPRIM_ADD_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPRIM_ADD_TAG` writer"]
pub struct W(crate::W<FPRIM_ADD_TAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPRIM_ADD_TAG_SPEC>;
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
impl From<crate::W<FPRIM_ADD_TAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPRIM_ADD_TAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIM_ADD_TAG` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type PRIM_ADD_TAG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRIM_ADD_TAG` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type PRIM_ADD_TAG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FPRIM_ADD_TAG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn prim_add_tag(&self) -> PRIM_ADD_TAG_R {
        PRIM_ADD_TAG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn prim_add_tag(&mut self) -> PRIM_ADD_TAG_W<0> {
        PRIM_ADD_TAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprim_add_tag](index.html) module"]
pub struct FPRIM_ADD_TAG_SPEC;
impl crate::RegisterSpec for FPRIM_ADD_TAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fprim_add_tag::R](R) reader structure"]
impl crate::Readable for FPRIM_ADD_TAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fprim_add_tag::W](W) writer structure"]
impl crate::Writable for FPRIM_ADD_TAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPRIM_ADD_TAG to value 0"]
impl crate::Resettable for FPRIM_ADD_TAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `FUNC_ERR_ADD` reader"]
pub struct R(crate::R<FUNC_ERR_ADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC_ERR_ADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC_ERR_ADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC_ERR_ADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC_ERR_ADD` writer"]
pub struct W(crate::W<FUNC_ERR_ADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC_ERR_ADD_SPEC>;
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
impl From<crate::W<FUNC_ERR_ADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC_ERR_ADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNC_ERR_ADD` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FUNC_ERR_ADD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FUNC_ERR_ADD` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FUNC_ERR_ADD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FUNC_ERR_ADD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn func_err_add(&self) -> FUNC_ERR_ADD_R {
        FUNC_ERR_ADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn func_err_add(&mut self) -> FUNC_ERR_ADD_W<0> {
        FUNC_ERR_ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func_err_add](index.html) module"]
pub struct FUNC_ERR_ADD_SPEC;
impl crate::RegisterSpec for FUNC_ERR_ADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func_err_add::R](R) reader structure"]
impl crate::Readable for FUNC_ERR_ADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func_err_add::W](W) writer structure"]
impl crate::Writable for FUNC_ERR_ADD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUNC_ERR_ADD to value 0"]
impl crate::Resettable for FUNC_ERR_ADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

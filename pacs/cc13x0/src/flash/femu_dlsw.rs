#[doc = "Register `FEMU_DLSW` reader"]
pub struct R(crate::R<FEMU_DLSW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEMU_DLSW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEMU_DLSW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEMU_DLSW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEMU_DLSW` writer"]
pub struct W(crate::W<FEMU_DLSW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEMU_DLSW_SPEC>;
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
impl From<crate::W<FEMU_DLSW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEMU_DLSW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEMU_DLSW` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FEMU_DLSW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FEMU_DLSW` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FEMU_DLSW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FEMU_DLSW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn femu_dlsw(&self) -> FEMU_DLSW_R {
        FEMU_DLSW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn femu_dlsw(&mut self) -> FEMU_DLSW_W<0> {
        FEMU_DLSW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [femu_dlsw](index.html) module"]
pub struct FEMU_DLSW_SPEC;
impl crate::RegisterSpec for FEMU_DLSW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [femu_dlsw::R](R) reader structure"]
impl crate::Readable for FEMU_DLSW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [femu_dlsw::W](W) writer structure"]
impl crate::Writable for FEMU_DLSW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEMU_DLSW to value 0"]
impl crate::Resettable for FEMU_DLSW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

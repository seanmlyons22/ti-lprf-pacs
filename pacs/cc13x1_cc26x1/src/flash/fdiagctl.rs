#[doc = "Register `FDIAGCTL` reader"]
pub struct R(crate::R<FDIAGCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDIAGCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDIAGCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDIAGCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDIAGCTL` writer"]
pub struct W(crate::W<FDIAGCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDIAGCTL_SPEC>;
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
impl From<crate::W<FDIAGCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDIAGCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIAGMODE` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type DIAGMODE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DIAGMODE` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type DIAGMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDIAGCTL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn diagmode(&self) -> DIAGMODE_R {
        DIAGMODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn diagmode(&mut self) -> DIAGMODE_W<0> {
        DIAGMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdiagctl](index.html) module"]
pub struct FDIAGCTL_SPEC;
impl crate::RegisterSpec for FDIAGCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdiagctl::R](R) reader structure"]
impl crate::Readable for FDIAGCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdiagctl::W](W) writer structure"]
impl crate::Writable for FDIAGCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDIAGCTL to value 0"]
impl crate::Resettable for FDIAGCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

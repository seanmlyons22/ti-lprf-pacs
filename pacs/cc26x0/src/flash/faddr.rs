#[doc = "Register `FADDR` reader"]
pub struct R(crate::R<FADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FADDR` writer"]
pub struct W(crate::W<FADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FADDR_SPEC>;
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
impl From<crate::W<FADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FADDR` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FADDR` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn faddr(&self) -> FADDR_R {
        FADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn faddr(&mut self) -> FADDR_W<0> {
        FADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faddr](index.html) module"]
pub struct FADDR_SPEC;
impl crate::RegisterSpec for FADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [faddr::R](R) reader structure"]
impl crate::Readable for FADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [faddr::W](W) writer structure"]
impl crate::Writable for FADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FADDR to value 0"]
impl crate::Resettable for FADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

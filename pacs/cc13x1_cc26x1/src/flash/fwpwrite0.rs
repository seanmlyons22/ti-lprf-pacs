#[doc = "Register `FWPWRITE0` reader"]
pub struct R(crate::R<FWPWRITE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWPWRITE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWPWRITE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWPWRITE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWPWRITE0` writer"]
pub struct W(crate::W<FWPWRITE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWPWRITE0_SPEC>;
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
impl From<crate::W<FWPWRITE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWPWRITE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FWPWRITE0` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FWPWRITE0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FWPWRITE0` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FWPWRITE0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FWPWRITE0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwpwrite0(&self) -> FWPWRITE0_R {
        FWPWRITE0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fwpwrite0(&mut self) -> FWPWRITE0_W<0> {
        FWPWRITE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite0](index.html) module"]
pub struct FWPWRITE0_SPEC;
impl crate::RegisterSpec for FWPWRITE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwpwrite0::R](R) reader structure"]
impl crate::Readable for FWPWRITE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwpwrite0::W](W) writer structure"]
impl crate::Writable for FWPWRITE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FWPWRITE0 to value 0xffff_ffff"]
impl crate::Resettable for FWPWRITE0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

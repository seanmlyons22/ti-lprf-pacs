#[doc = "Register `FWPWRITE7` reader"]
pub struct R(crate::R<FWPWRITE7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWPWRITE7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWPWRITE7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWPWRITE7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWPWRITE7` writer"]
pub struct W(crate::W<FWPWRITE7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWPWRITE7_SPEC>;
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
impl From<crate::W<FWPWRITE7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWPWRITE7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FWPWRITE7` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FWPWRITE7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FWPWRITE7` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FWPWRITE7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FWPWRITE7_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwpwrite7(&self) -> FWPWRITE7_R {
        FWPWRITE7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fwpwrite7(&mut self) -> FWPWRITE7_W<0> {
        FWPWRITE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite7](index.html) module"]
pub struct FWPWRITE7_SPEC;
impl crate::RegisterSpec for FWPWRITE7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwpwrite7::R](R) reader structure"]
impl crate::Readable for FWPWRITE7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwpwrite7::W](W) writer structure"]
impl crate::Writable for FWPWRITE7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FWPWRITE7 to value 0xffff_ffff"]
impl crate::Resettable for FWPWRITE7_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

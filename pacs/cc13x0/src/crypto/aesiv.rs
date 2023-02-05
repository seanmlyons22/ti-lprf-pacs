#[doc = "Register `AESIV` reader"]
pub struct R(crate::R<AESIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESIV` writer"]
pub struct W(crate::W<AESIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESIV_SPEC>;
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
impl From<crate::W<AESIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV` reader - 31:0\\]
The interpretation of this field depends on the crypto operation mode."]
pub type IV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IV` writer - 31:0\\]
The interpretation of this field depends on the crypto operation mode."]
pub type IV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESIV_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn iv(&mut self) -> IV_W<0> {
        IV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Initialization Vector\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesiv](index.html) module"]
pub struct AESIV_SPEC;
impl crate::RegisterSpec for AESIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesiv::R](R) reader structure"]
impl crate::Readable for AESIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesiv::W](W) writer structure"]
impl crate::Writable for AESIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESIV to value 0"]
impl crate::Resettable for AESIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

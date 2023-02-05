#[doc = "Register `AESKEY2` reader"]
pub struct R(crate::R<AESKEY2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESKEY2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESKEY2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESKEY2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESKEY2` writer"]
pub struct W(crate::W<AESKEY2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESKEY2_SPEC>;
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
impl From<crate::W<AESKEY2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESKEY2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY2` reader - 31:0\\]
AESKEY2.* bits 31+x:0+x or AES_GHASH_H.* bits 31+x:0+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register array. The interpretation of this field depends on the crypto operation mode."]
pub type KEY2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEY2` writer - 31:0\\]
AESKEY2.* bits 31+x:0+x or AES_GHASH_H.* bits 31+x:0+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register array. The interpretation of this field depends on the crypto operation mode."]
pub type KEY2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESKEY2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
AESKEY2.* bits 31+x:0+x or AES_GHASH_H.* bits 31+x:0+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register array. The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    pub fn key2(&self) -> KEY2_R {
        KEY2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
AESKEY2.* bits 31+x:0+x or AES_GHASH_H.* bits 31+x:0+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register array. The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn key2(&mut self) -> KEY2_W<0> {
        KEY2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear AES_KEY2/GHASH Key\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aeskey2](index.html) module"]
pub struct AESKEY2_SPEC;
impl crate::RegisterSpec for AESKEY2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aeskey2::R](R) reader structure"]
impl crate::Readable for AESKEY2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aeskey2::W](W) writer structure"]
impl crate::Writable for AESKEY2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESKEY2 to value 0"]
impl crate::Resettable for AESKEY2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

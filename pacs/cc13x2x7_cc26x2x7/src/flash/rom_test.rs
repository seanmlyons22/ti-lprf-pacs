#[doc = "Register `ROM_TEST` reader"]
pub struct R(crate::R<ROM_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_TEST` writer"]
pub struct W(crate::W<ROM_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_TEST_SPEC>;
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
impl From<crate::W<ROM_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_KEY` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type ROM_KEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ROM_KEY` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type ROM_KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROM_TEST_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rom_key(&self) -> ROM_KEY_R {
        ROM_KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rom_key(&mut self) -> ROM_KEY_W<0> {
        ROM_KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_test](index.html) module"]
pub struct ROM_TEST_SPEC;
impl crate::RegisterSpec for ROM_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_test::R](R) reader structure"]
impl crate::Readable for ROM_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_test::W](W) writer structure"]
impl crate::Writable for ROM_TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROM_TEST to value 0"]
impl crate::Resettable for ROM_TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

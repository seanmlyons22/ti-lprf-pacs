#[doc = "Register `EFUSERELEASE` reader"]
pub struct R(crate::R<EFUSERELEASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSERELEASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSERELEASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSERELEASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSERELEASE` writer"]
pub struct W(crate::W<EFUSERELEASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSERELEASE_SPEC>;
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
impl From<crate::W<EFUSERELEASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSERELEASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EFUSEDAY` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type EFUSEDAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFUSEDAY` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type EFUSEDAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSERELEASE_SPEC, u8, u8, 5, O>;
#[doc = "Field `EFUSEMONTH` reader - 8:5\\]
Internal. Only to be used through TI provided API."]
pub type EFUSEMONTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFUSEMONTH` writer - 8:5\\]
Internal. Only to be used through TI provided API."]
pub type EFUSEMONTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EFUSERELEASE_SPEC, u8, u8, 4, O>;
#[doc = "Field `EFUSEYEAR` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type EFUSEYEAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFUSEYEAR` writer - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type EFUSEYEAR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EFUSERELEASE_SPEC, u8, u8, 7, O>;
#[doc = "Field `ODPDAY` reader - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type ODPDAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ODPDAY` writer - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type ODPDAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSERELEASE_SPEC, u8, u8, 5, O>;
#[doc = "Field `ODPMONTH` reader - 24:21\\]
Internal. Only to be used through TI provided API."]
pub type ODPMONTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ODPMONTH` writer - 24:21\\]
Internal. Only to be used through TI provided API."]
pub type ODPMONTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSERELEASE_SPEC, u8, u8, 4, O>;
#[doc = "Field `ODPYEAR` reader - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type ODPYEAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ODPYEAR` writer - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type ODPYEAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSERELEASE_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuseday(&self) -> EFUSEDAY_R {
        EFUSEDAY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efusemonth(&self) -> EFUSEMONTH_R {
        EFUSEMONTH_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuseyear(&self) -> EFUSEYEAR_R {
        EFUSEYEAR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpday(&self) -> ODPDAY_R {
        ODPDAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpmonth(&self) -> ODPMONTH_R {
        ODPMONTH_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpyear(&self) -> ODPYEAR_R {
        ODPYEAR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efuseday(&mut self) -> EFUSEDAY_W<0> {
        EFUSEDAY_W::new(self)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efusemonth(&mut self) -> EFUSEMONTH_W<5> {
        EFUSEMONTH_W::new(self)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efuseyear(&mut self) -> EFUSEYEAR_W<9> {
        EFUSEYEAR_W::new(self)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn odpday(&mut self) -> ODPDAY_W<16> {
        ODPDAY_W::new(self)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn odpmonth(&mut self) -> ODPMONTH_W<21> {
        ODPMONTH_W::new(self)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn odpyear(&mut self) -> ODPYEAR_W<25> {
        ODPYEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuserelease](index.html) module"]
pub struct EFUSERELEASE_SPEC;
impl crate::RegisterSpec for EFUSERELEASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuserelease::R](R) reader structure"]
impl crate::Readable for EFUSERELEASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuserelease::W](W) writer structure"]
impl crate::Writable for EFUSERELEASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFUSERELEASE to value 0"]
impl crate::Resettable for EFUSERELEASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

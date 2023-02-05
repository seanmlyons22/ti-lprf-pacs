#[doc = "Register `LPMBIAS` reader"]
pub struct R(crate::R<LPMBIAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMBIAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMBIAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMBIAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMBIAS` writer"]
pub struct W(crate::W<LPMBIAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMBIAS_SPEC>;
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
impl From<crate::W<LPMBIAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMBIAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPM_TRIM_IOUT` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type LPM_TRIM_IOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPM_TRIM_IOUT` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type LPM_TRIM_IOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LPMBIAS_SPEC, u8, u8, 6, O>;
#[doc = "Field `SPARE6` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type SPARE6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPARE6` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type SPARE6_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LPMBIAS_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_trim_iout(&self) -> LPM_TRIM_IOUT_R {
        LPM_TRIM_IOUT_R::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare6(&self) -> SPARE6_R {
        SPARE6_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_trim_iout(&mut self) -> LPM_TRIM_IOUT_W<0> {
        LPM_TRIM_IOUT_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn spare6(&mut self) -> SPARE6_W<6> {
        SPARE6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmbias](index.html) module"]
pub struct LPMBIAS_SPEC;
impl crate::RegisterSpec for LPMBIAS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lpmbias::R](R) reader structure"]
impl crate::Readable for LPMBIAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmbias::W](W) writer structure"]
impl crate::Writable for LPMBIAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMBIAS to value 0"]
impl crate::Resettable for LPMBIAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

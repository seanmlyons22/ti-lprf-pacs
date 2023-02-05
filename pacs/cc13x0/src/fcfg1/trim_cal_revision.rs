#[doc = "Register `TRIM_CAL_REVISION` reader"]
pub struct R(crate::R<TRIM_CAL_REVISION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_CAL_REVISION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_CAL_REVISION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_CAL_REVISION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_CAL_REVISION` writer"]
pub struct W(crate::W<TRIM_CAL_REVISION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_CAL_REVISION_SPEC>;
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
impl From<crate::W<TRIM_CAL_REVISION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_CAL_REVISION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MP1` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type MP1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MP1` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type MP1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIM_CAL_REVISION_SPEC, u16, u16, 16, O>;
#[doc = "Field `FT1` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type FT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FT1` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type FT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIM_CAL_REVISION_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mp1(&self) -> MP1_R {
        MP1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ft1(&self) -> FT1_R {
        FT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn mp1(&mut self) -> MP1_W<0> {
        MP1_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ft1(&mut self) -> FT1_W<16> {
        FT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_cal_revision](index.html) module"]
pub struct TRIM_CAL_REVISION_SPEC;
impl crate::RegisterSpec for TRIM_CAL_REVISION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_cal_revision::R](R) reader structure"]
impl crate::Readable for TRIM_CAL_REVISION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_cal_revision::W](W) writer structure"]
impl crate::Writable for TRIM_CAL_REVISION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIM_CAL_REVISION to value 0"]
impl crate::Resettable for TRIM_CAL_REVISION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

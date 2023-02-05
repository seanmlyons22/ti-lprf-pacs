#[doc = "Register `TFW_PROBE` reader"]
pub struct R(crate::R<TFW_PROBE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFW_PROBE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFW_PROBE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFW_PROBE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFW_PROBE` writer"]
pub struct W(crate::W<TFW_PROBE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFW_PROBE_SPEC>;
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
impl From<crate::W<TFW_PROBE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFW_PROBE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REV` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type REV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REV` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type REV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TFW_PROBE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rev(&mut self) -> REV_W<0> {
        REV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfw_probe](index.html) module"]
pub struct TFW_PROBE_SPEC;
impl crate::RegisterSpec for TFW_PROBE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfw_probe::R](R) reader structure"]
impl crate::Readable for TFW_PROBE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tfw_probe::W](W) writer structure"]
impl crate::Writable for TFW_PROBE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFW_PROBE to value 0"]
impl crate::Resettable for TFW_PROBE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

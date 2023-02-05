#[doc = "Register `FSWSTAT` reader"]
pub struct R(crate::R<FSWSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSWSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSWSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSWSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSWSTAT` writer"]
pub struct W(crate::W<FSWSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSWSTAT_SPEC>;
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
impl From<crate::W<FSWSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSWSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAFELV` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type SAFELV_R = crate::BitReader<bool>;
#[doc = "Field `SAFELV` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type SAFELV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSWSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSWSTAT_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn safelv(&self) -> SAFELV_R {
        SAFELV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn safelv(&mut self) -> SAFELV_W<0> {
        SAFELV_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fswstat](index.html) module"]
pub struct FSWSTAT_SPEC;
impl crate::RegisterSpec for FSWSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fswstat::R](R) reader structure"]
impl crate::Readable for FSWSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fswstat::W](W) writer structure"]
impl crate::Writable for FSWSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSWSTAT to value 0x01"]
impl crate::Resettable for FSWSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

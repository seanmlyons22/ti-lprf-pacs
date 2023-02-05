#[doc = "Register `SYSCODE_START` reader"]
pub struct R(crate::R<SYSCODE_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCODE_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCODE_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCODE_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCODE_START` writer"]
pub struct W(crate::W<SYSCODE_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCODE_START_SPEC>;
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
impl From<crate::W<SYSCODE_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCODE_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCODE_START` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type SYSCODE_START_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCODE_START` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type SYSCODE_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSCODE_START_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED5` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED5` writer - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSCODE_START_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn syscode_start(&self) -> SYSCODE_START_R {
        SYSCODE_START_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn syscode_start(&mut self) -> SYSCODE_START_W<0> {
        SYSCODE_START_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<6> {
        RESERVED5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscode_start](index.html) module"]
pub struct SYSCODE_START_SPEC;
impl crate::RegisterSpec for SYSCODE_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscode_start::R](R) reader structure"]
impl crate::Readable for SYSCODE_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscode_start::W](W) writer structure"]
impl crate::Writable for SYSCODE_START_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCODE_START to value 0"]
impl crate::Resettable for SYSCODE_START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

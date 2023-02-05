#[doc = "Register `EFUSEFLAG` reader"]
pub struct R(crate::R<EFUSEFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSEFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSEFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSEFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSEFLAG` writer"]
pub struct W(crate::W<EFUSEFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSEFLAG_SPEC>;
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
impl From<crate::W<EFUSEFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSEFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type KEY_R = crate::BitReader<bool>;
#[doc = "Field `KEY` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type KEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSEFLAG_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EFUSEFLAG_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 1) != 0)
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
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuseflag](index.html) module"]
pub struct EFUSEFLAG_SPEC;
impl crate::RegisterSpec for EFUSEFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuseflag::R](R) reader structure"]
impl crate::Readable for EFUSEFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuseflag::W](W) writer structure"]
impl crate::Writable for EFUSEFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFUSEFLAG to value 0"]
impl crate::Resettable for EFUSEFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `EFUSEADDR` reader"]
pub struct R(crate::R<EFUSEADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSEADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSEADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSEADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSEADDR` writer"]
pub struct W(crate::W<EFUSEADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSEADDR_SPEC>;
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
impl From<crate::W<EFUSEADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSEADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROW` reader - 10:0\\]
Internal. Only to be used through TI provided API."]
pub type ROW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ROW` writer - 10:0\\]
Internal. Only to be used through TI provided API."]
pub type ROW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSEADDR_SPEC, u16, u16, 11, O>;
#[doc = "Field `BLOCK` reader - 15:11\\]
Internal. Only to be used through TI provided API."]
pub type BLOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLOCK` writer - 15:11\\]
Internal. Only to be used through TI provided API."]
pub type BLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSEADDR_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EFUSEADDR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn row(&self) -> ROW_R {
        ROW_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn row(&mut self) -> ROW_W<0> {
        ROW_W::new(self)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BLOCK_W<11> {
        BLOCK_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuseaddr](index.html) module"]
pub struct EFUSEADDR_SPEC;
impl crate::RegisterSpec for EFUSEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuseaddr::R](R) reader structure"]
impl crate::Readable for EFUSEADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuseaddr::W](W) writer structure"]
impl crate::Writable for EFUSEADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFUSEADDR to value 0"]
impl crate::Resettable for EFUSEADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

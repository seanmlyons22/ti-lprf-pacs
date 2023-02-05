#[doc = "Register `EFUSEERROR` reader"]
pub struct R(crate::R<EFUSEERROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSEERROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSEERROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSEERROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSEERROR` writer"]
pub struct W(crate::W<EFUSEERROR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSEERROR_SPEC>;
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
impl From<crate::W<EFUSEERROR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSEERROR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODE` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CODE` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSEERROR_SPEC, u8, u8, 5, O>;
#[doc = "Field `DONE` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSEERROR_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EFUSEERROR_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CODE_W<0> {
        CODE_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<5> {
        DONE_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuseerror](index.html) module"]
pub struct EFUSEERROR_SPEC;
impl crate::RegisterSpec for EFUSEERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuseerror::R](R) reader structure"]
impl crate::Readable for EFUSEERROR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuseerror::W](W) writer structure"]
impl crate::Writable for EFUSEERROR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFUSEERROR to value 0"]
impl crate::Resettable for EFUSEERROR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

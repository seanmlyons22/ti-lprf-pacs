#[doc = "Register `IOSTRMAX` reader"]
pub struct R(crate::R<IOSTRMAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOSTRMAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOSTRMAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOSTRMAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOSTRMAX` writer"]
pub struct W(crate::W<IOSTRMAX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOSTRMAX_SPEC>;
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
impl From<crate::W<IOSTRMAX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOSTRMAX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GRAY_CODE` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type GRAY_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GRAY_CODE` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type GRAY_CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOSTRMAX_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOSTRMAX_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gray_code(&self) -> GRAY_CODE_R {
        GRAY_CODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn gray_code(&mut self) -> GRAY_CODE_W<0> {
        GRAY_CODE_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iostrmax](index.html) module"]
pub struct IOSTRMAX_SPEC;
impl crate::RegisterSpec for IOSTRMAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iostrmax::R](R) reader structure"]
impl crate::Readable for IOSTRMAX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iostrmax::W](W) writer structure"]
impl crate::Writable for IOSTRMAX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOSTRMAX to value 0x05"]
impl crate::Resettable for IOSTRMAX_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}

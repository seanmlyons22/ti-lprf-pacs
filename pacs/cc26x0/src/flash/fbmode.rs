#[doc = "Register `FBMODE` reader"]
pub struct R(crate::R<FBMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBMODE` writer"]
pub struct W(crate::W<FBMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBMODE_SPEC>;
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
impl From<crate::W<FBMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBMODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBMODE_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
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
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbmode](index.html) module"]
pub struct FBMODE_SPEC;
impl crate::RegisterSpec for FBMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbmode::R](R) reader structure"]
impl crate::Readable for FBMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbmode::W](W) writer structure"]
impl crate::Writable for FBMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBMODE to value 0"]
impl crate::Resettable for FBMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

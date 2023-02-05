#[doc = "Register `FWFLAG` reader"]
pub struct R(crate::R<FWFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWFLAG` writer"]
pub struct W(crate::W<FWFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWFLAG_SPEC>;
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
impl From<crate::W<FWFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FWFLAG` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type FWFLAG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FWFLAG` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type FWFLAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FWFLAG_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FWFLAG_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwflag(&self) -> FWFLAG_R {
        FWFLAG_R::new((self.bits & 7) as u8)
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
    pub fn fwflag(&mut self) -> FWFLAG_W<0> {
        FWFLAG_W::new(self)
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwflag](index.html) module"]
pub struct FWFLAG_SPEC;
impl crate::RegisterSpec for FWFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwflag::R](R) reader structure"]
impl crate::Readable for FWFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwflag::W](W) writer structure"]
impl crate::Writable for FWFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FWFLAG to value 0"]
impl crate::Resettable for FWFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

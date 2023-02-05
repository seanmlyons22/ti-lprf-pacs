#[doc = "Register `FVREADCT` reader"]
pub struct R(crate::R<FVREADCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FVREADCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FVREADCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FVREADCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FVREADCT` writer"]
pub struct W(crate::W<FVREADCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FVREADCT_SPEC>;
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
impl From<crate::W<FVREADCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FVREADCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREADCT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VREADCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREADCT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VREADCT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVREADCT_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVREADCT_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vreadct(&self) -> VREADCT_R {
        VREADCT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vreadct(&mut self) -> VREADCT_W<0> {
        VREADCT_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvreadct](index.html) module"]
pub struct FVREADCT_SPEC;
impl crate::RegisterSpec for FVREADCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fvreadct::R](R) reader structure"]
impl crate::Readable for FVREADCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fvreadct::W](W) writer structure"]
impl crate::Writable for FVREADCT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FVREADCT to value 0x08"]
impl crate::Resettable for FVREADCT_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}

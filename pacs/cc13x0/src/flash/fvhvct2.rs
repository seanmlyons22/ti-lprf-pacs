#[doc = "Register `FVHVCT2` reader"]
pub struct R(crate::R<FVHVCT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FVHVCT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FVHVCT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FVHVCT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FVHVCT2` writer"]
pub struct W(crate::W<FVHVCT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FVHVCT2_SPEC>;
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
impl From<crate::W<FVHVCT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FVHVCT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED0` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVHVCT2_SPEC, u16, u16, 16, O>;
#[doc = "Field `VHVCT_P` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VHVCT_P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VHVCT_P` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VHVCT_P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVHVCT2_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIM13_P` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type TRIM13_P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM13_P` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type TRIM13_P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVHVCT2_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVHVCT2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_p(&self) -> VHVCT_P_R {
        VHVCT_P_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_p(&self) -> TRIM13_P_R {
        TRIM13_P_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhvct_p(&mut self) -> VHVCT_P_W<16> {
        VHVCT_P_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim13_p(&mut self) -> TRIM13_P_W<20> {
        TRIM13_P_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvhvct2](index.html) module"]
pub struct FVHVCT2_SPEC;
impl crate::RegisterSpec for FVHVCT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fvhvct2::R](R) reader structure"]
impl crate::Readable for FVHVCT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fvhvct2::W](W) writer structure"]
impl crate::Writable for FVHVCT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FVHVCT2 to value 0x00a2_0000"]
impl crate::Resettable for FVHVCT2_SPEC {
    const RESET_VALUE: Self::Ux = 0x00a2_0000;
}

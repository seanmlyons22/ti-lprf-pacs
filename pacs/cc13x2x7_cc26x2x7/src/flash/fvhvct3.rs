#[doc = "Register `FVHVCT3` reader"]
pub struct R(crate::R<FVHVCT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FVHVCT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FVHVCT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FVHVCT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FVHVCT3` writer"]
pub struct W(crate::W<FVHVCT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FVHVCT3_SPEC>;
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
impl From<crate::W<FVHVCT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FVHVCT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VHVCT_READ` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VHVCT_READ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VHVCT_READ` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VHVCT_READ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVHVCT3_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED4` reader - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED4` writer - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVHVCT3_SPEC, u16, u16, 12, O>;
#[doc = "Field `WCT` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type WCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WCT` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type WCT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVHVCT3_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED20_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FVHVCT3_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_read(&self) -> VHVCT_READ_R {
        VHVCT_READ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wct(&self) -> WCT_R {
        WCT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhvct_read(&mut self) -> VHVCT_READ_W<0> {
        VHVCT_READ_W::new(self)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wct(&mut self) -> WCT_W<16> {
        WCT_W::new(self)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> RESERVED20_W<20> {
        RESERVED20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvhvct3](index.html) module"]
pub struct FVHVCT3_SPEC;
impl crate::RegisterSpec for FVHVCT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fvhvct3::R](R) reader structure"]
impl crate::Readable for FVHVCT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fvhvct3::W](W) writer structure"]
impl crate::Writable for FVHVCT3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FVHVCT3 to value 0x000f_0000"]
impl crate::Resettable for FVHVCT3_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_0000;
}

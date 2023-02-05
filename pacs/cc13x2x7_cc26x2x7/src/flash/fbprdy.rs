#[doc = "Register `FBPRDY` reader"]
pub struct R(crate::R<FBPRDY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBPRDY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBPRDY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBPRDY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBPRDY` writer"]
pub struct W(crate::W<FBPRDY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBPRDY_SPEC>;
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
impl From<crate::W<FBPRDY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBPRDY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BANKRDY` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type BANKRDY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANKRDY` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type BANKRDY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBPRDY_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED3` reader - 14:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED3` writer - 14:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBPRDY_SPEC, u16, u16, 13, O>;
#[doc = "Field `PUMPRDY` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type PUMPRDY_R = crate::BitReader<bool>;
#[doc = "Field `PUMPRDY` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type PUMPRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBPRDY_SPEC, bool, O>;
#[doc = "Field `BANKBUSY` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type BANKBUSY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANKBUSY` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type BANKBUSY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBPRDY_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED19` reader - 31:18\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED19_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED19` writer - 31:18\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBPRDY_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankrdy(&self) -> BANKRDY_R {
        BANKRDY_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:14 - 14:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 2) & 0x1fff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumprdy(&self) -> PUMPRDY_R {
        PUMPRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankbusy(&self) -> BANKBUSY_R {
        BANKBUSY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankrdy(&mut self) -> BANKRDY_W<0> {
        BANKRDY_W::new(self)
    }
    #[doc = "Bits 2:14 - 14:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<2> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pumprdy(&mut self) -> PUMPRDY_W<15> {
        PUMPRDY_W::new(self)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankbusy(&mut self) -> BANKBUSY_W<16> {
        BANKBUSY_W::new(self)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> RESERVED19_W<18> {
        RESERVED19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbprdy](index.html) module"]
pub struct FBPRDY_SPEC;
impl crate::RegisterSpec for FBPRDY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbprdy::R](R) reader structure"]
impl crate::Readable for FBPRDY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbprdy::W](W) writer structure"]
impl crate::Writable for FBPRDY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBPRDY to value 0x00ff_00fc"]
impl crate::Resettable for FBPRDY_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_00fc;
}

#[doc = "Register `FCFG_BNK_TYPE` reader"]
pub struct R(crate::R<FCFG_BNK_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG_BNK_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG_BNK_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG_BNK_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFG_BNK_TYPE` writer"]
pub struct W(crate::W<FCFG_BNK_TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFG_BNK_TYPE_SPEC>;
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
impl From<crate::W<FCFG_BNK_TYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFG_BNK_TYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B0_TYPE` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type B0_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B0_TYPE` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type B0_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFG_BNK_TYPE_SPEC, u8, u8, 4, O>;
#[doc = "Field `B1_TYPE` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type B1_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B1_TYPE` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type B1_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFG_BNK_TYPE_SPEC, u8, u8, 4, O>;
#[doc = "Field `B2_TYPE` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type B2_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B2_TYPE` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type B2_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFG_BNK_TYPE_SPEC, u8, u8, 4, O>;
#[doc = "Field `B3_TYPE` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type B3_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B3_TYPE` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type B3_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFG_BNK_TYPE_SPEC, u8, u8, 4, O>;
#[doc = "Field `B4_TYPE` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type B4_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B4_TYPE` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type B4_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFG_BNK_TYPE_SPEC, u8, u8, 4, O>;
#[doc = "Field `B5_TYPE` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type B5_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B5_TYPE` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type B5_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFG_BNK_TYPE_SPEC, u8, u8, 4, O>;
#[doc = "Field `B6_TYPE` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B6_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B6_TYPE` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B6_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFG_BNK_TYPE_SPEC, u8, u8, 4, O>;
#[doc = "Field `B7_TYPE` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B7_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B7_TYPE` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B7_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFG_BNK_TYPE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_type(&self) -> B0_TYPE_R {
        B0_TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b1_type(&self) -> B1_TYPE_R {
        B1_TYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_type(&self) -> B2_TYPE_R {
        B2_TYPE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b3_type(&self) -> B3_TYPE_R {
        B3_TYPE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b4_type(&self) -> B4_TYPE_R {
        B4_TYPE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b5_type(&self) -> B5_TYPE_R {
        B5_TYPE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b6_type(&self) -> B6_TYPE_R {
        B6_TYPE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b7_type(&self) -> B7_TYPE_R {
        B7_TYPE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b0_type(&mut self) -> B0_TYPE_W<0> {
        B0_TYPE_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b1_type(&mut self) -> B1_TYPE_W<4> {
        B1_TYPE_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b2_type(&mut self) -> B2_TYPE_W<8> {
        B2_TYPE_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b3_type(&mut self) -> B3_TYPE_W<12> {
        B3_TYPE_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b4_type(&mut self) -> B4_TYPE_W<16> {
        B4_TYPE_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b5_type(&mut self) -> B5_TYPE_W<20> {
        B5_TYPE_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b6_type(&mut self) -> B6_TYPE_W<24> {
        B6_TYPE_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b7_type(&mut self) -> B7_TYPE_W<28> {
        B7_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_bnk_type](index.html) module"]
pub struct FCFG_BNK_TYPE_SPEC;
impl crate::RegisterSpec for FCFG_BNK_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg_bnk_type::R](R) reader structure"]
impl crate::Readable for FCFG_BNK_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfg_bnk_type::W](W) writer structure"]
impl crate::Writable for FCFG_BNK_TYPE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCFG_BNK_TYPE to value 0x04"]
impl crate::Resettable for FCFG_BNK_TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}

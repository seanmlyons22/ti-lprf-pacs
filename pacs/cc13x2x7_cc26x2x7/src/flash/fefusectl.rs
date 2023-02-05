#[doc = "Register `FEFUSECTL` reader"]
pub struct R(crate::R<FEFUSECTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEFUSECTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEFUSECTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEFUSECTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEFUSECTL` writer"]
pub struct W(crate::W<FEFUSECTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEFUSECTL_SPEC>;
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
impl From<crate::W<FEFUSECTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEFUSECTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EFUSE_EN` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type EFUSE_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFUSE_EN` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type EFUSE_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FEFUSECTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `EF_TEST` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type EF_TEST_R = crate::BitReader<bool>;
#[doc = "Field `EF_TEST` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type EF_TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEFUSECTL_SPEC, bool, O>;
#[doc = "Field `RESERVED5` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FEFUSECTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `EF_CLRZ` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type EF_CLRZ_R = crate::BitReader<bool>;
#[doc = "Field `EF_CLRZ` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type EF_CLRZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEFUSECTL_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FEFUSECTL_SPEC, u8, u8, 7, O>;
#[doc = "Field `BP_SEL` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type BP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `BP_SEL` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type BP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEFUSECTL_SPEC, bool, O>;
#[doc = "Field `WRITE_EN` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type WRITE_EN_R = crate::BitReader<bool>;
#[doc = "Field `WRITE_EN` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type WRITE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEFUSECTL_SPEC, bool, O>;
#[doc = "Field `RESERVED18` reader - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED18` writer - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FEFUSECTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `CHAIN_SEL` reader - 26:24\\]
Internal. Only to be used through TI provided API."]
pub type CHAIN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHAIN_SEL` writer - 26:24\\]
Internal. Only to be used through TI provided API."]
pub type CHAIN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FEFUSECTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED27` reader - 31:27\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED27` writer - 31:27\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED27_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FEFUSECTL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuse_en(&self) -> EFUSE_EN_R {
        EFUSE_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ef_test(&self) -> EF_TEST_R {
        EF_TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ef_clrz(&self) -> EF_CLRZ_R {
        EF_CLRZ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bp_sel(&self) -> BP_SEL_R {
        BP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn write_en(&self) -> WRITE_EN_R {
        WRITE_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn chain_sel(&self) -> CHAIN_SEL_R {
        CHAIN_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved27(&self) -> RESERVED27_R {
        RESERVED27_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efuse_en(&mut self) -> EFUSE_EN_W<0> {
        EFUSE_EN_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ef_test(&mut self) -> EF_TEST_W<4> {
        EF_TEST_W::new(self)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ef_clrz(&mut self) -> EF_CLRZ_W<8> {
        EF_CLRZ_W::new(self)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bp_sel(&mut self) -> BP_SEL_W<16> {
        BP_SEL_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn write_en(&mut self) -> WRITE_EN_W<17> {
        WRITE_EN_W::new(self)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> RESERVED18_W<18> {
        RESERVED18_W::new(self)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn chain_sel(&mut self) -> CHAIN_SEL_W<24> {
        CHAIN_SEL_W::new(self)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved27(&mut self) -> RESERVED27_W<27> {
        RESERVED27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fefusectl](index.html) module"]
pub struct FEFUSECTL_SPEC;
impl crate::RegisterSpec for FEFUSECTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fefusectl::R](R) reader structure"]
impl crate::Readable for FEFUSECTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fefusectl::W](W) writer structure"]
impl crate::Writable for FEFUSECTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEFUSECTL to value 0x0701_010a"]
impl crate::Resettable for FEFUSECTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0701_010a;
}
